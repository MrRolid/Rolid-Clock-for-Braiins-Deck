#!/bin/sh
set -eu
cd "$(dirname "$0")"
export CARGO_TARGET_DIR="$PWD/target"
cargo +1.99.1 test --locked --lib
cargo +1.99.1 test --locked --manifest-path manifest-check/Cargo.toml
python3 tests/smoke.py
python3 tests/audio_service.py
python3 tests/installer.py
sh -n audio-service/install.sh
sh -n audio-service/rolid-audio.init
sh -n audio-service/www/cgi-bin/audio
sh -n audio-service/stop.sh
