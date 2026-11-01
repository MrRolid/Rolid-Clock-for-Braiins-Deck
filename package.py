from pathlib import Path
import shutil, sys
root=Path(__file__).parent
manifest=Path(sys.argv[1]) if len(sys.argv)>1 else root/'manifest.json'
output=Path(sys.argv[2]) if len(sys.argv)>2 else root/'package-root'
widget=output/'lib/bmc-widgets/rolid/rolid-clock'
(widget/'bin').mkdir(parents=True,exist_ok=True)
(widget/'lib/wasm').mkdir(parents=True,exist_ok=True)
(widget/'lib/assets').mkdir(parents=True,exist_ok=True)
shutil.copyfile(manifest,widget/'manifest.json')
shutil.copyfile(root/'target/wasm32-unknown-unknown/release/rolid_clock.wasm',widget/'lib/wasm/rolid-clock.wasm')
launcher=widget/'bin/rolid-clock'
launcher.write_text('#!/bin/sh\nexec /run/current-profile/bin/bmc-wasm-thin-v0 \\\n    --wasm "${0%/*}/../lib/wasm/rolid-clock.wasm" \\\n    --asset-root "${0%/*}/../lib/assets" \\\n    "$@"\n')
launcher.chmod(0o755)
