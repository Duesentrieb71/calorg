extern crate ical;
use crate::ical::generator::{IcalCalendar, Property};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use ical::IcalParser;
use ics::properties::*;
use ics::{Alarm, Event, ICalendar};
use rand::Rng;
use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, Result};

pub fn naive_convert_berlin_to_utc(naive_datetime: NaiveDateTime) -> NaiveDateTime {
    let datetime: DateTime<chrono_tz::Tz> = Berlin.from_local_datetime(&naive_datetime).unwrap();
    let datetime_utc: DateTime<Utc> = datetime.with_timezone(&Utc);
    datetime_utc.naive_utc()
}

pub fn naive_convert_utc_to_berlin(naive_datetime: NaiveDateTime) -> NaiveDateTime {
    let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
    let datetime_berlin: DateTime<chrono_tz::Tz> = datetime.with_timezone(&Berlin);
    datetime_berlin.naive_local()
}

//supported properties
pub const CALORG_KEYS: [&str; 20] = [
    "UID",
    "COMMENT",
    "DTSTART",
    "DTEND",
    "SUMMARY",
    "DESCRIPTION",
    "LOCATION",
    "CATEGORIES",
    "CREATED",
    "LAST-MODIFIED",
    "SEQUENCE",
    "STATUS",
    "ATTENDEE",
    "ORGANIZER",
    "REPEAT",
    "RRULE",
    "DUE",
    "TRIGGER",
    "URL",
    "DTSTAMP",
];

#[derive(Debug)]
pub struct Job {
    pub summary: String,
    pub description: String,
    pub comment: String, //used to store the job id
    pub no_start_before: Option<NaiveDateTime>,
    pub deadline: NaiveDateTime,
}

#[derive(Debug)]
pub struct CalorgCalendar {
    pub version: String,
    pub prodid: String,
    pub events: Vec<CalorgEvent>,
}

impl CalorgCalendar {
    pub fn new() -> Self {
        Self {
            version: String::new(),
            prodid: String::new(),
            events: Vec::new(),
        }
    }

    fn get_property_value(properties: &[Property], name: &str, default: &str) -> String {
        properties
            .iter()
            .find(|property| property.name == name)
            .and_then(|p| p.value.clone())
            .unwrap_or_else(|| default.to_string())
    }

    pub fn new_from_crate_ical(icalcal: IcalCalendar) -> Self {
        let mut calendar = Self::new();
        calendar.version = Self::get_property_value(&icalcal.properties, "VERSION", "2.0");
        calendar.prodid = Self::get_property_value(&icalcal.properties, "PRODID", "-//Calorg//EN");

        calendar.events = icalcal
            .events
            .into_iter()
            .map(|ical_event| {
                let mut event = CalorgEvent::new();
                for property in ical_event.properties {
                    if let Some(value) = property.value {
                        event.set(property.name, value);
                    }
                }
                event
            })
            .collect();

        calendar
    }

    pub fn save_ics(&self, path: &str) -> Result<()> {
        let mut ics_calendar = ICalendar::new(&self.version, &self.prodid);
        for event in &self.events {
            let mut ics_event =
                Event::new(event.get("UID").unwrap(), event.get("DTSTAMP").unwrap());

            for key in CALORG_KEYS {
                if let Some(value) = event.get(key) {
                    match key {
                        "UID" => ics_event.push(UID::new(value)),
                        "COMMENT" => ics_event.push(Comment::new(value)),
                        "DTSTART" => ics_event.push(DtStart::new(value)),
                        "DTEND" => ics_event.push(DtEnd::new(value)),
                        "SUMMARY" => ics_event.push(Summary::new(value)),
                        "DESCRIPTION" => ics_event.push(Description::new(value)),
                        "LOCATION" => ics_event.push(Location::new(value)),
                        "CATEGORIES" => ics_event.push(Categories::new(value)),
                        "CREATED" => ics_event.push(Created::new(value)),
                        "LAST-MODIFIED" => ics_event.push(LastModified::new(value)),
                        "SEQUENCE" => ics_event.push(Sequence::new(value)),
                        "STATUS" => ics_event.push(Status::new(value)),
                        "ATTENDEE" => ics_event.push(Attendee::new(value)),
                        "ORGANIZER" => ics_event.push(Organizer::new(value)),
                        "REPEAT" => ics_event.push(Repeat::new(value)),
                        "RRULE" => ics_event.push(RRule::new(value)),
                        "DUE" => ics_event.push(Due::new(value)),
                        "TRIGGER" => ics_event.push(Trigger::new(value)),
                        "URL" => ics_event.push(URL::new(value)),
                        "DTSTAMP" => ics_event.push(DtStamp::new(value)),
                        _ => (),
                    }
                }
            }

            ics_event.add_alarm(Alarm::new(Action::new("DISPLAY"), Trigger::new("-PT10M")));
            ics_calendar.add_event(ics_event);
        }
        ics_calendar.save_file(path)
    }

    pub fn add(&mut self, event: CalorgEvent) {
        self.events.push(event);
    }
    pub fn search(&self, key: &str, value: &str) -> Vec<&CalorgEvent> {
        self.events
            .iter()
            .filter(|event| event.get(key).map_or(false, |v| v == value))
            .collect()
    }
    pub fn search_mut(&mut self, key: &str, value: &str) -> Vec<&mut CalorgEvent> {
        self.events
            .iter_mut()
            .filter(|event| event.get(key).map_or(false, |v| v == value))
            .collect()
    }
    pub fn search_index(&self, key: &str, value: &str) -> Vec<usize> {
        self.events
            .iter()
            .enumerate()
            .filter(|(_, event)| event.get(key).map_or(false, |v| v == value))
            .map(|(i, _)| i)
            .collect()
    }
    pub fn get_event(&self, index: usize) -> Option<&CalorgEvent> {
        self.events.get(index)
    }
}

impl Default for CalorgCalendar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct CalorgEvent {
    properties: HashMap<String, String>,
}

impl CalorgEvent {
    pub fn new() -> CalorgEvent {
        CalorgEvent {
            properties: HashMap::new(),
        }
    }
    // basic constructor with random uid, comment, dtstart, dtend, summary, description, location, categories, created, last-modified, status, dtstamp
    pub fn new_basic(
        comment: String,
        dtstart: String,
        dtend: String,
        summary: String,
        description: String,
        location: String,
        categories: String,
    ) -> CalorgEvent {
        let mut event = CalorgEvent::new();
        let uid: String = rand::thread_rng().gen_range(0..10000000).to_string();
        let created = chrono::Local::now().naive_local().to_string();
        let last_modified = created.clone();
        let status = "CONFIRMED".to_string();
        let dtstamp = created.clone();
        event.set("UID".to_string(), uid);
        event.set("COMMENT".to_string(), comment);
        event.set("DTSTART".to_string(), dtstart);
        event.set("DTEND".to_string(), dtend);
        event.set("SUMMARY".to_string(), summary);
        event.set("DESCRIPTION".to_string(), description);
        event.set("LOCATION".to_string(), location);
        event.set("CATEGORIES".to_string(), categories);
        event.set("CREATED".to_string(), created);
        event.set("LAST-MODIFIED".to_string(), last_modified);
        event.set("STATUS".to_string(), status);
        event.set("DTSTAMP".to_string(), dtstamp);
        event
    }
    pub fn set(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
    pub fn get_cloned(&self, key: &str) -> Option<String> {
        self.properties.get(key).cloned()
    }
}

impl Default for CalorgEvent {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_ical_calendar(path: &str) -> IcalCalendar {
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let mut reader = IcalParser::new(buf);
    reader.next().unwrap().unwrap()
}
#[deprecated(note = "not optimized and missing some properties")]
pub fn crate_ical_to_ics<'a>(icalcal: IcalCalendar) -> ICalendar<'a> {
    let get_property_value = |name: &str, default: &str| {
        icalcal
            .properties
            .iter()
            .find(|property| property.name == name)
            .and_then(|p| p.value.clone())
            .unwrap_or_else(|| default.to_string())
    };

    let version = get_property_value("VERSION", "2.0");
    let prodid = get_property_value("PRODID", "-//Calorg//EN");

    let mut calendar = ICalendar::new(version, prodid);
    for ical_event in icalcal.events {
        let mut event = Event::new(
            ical_event
                .properties
                .iter()
                .filter(|property| property.name == "UID")
                .collect::<Vec<&Property>>()[0]
                .value
                .clone()
                .unwrap_or("FAILED".to_string()),
            ical_event
                .properties
                .iter()
                .filter(|property| property.name == "DTSTAMP")
                .collect::<Vec<&Property>>()[0]
                .value
                .clone()
                .unwrap_or("FAILED".to_string()),
        );
        for property in ical_event.properties {
            if let Some(value) = property.value {
                match property.name.as_str() {
                    "UID" => event.push(UID::new(value)),
                    "COMMENT" => event.push(Comment::new(value)),
                    "DTSTART" => event.push(DtStart::new(value)),
                    "DTEND" => event.push(DtEnd::new(value)),
                    "SUMMARY" => event.push(Summary::new(value)),
                    "DESCRIPTION" => event.push(Description::new(value)),
                    "LOCATION" => event.push(Location::new(value)),
                    "CATEGORIES" => event.push(Categories::new(value)),
                    "CREATED" => event.push(Created::new(value)),
                    "LAST-MODIFIED" => event.push(LastModified::new(value)),
                    "SEQUENCE" => event.push(Sequence::new(value)),
                    "STATUS" => event.push(Status::new(value)),
                    "ATTENDEE" => event.push(Attendee::new(value)),
                    "ORGANIZER" => event.push(Organizer::new(value)),
                    "REPEAT" => event.push(Repeat::new(value)),
                    "RRULE" => event.push(RRule::new(value)),
                    "DUE" => event.push(Due::new(value)),
                    "TRIGGER" => event.push(Trigger::new(value)),
                    "URL" => event.push(URL::new(value)),
                    "DTSTAMP" => event.push(DtStamp::new(value)),
                    _ => (),
                }
            }
        }
        calendar.add_event(event);
    }
    calendar
}

#[deprecated]
pub fn save_ics_calendar(path: &str, calendar: ICalendar) -> Result<()> {
    calendar.save_file(path)
}

// let datetime_str = "19960918T143000Z";
// let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%dT%H%M%SZ").unwrap();
// let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
// let ten_minutes = Duration::from_secs(60 * 10);
// let ten_minutes_before: DateTime<Utc> = datetime - ten_minutes;
