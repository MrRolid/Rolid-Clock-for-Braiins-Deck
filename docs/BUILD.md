# Sestavení a testování

Linux, Git, Rust 1.99.1 s targetem wasm32-unknown-unknown, Python 3.11+ a
`nix-store` (například balíček nix-bin). Pro testy navíc Python balíček wasmtime.
SDK se stahuje do `../bmc-main`, proto nepoužívejte pro tento checkout název bmc-main.
Existující SDK musí být na přesné revizi z build.sh; skript cizí checkout nepřepíná.
Cargo.lock i manifest-check/Cargo.lock jsou součástí repozitáře.

```sh
rustup toolchain install 1.99.1 --profile minimal --target wasm32-unknown-unknown
python3 -m venv .venv
. .venv/bin/activate
pip install -r requirements-test.txt
sh build.sh
sh test.sh
```

Výsledkem je `rolid-clock-0.5.1.nar`. Build používá lokální adresář target,
validuje manifest oficiálním parserem, sestaví WASM a zabalí skutečný Nix Archive.
Nepotřebuje Nix daemon ani registraci balíčku v Nix store.

Pro všechny soubory vydání po úspěšném sestavení spusťte:

```sh
python3 scripts/release.py
```

V dist vznikne NAR, pomocná zvuková služba, zdrojový ZIP a SHA256SUMS.
SHA256SUMS zkontrolujte příkazem `sha256sum -c SHA256SUMS` ve složce dist.
GitHub Actions opakuje build a testy, ověřuje rozbalení NAR a uloží dist jako
workflow artifact; samo nic nepublikuje do Releases.

Testy pokrývají plánování a DST, manifest včetně původní chyby unique_items,
WASM host, zobrazení na čtyřech velikostech, RGB, zvukové HTTP požadavky a CGI.
Instalační regrese ověřuje režim CGI 755 a hlášení selhané HTTP kontroly v izolaci.
CI neověřuje skutečný reproduktor, fyzický pásek ani procd na zařízení.

Zvuky WAV regeneruje `make_sounds.py`. Přiložené MP3 jsou vytvořené z gong.wav;
obě MP3 jsou v první veřejné verzi stejný gong. Přegenerování vyžaduje ffmpeg:

```sh
python3 make_sounds.py
ffmpeg -y -i assets/gong.wav -codec:a libmp3lame -b:a 96k audio-service/sounds/gong.mp3
cp audio-service/sounds/gong.mp3 audio-service/sounds/alarm.mp3
```
