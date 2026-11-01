"""Run installer in isolation through a failing HTTP probe; catch CGI 403 regression."""
import os, shutil, stat, subprocess, tempfile
from pathlib import Path
source = Path(__file__).resolve().parents[1] / 'audio-service'
with tempfile.TemporaryDirectory() as temp:
    tmp = Path(temp)
    package = tmp / 'package'
    shutil.copytree(source, package)
    data = tmp / 'data'; data.mkdir()
    etc = tmp / 'etc'; (etc / 'init.d').mkdir(parents=True)
    procd = tmp / 'procd.sh'; procd.touch()
    (etc / 'rc.common').touch()
    tools = tmp / 'bin'; tools.mkdir()
    for name, code in [('uhttpd', 'exec sleep 30'), ('madplay', 'exit 0'), ('curl', 'echo "HTTP test failure" >&2; exit 22')]:
        f = tools / name; f.write_text('#!/bin/sh\n' + code + '\n'); f.chmod(0o755)
    script = (package / 'install.sh').read_text()
    for before, after in [('/mnt/data', str(data)), ('/etc/init.d', str(etc / 'init.d')), ('/etc/rc.common', str(etc / 'rc.common')), ('/lib/functions/procd.sh', str(procd)), ('/usr/sbin/uhttpd', str(tools / 'uhttpd')), ('/usr/bin/madplay', str(tools / 'madplay')), ('/tmp/rolid-audio-install.log', str(tmp / 'probe.log'))]:
        script = script.replace(before, after)
    # Test works without root on CI; production root check remains unchanged.
    script = script.replace('[ "$(id -u)" = 0 ]', '[ 0 = 0 ]')
    (package / 'install.sh').write_text(script)
    run = subprocess.run(['sh', str(package / 'install.sh')], env={**os.environ, 'PATH': str(tools) + ':' + os.environ['PATH']}, capture_output=True, text=True, timeout=10)
    assert run.returncode != 0, run.stdout
    assert 'Kontrola CGI selhala' in run.stderr, run.stderr
    cgi = data / 'rolid-audio/www/cgi-bin/audio'
    assert stat.S_ISREG(cgi.stat().st_mode)
    assert cgi.stat().st_mode & stat.S_IXOTH, 'uhttpd rejects CGI without S_IXOTH with HTTP 403'
    assert stat.S_IMODE((data / 'rolid-audio').stat().st_mode) == 0o700
print('PASS installer: CGI executable for uhttpd, private data directory, explicit HTTP failure')
