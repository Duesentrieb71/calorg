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

fn main() -> std::io::Result<()> {
    let icalcalendar = get_ical_calendar("../../../test.ics");
    println!("{:?}", icalcalendar.properties[0].value);
    println!("{:?}", icalcalendar.properties[1].value);
    let mut calendar = CalorgCalendar::new_from_crate_ical(icalcalendar);

    // add comment to event with summary "THUNDER_test"
    if let Some(test_event) = calendar.search_mut("SUMMARY", "THUNDER_test").get_mut(0) {
        test_event.set("COMMENT".to_string(), "test".to_string());
    } else {
        // Handle the case where no event with the summary "THUNDER_test" is found
        println!("Event with summary 'THUNDER_test' not found.");
    }

    calendar.save_ics("../../../test2.ics")
}
