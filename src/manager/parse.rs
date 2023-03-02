use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Entry {
    name: String,
    author: String,
    packageid: String,
    steamid: String,
    versions: Vec<f32>,
    dependencies: Vec<String>,
    loadafter: Vec<String>,
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            name: String::from(""),
            author: String::from(""),
            packageid: String::from(""),
            steamid: String::from(""),
            versions: Vec::new(), //TODO: Don't initialize empty if nothing there?
            dependencies: Vec::new(),
            loadafter: Vec::new(),
        }
    }
}

//TODO: Figure out paths and passing stuff around..
pub fn parse_mod(path: PathBuf) -> Entry {
    let mut entry = Entry::default();

    entry.steamid = String::from(path.file_name().unwrap().to_str().unwrap());

    let contents = fs::read_to_string(&(String::from(path.to_str().unwrap()) + "/About/About.xml"))
        .expect("Should have been able to read the file...");

    let mut reader = Reader::from_str(&contents);

    // let file = File::open(&(String::from(path.to_str().unwrap()) + "/About/About.xml"))
    // .expect("Should have been able to read the file..."// );

    // let r = BufReader::new(file);

    // let mut reader = Reader::from_reader(r);
    //let xml = &contents[contents.find("<ModMetaData>").unwrap_or(0)..contents.len()];

    //let start = BytesStart::new("ModMetaData");
    //let end   = start.to_end().into_owned();

    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) => {
                println!("{:?}", tag.name());
                // match tag.name().as_ref() {
                // _ => (),
                // }
            }
            // Event::Text(e) => txt.push(e.unescape().unwrap().into_owned()),
            Event::Eof => break,
            _ => (),
        }
    }

    return entry;
}
