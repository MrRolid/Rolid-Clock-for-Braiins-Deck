"""Execute the real shell CGI with isolated paths and a fake madplay process."""
import os,sys,subprocess,tempfile,time,signal
from pathlib import Path
source=Path(__file__).parents[1]/'audio-service'
with tempfile.TemporaryDirectory() as tmp:
    root=Path(tmp)/'data';root.mkdir();run=Path(tmp)/'run'
    sounds=root/'sounds';sounds.mkdir()
    for name in ['gong','alarm']:(sounds/(name+'.mp3')).write_bytes(b'test')
    args=Path(tmp)/'args';fake=Path(tmp)/'madplay'
    fake.write_text('#!'+sys.executable+'\nimport sys,time\nfrom pathlib import Path\nPath('+repr(str(args))+').write_text(repr(sys.argv[1:]))\ntime.sleep(30)\n');fake.chmod(0o700)
    def rewrite(s):return s.replace('/mnt/data/rolid-audio',str(root)).replace('/tmp/rolid-audio',str(run)).replace('/usr/bin/madplay',str(fake))
    (root/'stop.sh').write_text(rewrite((source/'stop.sh').read_text()))
    cgi=root/'audio';cgi.write_text(rewrite((source/'www/cgi-bin/audio').read_text()))
    def call(query,method='POST',header='Rolid-local-v1',addr='127.0.0.1'):
        env={**os.environ,'QUERY_STRING':query,'REQUEST_METHOD':method,'HTTP_AUTHORIZATION':header,'REMOTE_ADDR':addr}
        return subprocess.check_output(['/bin/sh',str(cgi)],env=env,text=True)
    assert '200 OK' in call('status','GET')
    for q in ['action=gong&volume=-1','action=gong&volume=101','action=gong&volume=08','action=gong&volume=1;touch /tmp/x','action=../../bin/sh&volume=90']:
        assert '400 Bad Request' in call(q),q
    assert '403 Forbidden' in call('action=stop',header='')
    assert '403 Forbidden' in call('action=stop',addr='192.168.1.2')
    assert '405 Method' in call('action=gong&volume=75','GET')
    assert '200 OK' in call('action=gong&volume=75')
    pid=int((run/'player.pid').read_text())
    try:
        for _ in range(50):
            if args.exists():break
            time.sleep(.02)
        assert "'-10'" in args.read_text() and 'gong.mp3' in args.read_text()
        assert '200 OK' in call('action=stop');assert not (run/'player.pid').exists()
        assert '200 OK' in call('action=alarm&volume=0');assert not (run/'player.pid').exists()
        (run/'request.lock').mkdir();assert '409 Conflict' in call('action=stop');(run/'request.lock').rmdir()
        (sounds/'gong.mp3').unlink();assert '503 Service' in call('status','GET')
    finally:
        try:os.kill(pid,signal.SIGTERM)
        except ProcessLookupError:pass
print('PASS CGI: local access, method/header, strict action/volume, -10 dB mapping, stop, mute, lock and missing assets')
