extern crate ical;
use crate::ical::{generator::*, *};
use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use ical::parser::Component;
use ical::IcalParser;
use ical::LineReader;
use ical::*;
use ics::properties::{
    Action, Attach, Attendee, Categories, Comment, Description, DtEnd, DtStart, Due, Organizer,
    Repeat, Sequence, Status, Summary, Trigger,
};
use ics::{escape_text, Alarm, Event, ICalendar, ToDo};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::Duration;

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

#[derive(Debug)]
pub struct CalorgCalendar {
    pub version: String,
    pub prodid: String,
    pub events: Vec<CalorgEvent>,
}

impl CalorgCalendar {
    pub fn new() -> CalorgCalendar {
        CalorgCalendar {
            version: String::new(),
            prodid: String::new(),
            events: Vec::new(),
        }
    }
    pub fn new_from_crate_ical(icalcal: IcalCalendar) -> CalorgCalendar {
        let mut calendar = CalorgCalendar::new();
        let verson = icalcal
            .properties
            .iter()
            .filter(|property| property.name == "VERSION")
            .collect::<Vec<&Property>>()[0]
            .value
            .clone()
            .unwrap_or("2.0".to_string());
        let prodid = icalcal
            .properties
            .iter()
            .filter(|property| property.name == "PRODID")
            .collect::<Vec<&Property>>()[0]
            .value
            .clone()
            .unwrap_or("-//Calorg//EN".to_string());
        calendar.version = verson;
        calendar.prodid = prodid;
        for ical_event in icalcal.events {
            let mut event = CalorgEvent::new();
            for property in ical_event.properties {
                match property.name.as_str() {
                    "UID" => {
                        event.set(CalorgKey::UID, property.value.unwrap());
                    }
                    "ATTENDEE" => {
                        event.set(CalorgKey::ATTENDEE, property.value.unwrap());
                    }
                    "CATEGORIES" => {
                        event.set(CalorgKey::CATEGORIES, property.value.unwrap());
                    }
                    "DESCRIPTION" => {
                        event.set(CalorgKey::DESCRIPTION, property.value.unwrap());
                    }
                    "COMMENT" => {
                        event.set(CalorgKey::COMMENT, property.value.unwrap());
                    }
                    "DTSTART" => {
                        event.set(CalorgKey::DTSTART, property.value.unwrap());
                    }
                    "DTEND" => {
                        event.set(CalorgKey::DTEND, property.value.unwrap());
                    }
                    "DUE" => {
                        event.set(CalorgKey::DUE, property.value.unwrap());
                    }
                    "ORGANIZER" => {
                        event.set(CalorgKey::ORGANIZER, property.value.unwrap());
                    }
                    "REPEAT" => {
                        event.set(CalorgKey::REPEAT, property.value.unwrap());
                    }
                    "SEQUENCE" => {
                        event.set(CalorgKey::SEQUENCE, property.value.unwrap());
                    }
                    "STATUS" => {
                        event.set(CalorgKey::STATUS, property.value.unwrap());
                    }
                    "SUMMARY" => {
                        event.set(CalorgKey::SUMMARY, property.value.unwrap());
                    }
                    "TRIGGER" => {
                        event.set(CalorgKey::TRIGGER, property.value.unwrap());
                    }
                    _ => {}
                }
            }
            calendar.add(event);
        }
        calendar
    }
    pub fn save_ics(&mut self, path: &str) {
        let mut ics_calendar = ICalendar::new(&self.version, &self.prodid);
        for event in &self.events {
            let mut ics_event = Event::new(
                event.get(CalorgKey::UID).unwrap(),
                event.get(CalorgKey::DTSTART).unwrap(),
            );
            if let Some(attendee) = event.get_cloned(CalorgKey::ATTENDEE) {
                ics_event.push(Attendee::new(attendee));
            }
            if let Some(categories) = event.get_cloned(CalorgKey::CATEGORIES) {
                ics_event.push(Categories::new(categories));
            }
            if let Some(description) = event.get_cloned(CalorgKey::DESCRIPTION) {
                ics_event.push(Description::new(description));
            }
            if let Some(comment) = event.get_cloned(CalorgKey::COMMENT) {
                ics_event.push(Comment::new(comment));
            }
            if let Some(dtend) = event.get_cloned(CalorgKey::DTEND) {
                ics_event.push(DtEnd::new(dtend));
            }
            if let Some(dtstart) = event.get_cloned(CalorgKey::DTSTART) {
                ics_event.push(DtStart::new(dtstart));
            }
            if let Some(due) = event.get_cloned(CalorgKey::DUE) {
                ics_event.push(Due::new(due));
            }
            if let Some(organizer) = event.get_cloned(CalorgKey::ORGANIZER) {
                ics_event.push(Organizer::new(organizer));
            }
            if let Some(repeat) = event.get_cloned(CalorgKey::REPEAT) {
                ics_event.push(Repeat::new(repeat));
            }
            if let Some(sequence) = event.get_cloned(CalorgKey::SEQUENCE) {
                ics_event.push(Sequence::new(sequence));
            }
            if let Some(status) = event.get_cloned(CalorgKey::STATUS) {
                ics_event.push(Status::new(status));
            }
            if let Some(summary) = event.get_cloned(CalorgKey::SUMMARY) {
                ics_event.push(Summary::new(summary));
            }
            if let Some(trigger) = event.get_cloned(CalorgKey::TRIGGER) {
                ics_event.push(Trigger::new(trigger));
            }
            ics_event.add_alarm(Alarm::new(Action::new("DISPLAY"), Trigger::new("-PT10M")));
            ics_calendar.add_event(ics_event);
        }
        ics_calendar.save_file(path).unwrap();
    }

    pub fn add(&mut self, event: CalorgEvent) {
        self.events.push(event);
    }
    pub fn search(&self, key: CalorgKey, value: &str) -> Vec<&CalorgEvent> {
        self.events
            .iter()
            .filter(|event| event.get(key).unwrap() == value)
            .collect()
    }
}

impl Default for CalorgCalendar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum CalorgKey {
    UID,
    COMMENT, // used for the id of the calorg event
    DTSTART,
    DTEND,
    SUMMARY,
    DESCRIPTION,
    LOCATION,
    CATEGORIES,
    CREATED,
    LAST_MODIFIED,
    SEQUENCE,
    STATUS,
    ATTENDEE,
    ORGANIZER,
    REPEAT,
    DUE,
    TRIGGER,
    URL,
}

#[derive(Debug)]
pub struct CalorgEvent {
    properties: HashMap<CalorgKey, String>,
}

impl CalorgEvent {
    pub fn new() -> CalorgEvent {
        CalorgEvent {
            properties: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: CalorgKey, value: String) {
        self.properties.insert(key, value.to_string());
    }
    pub fn get(&self, key: CalorgKey) -> Option<&String> {
        self.properties.get(&key)
    }
    pub fn get_cloned(&self, key: CalorgKey) -> Option<String> {
        self.properties.get(&key).cloned()
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

pub fn crate_ical_to_ics<'a>(icalcal: IcalCalendar) -> ICalendar<'a> {
    let version = icalcal
        .properties
        .iter()
        .filter(|property| property.name == "VERSION")
        .collect::<Vec<&Property>>()[0]
        .value
        .clone()
        .unwrap_or("2.0".to_string());
    let prodid = icalcal
        .properties
        .iter()
        .filter(|property| property.name == "PRODID")
        .collect::<Vec<&Property>>()[0]
        .value
        .clone()
        .unwrap_or("-//Calorg//EN".to_string());

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
                .filter(|property| property.name == "DTSTART")
                .collect::<Vec<&Property>>()[0]
                .value
                .clone()
                .unwrap_or("FAILED".to_string()),
        );
        for property in ical_event.properties {
            match property.name.as_str() {
                "ATTENDEE" => {
                    event.push(Attendee::new(property.value.unwrap()));
                }
                "CATEGORIES" => {
                    event.push(Categories::new(property.value.unwrap()));
                }
                "DESCRIPTION" => {
                    event.push(Description::new(property.value.unwrap()));
                }
                "COMMENT" => {
                    event.push(Comment::new(property.value.unwrap()));
                }
                "DTEND" => {
                    event.push(DtEnd::new(property.value.unwrap()));
                }
                "DUE" => {
                    event.push(Due::new(property.value.unwrap()));
                }
                "ORGANIZER" => {
                    event.push(Organizer::new(property.value.unwrap()));
                }
                "REPEAT" => {
                    event.push(Repeat::new(property.value.unwrap()));
                }
                "SEQUENCE" => {
                    event.push(Sequence::new(property.value.unwrap()));
                }
                "STATUS" => {
                    event.push(Status::new(property.value.unwrap()));
                }
                "SUMMARY" => {
                    event.push(Summary::new(property.value.unwrap()));
                }
                "TRIGGER" => {
                    event.push(Trigger::new(property.value.unwrap()));
                }
                _ => {}
            }
        }
        calendar.add_event(event);
    }
    calendar
}

pub fn save_ics_calendar(path: &str, calendar: ICalendar) -> Result<(), std::io::Error> {
    calendar.save_file(path)
}

// let datetime_str = "19960918T143000Z";
// let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%dT%H%M%SZ").unwrap();
// let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
// let ten_minutes = Duration::from_secs(60 * 10);
// let ten_minutes_before: DateTime<Utc> = datetime - ten_minutes;
