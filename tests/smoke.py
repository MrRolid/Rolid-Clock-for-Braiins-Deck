import datetime as dt
import json
import struct as s
from pathlib import Path
from zoneinfo import ZoneInfo
import wasmtime as w

engine=w.Engine()
module=w.Module.from_file(engine,str(Path(__file__).parents[1]/'target/wasm32-unknown-unknown/release/rolid_clock.wasm'))
def value(v):
    if v is None:return 4,b''
    if isinstance(v,bool):return 3,s.pack('<?',v)
    if isinstance(v,int):return 1,s.pack('<i',v)
    if isinstance(v,list):return 5,s.pack('<I',len(v))+b''.join(bytes([k])+b for k,b in map(value,v))
    if isinstance(v,dict):return 6,packed(v)
    b=v.encode();return 0,s.pack('<I',len(b))+b

def packed(values):
    out=s.pack('<I',len(values))
    for key,v in values.items():
        k,b=value(v);key=key.encode();out+=s.pack('<BH',k,len(key))+key+b
    return out

def timestamp(text):return int(dt.datetime.fromisoformat(text).timestamp())

class Device:
    def __init__(self,width=1280,height=480,params=None,kv=None,now=None,system_alarm=None):
        self.width,self.height=width,height;self.params={"use_audio_service":False,**(params or {})};self.pv=1;self.sv=1
        self.now=now if now is not None else timestamp('2026-11-01T12:59:59+01:00')
        self.kv=kv if kv is not None else {};self.tree=b'';self.leds=[];self.led_stops=0;self.sounds=[];self.stops=[];self.audio={};self.delay=0;self.requests=0;self.click=None;self.timer=0;self.timer_count=0;self.fetches=[];self.night=False;self.system_alarm=system_alarm
        self.store=w.Store(engine);linker=w.Linker(engine)
        for i in module.imports:linker.define_func(i.module,i.name,i.type,self.callback(i.name),access_caller=True)
        self.instance=linker.instantiate(self.store,module);self.e=self.instance.exports(self.store)
        assert self.call('__bmc_sdk_init')==9<<16
        self.call('init')
    def call(self,name,*args):return self.e[name](self.store,*args)
    def render(self):self.call('render',1000);return self.tree
    def tick(self,seconds=1):self.now+=seconds;self.call('__on_timer',self.timer,seconds)
    def change(self,**values):self.params.update(values);self.pv+=1;self.call('on_params_update')
    def tap(self,key):self.click=key;self.call('on_touch');self.render()
    def callback(self,name):
        def call(c,*a):
            mem=c.get('memory')
            read=lambda p,n:bytes(mem.read(c,p,p+n))
            if name=='host_fetch':
                timeout,mp,ml,up,ul,hp,hl,bp,bl=a
                method,url,headers,body=read(mp,ml),read(up,ul),read(hp,hl),read(bp,bl)
                assert method==b'POST' and url.startswith(b'http://127.0.0.1:18764/cgi-bin/audio?')
                assert headers==b'Authorization: Rolid-local-v1' and timeout==2000
                self.fetches.append(url.decode());return len(self.fetches)
            if name=='host_params_version':return self.pv
            if name=='host_system_version':return self.sv
            if name.endswith('_snapshot'):
                if name=='host_params_snapshot':b=packed(self.params)
                else:
                    zone=b'Europe/Prague';b=s.pack('<IBH',2+(self.system_alarm is not None),0,len(zone))+zone+bytes([8,self.night])
                    if self.system_alarm:
                        at,label=self.system_alarm;label=label.encode();b+=s.pack('<BBqH',7,1,at*1000,len(label))+label
                if a[1]>=len(b):mem.write(c,b,a[0])
                return len(b)
            if name=='host_widget_size':return (self.width<<32)|self.height
            if name=='host_widget_viewport_shape':return 0
            if name=='host_get_system_time':mem.write(c,s.pack('<q',self.now),a[0]);return
            if name=='host_submit_tree':
                p,n,ww,hh=a;assert (ww,hh)==(self.width,self.height);self.tree=read(p,n);return
            if name=='host_request_frame_after':self.delay=a[0];return
            if name=='host_request_frame':self.requests+=1;return
            if name=='host_get_touch_click':
                key=read(a[0],a[1]).decode()
                if key==self.click:
                    self.click=None;mem.write(c,s.pack('<ffff',10,10,100,40),a[2]);return 1
                return 0
            if name=='host_get_touch_drag':return 0
            if name=='host_timer_start':
                if a[0]<1000:return 0
                self.timer_count+=1
                if self.timer_count==1:self.timer=1
                return self.timer_count
            if name=='host_timer_stop':return
            if name=='host_register_audio':
                b=read(a[0],a[1]);tag=read(a[2],a[3]).decode();assert b[:4]==b'RIFF';key=len(self.audio)+1;self.audio[key]=tag;return key
            if name=='host_audio_play':assert a[0] in self.audio;self.sounds.append((self.now,self.audio[a[0]],a[1]));return
            if name=='host_led_set_temporary':self.leds.append(a);return
            if name=='host_led_stop':self.led_stops+=1;return
            if name=='host_audio_stop':self.stops.append(a[0]);return
            if name=='host_kv_set':self.kv[read(a[0],a[1]).decode()]=read(a[2],a[3]);return
            if name=='host_kv_get':
                b=self.kv.get(read(a[0],a[1]).decode())
                if b is None:return -1
                if a[3]>=len(b):mem.write(c,b,a[2])
                return len(b)
            if name=='host_log':return
            raise AssertionError(name)
        return call

for width,height in [(317,238),(638,238),(638,480),(1280,480)]:
    d=Device(width,height,params={'color':'cyan','font_size':160})
    d.render();assert b'12:59:59' in d.tree and 'neděle'.encode() in d.tree and b'01. 11. 2026' in d.tree
    assert s.pack('<I',0x22d3eeff) in d.tree
    d.change(color='#FF0000',font_size=24,show_date=False,show_weekday=False,show_countdown=False,show_seconds=False)
    d.render();assert s.pack('<II',24,0xff0000ff) in d.tree and b'12:59:59' not in d.tree and b'12:59' in d.tree
    assert b'01. 11. 2026' not in d.tree and 'neděle'.encode() not in d.tree
    d.tap('test_led');assert d.leds
    print(f'PASS {width}x{height}: date, weekday, colors, sizes, toggles, touch test')

def alarm(i=1,**kw):return dict(id=i,name='Test',enabled=True,mode='daily',hour=13,minute=0,**kw)
d=Device(params={'alarms':[alarm()], 'gong_enabled':True,'sound_verified':True})
d.render();assert b'00:00:01' in d.tree
d.tick();assert d.sounds[-1][1]=='rolid-alarm-v2' and len(d.sounds)==1
# No renders are needed for the alarm to fire.
d.render();assert 'BUDÍK: Test'.encode() in d.tree
d.tap('snooze');assert d.stops and len(json.loads(d.kv['clock-state-v2'])['snoozed'])==1
d.render();assert b'00:05:00' in d.tree
restart=Device(params=d.params,kv=d.kv.copy(),now=d.now+1)
restart.tick(299);assert restart.sounds[-1][1]=='rolid-alarm-v2'
restart.tap('stop');assert not json.loads(restart.kv['clock-state-v2'])['active']
print('PASS background alarm, priority over gong, snooze, persistence, restart, dismiss')

g=Device(params={'gong_enabled':True});g.tick();assert g.sounds[-1][1]=='rolid-gong-v2'
g.tick();assert len(g.sounds)==1
g.night=True;g.sv+=1;g.call('on_system_update');g.tick(3599);assert len(g.sounds)==1
print('PASS hourly gong deduplication and night-mode mute')

n=Device(system_alarm=(timestamp('2026-11-01T13:00:59+01:00'),'Systémový'))
n.render();assert 'Deck: Systémový'.encode() in n.tree and b'00:01:00' in n.tree
print('PASS system alarm countdown')

bad=Device(params={'alarms':[dict(id=1,mode='once',date='2026-02-31')]})
bad.render();assert 'zadej datum'.encode() in bad.tree
print('PASS invalid date validation')

many=Device(params={'alarms':[alarm(i=i,name2='unused') for i in range(1,33)]})
many.tick();many.render();assert len(json.loads(many.kv['clock-state-v2'])['active'])==32
assert len(many.sounds)==1
print('PASS 32 simultaneous alarms grouped into one sound')

l=Device();l.tick();assert l.leds==[(2,255,208,0,2000,2000,1)] and not l.sounds
l.tick();assert len(l.leds)==1
r=Device(kv=l.kv.copy(),now=l.now);r.tick();assert not r.leds
l.change(led_color='#00FF80',led_period_ms=500,led_cycles=3,led_effect='knight_rider')
l.tap('test_led');assert l.leds[-1]==(1,0,255,128,500,1500,1)
for params in [{'hourly_led_enabled':False},{'led_all_hours':False,'led_start':15,'led_end':20}]:
    d=Device(params=params);d.tick();assert not d.leds
n=Device();n.night=True;n.sv+=1;n.call('on_system_update');n.tick();assert n.leds[-1][1:4]==(38,9,0)
print('PASS LED yellow scan, independent sound, deduplication, restart, custom color/effect/speed/cycles, touch, disabled/window/night')

for ww,hh in [(317,238),(638,238),(638,480),(1280,480)]:
    d=Device(ww,hh,params={'alarms':[{'time':'7:00','repeat':'Po,Ut','name':'Ráno'},{'time':'15:00','repeat':'Ut','name':'Odpoledne'}]})
    d.render()
    assert 'neděle · 01. 11. 2026'.encode() in d.tree
    assert d.tree.count(b'Po 7:00')==2 and d.tree.count('Út 7:00'.encode())==2 and 'Út 15:00'.encode() in d.tree
    d.change(show_alarm_list=False);d.render();assert b'Po 7:00' not in d.tree
print('PASS combined date row and five chronological occurrences at all four viewport sizes')
m=Device(now=timestamp('2026-11-01T23:59:59+01:00'));m.tick();assert m.leds[-1][0]==5
for _ in range(15):m.call('__on_timer',2,1)
assert len(m.leds)==12 and len(set(x[1:4] for x in m.leds))>=12
assert max(max(x[1:4]) for x in m.leds)<=38
m.tick();assert len(m.leds)==12
r=Device(kv=m.kv.copy(),now=m.now);r.tick();assert not r.leds
n=Device(now=timestamp('2026-11-01T22:59:59+01:00'));n.tick();assert n.leds[-1][1:4]==(38,9,0)
x=Device(params={'midnight_rainbow':False},now=timestamp('2026-11-01T23:59:59+01:00'));x.tick();assert x.leds[-1][0]==2 and x.leds[-1][1:4]==(38,9,0)
t=Device();t.tap('test_rainbow');assert t.leds[-1][0]==5
t.change(led_color='00ff00');before=len(t.leds);t.call('__on_timer',2,1);assert len(t.leds)==before
print('PASS midnight rainbow sequence/brightness/expiry/dedup/restart, night orange, rainbow toggle and test')

old=Device(params={'night_brightness':4},now=timestamp('2026-11-01T22:59:59+01:00'));old.tick();assert old.leds[-1][1:4]==(38,9,0)
print('PASS old 4 percent setting upgrades to 15 percent')

b=Device(params={'use_audio_service':True,'gong_volume':75})
b.tap('test');assert b.fetches[-1].endswith('action=gong&volume=75') and not b.sounds
# Commands are serialized: a stop waits for the already dispatched play.
b.change(gong_volume=60);assert len(b.fetches)==1
b.call('__on_fetch_response',1,200,0,0);assert b.fetches[-1].endswith('action=stop')
b.call('__on_fetch_response',2,200,0,0)
b.tap('test');b.call('__on_fetch_response',3,503,0,0);b.render();assert 'služba nedostupná'.encode() in b.tree
b.tap('test');b.call('__on_fetch_response',4,200,0,0);b.render();assert 'služba nedostupná'.encode() not in b.tree
b=Device(params={'use_audio_service':True,'alarms':[alarm()],'alarm_volume':80});b.tick();assert b.fetches[-1].endswith('action=alarm&volume=80')
b.call('__on_fetch_response',1,200,0,0);b.tap('snooze');assert b.fetches[-1].endswith('action=stop')
b.call('__on_fetch_response',2,200,0,0);b.tick(300);assert b.fetches[-1].endswith('action=alarm&volume=80')
b.call('__on_fetch_response',3,200,0,0);b.tap('stop');assert b.fetches[-1].endswith('action=stop')
g=Device(params={'use_audio_service':True,'gong_enabled':True,'gong_volume':0});g.tick();assert g.fetches[-1].endswith('action=gong&volume=0')
print('PASS MP3 bridge: POST/loopback/header, volume, alarm/snooze/dismiss, serial stop, error and recovery, mute')
