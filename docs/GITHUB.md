# První publikace na GitHub

Doporučený název repozitáře: `rolid-clock`.
Description: `Customizable clock, alarms, MP3 chimes and RGB effects for Braiins Deck.`
Topics: `braiins-deck`, `widget`, `rust`, `wasm`, `clock`, `alarm`, `openwrt`.
Licence je již přiložena; při zakládání prázdného repozitáře nepřidávejte další README/licenci.

1. Rozbalte balíček a nahrajte **obsah adresáře rolid-clock** do prázdného repozitáře.
   Binární soubory vedle v `release-assets` do zdrojového repozitáře nepřidávejte.
   Pokud používáte Git, nahraďte VAS_UCET skutečným účtem:

   ```sh
   cd rolid-clock
   git init -b main
   git add .
   git commit -m "Initial public release v0.6.0"
   git remote add origin https://github.com/VAS_UCET/rolid-clock.git
   git push -u origin main
   ```

2. Po kontrole výsledku GitHub Actions vytvořte GitHub Release s tagem `v0.6.0`
   z větve main a názvem **RoliD Clock 0.6.0 — první veřejné vydání**.
   Do popisu vložte obsah RELEASE_NOTES.md, označte vydání jako **pre-release**
   kvůli omezenému hardwarovému ověření a přiložte všech sedm souborů z
   `release-assets`: čtyři jazykové NAR, společný audio tar.gz, source.zip a SHA256SUMS.

UUID widgetu zachovejte i v dalších verzích. Pro další release aktualizujte
Cargo.toml, vlastní záznam v Cargo.lock, manifest.json a návody; číslo NAR
sestavovací skript čte z manifestu. Nikdy nenahrazujte verze globálně v Cargo.lock,
protože obsahuje také nesouvisející závislosti.
