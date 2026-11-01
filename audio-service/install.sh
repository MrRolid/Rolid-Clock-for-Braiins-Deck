#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=/mnt/data/rolid-audio
[ "$(id -u)" = 0 ] || { echo 'Run as root on the Deck.'; exit 1; }
for f in /usr/sbin/uhttpd /usr/bin/madplay /etc/rc.common /lib/functions/procd.sh; do
    [ -r "$f" ] || { echo "Missing $f; installation stopped."; exit 1; }
done
[ -d /mnt/data ] || { echo 'Missing /mnt/data'; exit 1; }
for f in sounds/gong.mp3 sounds/alarm.mp3 www/cgi-bin/audio stop.sh rolid-audio.init; do
    [ -s "$f" ] || { echo "Missing package file $f"; exit 1; }
done
if [ -e /etc/init.d/rolid-audio ]; then
    grep -q ROLID_AUDIO_SERVICE_V1 /etc/init.d/rolid-audio || { echo 'An unrelated rolid-audio service exists; stopping.'; exit 1; }
fi
if [ -e "$root" ]; then
    [ -f "$root/.rolid-audio-v1" ] || { echo 'Target directory is not managed by this installer.'; exit 1; }
    backup="$root.backup.$(date +%Y%m%d-%H%M%S)"
    cp -a "$root" "$backup"
    echo "Backup: $backup"
fi
[ ! -x /etc/init.d/rolid-audio ] || /etc/init.d/rolid-audio stop
umask 077
mkdir -p "$root/www/cgi-bin" "$root/sounds"
cp stop.sh "$root/stop.sh"
cp www/cgi-bin/audio "$root/www/cgi-bin/audio"
# Preserve any user-supplied recordings on a reinstall.
[ -s "$root/sounds/gong.mp3" ] || cp sounds/gong.mp3 "$root/sounds/gong.mp3"
[ -s "$root/sounds/alarm.mp3" ] || cp sounds/alarm.mp3 "$root/sounds/alarm.mp3"
: > "$root/httpd.conf"
: > "$root/.rolid-audio-v1"
chmod 700 "$root" "$root/www" "$root/www/cgi-bin" "$root/sounds" "$root/stop.sh"
# uhttpd requires S_IXOTH for CGI, including when the server runs as root.
chmod 755 "$root/www/cgi-bin/audio"
cp rolid-audio.init /etc/init.d/rolid-audio
chmod 755 /etc/init.d/rolid-audio
# First verify binding before enabling the persistent service.
/usr/sbin/uhttpd -f -p 127.0.0.1:18764 -h "$root/www" -c "$root/httpd.conf" -x /cgi-bin -t 5 -T 5 -D -S >/tmp/rolid-audio-install.log 2>&1 &
probe=$!
trap 'kill "$probe" 2>/dev/null || :' EXIT HUP INT TERM
sleep 1
if ! kill -0 "$probe" 2>/dev/null; then
    cat /tmp/rolid-audio-install.log
    echo 'Service failed to start (port 18764 may be in use). Autostart not enabled.'
    exit 1
fi
url='http://127.0.0.1:18764/cgi-bin/audio?status'
result=''
if command -v curl >/dev/null 2>&1; then result=$(curl -fsS --max-time 5 "$url") || { echo "CGI check failed; inspect /tmp/rolid-audio-install.log" >&2; exit 1; }
elif command -v wget >/dev/null 2>&1; then result=$(wget -O - "$url") || { echo "CGI check failed; inspect /tmp/rolid-audio-install.log" >&2; exit 1; }
elif command -v uclient-fetch >/dev/null 2>&1; then result=$(uclient-fetch -O - "$url") || { echo "CGI check failed; inspect /tmp/rolid-audio-install.log" >&2; exit 1; }
else echo 'No HTTP client available; use the widget sound test to check CGI.'
fi
[ -z "$result" ] || [ "$result" = 'rolid-audio-v1 ready' ] || { echo "Unexpected response: $result"; exit 1; }
kill "$probe"
wait "$probe" 2>/dev/null || :
trap - EXIT HUP INT TERM
/etc/init.d/rolid-audio enable
/etc/init.d/rolid-audio start
echo 'Installed: rolid-audio, loopback only at 127.0.0.1:18764.'
echo 'Test sound in the widget. Chime volume 75% maps to madplay -A -10.'
