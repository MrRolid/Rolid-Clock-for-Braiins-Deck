# Původ a závislosti

RoliD Clock: GPL-3.0-or-later, viz LICENSE.
Braiins Deck SDK a parser manifestu: BraiinsForge/bmc-main, GPL-3.0-or-later,
revize b2c945b5f7a8ea3a7d38a8410a1ca64e8b69b9af. Jejich vlastní copyright
a licenční oznámení zůstávají v upstream zdrojích. Repo SDK se nevkládá do tohoto ZIP.

Další Rust závislosti a přesné verze jsou v obou Cargo.lock; licence se řídí
příslušnými upstream balíčky. uhttpd, madplay a procd jsou programy zařízení,
jejich binární soubory zde nedistribuujeme. Název Braiins Deck označuje cílové
zařízení; nejde o oficiální projekt Braiins Forge.

Zvukové WAV jsou syntetizované skriptem make_sounds.py, MP3 gong a alarm
v audio-service/sounds jsou převodem tohoto gongu. Nepoužívají externí nahrávky.
Zvuky jsou distribuovány pod GPL-3.0-or-later spolu s generátorem.
