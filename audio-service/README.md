# RoliD audio service 0.5.1

Pro Braiins Deck s uhttpd, madplay a OpenWrt procd. Jednorázová instalace
přes SSH doplňuje přehrávání vlastního MP3 do RoliD Clock 0.5.1.

## Instalace
Na počítači, ve složce se staženým archivem:

    ssh root@IP_DECKU 'mkdir -p /tmp/rolid-audio-051 && tar -xz -C /tmp/rolid-audio-051 && sh /tmp/rolid-audio-051/install.sh' < rolid-audio-0.5.1.tar.gz

Pak nahrajte rolid-clock-0.5.1.nar přes Install Widget a stiskněte Zvuk.
Volba „Přehrávat vlastní MP3 přes místní službu“ je standardně zapnutá.
Gong každou hodinu má vlastní vypínač, který je potřeba zapnout, pokud je vypnutý.
Hlasitost 75 % odpovídá madplay -A -10, 88 % přibližně -5 dB. Nula je ticho.
Zachovají se starší uložené hodnoty hlasitosti; v případě potřeby je změňte.

## Co se instaluje
- /mnt/data/rolid-audio: skripty a dvě MP3 (gong.mp3 a alarm.mp3).
- /etc/init.d/rolid-audio: vlastní služba procd spouštěná při startu.
- HTTP pouze na 127.0.0.1:18764, nezveřejňuje se do LAN.
- Veřejný web Decku, jeho konfigurace ani firmware se nepřepisují.
- Dočasný PID a log přehrávače v /tmp/rolid-audio.

Při reinstalaci se existující vlastní MP3 zachovají a adresář se zálohuje.
Instalátor ověří potřebné programy a bind portu. Je-li dostupný HTTP klient,
ověří i CGI odpověď. Přijetí příkazu ještě nepotvrzuje fyzicky slyšitelný zvuk.
Po kompletní aktualizaci firmwaru může být nutné znovu spustit instalátor,
pokud systém neuchová vlastní init skript. Soubory jsou v /mnt/data.

## Vlastní nahrávky
Oba přiložené zvuky jsou příjemný gong. Pro budík se přehrání opakuje po
5 sekundách; před opakováním se předchozí přehrávání zastaví. Doporučená
nahrávka budíku do 5 sekund. Zastavit a Odložit ukončí jen náš přehrávač.
Ostatní systémové zvuky zůstávají nezávislé a mohou se s naším zvukem překrývat.
Tato verze nepřidává zvuk do nativní aplikace systémových budíků.

Nový zvuk lze nahrát z počítače (nejprve zastavte test/budík):

    ssh root@IP_DECKU 'cat > /mnt/data/rolid-audio/sounds/gong.mp3' < muj-gong.mp3

Pro vlastní budík změňte cílový název na alarm.mp3. Nedávejte zvuky do /tmp:
po restartu se toto úložiště vymaže.

## Diagnostika (na Decku)

    /etc/init.d/rolid-audio restart
    cat /tmp/rolid-audio/player.log
    logread | grep -E 'rolid-audio|18764'

Widget při neúspěšném HTTP požadavku ukáže „služba nedostupná“.
Příčina může být neběžící služba, obsazený port, CGI chyba nebo chybějící MP3.
Při úspěšném dalším testu hláška zmizí. Zvuk funguje pouze pokud běží widget
 i pomocná služba. Budíky widgetu nepřepínají automaticky zobrazenou scénu.

## Vypnutí a odinstalace (na Decku)

    /etc/init.d/rolid-audio stop
    /etc/init.d/rolid-audio disable

Poté lze smazat /etc/init.d/rolid-audio a /mnt/data/rolid-audio, chcete-li
odstranit i nahrávky. Nejprve je případně zálohujte. Vypněte ve widgetu
„Přehrávat vlastní MP3 přes místní službu“; standardní SDK audio však na
aktuálním hostu Decku stále nemá funkční přehrávání.

## HTTP protokol a testy
GET /cgi-bin/audio?status je kontrola dostupnosti.
Přehrání: POST /cgi-bin/audio?action=gong&volume=75 (nebo action=alarm).
Zastavení: POST /cgi-bin/audio?action=stop.
Zápisy vyžadují Authorization: Rolid-local-v1; není to tajný token,
ale hlavička, kterou běžný cizí web bez povoleného CORS nemůže odeslat.
Služba kontroluje loopback adresu, metodu a přesné povolené hodnoty.
Žádný parametr se nespouští jako příkaz a nelze vybrat libovolnou cestu.
Příkazy widget řadí postupně, aby stop nepředběhl předchozí přehrávání.
PID kontrola omezuje zastavování na madplay s cestou našich MP3.

Ověřeno: shell syntaxe, CGI s izolovanými cestami a testovacím přehrávačem,
WASM integrace požadavků, hlasitost, stop/odložení, chyba a obnovení spojení.
Přehrání tlačítkem Zvuk přes uhttpd a fyzický reproduktor uživatel potvrdil
1. listopadu 2026 po opravě CGI oprávnění na 755. Automatické spuštění po
restartu a dlouhodobé buzení zatím nebylo na zařízení samostatně ověřeno.
