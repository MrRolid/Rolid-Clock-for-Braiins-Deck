#!/bin/sh
set -eu
cd "$(dirname "$0")"
root=/mnt/data/rolid-audio
[ "$(id -u)" = 0 ] || { echo 'Spustte jako root na Decku.'; exit 1; }
for f in /usr/sbin/uhttpd /usr/bin/madplay /etc/rc.common /lib/functions/procd.sh; do
    [ -r "$f" ] || { echo "Chybi $f; instalace zastavena."; exit 1; }
done
[ -d /mnt/data ] || { echo 'Chybi /mnt/data'; exit 1; }
for f in sounds/gong.mp3 sounds/alarm.mp3 www/cgi-bin/audio stop.sh rolid-audio.init; do
    [ -s "$f" ] || { echo "Chybi soubor balicku $f"; exit 1; }
done
if [ -e /etc/init.d/rolid-audio ]; then
    grep -q ROLID_AUDIO_SERVICE_V1 /etc/init.d/rolid-audio || { echo 'Existuje cizi sluzba rolid-audio; zastavuji.'; exit 1; }
fi
if [ -e "$root" ]; then
    [ -f "$root/.rolid-audio-v1" ] || { echo 'Cilovy adresar neni spravovan timto instalatorem.'; exit 1; }
    backup="$root.backup.$(date +%Y%m%d-%H%M%S)"
    cp -a "$root" "$backup"
    echo "Zaloha: $backup"
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
    echo 'Sluzba se nespustila (napr. obsazeny port 18764). Neni zapnuta pri startu.'
    exit 1
fi
url='http://127.0.0.1:18764/cgi-bin/audio?status'
result=''
if command -v curl >/dev/null 2>&1; then result=$(curl -fsS --max-time 5 "$url") || { echo "Kontrola CGI selhala; zkontrolujte /tmp/rolid-audio-install.log" >&2; exit 1; }
elif command -v wget >/dev/null 2>&1; then result=$(wget -O - "$url") || { echo "Kontrola CGI selhala; zkontrolujte /tmp/rolid-audio-install.log" >&2; exit 1; }
elif command -v uclient-fetch >/dev/null 2>&1; then result=$(uclient-fetch -O - "$url") || { echo "Kontrola CGI selhala; zkontrolujte /tmp/rolid-audio-install.log" >&2; exit 1; }
else echo 'HTTP klient neni dostupny; CGI overi tlacitko Zvuk ve widgetu.'
fi
[ -z "$result" ] || [ "$result" = 'rolid-audio-v1 ready' ] || { echo "Neocekavana odpoved: $result"; exit 1; }
kill "$probe"
wait "$probe" 2>/dev/null || :
trap - EXIT HUP INT TERM
/etc/init.d/rolid-audio enable
/etc/init.d/rolid-audio start
echo 'Nainstalovano: rolid-audio, pouze 127.0.0.1:18764.'
echo 'Ve widgetu 0.5.1 zvolte Zvuk. Hlasitost gongu 75 % odpovida madplay -A -10.'
