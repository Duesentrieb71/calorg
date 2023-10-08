use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Europe::Berlin;
use ics::properties::{
    Attach, Attendee, Categories, Description, DtEnd, DtStart, Due, Organizer, Repeat, Sequence,
    Status, Summary, Trigger,
};
use ics::{escape_text, Alarm, Event, ICalendar, ToDo};
use std::collections::HashMap;
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
pub struct CalorgEvents {
    pub events: Vec<CalorgEvent>,
}

impl CalorgEvents {
    pub fn new() -> CalorgEvents {
        CalorgEvents { events: Vec::new() }
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

impl Default for CalorgEvents {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub enum CalorgKey {
    UID,
    CID,
    DTSTART,
    DTEND,
    SUMMARY,
    DESCRIPTION,
    LOCATION,
    CATEGORIES,
    CREATED,
    LAST_MODIFIED,
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
    pub fn new_empty_keys() -> CalorgEvent {
        let mut event = CalorgEvent::new();
        event.set(CalorgKey::UID, "");
        event.set(CalorgKey::CID, "");
        event.set(CalorgKey::DTSTART, "");
        event.set(CalorgKey::DTEND, "");
        event.set(CalorgKey::SUMMARY, "");
        event.set(CalorgKey::DESCRIPTION, "");
        event.set(CalorgKey::LOCATION, "");
        event.set(CalorgKey::CATEGORIES, "");
        event.set(CalorgKey::CREATED, "");
        event.set(CalorgKey::LAST_MODIFIED, "");
        event
    }
    pub fn set(&mut self, key: CalorgKey, value: &str) {
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

// let datetime_str = "19960918T143000Z";
// let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y%m%dT%H%M%SZ").unwrap();
// let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
// let ten_minutes = Duration::from_secs(60 * 10);
// let ten_minutes_before: DateTime<Utc> = datetime - ten_minutes;
