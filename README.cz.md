[English](README.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Čeština](README.cz.md)

# RoliD Clock

Neoficiální widget pro **Braiins Deck**: přizpůsobitelné hodiny, české datum,
budíky, odpočet a hodinové upozornění světlem i vlastním MP3.

**Vícejazyčné vydání: 0.6.0.** Navazuje na soukromé testovací verze 0.1–0.5.0.
Projekt není oficiálním produktem Braiins Forge. Rozhraní je česky, anglicky, německy a španělsky.

## Funkce

- Barva a velikost času, data a odpočtu; datum a den na jednom řádku.
- Levý sloupec s pěti nejbližšími zazvoněními.
- Až 32 pravidel: dny v týdnu, jednorázové datum, intervaly, měsíční a roční opakování.
- Odložení a zastavení vlastních budíků, odpočet do dalšího budíku.
- Volitelně také nejbližší systémový budík v přehledu (SDK poskytuje pouze jeden).
- Hodinové RGB efekty, v noci oranžová s výchozí intenzitou 15 %.
- Půlnoční střídání 12 barev celého pásku; nejde o prostorový duhový přejezd.
- Vlastní MP3 pro gong a budíky přes samostatnou lokální službu.

## Instalace

Stáhněte z GitHub Releases `rolid-clock-0.6.0-cz.nar` a `rolid-audio-0.6.0.tar.gz`.
Z GitHubem automaticky vytvořeného archivu zdrojů se widget neinstaluje.
Nahraďte `IP_DECKU` skutečnou IP adresou Decku.

1. Pro zvuk spusťte **na svém počítači ve složce se staženým archivem**:

   ```sh
   ssh root@IP_DECKU 'mkdir -p /tmp/rolid-audio-060 && tar -xz -C /tmp/rolid-audio-060 && sh /tmp/rolid-audio-060/install.sh' < rolid-audio-0.6.0.tar.gz
   ```

   Vyžaduje SSH přístup root a přítomné `madplay`, `uhttpd` a OpenWrt/procd.
   Úspěšná instalace vypíše `Installed: rolid-audio, loopback only at 127.0.0.1:18764.`

2. Ve webu Decku otevřete **Display Widgets → Install Widget**, nahrajte `.nar`
   a přidejte RoliD Clock do scény. Ponechte zapnuté přehrávání přes místní službu,
   nastavte hlasitost gongu například na **75 %** a stiskněte **Zvuk**.
   Hodinový gong zapněte samostatnou volbou; výchozí nastavení je vypnuté.

Bez pomocné služby fungují hodiny a RGB; při pokusu o MP3 se zobrazí chybové hlášení
na displeji, nikoliv zvuk. Standardní SDK audio na testovaném firmwaru nepřehrává.

## Dokumentace

- [Nastavení a syntaxe budíků](docs/NASTAVENI.md)
- [Zvuková služba, vlastní MP3 a odinstalace](audio-service/README.md)
- [Sestavení a testy](docs/BUILD.md)
- [Vydání na GitHubu](docs/GITHUB.md)
- [Poznámky k vydání 0.6.0](RELEASE_NOTES.md)
- [Historie změn](CHANGELOG.md)

## Kompatibilita a omezení

Vyvíjeno proti [BraiinsForge/bmc-main](https://github.com/BraiinsForge/bmc-main)
na revizi `b2c945b5f7a8ea3a7d38a8410a1ca64e8b69b9af`, protokol SDK 0.9.0.
Zařízení použité pro ruční ověření:

| Součást | Verze |
|---|---|
| System | `2026-09-11-0-ff0d18a2-26.09` |
| Packages | `2026.10.26-43d1b6e7-26.10` |

Na zařízení je potvrzen RGB test a přehrávání vlastního MP3 tlačítkem Zvuk.
Plánování, odložení, restarty stavu widgetu a komunikace jsou testovány automaticky
s napodobeným hostem. Celonoční provoz, budík ve skutečný čas a automatický start
služby po restartu zařízení ještě nejsou samostatně potvrzeny.

Widget musí běžet. Systémové nastavení LED a efekty jiných widgetů mohou mít
přednost. Půlnoční „duha“ střídá barvy po sekundě na celém pásku. Widgetové API
neumožňuje zadat jednotlivým LED různé barvy. Po aktualizaci firmwaru může být
nutné obnovit init skript zvukové služby. Vlastní systémové budíky Decku tato
služba nemění.

## Licence

[GPL-3.0-or-later](LICENSE). Vygenerované zvuky jsou součástí projektu pod stejnou
licencí. Viz [THIRD_PARTY.md](THIRD_PARTY.md). Zdrojové kódy připnutého SDK nejsou
součástí tohoto repozitáře; sestavovací skript je stáhne do sousedního adresáře.

## Jazyk

Nainstalujte pouze jednu variantu: `cz`, `en`, `de` nebo `es`. Všechny mají stejné UUID a vzájemně se nahrazují. Varianta určuje jazyk nastavení a výchozí jazyk displeje. Jazyk displeje lze změnit volbou Language / Sprache / Idioma / Jazyk. Při aktualizaci se může zachovat dříve uložený jazyk; pak jej změňte ručně. Názvy vlastních budíků se nepřekládají.

Nově fungují i anglické zápisy pravidel z README.md. Česká pravidla jsou zpětně kompatibilní. Jazyk displeje nemění význam pravidel.
