use crate::model::{Alarm, Engine, Occurrence, Saved, countdown, gong_allowed, parse_alarm, upcoming};
use bmc_wasm_sdk::*;
use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::Tz as Zone;
use std::cell::{RefCell, Cell};

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    static BRIDGE: RefCell<Bridge> = RefCell::new(Bridge::default());
    static RAINBOW_TIMER: RefCell<Option<timer::Timer>> = const { RefCell::new(None) };
    static RAINBOW: Cell<u32> = const { Cell::new(0) };
    static TICKER: RefCell<Option<timer::Timer>> = const { RefCell::new(None) };
}

#[derive(Default)]
struct Bridge {
    busy: bool,
    queued: Option<(&'static str,i32)>,
    error: bool,
    contacted: bool,
}
fn bridge_command(action: &'static str, volume: i32) {
    BRIDGE.with_borrow_mut(|b|b.queued=Some((action,volume)));
    bridge_send();
}
fn bridge_send() {
    let command=BRIDGE.with_borrow_mut(|b|{
        if b.busy{return None}
        let c=b.queued.take();if c.is_some(){b.busy=true}c
    });
    let Some((action,volume))=command else{return};
    let url=if action=="stop" {"http://127.0.0.1:18764/cgi-bin/audio?action=stop".into()}
        else {format!("http://127.0.0.1:18764/cgi-bin/audio?action={action}&volume={}",volume.clamp(0,100))};
    let accepted=net::FetchRequest::post(&url)
        .headers("Authorization: Rolid-local-v1")
        .timeout(std::time::Duration::from_secs(2))
        .send(|r|{
            BRIDGE.with_borrow_mut(|b|{b.busy=false;b.error=!r.ok();b.contacted=true;});
            bridge_send();request_frame();
        });
    if accepted.is_none(){BRIDGE.with_borrow_mut(|b|{b.busy=false;b.error=true;b.contacted=true;});request_frame();}
}
fn play_sound(audio: Option<AudioId>, action: &'static str, volume: i32) {
    if enabled(&params::current(),"use_audio_service",true) {bridge_command(action,volume);}
    else if volume>0 {audio_play(audio,Volume::new(volume.clamp(0,100) as u8));}
}
fn stop_sound(audio: Option<AudioId>) {
    audio_stop(audio);
    if enabled(&params::current(),"use_audio_service",true){bridge_command("stop",0);}
}

struct State {
    engine: Engine,
    alarms: Vec<Alarm>,
    zone: Zone,
    errors: Vec<String>,
    timer_ok: bool,
    alarm_audio: Option<AudioId>,
    gong_audio: Option<AudioId>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            engine: Engine::default(),
            alarms: vec![],
            zone: chrono_tz::UTC,
            errors: vec![],
            timer_ok: false,
            alarm_audio: None,
            gong_audio: None,
        }
    }
}
fn now() -> i64 {
    SystemTime::now().unix_secs
}
fn integer(p: &params::Params, key: &str, default: i32, min: i32, max: i32) -> i32 {
    p.get_i32(key).unwrap_or(default).clamp(min, max)
}
fn enabled(p: &params::Params, key: &str, default: bool) -> bool {
    p.get_bool(key).unwrap_or(default)
}
fn color(p: &params::Params, key: &str, default: &str) -> Color {
    let value = p.get_str(key).unwrap_or(default);
    let rgb = match value {
        "white" => 0xffffff,
        "green" => 0x4ade80,
        "amber" => 0xffbf47,
        "red" => 0xff6464,
        "blue" => 0x60a5fa,
        "purple" => 0xc084fc,
        "pink" => 0xf472b6,
        "cyan" => 0x22d3ee,
        _ => u32::from_str_radix(value.trim().trim_start_matches('#'), 16)
            .ok()
            .filter(|v| *v <= 0xffffff)
            .unwrap_or(0xffffff),
    };
    Color::from_hex(rgb)
}
fn persist(s: &State) {
    if let Ok(data) = serde_json::to_vec(&s.engine.saved) {
        kv::set("clock-state-v2", &data);
    }
}
fn load_config(s: &mut State) {
    let p = params::current();
    let system = system::current();
    let zone = system.timezone().unwrap_or("UTC");
    s.errors.clear();
    s.zone = zone.parse().unwrap_or_else(|_| {
        s.errors.push(format!("Neznámé pásmo {zone}; používám UTC"));
        chrono_tz::UTC
    });
    s.alarms.clear();
    if let Some(list) = p.get_list("alarms") {
        for (index,item) in list.iter().take(32).enumerate() {
            let Some(o) = item.as_object() else { continue };
            let i = |key, def| o.get(key).and_then(|v| v.as_i32()).unwrap_or(def);
            let b = |key, def| o.get(key).and_then(|v| v.as_bool()).unwrap_or(def);
            let t = |key, def: &str| {
                o.get(key)
                    .and_then(|v| v.as_str())
                    .unwrap_or(def)
                    .to_owned()
            };
            if let Some(time)=o.get("time").and_then(|v|v.as_str()) {
                match parse_alarm(index as i32+1,time,&t("repeat","Po-Pa"),&t("name","")) {
                    Ok(a)=>{if a.enabled{s.alarms.push(a);}},
                    Err(e)=>s.errors.push(e),
                }
                continue;
            }
            let a = Alarm {
                id: i("id", 0),
                name: t("name", "Budík"),
                enabled: b("enabled", true),
                mode: t("mode", "weekly"),
                hour: i("hour", 7) as u32,
                minute: i("minute", 0) as u32,
                days: [
                    b("mon", true),
                    b("tue", true),
                    b("wed", true),
                    b("thu", true),
                    b("fri", true),
                    b("sat", false),
                    b("sun", false),
                ],
                date: chrono::NaiveDate::parse_from_str(&t("date", ""), "%Y-%m-%d").ok(),
                interval: i("interval", 1) as u32,
                day: i("day", 1) as u32,
                month: i("month", 1) as u32,
            };
            if !a.enabled {
                continue;
            }
            if s.alarms.iter().any(|x| x.id == a.id) {
                s.errors.push(format!("Duplicitní ID budíku {}", a.id));
                continue;
            }
            if let Err(err) = a.validate() {
                s.errors.push(err);
                continue;
            }
            s.alarms.push(a);
        }
    }
    s.engine.configure(&s.alarms, now(), s.zone);
    persist(s);
}

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    STATE.with_borrow_mut(|s| {
        s.engine.saved = kv::get("clock-state-v2")
            .and_then(|b| serde_json::from_slice::<Saved>(&b).ok())
            .unwrap_or_default();
        load_config(s);
        s.alarm_audio = register_audio(include_bytes!("../assets/alarm.wav"), "rolid-alarm-v2");
        s.gong_audio = register_audio(include_bytes!("../assets/gong.wav"), "rolid-gong-v2");
    });
    let timer = timer::start(1000, tick);
    STATE.with_borrow_mut(|s| s.timer_ok = timer.is_some());
    TICKER.set(timer);
    RAINBOW_TIMER.set(timer::start(1000, |_,_| rainbow_frame()));
    request_frame();
}

fn tick(_: &timer::Timer, _: u32) {
    STATE.with_borrow_mut(|s| {
        let p = params::current();
        let t = now();
        let before = s.engine.saved.clone();
        let play = s.engine.tick(
            &s.alarms,
            t,
            s.zone,
            i64::from(integer(&p, "ring_seconds", 120, 10, 600)),
        );
        if play {
            play_sound(s.alarm_audio,"alarm",integer(&p,"alarm_volume",70,0,100));
        }
        if !before.active.is_empty() && s.engine.saved.active.is_empty() {
            stop_sound(s.alarm_audio);
        }
        let local = s.zone.timestamp_opt(t, 0).single();
        if let Some(local) = local {
            let key = format!("{}-{}", local.date_naive(), local.hour());
            let night = system::current().night_mode().unwrap_or(false);
            if local.minute() == 0 && local.second() < 5 {
                if enabled(&p, "gong_enabled", false) && s.engine.saved.gong.as_ref() != Some(&key) {
                    s.engine.saved.gong = Some(key.clone());
                    if gong_allowed(local.hour(), integer(&p,"gong_start",8,0,23) as u32, integer(&p,"gong_end",22,0,23) as u32)
                        && !(enabled(&p,"gong_quiet_night",true) && night) && s.engine.saved.active.is_empty() {
                        play_sound(s.gong_audio,"gong",integer(&p,"gong_volume",45,0,100));
                    }
                }
                if enabled(&p,"hourly_led_enabled",true) && s.engine.saved.led_hour.as_ref()!=Some(&key) {
                    s.engine.saved.led_hour=Some(key);
                    if local.hour()==0 && enabled(&p,"midnight_rainbow",true) { start_rainbow(); }
                    else if enabled(&p,"led_all_hours",true) || gong_allowed(local.hour(),integer(&p,"led_start",8,0,23) as u32,integer(&p,"led_end",22,0,23) as u32) { play_led(&p); }
                }
            }
        }
        if before != s.engine.saved {
            persist(s);
        }
    });
    request_frame();
}

fn is_night(p: &params::Params) -> bool {
    let zone:Zone=system::current().timezone().unwrap_or("UTC").parse().unwrap_or(chrono_tz::UTC);
    let hour=zone.timestamp_opt(now(),0).single().map(|v|v.hour()).unwrap_or(0);
    system::current().night_mode().unwrap_or(false) || gong_allowed(hour,integer(p,"night_start",22,0,23) as u32,integer(p,"night_end",8,0,23) as u32)
}
fn start_rainbow() { led::stop();RAINBOW.set(12);rainbow_frame(); }
fn rainbow_frame() {
    let remaining=RAINBOW.get();if remaining==0{return}
    let p=params::current();let phase=(12-remaining) as f32/12.0*6.0;
    let x=(255.0*(1.0-(phase%2.0-1.0).abs())) as u32;
    let (r,g,b)=match phase as u32{0=>(255,x,0),1=>(x,255,0),2=>(0,255,x),3=>(0,x,255),4=>(x,0,255),_=>(255,0,x)};
    let level=if is_night(&p){integer(&p,"night_brightness_pct",15,1,15) as u32}else{100};
    led::set_effect_global(LedEffect::Solid,Color::from_hex(((r*level/100)<<16)|((g*level/100)<<8)|(b*level/100)),1000,Some(1100));
    RAINBOW.set(remaining-1);
}

fn play_led(p: &params::Params) {
    RAINBOW.set(0);
    let effect=match p.get_str("led_effect").unwrap_or("scan") {
        "knight_rider"=>LedEffect::KnightRider,
        "breathe"=>LedEffect::Breathe,
        "solid"=>LedEffect::Solid,
        _=>LedEffect::Scan,
    };
    let period=integer(p,"led_period_ms",2000,250,10000) as u32;
    let repeats=integer(p,"led_cycles",1,1,10) as u32;
    led::stop();
    let c=if is_night(p){let n=integer(p,"night_brightness_pct",15,1,15) as u32;Color::from_hex(((255*n/100)<<16)|((64*n/100)<<8))}else{color(p,"led_color","FFD000")};
    led::set_effect_global(effect,c,period,Some(period*repeats));
}

fn short(s: &str, limit: usize) -> String {
    let mut out: String = s.chars().take(limit).collect();
    if s.chars().count() > limit {
        out.push('…');
    }
    out
}

#[unsafe(no_mangle)]
pub extern "C" fn render(_delta: u32) {
    let p = params::current();
    let vp = widget_viewport();
    let t = now();
    STATE.with_borrow_mut(|s| {
        let local=s.zone.timestamp_opt(t,0).single().expect("system epoch fits chrono range");
        let mut lines:Vec<(String,u32,Color)>=vec![];
        let seconds=enabled(&p,"show_seconds",true);
        lines.push((local.format(if seconds {"%H:%M:%S"}else{"%H:%M"}).to_string(),integer(&p,"font_size",160,24,240) as u32,color(&p,"color","cyan")));
        let mut date_parts=Vec::new();
        if enabled(&p,"show_weekday",true){date_parts.push(["pondělí","úterý","středa","čtvrtek","pátek","sobota","neděle"][local.weekday().num_days_from_monday() as usize].to_owned());}
        if enabled(&p,"show_date",true){date_parts.push(local.format("%d. %m. %Y").to_string());}
        if !date_parts.is_empty(){lines.push((date_parts.join(" · "),integer(&p,"date_size",32,14,80) as u32,color(&p,"date_color","white")));}
        let mut preview=upcoming(&s.alarms,&s.engine.saved,t,s.zone,5);
        let mut next=s.engine.next_alarm(t);
        if enabled(&p,"include_system_alarm",true) {
            let snapshot=system::current();
            if let Some(a)=snapshot.next_alarm() {
                let at=a.fire_at_utc_ms/1000;
                if at>t {preview.push(Occurrence{key:"system".into(),name:format!("Deck: {}",a.name),at});}
                if at>t && next.as_ref().is_none_or(|v|at<v.at){next=Some(Occurrence {key:"system".into(),name:format!("Deck: {}",a.name),at});}
            }
        }
        preview.sort_by_key(|o|o.at);preview.truncate(5);
        if !s.engine.saved.active.is_empty() {
            lines.push((format!("BUDÍK: {}",short(&s.engine.saved.active.iter().map(|a|a.name.clone()).collect::<Vec<_>>().join(" + "),32)),28,Color::from_hex(0xffbf47)));
        } else if enabled(&p,"show_countdown",true) {
            let label=next.map(|a|format!("{} · za {}",short(&a.name,24),countdown(a.at-t))).unwrap_or_else(||"Žádný další budík".into());
            lines.push((label,integer(&p,"countdown_size",26,14,56) as u32,color(&p,"countdown_color","amber")));
        }
        if let Some(error)=s.errors.first(){lines.push((short(error,if vp.width<500 {38}else{65}),18,Color::from_hex(0xff6464)));}
        if !s.timer_ok {lines.push(("Časovač není dostupný – budíky neběží".into(),18,Color::from_hex(0xff6464)));}
        if enabled(&p,"use_audio_service",true) {
            BRIDGE.with_borrow(|b|{
                if b.error {lines.push(("Zvuk: služba nedostupná – zkontroluj instalaci".into(),14,Color::from_hex(0xff6464)));}
                else if !b.contacted {lines.push(("Zvuk: vyzkoušej tlačítko Zvuk".into(),14,Color::from_hex(0xffbf47)));}
            });
        } else if (!s.alarms.is_empty() || enabled(&p,"gong_enabled",false)) && !enabled(&p,"sound_verified",false) {
            lines.push(("Zvuk přes SDK není ověřen".into(),14,Color::from_hex(0xffbf47)));
        }
        let active=!s.engine.saved.active.is_empty();
        let controls=active || enabled(&p,"show_test_button",true);
        let sidebar=if enabled(&p,"show_alarm_list",true){if vp.width<500 {86.0}else{126.0}}else{0.0};
        let main_width=vp.width as f32-16.0-sidebar-if sidebar>0.0{8.0}else{0.0};
        let available=(vp.height as f32-24.0-if controls {44.0}else{0.0}).max(40.0);
        let sizes:Vec<f32>=lines.iter().map(|(text,size,_)|(*size as f32).min((main_width-16.0)/(text.chars().count().max(1) as f32*0.68))).collect();
        let total=sizes.iter().map(|v|v*1.4+4.0).sum::<f32>();
        let scale=(available/total).min(1.0);
        let mut children=vec![spacer(1.0)];
        for ((label,_,c),size) in lines.into_iter().zip(sizes) {
            children.push(text(label,style!(size:(size*scale).max(9.0) as u32,color:c,weight:FontWeight::SEMIBOLD)));
        }
        children.push(spacer(1.0));
        if active {
            children.push(row(props!(gap:8.0),[
                button!("stop","Zastavit",style:Danger,size:Small),
                button!("snooze",format!("Odložit {} min",integer(&p,"snooze_minutes",5,1,60)),style:Secondary,size:Small),
            ]));
        } else if controls {children.push(row(props!(gap:8.0),[button!("test_led","Světlo",style:Secondary,size:Small),button!("test_rainbow","Duha",style:Secondary,size:Small),button!("test","Zvuk",style:Secondary,size:Small)]));}
        let main=col(props!(width:main_width,height:vp.height as f32-16.0,gap:4.0,cross_align:CrossAlign::Center),children);
        let mut columns=Vec::new();
        if sidebar>0.0 {
            let mut badges=vec![text("BUDÍKY",style!(size:12,color:Color::from_hex(0x94a3b8)))];
            for (index,o) in preview.iter().enumerate(){
                let at=s.zone.timestamp_opt(o.at,0).single().unwrap();
                let label=format!("{} {}:{:02}",["Po","Út","St","Čt","Pá","So","Ne"][at.weekday().num_days_from_monday() as usize],at.hour(),at.minute());
                badges.push(col(props!(width:sidebar,height:((vp.height as f32-64.0)/5.0).min(64.0),padding:4.0,border_radius:8.0,background:Color::from_hex(if index==0{0x173545}else{0x17202e}),cross_align:CrossAlign::Center),[spacer(1.0),text(label,style!(size:if sidebar<100.0{16}else{22},color:Color::from_hex(if index==0{0xffbf47}else{0xcbd5e1}),weight:FontWeight::SEMIBOLD)),spacer(1.0)]));
            }
            if preview.is_empty(){badges.push(text("Žádný",style!(size:14,color:Color::from_hex(0x94a3b8))));}
            columns.push(col(props!(width:sidebar,height:vp.height as f32-16.0,gap:4.0,cross_align:CrossAlign::Center),badges));
        }
        columns.push(main);
        let result=render_ui(vp.width,vp.height,row(props!(width:vp.width as f32,height:vp.height as f32,padding:8.0,gap:8.0,background:color(&p,"background_color","080d16")),columns));
        if result.clicks.contains_key("stop") {s.engine.dismiss();stop_sound(s.alarm_audio);persist(s);request_frame();}
        if result.clicks.contains_key("snooze") {s.engine.snooze(t,i64::from(integer(&p,"snooze_minutes",5,1,60)));stop_sound(s.alarm_audio);persist(s);request_frame();}
        if result.clicks.contains_key("test_rainbow") {start_rainbow();}
        if result.clicks.contains_key("test_led") {play_led(&p);}
        if result.clicks.contains_key("test") {play_sound(s.gong_audio,"gong",integer(&p,"gong_volume",45,0,100));}
    });
    request_frame_after(1000);
}

#[unsafe(no_mangle)]
pub extern "C" fn on_params_update() {
    if BRIDGE.with_borrow(|b|b.contacted || b.busy){bridge_command("stop",0);}
    RAINBOW.set(0);
    led::stop();
    STATE.with_borrow_mut(|s| {
        stop_sound(s.alarm_audio);
        load_config(s);
    });
    request_frame();
}
#[unsafe(no_mangle)]
pub extern "C" fn on_system_update() {
    STATE.with_borrow_mut(load_config);
    request_frame();
}
#[unsafe(no_mangle)]
pub extern "C" fn on_touch() {
    request_frame();
}
