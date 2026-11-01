# Nastavení RoliD Clock

## Opravený editor budíků
Původní tabulka měla 17 sloupců, které web Decku stlačil do jednoho řádku.
Nová má jen tři textová pole: Čas, Opakování, Název. Až 32 pravidel.
Čas se zadává jako 7:00 nebo 07:00. Výchozí nový řádek: 07:00, Po-Pa, Budík.

| Opakování | Význam |
|---|---|
| Po-Pa | Pracovní dny |
| So,Ne | Víkend |
| Po,Ut,So | Libovolný výběr dnů |
| denne | Každý den |
| vyp | Dočasně vypnutý řádek |
| jednou 2026-12-24 | Jediné datum |
| mesicne 15 | Každého 15. dne v měsíci |
| rocne 24.12 | Každoročně 24. prosince |
| dny 2 2026-11-01 | Každé dva dny od data |
| hodiny 6 2026-11-01 | Každých šest hodin od data a času v poli Čas |
| minuty 30 2026-11-01 | Každých 30 minut od data a času v poli Čas |
| Po,St tydny 2 2026-11-02 | Po a St každý druhý týden od data |

Zkratky dnů: Po, Ut, St, Ct, Pa, So, Ne. Lze i českou diakritiku.
U seznamu dnů nedávejte mezery za čárku. Interval je 1–365.
Neexistující datum se odmítne; měsíční 31. den se v krátkém měsíci přeskočí.
Budoucí termíny respektují pásmo Decku a změny letního času.
Chyba zápisu se zobrazí na widgetu a daný budík se nespustí.
Web kontroluje délku textu; význam pravidla kontroluje widget po uložení.

Starý formát widget umí číst, dokud jsou původní hodnoty zachované.
Před úpravou již uložených starých řádků si poznamenejte jejich pravidla
 a v novém editoru je zadejte znovu: web už původní sloupce nezobrazuje.
Při změně pořadí řádků nepřesouvejte právě odložený nebo aktivní budík.

## Displej
Den a datum tvoří jeden řádek se společnou barvou a velikostí (volby data).
Den a datum lze jednotlivě skrýt. Čas a odpočet mají vlastní barvy/velikosti.
Levý sloupec zobrazuje nejbližších pět událostí, nikoli pět definic budíků.
Například opakovaný Po/Ut 7:00 se objeví jako Po 7:00 a Ut 7:00.
Připočítají se odložené budíky a nejbližší systémový budík, pokud je povolen.
SDK poskytuje jen jeden příští systémový budík, jeho další termíny nehádáme.
Při menším počtu budoucích událostí je kartiček méně; prázdné se nedoplňují.
Sloupec lze vypnout. Rozložení podporuje 317×238 až 1280×480.

## RGB
Hodinové RGB je standardně zapnuté po celý den i noc. Ve dne je žluté,
v noci velmi tmavě oranžové. Noc je 22:00–08:00 nebo systémový noční režim.
Oranžová má RGB 38,9,0 při výchozí 15% intenzitě; intenzita je nastavitelná 1–15 %.
Půlnoční upozornění nahrazuje přejezd duhovým střídáním barev celého pásku
na přibližně 12 sekund. I duha je v noci ztlumená. SDK neumožňuje řídit barvu
jednotlivých LED, takže nejde o prostorovou duhu s více barvami současně.

Test světla použije aktuální denní/noční barvu. Duha okamžitě otestuje duhu.
Změna nastavení ukončí právě testovaný efekt. Půlnoční duhu lze vypnout.
Pro původní časové okno vypněte „Hodinové světlo po celý den i noc“.
Půlnoční duha má vlastní volbu a časové okno obchází, ale respektuje hlavní
vypínač hodinového RGB. Efekty se spustí jen v prvních 5 sekundách hodiny.
Zmeškané efekty se nedohánějí a restart stejnou hodinovou událost neopakuje.
Lokální LED efekty jiných widgetů a systémové nastavení mohou mít přednost.
Widget musí běžet; SDK timer funguje i mimo právě zobrazenou scénu.

## Vlastní MP3 (od 0.5.0)
Widget umí volat místní službu rolid-audio, která přehrává MP3 přes madplay.
Instalace a diagnostika jsou v ../audio-service/README.md a v samostatném
archivu rolid-audio-0.6.0.tar.gz. Vyžaduje jednorázovou instalaci přes SSH.
Bez této služby je výchozí režim bez zvuku a po pokusu zobrazí chybu.
Hodinový gong, vlastní budíky a tlačítko Zvuk používají příslušné hlasitosti.
Odložení/Zastavit ukončí přehrávání. Při každém opakování budíku po 5 s
se předchozí přehrání ukončí. Předvolba gongu zůstává vypnutá, je-li dosud vypnutá.
Tlačítka na displeji: Světlo, Duha, Zvuk. Vlastní budíky mají Zastavit/Odložit.
Po instalaci služby použijte tlačítko Zvuk. 75 % odpovídá přibližně -10 dB.
Služba je lokální (127.0.0.1), uložená v /mnt/data a spuštěná přes procd.
Při vypnutí volby služby se použije původní SDK cesta, která na standardním
hostu bez feature audio stále nic nepřehraje. Přehrání MP3 tlačítkem Zvuk bylo ověřeno na skutečném Decku po opravě
oprávnění CGI v instalátoru. Dlouhodobé buzení a start po restartu zatím
nejsou na zařízení samostatně ověřeny.


## Jazyk (0.6.0)

Volba Jazyk mění displej, nainstalovaná jazyková varianta určuje texty formuláře. Anglické ekvivalenty pravidel jsou v ../README.md; obě syntaxe lze používat v každém jazyce.
