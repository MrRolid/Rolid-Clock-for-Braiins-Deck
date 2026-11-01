# Ověření vydání 0.5.1

Datum: 2026-11-01.

- Sestavení WASM v release režimu: úspěšné.
- Oficiální parser manifestu: úspěšné.
- Rust model: 12/12 testů.
- Parser manifestu: 2/2 testů včetně regrese unique_items.
- Wasmtime 50.0.1: všechny scénáře ve tests/smoke.py prošly.
- Izolovaný shell CGI: všechny scénáře tests/audio_service.py prošly.
- Izolovaný instalátor: CGI má S_IXOTH (755), soukromý kořen 700, HTTP chyba je vypsána.
- Shell syntaxe: install.sh, init služba, CGI a stop.sh prošly.
- NAR: dump → restore → diff bez rozdílů.

Na fyzickém zařízení uživatel potvrdil RGB test a vlastní MP3 tlačítkem Zvuk
ve verzi 0.5.0 po ruční opravě CGI na 755. Verze 0.5.1 obsahuje tuto opravu
v instalátoru; logika widgetu se nemění. Nový instalační archiv jako celek
zatím nebyl znovu spuštěn na Decku. Restart celého zařízení a dlouhodobé buzení
zatím nejsou samostatně potvrzené. GitHub Actions workflow je připravený,
na GitHubu dosud neběžel; jeho build/test příkazy byly provedeny lokálně.
