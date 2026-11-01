[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Čeština](README.cz.md)

# RoliD Clock 0.6.0

Unofficial **Braiins Deck** widget with a customizable clock, weekday/date,
upcoming alarms, countdown, hourly RGB effects and custom MP3 chimes.
Not affiliated with Braiins Forge. Display and settings: English, German,
Spanish and Czech. License: [GPL-3.0-or-later](LICENSE).

## Choose a language

Install **one** package: `rolid-clock-0.6.0-en.nar`, `-de.nar`, `-es.nar` or
`-cz.nar`. All share the same widget UUID and replace one another, rather than
creating four widgets. The package determines the settings form language and
default display language. The **Language** setting changes the display only.
An update may retain an existing language preference; change it manually if needed.
Custom alarm names are preserved, not translated.

## Install

Download your NAR and `rolid-audio-0.6.0.tar.gz` from Releases.
Replace `DECK_IP` with your Deck's IP address.

1. For MP3 sound, run on your computer in the download folder:

   ```sh
   ssh root@DECK_IP 'mkdir -p /tmp/rolid-audio-060 && tar -xz -C /tmp/rolid-audio-060 && sh /tmp/rolid-audio-060/install.sh' < rolid-audio-0.6.0.tar.gz
   ```

2. In the Deck web UI, use **Display Widgets → Install Widget** to upload the NAR.
   Add the widget to a scene, keep the local MP3 service enabled, set chime volume
   to 75% and press **Sound**. Enable the hourly chime separately if desired.

The service requires root SSH, uhttpd, madplay and OpenWrt/procd. It listens only
on `127.0.0.1:18764`. CGI permissions are fixed to 755 to prevent HTTP 403.
Without the service, clock and lights work but custom MP3 does not.

## Features and settings

Time, date and countdown have configurable colors and font sizes. Day and date
share one line. Time is 24-hour; English dates use day/month/year. The left column
shows the next five occurrences, including repeated instances of the same alarm.
There are up to 32 custom rules, snooze, dismiss and automatic stop. Optionally,
the next native Deck alarm is included, but its sound is not replaced.

The three alarm fields are **Time**, **Repeat**, **Name**. Use `07:00` or `7:00`.
The rule syntax below is the same in every UI language. Existing Czech rules
remain supported. Display language changes never reinterpret the schedule.

| Repeat | Meaning |
|---|---|
| Mon-Fri | Monday through Friday |
| Sat,Sun | Weekend |
| Mon,Tue,Sat | Selected days |
| daily | Every day |
| off | Disable this rule |
| once 2026-12-24 | One date |
| monthly 15 | Day 15 of each month |
| yearly 24.12 | December 24 every year (day.month) |
| days 2 2026-11-01 | Every two days from the date |
| hours 6 2026-11-01 | Every six hours from the date and Time field |
| minutes 30 2026-11-01 | Every 30 minutes from the date and Time field |
| Mon,Wed weeks 2 2026-11-02 | Monday/Wednesday every other week from the date |

Use Mon,Tue,Wed,Thu,Fri,Sat,Sun; no spaces in comma-separated lists.
Intervals: 1–365. Dates: YYYY-MM-DD, except yearly uses day.month.
Invalid dates/rules show an error. Monthly day 31 skips months without that day.
The Deck time zone and daylight-saving transitions are used. Before editing old
17-column rules, note them down and recreate them in this compact editor. Avoid
reordering rows while an alarm is active or snoozed.

## Light and sound

Hourly light is yellow by day and orange at night (default 22:00–08:00 or system
night mode). Default night intensity: 15%, adjustable 1–15%. At midnight the
whole strip cycles through 12 colors, one per second: **not a moving rainbow**.
Available effects have one color; the widget API cannot set each LED separately.
Light and chime have separate controls/time windows. Equal start/end means all day.
Missed hourly events are not replayed. Other widgets and system LED settings may
have priority. Test buttons: **Light**, **RGB**, **Sound**.

Sound files are `/mnt/data/rolid-audio/sounds/gong.mp3` and `alarm.mp3`.
Both supplied files are a synthesized gong. Alarm audio repeats every five seconds;
use recordings up to five seconds. Stop/snooze stops this service's player.
75% maps to madplay -A -10; 0% is mute. To replace a sound, stop any active alarm,
then run on your computer:

```sh
ssh root@DECK_IP 'cat > /mnt/data/rolid-audio/sounds/gong.mp3' < my-gong.mp3
```

Use `alarm.mp3` as the destination for the alarm recording. Reinstallation preserves
custom recordings. To disable the service on the Deck:

```sh
/etc/init.d/rolid-audio stop
/etc/init.d/rolid-audio disable
```

This leaves recordings in place. To remove it entirely, back up recordings, then
remove only `/etc/init.d/rolid-audio` and `/mnt/data/rolid-audio`.
After a firmware upgrade, the init script may need reinstalling.

## Troubleshooting and validation

On the Deck, `wget -O - 'http://127.0.0.1:18764/cgi-bin/audio?status'` must return
`rolid-audio-v1 ready`. Check `/tmp/rolid-audio/player.log` or restart with
`/etc/init.d/rolid-audio restart`. A successful HTTP command is not proof of audible sound.
The widget and service must both be running. Native SDK audio is inactive on the
tested firmware, so disabling the MP3 bridge does not provide working sound there.

Hardware tested: system `2026-09-11-0-ff0d18a2-26.09`, packages
`2026.10.26-43d1b6e7-26.10`. RGB and MP3 test playback were confirmed on the device
before localization. All languages are tested in WASM with a mock host; localized
layout, reboot startup and long-running alarms still need device verification.

[Build and tests](docs/BUILD.md) · [Release notes](RELEASE_NOTES.md) ·
[Validation](docs/VALIDATION.md) · [Third-party notices](THIRD_PARTY.md)
