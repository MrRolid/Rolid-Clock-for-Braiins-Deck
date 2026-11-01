#!/bin/sh
# ROLID_AUDIO_SERVICE_V1 -- stop only the player started by this service.
pidfile=/tmp/rolid-audio/player.pid
[ -r "$pidfile" ] || exit 0
read -r pid < "$pidfile"
case "$pid" in ''|*[!0-9]*) rm -f "$pidfile"; exit 0;; esac
if [ -r "/proc/$pid/cmdline" ]; then
    cmd=$(tr '\000' ' ' < "/proc/$pid/cmdline")
    case "$cmd" in *madplay*'/mnt/data/rolid-audio/sounds/'*) kill "$pid" 2>/dev/null || :;; esac
fi
rm -f "$pidfile"
