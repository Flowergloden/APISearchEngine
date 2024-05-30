use std::{fs::File, io::BufReader};

use server::data::TagData;
use xml::reader::{XmlEvent, EventReader};

fn main() {
    let file = File::open("test.tag.xml").unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);
}
