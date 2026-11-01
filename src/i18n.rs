// SPDX-License-Identifier: GPL-3.0-or-later
#[derive(Clone, Copy)]
pub struct Lang(usize);
impl Lang {
 pub fn new(value: &str) -> Self {Self(match value {"en"=>1,"de"=>2,"es"=>3,_=>0})}
 pub fn text(self, key: &str) -> &'static str {match key {
  "alarm" => ["Budík","Alarm","Wecker","Alarma"][self.0],
  "active" => ["BUDÍK","ALARM","WECKER","ALARMA"][self.0],
  "alarms" => ["BUDÍKY","ALARMS","WECKER","ALARMAS"][self.0],
  "none" => ["Žádný","None","Keine","Ninguna"][self.0],
  "no_next" => ["Žádný další budík","No upcoming alarm","Kein nächster Wecker","No hay próxima alarma"][self.0],
  "in" => ["za","in","in","en"][self.0],
  "stop" => ["Zastavit","Stop","Stopp","Parar"][self.0],
  "snooze" => ["Odložit","Snooze","Später","Posponer"][self.0],
  "light" => ["Světlo","Light","Licht","Luz"][self.0],
  "rainbow" => ["Duha","RGB","RGB","RGB"][self.0],
  "sound" => ["Zvuk","Sound","Ton","Sonido"][self.0],
  "audio_error" => ["Zvuk: služba nedostupná – zkontroluj instalaci","Audio service unavailable – check installation","Audiodienst nicht verfügbar – Installation prüfen","Servicio de audio no disponible – revisa la instalación"][self.0],
  "audio_test" => ["Zvuk: vyzkoušej tlačítko Zvuk","Audio: try the Sound button","Audio: Taste Ton testen","Audio: prueba el botón Sonido"][self.0],
  "sdk" => ["Zvuk přes SDK není ověřen","SDK audio is not verified","SDK-Audio ist nicht geprüft","Audio del SDK sin verificar"][self.0],
  "no_timer" => ["Časovač není dostupný – budíky neběží","Timer unavailable – alarms inactive","Timer nicht verfügbar – Wecker inaktiv","Temporizador no disponible – alarmas inactivas"][self.0],
  "zone" => ["Neznámé pásmo","Unknown time zone","Unbekannte Zeitzone","Zona horaria desconocida"][self.0],
  "utc" => ["používám UTC","using UTC","UTC wird verwendet","se usa UTC"][self.0],
  "duplicate" => ["Duplicitní ID budíku","Duplicate alarm ID","Doppelte Wecker-ID","ID de alarma duplicado"][self.0],
  "neplatný čas, ID nebo interval" => ["neplatný čas, ID nebo interval","invalid time, ID or interval","Ungültige Zeit, ID oder Intervall","hora, ID o intervalo no válido"][self.0],
  "vyber alespoň jeden den" => ["vyber alespoň jeden den","select at least one day","Mindestens einen Tag wählen","selecciona al menos un día"][self.0],
  "zadej počáteční datum pro interval týdnů" => ["zadej počáteční datum pro interval týdnů","set a start date for the weekly interval","Startdatum für das Wochenintervall angeben","indica la fecha inicial del intervalo semanal"][self.0],
  "zadej datum RRRR-MM-DD" => ["zadej datum RRRR-MM-DD","enter a date YYYY-MM-DD","Datum JJJJ-MM-TT eingeben","introduce una fecha AAAA-MM-DD"][self.0],
  "den musí být 1–31" => ["den musí být 1–31","day must be 1–31","Tag muss zwischen 1 und 31 liegen","el día debe estar entre 1 y 31"][self.0],
  "neplatný měsíc a den" => ["neplatný měsíc a den","invalid month and day","Ungültiger Monat und Tag","mes y día no válidos"][self.0],
  "neznámé opakování" => ["neznámé opakování","unknown repeat rule","Unbekannte Wiederholung","regla de repetición desconocida"][self.0],
  "zkontroluj čas a opakování" => ["zkontroluj čas a opakování","check time and repeat rule","Zeit und Wiederholung prüfen","revisa la hora y la repetición"][self.0],
  _ => "?",
 }}
 pub fn weekday(self, day: usize) -> &'static str {[["pondělí","úterý","středa","čtvrtek","pátek","sobota","neděle"],["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],["Montag","Dienstag","Mittwoch","Donnerstag","Freitag","Samstag","Sonntag"],["lunes","martes","miércoles","jueves","viernes","sábado","domingo"]][self.0][day]}
 pub fn short_day(self, day: usize) -> &'static str {[["Po","Út","St","Čt","Pá","So","Ne"],["Mon","Tue","Wed","Thu","Fri","Sat","Sun"],["Mo","Di","Mi","Do","Fr","Sa","So"],["Lu","Ma","Mi","Ju","Vi","Sá","Do"]][self.0][day]}
 pub fn date_format(self) -> &'static str {["%d. %m. %Y","%d/%m/%Y","%d.%m.%Y","%d/%m/%Y"][self.0]}
 pub fn error(self, message: &str) -> String {
 let mut out=message.to_owned();
 out=out.replace("neplatný čas, ID nebo interval",self.text("neplatný čas, ID nebo interval"));
 out=out.replace("vyber alespoň jeden den",self.text("vyber alespoň jeden den"));
 out=out.replace("zadej počáteční datum pro interval týdnů",self.text("zadej počáteční datum pro interval týdnů"));
 out=out.replace("zadej datum RRRR-MM-DD",self.text("zadej datum RRRR-MM-DD"));
 out=out.replace("den musí být 1–31",self.text("den musí být 1–31"));
 out=out.replace("neplatný měsíc a den",self.text("neplatný měsíc a den"));
 out=out.replace("neznámé opakování",self.text("neznámé opakování"));
 out=out.replace("zkontroluj čas a opakování",self.text("zkontroluj čas a opakování"));
 out.replace("Budík",self.text("alarm"))
 }
}
