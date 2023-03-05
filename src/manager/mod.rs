use glob::glob;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::str;

use serde::{Deserialize, Serialize};

mod parse;

// Required Functionality:

// In:
// Create a full mod list
// Get the list of active mods, from xml

// Out:
// Output a ModsConfig.xml with updated mods

// Later:
// Get Dependencies
// Reload mod list

#[derive(Debug)]
pub struct Manager {
    mods: HashMap<String, parse::Entry>,
    pub inactive_list: Vec<String>,
    pub active_list: Vec<String>,
    modlisthead: String,
    modlisttail: String,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            mods: HashMap::new(),
            inactive_list: Vec::new(),
            active_list: Vec::new(),
            modlisthead: String::from(""),
            modlisttail: String::from(""),
        }
    }
    pub fn fetch_mods(&mut self, path: String) {
        for dir in glob(&(path + "/*/About/About.xml")).expect("Cannot find mods directory...") {
            //TODO: Only glob directories
            //TODO actually parse path
            match dir {
                Err(e) => println!("{:?}", e),
                Ok(path) => {
                    let (k, v) = parse::parse_mod(path);
                    self.mods.insert(k, v);
                }
            }
        }
    }

    pub fn load_active_from_file(&mut self, path: &str) {
        let contents = fs::read_to_string(path).expect("Cannot find ModsConfig.xml...");

        let re = regex::Regex::new(r"<activeMods>|</activeMods>").unwrap();
        let mut split = re.split(&contents);

        self.modlisthead = String::from(split.next().unwrap());
        let body = split.next().unwrap();
        self.modlisttail = String::from(split.next().unwrap());

        let mut reader = Reader::from_str(body);
        //reader.trim_text(true);

        loop {
            match reader.read_event().unwrap() {
                Event::Start(tag) => {
                    if str::from_utf8(tag.name().as_ref()).unwrap() != "li" {
                        continue;
                    }

                    let mod_name: String = match reader.read_event().unwrap() {
                        Event::Text(txt) => String::from(txt.unescape().unwrap().into_owned()),
                        _ => String::from(""),
                    };
                    let mut entry = self.mods.get_mut(&mod_name).unwrap();

                    entry.active = true;

                    self.active_list.push(mod_name);
                }
                Event::Eof => break,
                _ => continue,
            }
        }
        println!("{:#?}", self.mods);
        println!("{:#?}", self.active_list);
    }

    pub fn save_mods(&self, path: &str) -> () {
        let active_refs = self
            .active_list
            .iter()
            .map(|k| self.mods.get(k))
            .collect::<Vec<_>>();
        println!("{}", serde_json::to_string(&self.mods).unwrap());
    }

    //TODO: Merge lists together

    pub fn save_mod_list(&self, path: &str) -> () {
        let active_refs = self
            .active_list
            .iter()
            .map(|k| self.mods.get(k))
            .collect::<Vec<_>>();

        let inactive_refs = self
            .active_list
            .iter()
            .map(|k| self.mods.get(k))
            .collect::<Vec<_>>();

        fs::write(
            &(String::from(path) + "active_modlist.json"),
            serde_json::to_string(&active_refs).unwrap(),
        )
        .expect("Could not write to save file.");

        fs::write(
            &(String::from(path) + "inactive_modlist.json"),
            serde_json::to_string(&inactive_refs).unwrap(),
        )
        .expect("Could not write to save file.");

        //println!("{}", serde_json::to_string(&active_refs).unwrap());

        // for rwmod in &self.active_list {
        // println!(
        // "{:#}",
        // serde_json::to_string(self.mods.get(rwmod).unwrap()).unwrap()
        // );
        // }
    }
    pub fn load_mod_list(&mut self, path: &str) -> () {
        let inactive = File::open(&(String::from(path) + "inactive_modlist.json"))
            .expect("Cannot find mod list...");

        //self.active_list = serde_json::from_reader(inactive).unwrap().map;

        let active = File::open(&(String::from(path) + "active_modlist.json"))
            .expect("Cannot find mod list...");

        self.inactive_list = serde_json::from_reader(active).unwrap();

        println!("{:#?}", self.active_list);
        println!("{:#?}", self.inactive_list);
    }
}
