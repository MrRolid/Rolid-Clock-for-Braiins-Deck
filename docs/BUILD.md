# Build and test

Linux, Git, Python 3.11+, Rust 1.99.1 and nix-store (e.g. nix-bin).
The build fetches BraiinsForge/bmc-main into ../bmc-main at revision
b2c945b5f7a8ea3a7d38a8410a1ca64e8b69b9af. An existing checkout must match;
the script will not switch unrelated working trees. Rust dependencies are locked.

```sh
rustup toolchain install 1.99.1 --profile minimal --target wasm32-unknown-unknown
python3 -m venv .venv
. .venv/bin/activate
pip install -r requirements-test.txt
sh build.sh
sh test.sh
python3 scripts/release.py
```

Build produces four rolid-clock-0.6.0-{cz,en,de,es}.nar packages with identical WASM
and translated manifests. Czech manifest.json is the base; locales/*.json supplies
other form translations. src/i18n.rs contains display strings. Each generated
manifest passes the native Deck parser. The build uses local target/ regardless
of a pre-existing CARGO_TARGET_DIR. Nix daemon is not needed for dump/restore.

Release assets appear in dist/: four NARs, shared audio-service archive, source
ZIP and SHA256SUMS. Verify with sha256sum -c SHA256SUMS inside dist/.
CI runs build/tests, verifies all four NAR restores and uploads artifacts without
publishing a GitHub Release. Hardware behavior is not tested by CI.

make_sounds.py generates WAV sources. To regenerate the included MP3 with ffmpeg:

```sh
python3 make_sounds.py
ffmpeg -y -i assets/gong.wav -codec:a libmp3lame -b:a 96k audio-service/sounds/gong.mp3
cp audio-service/sounds/gong.mp3 audio-service/sounds/alarm.mp3
```

Both bundled MP3 recordings are the same synthesized gong.
