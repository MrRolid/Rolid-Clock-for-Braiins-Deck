import math, struct, wave
from pathlib import Path
root=Path(__file__).parent/'assets'
root.mkdir(exist_ok=True)
for name,length,partials in [('gong',3.8,[(220,1,1.2),(332,.45,.9),(481,.3,.65),(587,.25,.55),(763,.12,.4)]),('alarm',2.2,[(660,1,.8),(880,.4,.7),(1320,.15,.5)])]:
    data=[]; rate=16000
    for n in range(int(rate*length)):
        t=n/rate
        v=sum(a*math.sin(2*math.pi*f*t)*math.exp(-t/d) for f,a,d in partials)
        v*=min(1,t/.006)
        data.append(v)
    peak=max(abs(v) for v in data)
    with wave.open(str(root/(name+'.wav')),'wb') as f:
        f.setnchannels(1);f.setsampwidth(2);f.setframerate(rate)
        f.writeframes(b''.join(struct.pack('<h',int(v/peak*24000)) for v in data))
