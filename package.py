from pathlib import Path
import shutil
root=Path(__file__).parent
widget=root/'package-root/lib/bmc-widgets/rolid/rolid-clock'
(widget/'bin').mkdir(parents=True,exist_ok=True)
(widget/'lib/wasm').mkdir(parents=True,exist_ok=True)
(widget/'lib/assets').mkdir(parents=True,exist_ok=True)
shutil.copyfile(root/'manifest.json',widget/'manifest.json')
shutil.copyfile(root/'target/wasm32-unknown-unknown/release/rolid_clock.wasm',widget/'lib/wasm/rolid-clock.wasm')
launcher=widget/'bin/rolid-clock'
launcher.write_text('#!/bin/sh\nexec /run/current-profile/bin/bmc-wasm-thin-v0 \\\n    --wasm "${0%/*}/../lib/wasm/rolid-clock.wasm" \\\n    --asset-root "${0%/*}/../lib/assets" \\\n    "$@"\n')
launcher.chmod(0o755)
