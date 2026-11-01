# RoliD Clock 0.5.1 — první veřejné vydání

Neoficiální widget pro Braiins Deck s českým rozhraním.

- Nastavitelný čas, den a datum, barvy a velikosti písma.
- Pět nejbližších zazvonění vlevo a odpočet do dalšího budíku.
- Až 32 pravidel budíků, opakování, zastavení a odložení.
- Hodinové RGB upozornění a noční oranžová s výchozí intenzitou 15 %.
- Půlnoční střídání barev celého pásku (nikoli duhový přejezd).
- Vlastní MP3 gong a budíky přes pomocnou službu instalovanou přes SSH.
- Oprava instalátoru: CGI oprávnění 755 místo 700 odstraňuje HTTP 403.
- Viditelná chyba při neúspěšné HTTP kontrole během instalace.

## Soubory

| Soubor | Použití |
|---|---|
| rolid-clock-0.5.1.nar | Nahrát přes Display Widgets → Install Widget |
| rolid-audio-0.5.1.tar.gz | Nainstalovat přes SSH podle README pro přehrávání MP3 |
| rolid-clock-0.5.1-source.zip | Zdrojový kód, build a testy |
| SHA256SUMS | Kontrolní součty všech tří souborů |

Přehrávání tlačítkem Zvuk a RGB test jsou potvrzené na Decku. Automatické testy
pokrývají plánování, WASM a CGI. Start služby po restartu a dlouhodobé buzení
zatím nejsou samostatně potvrzené na zařízení; jde o první veřejnou testovací verzi.

Testovaný systém: 2026-09-11-0-ff0d18a2-26.09; packages: 2026.10.26-43d1b6e7-26.10.
Při aktualizaci z testovacích verzí zůstává UUID widgetu stejné. Instalátor služby
zachová vlastní MP3. Pro staré 17sloupcové budíky viz docs/NASTAVENI.md.
