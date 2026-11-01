#!/usr/bin/env python3
"""Package a built release; no upload or publication."""
import hashlib, json, shutil, tarfile, zipfile
from pathlib import Path
root = Path(__file__).resolve().parents[1]
version = json.loads((root / 'manifest.json').read_text())['version']
dist = root / 'dist'
dist.mkdir(exist_ok=True)
nars = [root / f'rolid-clock-{version}-{lang}.nar' for lang in ['cz','en','de','es']]
if not all(f.is_file() for f in nars):
    raise SystemExit('Run sh build.sh first')
for nar in nars: shutil.copyfile(nar, dist / nar.name)
audio = dist / f'rolid-audio-{version}.tar.gz'
with tarfile.open(audio, 'w:gz') as archive:
    for f in sorted((root / 'audio-service').rglob('*')):
        if f.is_file() and '__pycache__' not in f.parts:
            archive.add(f, arcname=str(f.relative_to(root / 'audio-service')))
source = dist / f'rolid-clock-{version}-source.zip'
skip = {'.git', 'target', 'dist', 'package-root', '.venv', '__pycache__', 'build-manifests', 'package-root-cz', 'package-root-en', 'package-root-de', 'package-root-es'}
with zipfile.ZipFile(source, 'w', zipfile.ZIP_DEFLATED) as archive:
    for f in sorted(root.rglob('*')):
        rel = f.relative_to(root)
        if any(part in skip for part in rel.parts) or f.suffix in {'.nar', '.pyc'}:
            continue
        if f.is_file():
            archive.write(f, str(Path('rolid-clock') / rel))
assets = [*[dist / nar.name for nar in nars], audio, source]
(dist / 'SHA256SUMS').write_text(''.join(hashlib.sha256(f.read_bytes()).hexdigest() + '  ' + f.name + '\n' for f in assets))
print('Release assets:', dist)
