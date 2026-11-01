use chrono::{Datelike, Duration, LocalResult, NaiveDate, TimeZone};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Alarm {
    pub id: i32,
    pub name: String,
    pub enabled: bool,
    pub mode: String,
    pub hour: u32,
    pub minute: u32,
    pub days: [bool; 7],
    pub date: Option<NaiveDate>,
    pub interval: u32,
    pub day: u32,
    pub month: u32,
}

impl Alarm {
    pub fn key(&self) -> String {
        let mut schedule = self.clone();
        schedule.name.clear();
        schedule.enabled = true;
        serde_json::to_string(&schedule).expect("alarm has only JSON values")
    }

    pub fn validate(&self) -> Result<(), String> {
        let fail = |s: &str| Err(format!("Budík {}: {}", self.id, s));
        if self.id <= 0
            || self.hour > 23
            || self.minute > 59
            || self.interval == 0
            || self.interval > 365
        {
            return fail("neplatný čas, ID nebo interval");
        }
        match self.mode.as_str() {
            "daily" => {}
            "weekly" => {
                if !self.days.iter().any(|v| *v) {
                    return fail("vyber alespoň jeden den");
                }
                if self.interval > 1 && self.date.is_none() {
                    return fail("zadej počáteční datum pro interval týdnů");
                }
            }
            "once" | "days" | "hours" | "minutes" => {
                if self.date.is_none() {
                    return fail("zadej datum RRRR-MM-DD");
                }
            }
            "monthly" => {
                if !(1..=31).contains(&self.day) {
                    return fail("den musí být 1–31");
                }
            }
            "yearly" => {
                if NaiveDate::from_ymd_opt(2024, self.month, self.day).is_none() {
                    return fail("neplatný měsíc a den");
                }
            }
            _ => return fail("neznámé opakování"),
        }
        Ok(())
    }

    pub fn next(&self, after: i64, zone: Tz, last: Option<i64>) -> Option<i64> {
        if !self.enabled || self.validate().is_err() {
            return None;
        }
        let after = after.max(last.unwrap_or(i64::MIN));
        let current = zone.timestamp_opt(after, 0).single()?;
        let today = current.date_naive();
        if self.mode == "hours" || self.mode == "minutes" {
            let start = local_stamp(zone, self.date?, self.hour, self.minute)?;
            let period = i64::from(self.interval) * if self.mode == "hours" { 3600 } else { 60 };
            return Some(if start > after {
                start
            } else {
                start + ((after - start) / period + 1) * period
            });
        }
        let candidates: Vec<NaiveDate> = match self.mode.as_str() {
            "once" => vec![self.date?],
            "monthly" => (0..=24)
                .filter_map(|n| {
                    let month_index = current.year() * 12 + current.month0() as i32 + n;
                    NaiveDate::from_ymd_opt(
                        month_index.div_euclid(12),
                        month_index.rem_euclid(12) as u32 + 1,
                        self.day,
                    )
                })
                .collect(),
            "yearly" => (0..=8)
                .filter_map(|n| NaiveDate::from_ymd_opt(current.year() + n, self.month, self.day))
                .collect(),
            "days" => {
                let start = self.date?;
                let delta = (today - start).num_days().max(0);
                let n = delta / i64::from(self.interval);
                (n..=n + 2)
                    .filter_map(|i| {
                        start.checked_add_signed(Duration::days(i * i64::from(self.interval)))
                    })
                    .collect()
            }
            "weekly" if self.interval > 1 => {
                let anchor = self.date?;
                let first_monday =
                    anchor - Duration::days(i64::from(anchor.weekday().num_days_from_monday()));
                let week = ((today - first_monday).num_days().max(0) / 7)
                    / i64::from(self.interval)
                    * i64::from(self.interval);
                [
                    week,
                    week + i64::from(self.interval),
                    week + 2 * i64::from(self.interval),
                ]
                .into_iter()
                .flat_map(|w| (0..7).map(move |d| first_monday + Duration::days(w * 7 + d)))
                .filter(|d| *d >= anchor && *d >= today)
                .collect()
            }
            _ => (0..=8)
                .filter_map(|i| today.checked_add_signed(Duration::days(i)))
                .collect(),
        };
        candidates
            .into_iter()
            .filter(|d| {
                self.mode != "weekly" || self.days[d.weekday().num_days_from_monday() as usize]
            })
            .filter_map(|d| local_stamp(zone, d, self.hour, self.minute))
            .filter(|t| *t > after)
            .min()
    }
}

/// Compact editor: time + repetition + optional name, without a wide table.
pub fn parse_alarm(id: i32, time: &str, rule: &str, name: &str) -> Result<Alarm, String> {
    let fail = || format!("Budík {id}: zkontroluj čas a opakování");
    let (h,m)=time.trim().split_once(':').ok_or_else(fail)?;
    let mut a=Alarm{id,name:if name.trim().is_empty(){format!("Budík {id}")}else{name.trim().into()},enabled:true,mode:"weekly".into(),hour:h.parse().map_err(|_|fail())?,minute:m.parse().map_err(|_|fail())?,days:[false;7],date:None,interval:1,day:1,month:1};
    let rule=rule.trim().to_lowercase().replace('č',"c").replace('ú',"u").replace('ě',"e").replace('ř',"r").replace('á',"a").replace('í',"i").replace('ý',"y");
    let parts:Vec<_>=rule.split_whitespace().collect();
    match parts.as_slice() {
        ["vyp"]|["off"]=>{a.enabled=false;a.mode="daily".into();},
        ["denne"]|["daily"]=>a.mode="daily".into(),
        ["jednou",date]=>{a.mode="once".into();a.date=chrono::NaiveDate::parse_from_str(date,"%Y-%m-%d").ok();},
        ["mesicne",day]=>{a.mode="monthly".into();a.day=day.parse().map_err(|_|fail())?;},
        ["rocne",date]=>{a.mode="yearly".into();let (d,m)=date.split_once('.').ok_or_else(fail)?;a.day=d.parse().map_err(|_|fail())?;a.month=m.trim_end_matches('.').parse().map_err(|_|fail())?;},
        [unit,n,date] if ["dny","hodiny","minuty"].contains(unit)=>{a.mode=match *unit {"dny"=>"days","hodiny"=>"hours",_=>"minutes"}.into();a.interval=n.parse().map_err(|_|fail())?;a.date=chrono::NaiveDate::parse_from_str(date,"%Y-%m-%d").ok();},
        _=>{
            let days=if parts.len()==1 {parts[0]} else if parts.len()==4 && parts[1]=="tydny" {a.interval=parts[2].parse().map_err(|_|fail())?;a.date=chrono::NaiveDate::parse_from_str(parts[3],"%Y-%m-%d").ok();parts[0]} else{return Err(fail())};
            let index=|d:&str|["po","ut","st","ct","pa","so","ne"].iter().position(|x|*x==d);
            for part in days.split(',') {
                if let Some((first,last))=part.split_once('-') {let first=index(first).ok_or_else(fail)?;let last=index(last).ok_or_else(fail)?;let mut i=first;loop{a.days[i]=true;if i==last{break}i=(i+1)%7;}}
                else {a.days[index(part).ok_or_else(fail)?]=true;}
            }
        }
    }
    a.validate()?;Ok(a)
}

pub fn upcoming(alarms: &[Alarm], saved: &Saved, now: i64, zone: Tz, count: usize) -> Vec<Occurrence> {
    let mut out:Vec<_>=saved.snoozed.iter().filter(|o|o.at>now).cloned().collect();
    for a in alarms {
        let key=a.key();let mut after=now;
        for _ in 0..count {
            let Some(at)=a.next(after,zone,saved.fired.get(&key).copied()) else{break};
            out.push(Occurrence{key:key.clone(),name:a.name.clone(),at});after=at;
        }
    }
    out.sort_by(|a,b|a.at.cmp(&b.at).then(a.key.cmp(&b.key)));out.dedup_by(|a,b|a.at==b.at && a.key==b.key);out.truncate(count);out
}

// Repeated autumn times ring only at the first occurrence; missing spring times are skipped.
fn local_stamp(zone: Tz, date: NaiveDate, hour: u32, minute: u32) -> Option<i64> {
    let local = date.and_hms_opt(hour, minute, 0)?;
    match zone.from_local_datetime(&local) {
        LocalResult::Single(t) => Some(t.timestamp()),
        LocalResult::Ambiguous(a, b) => Some(a.timestamp().min(b.timestamp())),
        LocalResult::None => None,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Occurrence {
    pub key: String,
    pub name: String,
    pub at: i64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Saved {
    #[serde(default)]
    pub fired: BTreeMap<String, i64>,
    #[serde(default)]
    pub snoozed: Vec<Occurrence>,
    #[serde(default)]
    pub active: Vec<Occurrence>,
    #[serde(default)]
    pub active_since: i64,
    #[serde(default)]
    pub gong: Option<String>,
    #[serde(default)]
    pub led_hour: Option<String>,
}

#[derive(Clone, Default)]
pub struct Engine {
    pub saved: Saved,
    pub previous: i64,
    pub next: Vec<Occurrence>,
    pub last_sound: i64,
}

impl Engine {
    pub fn configure(&mut self, alarms: &[Alarm], now: i64, zone: Tz) {
        let keys: Vec<_> = alarms
            .iter()
            .filter(|a| a.enabled && a.validate().is_ok())
            .map(Alarm::key)
            .collect();
        self.saved.fired.retain(|k, _| keys.contains(k));
        self.saved.snoozed.retain(|o| keys.contains(&o.key));
        self.saved.active.retain(|o| keys.contains(&o.key));
        self.next = alarms
            .iter()
            .filter_map(|a| {
                let key = a.key();
                a.next(now, zone, self.saved.fired.get(&key).copied())
                    .map(|at| Occurrence {
                        key,
                        name: a.name.clone(),
                        at,
                    })
            })
            .collect();
        self.previous = now;
    }

    pub fn tick(&mut self, alarms: &[Alarm], now: i64, zone: Tz, ring_seconds: i64) -> bool {
        if now < self.previous {
            self.configure(alarms, now, zone);
        }
        let lower = self.previous.max(now - 90);
        let mut due = Vec::new();
        for occurrence in &self.next {
            if occurrence.at > lower && occurrence.at <= now {
                self.saved
                    .fired
                    .insert(occurrence.key.clone(), occurrence.at);
                due.push(occurrence.clone());
            }
        }
        self.saved.snoozed.retain(|o| {
            if o.at > lower && o.at <= now {
                due.push(o.clone());
            }
            o.at > now
        });
        if !due.is_empty() {
            for occurrence in due {
                self.saved.active.retain(|o| o.key != occurrence.key);
                self.saved.active.push(occurrence);
            }
            self.saved.active_since = now;
            self.last_sound = 0;
        }
        if !self.saved.active.is_empty()
            && (now - self.saved.active_since >= ring_seconds || now < self.saved.active_since)
        {
            self.saved.active.clear();
        }
        if self.next.iter().any(|o| o.at <= now) {
            self.next = alarms
                .iter()
                .filter_map(|a| {
                    let key = a.key();
                    a.next(now, zone, self.saved.fired.get(&key).copied())
                        .map(|at| Occurrence {
                            key,
                            name: a.name.clone(),
                            at,
                        })
                })
                .collect();
        }
        self.previous = now;
        if !self.saved.active.is_empty() && now - self.last_sound >= 5 {
            self.last_sound = now;
            return true;
        }
        false
    }

    pub fn dismiss(&mut self) {
        self.saved.active.clear();
    }
    pub fn snooze(&mut self, now: i64, minutes: i64) {
        for mut o in self.saved.active.drain(..) {
            o.at = now + minutes * 60;
            self.saved.snoozed.retain(|old| old.key != o.key);
            self.saved.snoozed.push(o);
        }
    }
    pub fn next_alarm(&self, now: i64) -> Option<Occurrence> {
        self.next
            .iter()
            .chain(self.saved.snoozed.iter())
            .filter(|o| o.at > now)
            .min_by_key(|o| o.at)
            .cloned()
    }
}

pub fn countdown(seconds: i64) -> String {
    let s = seconds.max(0);
    if s >= 86400 {
        format!(
            "{} d {:02}:{:02}:{:02}",
            s / 86400,
            (s / 3600) % 24,
            (s / 60) % 60,
            s % 60
        )
    } else {
        format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
    }
}

pub fn gong_allowed(hour: u32, start: u32, end: u32) -> bool {
    if start == end {
        true
    } else if start < end {
        hour >= start && hour < end
    } else {
        hour >= start || hour < end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn zone() -> Tz {
        chrono_tz::Europe::Prague
    }
    fn stamp(s: &str) -> i64 {
        chrono::DateTime::parse_from_rfc3339(s).unwrap().timestamp()
    }
    fn alarm() -> Alarm {
        Alarm {
            id: 1,
            name: "Ráno".into(),
            enabled: true,
            mode: "daily".into(),
            hour: 7,
            minute: 30,
            days: [true; 7],
            date: None,
            interval: 1,
            day: 1,
            month: 1,
        }
    }
    #[test]
    fn daily_and_countdown() {
        let a = alarm();
        assert_eq!(
            a.next(stamp("2026-11-01T06:00:00+01:00"), zone(), None),
            Some(stamp("2026-11-01T07:30:00+01:00"))
        );
        assert_eq!(countdown(90061), "1 d 01:01:01");
    }
    #[test]
    fn weekdays() {
        let mut a = alarm();
        a.mode = "weekly".into();
        a.days = [true, true, true, true, true, false, false];
        assert_eq!(
            a.next(stamp("2026-11-06T08:00:00+01:00"), zone(), None),
            Some(stamp("2026-11-09T07:30:00+01:00"))
        );
    }
    #[test]
    fn weekly_interval() {
        let mut a = alarm();
        a.mode = "weekly".into();
        a.days = [true, false, false, false, false, false, false];
        a.interval = 2;
        a.date = NaiveDate::from_ymd_opt(2026, 11, 2);
        assert_eq!(
            a.next(stamp("2026-11-03T08:00:00+01:00"), zone(), None),
            Some(stamp("2026-11-16T07:30:00+01:00"))
        );
    }
    #[test]
    fn once_and_days() {
        let mut a = alarm();
        a.mode = "once".into();
        a.date = NaiveDate::from_ymd_opt(2026, 11, 3);
        assert!(
            a.next(stamp("2026-11-04T00:00:00+01:00"), zone(), None)
                .is_none()
        );
        a.mode = "days".into();
        a.interval = 3;
        assert_eq!(
            a.next(stamp("2026-11-04T00:00:00+01:00"), zone(), None),
            Some(stamp("2026-11-06T07:30:00+01:00"))
        );
    }
    #[test]
    fn monthly_and_leap_year() {
        let mut a = alarm();
        a.mode = "monthly".into();
        a.day = 31;
        assert_eq!(
            a.next(stamp("2026-04-01T00:00:00+02:00"), zone(), None),
            Some(stamp("2026-05-31T07:30:00+02:00"))
        );
        a.mode = "yearly".into();
        a.month = 2;
        a.day = 29;
        assert_eq!(
            a.next(stamp("2026-04-01T00:00:00+02:00"), zone(), None),
            Some(stamp("2028-02-29T07:30:00+01:00"))
        );
    }
    #[test]
    fn dst_gap_and_fold() {
        let mut a = alarm();
        a.hour = 2;
        assert_eq!(
            a.next(stamp("2026-03-29T00:00:00+01:00"), zone(), None),
            Some(stamp("2026-03-30T02:30:00+02:00"))
        );
        assert_eq!(
            a.next(stamp("2026-10-25T02:40:00+02:00"), zone(), None),
            Some(stamp("2026-10-26T02:30:00+01:00"))
        );
    }
    #[test]
    fn interval_minutes() {
        let mut a = alarm();
        a.mode = "minutes".into();
        a.interval = 15;
        a.date = NaiveDate::from_ymd_opt(2026, 11, 1);
        assert_eq!(
            a.next(stamp("2026-11-01T07:40:00+01:00"), zone(), None),
            Some(stamp("2026-11-01T07:45:00+01:00"))
        );
    }
    #[test]
    fn engine_snooze_restart_and_no_duplicates() {
        let a = alarm();
        let t = stamp("2026-11-01T07:30:00+01:00");
        let mut e = Engine::default();
        e.configure(&[a.clone()], t - 1, zone());
        assert!(e.tick(&[a.clone()], t, zone(), 120));
        assert!(!e.tick(&[a.clone()], t + 1, zone(), 120));
        e.snooze(t, 5);
        assert_eq!(e.next_alarm(t).unwrap().at, t + 300);
        let saved = serde_json::to_string(&e.saved).unwrap();
        let mut r = Engine {
            saved: serde_json::from_str(&saved).unwrap(),
            ..Default::default()
        };
        r.configure(&[a.clone()], t + 2, zone());
        assert!(r.tick(&[a.clone()], t + 300, zone(), 120));
        r.dismiss();
        assert!(!r.tick(&[a], t - 1, zone(), 120));
    }
    #[test]
    fn missed_alarms_are_not_replayed() {
        let a = alarm();
        let t = stamp("2026-11-01T07:30:00+01:00");
        let mut e = Engine::default();
        e.configure(&[a.clone()], t - 1, zone());
        assert!(!e.tick(&[a], t + 3600, zone(), 120));
        assert!(e.saved.active.is_empty());
    }
    #[test]
    fn gong_window() {
        assert!(gong_allowed(23, 22, 7));
        assert!(!gong_allowed(12, 22, 7));
        assert!(gong_allowed(12, 0, 0));
    }
}

#[cfg(test)]
mod compact_tests {
    use super::*;
    #[test]
    fn compact_rules_and_errors() {
        for (rule,mode) in [("Po-Pá","weekly"),("Po,Út,So","weekly"),("denně","daily"),("vyp","daily"),("jednou 2026-12-24","once"),("měsíčně 31","monthly"),("ročně 29.2","yearly"),("dny 2 2026-11-01","days"),("hodiny 6 2026-11-01","hours"),("minuty 30 2026-11-01","minutes"),("Po,St tydny 2 2026-11-02","weekly")] {
            assert_eq!(parse_alarm(1,"7:00",rule,"Test").unwrap().mode,mode);
        }
        for (time,rule) in [("24:00","denne"),("7:60","Po"),("7","Po"),("7:00","xxx"),("7:00","jednou 2026-02-31"),("7:00","dny 0 2026-11-01")] {assert!(parse_alarm(1,time,rule,"").is_err());}
    }
    #[test]
    fn five_occurrences_not_five_rules() {
        let zone=chrono_tz::Europe::Prague;
        let now=zone.with_ymd_and_hms(2026,11,1,20,0,0).unwrap().timestamp();
        let alarms=vec![parse_alarm(1,"7:00","Po,Ut","A").unwrap(),parse_alarm(2,"15:00","Ut","B").unwrap()];
        let list=upcoming(&alarms,&Saved::default(),now,zone,5);
        assert_eq!(list.len(),5);
        let times:Vec<_>=list.iter().map(|o|zone.timestamp_opt(o.at,0).single().unwrap().format("%d %H:%M").to_string()).collect();
        assert_eq!(times,vec!["02 07:00","03 07:00","03 15:00","09 07:00","10 07:00"]);
        let mut saved=Saved::default();saved.snoozed.push(Occurrence{key:alarms[0].key(),name:"Odloženo".into(),at:now+300});
        assert_eq!(upcoming(&alarms,&saved,now,zone,5)[0].name,"Odloženo");
    }
}
