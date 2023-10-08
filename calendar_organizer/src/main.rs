extern crate ical;
use crate::ical::{generator::*, *};
use ical::parser::Component;
use ical::IcalParser;
use ical::LineReader;
use ical::*;
use ics::properties::{
    Attach, Attendee, Categories, Comment, Description, DtEnd, DtStart, Due, Organizer, Repeat,
    Sequence, Status, Summary, Trigger,
};
use ics::{escape_text, Alarm, Event, ICalendar, ToDo};
use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
mod lib;
use lib::*;

fn main() {
    let icalcalendar = get_ical_calendar("event.ics");
    println!("{:?}", icalcalendar.properties[0].value);
    println!("{:?}", icalcalendar.properties[1].value);
    let mut calendar = CalorgCalendar::new_from_crate_ical(icalcalendar);
    calendar.save_ics("event3.ics")
}
