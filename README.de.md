[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Čeština](README.cz.md)

# RoliD Clock 0.6.0

Inoffizielles Widget für **Braiins Deck**: anpassbare Uhr, Datum und Wochentag,
Wecker, Countdown, stündliche RGB-Effekte und eigene MP3-Töne.
Keine Verbindung zum Hersteller Braiins Forge. Lizenz: [GPL-3.0-or-later](LICENSE).

## Sprache auswählen

Installiere **nur eine** Variante: `rolid-clock-0.6.0-de.nar`, `-en.nar`, `-es.nar`
oder `-cz.nar`. Alle haben dieselbe UUID und ersetzen einander. Die Variante
bestimmt die Sprache des Einstellungsformulars und die Standardsprache der Anzeige.
Mit **Sprache** lässt sich die Anzeige umschalten; das Formular bleibt unverändert.
Bei Updates kann eine gespeicherte Sprachwahl erhalten bleiben. Eigene Weckernamen
werden nicht übersetzt.

## Installation

Lade die gewünschte NAR-Datei und `rolid-audio-0.6.0.tar.gz` aus Releases herunter.
Ersetze `DECK_IP` durch die IP-Adresse des Decks.

1. Für MP3 auf deinem Computer im Downloadordner ausführen:

   ```sh
   ssh root@DECK_IP 'mkdir -p /tmp/rolid-audio-060 && tar -xz -C /tmp/rolid-audio-060 && sh /tmp/rolid-audio-060/install.sh' < rolid-audio-0.6.0.tar.gz
   ```

2. Im Deck-Webinterface unter **Display Widgets → Install Widget** die NAR-Datei
   hochladen und das Widget einer Szene hinzufügen. Den lokalen MP3-Dienst aktiviert
   lassen, Gong-Lautstärke auf 75% stellen und **Ton** drücken. Den stündlichen
   Gong bei Bedarf separat einschalten.

Erforderlich: Root-Zugang per SSH, uhttpd, madplay und OpenWrt/procd. Der Dienst
lauscht nur auf `127.0.0.1:18764`. CGI-Rechte 755 verhindern den Fehler HTTP 403.
Ohne Dienst funktionieren Uhr und Licht, aber keine eigenen MP3-Töne.

## Anzeige und Wecker

Farben und Schriftgrößen von Uhrzeit, Datum und Countdown sind einstellbar.
Datum und Wochentag stehen in einer Zeile. Anzeige im 24-Stunden-Format,
Datum TT.MM.JJJJ. Links stehen die fünf nächsten Auslösungen; ein wiederholter
Wecker kann mehrfach erscheinen. Bis zu 32 Regeln, Schlummern, Stoppen und
automatisches Abschalten. Optional wird der nächste Systemwecker des Decks
angezeigt; dessen Ton wird nicht verändert.

Die Felder heißen **Zeit**, **Wiederholung**, **Name**. Zeit: `07:00` oder `7:00`.
**Regeln werden in allen Sprachen auf Englisch eingegeben.** Alte tschechische
Regeln funktionieren weiterhin. Ein Sprachwechsel verändert keine Zeitplanung.

| Wiederholung | Bedeutung |
|---|---|
| Mon-Fri | Montag bis Freitag |
| Sat,Sun | Wochenende |
| Mon,Tue,Sat | Ausgewählte Tage |
| daily | Täglich |
| off | Regel deaktivieren |
| once 2026-12-24 | Einmaliges Datum |
| monthly 15 | Am 15. jedes Monats |
| yearly 24.12 | Jährlich am 24. Dezember (Tag.Monat) |
| days 2 2026-11-01 | Alle zwei Tage ab diesem Datum |
| hours 6 2026-11-01 | Alle sechs Stunden ab Datum und Feld Zeit |
| minutes 30 2026-11-01 | Alle 30 Minuten ab Datum und Feld Zeit |
| Mon,Wed weeks 2 2026-11-02 | Montag/Mittwoch alle zwei Wochen ab Datum |

Tage: Mon,Tue,Wed,Thu,Fri,Sat,Sun. Keine Leerzeichen in Tageslisten.
Intervalle 1–365. Datum JJJJ-MM-TT; jährlich Tag.Monat. Ungültige Regeln zeigen
einen Fehler. Der 31. wird in Monaten ohne diesen Tag übersprungen. Zeitzone
und Sommerzeit des Decks werden berücksichtigt. Alte Regeln aus dem früheren
17-Spalten-Editor vor dem Bearbeiten notieren und im neuen Format neu eingeben.
Während ein Wecker aktiv oder aufgeschoben ist, seine Zeile nicht verschieben.

## RGB und MP3

Stündliches Licht tagsüber gelb, nachts orange. Nacht: standardmäßig 22–8 Uhr
oder Systemnachtmodus; Intensität standardmäßig 15%, einstellbar von 1–15%.
Um Mitternacht wechselt der gesamte Streifen durch zwölf Farben im Sekundentakt.
**Kein wandernder Regenbogen:** Die Widget-API erlaubt keine einzelnen LED-Farben.
Gong und Licht haben getrennte Zeitfenster. Gleiche Start-/Endstunde bedeutet
ganztägig. Verpasste Stundenereignisse werden nicht nachgeholt. Andere Widgets
und Systemeinstellungen können Vorrang haben. Testtasten: **Licht**, **RGB**, **Ton**.

Dateien: `/mnt/data/rolid-audio/sounds/gong.mp3` und `alarm.mp3`. Beide enthalten
einen synthetischen Gong. Der Weckton startet alle fünf Sekunden neu; Aufnahmen
bis fünf Sekunden verwenden. Stoppen und Schlummern beenden unseren Player.
75% entspricht madplay -A -10, 0% ist stumm. Eigene Aufnahme bei gestopptem Wecker
vom Computer hochladen:

```sh
ssh root@DECK_IP 'cat > /mnt/data/rolid-audio/sounds/gong.mp3' < mein-gong.mp3
```

Für Wecker als Ziel `alarm.mp3` verwenden. Neuinstallation erhält eigene MP3.
Dienst auf dem Deck deaktivieren:

```sh
/etc/init.d/rolid-audio stop
/etc/init.d/rolid-audio disable
```

Für vollständige Entfernung Aufnahmen sichern und nur `/etc/init.d/rolid-audio`
sowie `/mnt/data/rolid-audio` entfernen. Nach einem Firmwareupdate kann eine
Neuinstallation des Init-Skripts nötig sein.

## Diagnose und Teststatus

Auf dem Deck muss `wget -O - 'http://127.0.0.1:18764/cgi-bin/audio?status'`
`rolid-audio-v1 ready` liefern. Player-Log: `/tmp/rolid-audio/player.log`.
Neustart: `/etc/init.d/rolid-audio restart`. HTTP-Erfolg beweist noch keinen hörbaren Ton.
Widget und Dienst müssen laufen. Die native SDK-Tonausgabe funktioniert auf der
getesteten Firmware nicht; Abschalten des MP3-Dienstes ist dort kein Ersatz.

Getestetes Gerät: System `2026-09-11-0-ff0d18a2-26.09`, Pakete
`2026.10.26-43d1b6e7-26.10`. RGB und MP3-Test vor der Übersetzung am Gerät bestätigt.
Alle Sprachen werden im WASM-Testhost geprüft. Übersetztes Layout, Start nach
Geräteneustart und langfristiger Weckbetrieb sind noch am Gerät zu überprüfen.

[Build-Anleitung (EN)](docs/BUILD.md) · [Release Notes (EN)](RELEASE_NOTES.md)
