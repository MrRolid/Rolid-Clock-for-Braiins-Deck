# Validation 0.6.0 — 2026-11-01

- Rust scheduling/model tests: 13 passed, including English/Czech rule equivalence.
- Native manifest parser regression tests: 2 passed.
- All four generated manifests accepted by the native Deck parser.
- WASM tests: original scenarios plus EN/DE/ES/CZ at four viewport sizes passed.
- Date, weekday, controls, preview labels, audio and validation errors, and live language switching checked.
- CGI and isolated installer tests passed; CGI is executable as required by uhttpd.
- Shell syntax checks passed.
- Four NAR archives restored and compared against package trees without differences.
- All NARs have the same UUID and identical WASM; their manifests select the language.

Tests inspect the emitted UI tree; they are not screenshot/physical rendering tests.
GitHub Actions is prepared but has not run on GitHub. Hardware RGB and MP3
were confirmed before localization. Physical localized layout, reboot autostart
and long-duration alarms still need hardware verification.
