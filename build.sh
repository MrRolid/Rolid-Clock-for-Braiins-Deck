#!/bin/sh
set -eu
cd "$(dirname "$0")"
export CARGO_TARGET_DIR="$PWD/target"
SDK_REV=b2c945b5f7a8ea3a7d38a8410a1ca64e8b69b9af
if [ ! -d ../bmc-main/.git ]; then
    GIT_LFS_SKIP_SMUDGE=1 git clone --no-checkout https://github.com/BraiinsForge/bmc-main.git ../bmc-main
    GIT_LFS_SKIP_SMUDGE=1 git -C ../bmc-main checkout "$SDK_REV"
fi
if [ "$(git -C ../bmc-main rev-parse HEAD)" != "$SDK_REV" ]; then
    echo "SDK checkout must be at $SDK_REV; use a separate checkout to avoid changing existing work." >&2
    exit 1
fi
cargo +1.99.1 run --locked --manifest-path manifest-check/Cargo.toml -- manifest.json
python3 make_sounds.py
cargo +1.99.1 build --locked --release --target wasm32-unknown-unknown
python3 package.py
version=$(python3 -c 'import json; print(json.load(open("manifest.json"))["version"])')
nix-store --dump package-root > "rolid-clock-$version.nar"
