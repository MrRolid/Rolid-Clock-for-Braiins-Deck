# RoliD Clock 0.6.0 — EN / DE / ES / CZ

- Localized display, weekdays, dates, controls, errors and configuration forms.
- Four alternative NAR packages: install only one, all share the same UUID.
- Live display language selector; form language follows the installed NAR.
- English alarm rule syntax added; all previous Czech rules remain supported.
- Existing custom alarm names and schedules are preserved.
- RGB behavior unchanged: midnight cycles colors across the whole strip.
- Includes the CGI 755 fix for custom MP3 audio.

Choose `rolid-clock-0.6.0-en.nar`, `-de.nar`, `-es.nar` or `-cz.nar`.
The shared `rolid-audio-0.6.0.tar.gz` is required for custom MP3 sound.
Also attached: `rolid-clock-0.6.0-source.zip` and `SHA256SUMS`.

The working audio service from 0.5.1 (or manually repaired 0.5.0) does not need
reinstalling just to change the widget language. Previously saved language
preferences may override the package default; use the Language selector.

Hardware RGB and MP3 were confirmed before localization. Localized WASM, parser,
CGI and installer checks pass locally. Physical localized layout, reboot startup
and long-duration alarms still require hardware verification. Mark as pre-release.

EN: English README.md · DE: README.de.md · ES: README.es.md · CZ: README.cz.md.
