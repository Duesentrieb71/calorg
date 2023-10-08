extern crate ical;
use crate::ical::{generator::*, *};
use ical::parser::Component;
use ical::IcalParser;
use ical::LineReader;
use ical::*;
use ics::properties::{
    Attach, Attendee, Categories, Description, DtEnd, DtStart, Due, Organizer, Repeat, Sequence,
    Status, Summary, Trigger,
};
use ics::{escape_text, Alarm, Event, ICalendar, ToDo};
use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
// mod lib;
// use lib::*;

fn main() {
    let file = File::open("event.ics").unwrap();
    let buf = BufReader::new(file);
    let mut reader = IcalParser::new(buf);
    //save to file
    let file = File::create("path_to_your_file.ics").unwrap();
    let mut writer = BufWriter::new(file);
    reader.write(&mut writer).unwrap();
}

// let mut cal = reader.next().unwrap().unwrap();
// let event = IcalEventBuilder::tzid("Europe/Berlin")
//     .uid("242424")
//     .changed("dtstamp")
//     .start("2424")
//     .end("2424")
//     .set(ical_property!("SUMMARY", "New Year"))
//     .build();
// cal.events.push(event);

// while let Some(event) = reader.next() {
//     // Process each event
//     // Add a new property
//     let mut event = event.next().unwrap();

//     // from string
//     let line_reader = LineReader::new(BufReader::new("BEGIN:VEVENT".as_bytes()));
//     let line_parser = &RefCell::new(PropertyParser::new(line_reader));

//     event.add_sub_component("VALARM", line_parser);

//     // let file = File::create("path_to_your_file.ics").unwrap();
//     // let mut writer = BufWriter::new(file);
//     // reader.write(&mut writer).unwrap();
// }
