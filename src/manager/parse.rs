use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::path::PathBuf;
use std::str;

#[derive(Debug)]
pub struct Entry {
    name: String,
    author: String,
    packageid: String,
    steamid: String,
    versions: Vec<String>,
    dependencies: Vec<String>,
    incompatible: Vec<String>,
    loadbefore: Vec<String>,
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
            incompatible: Vec::new(),
            loadbefore: Vec::new(),
            loadafter: Vec::new(),
        }
    }
}

// fields with basic list items:
// SupportedVersions
// inCompatibleWith
// loadBefore
// loadAfter
pub fn parse_basic_list(
    reader: &mut quick_xml::Reader<&[u8]>,
    vec: &mut Vec<String>,
    end_field: &str,
) {
    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) => {
                if str::from_utf8(tag.name().as_ref()).unwrap() == "li" {
                    let mod_name: String = match reader.read_event().unwrap() {
                        Event::Text(txt) => String::from(txt.unescape().unwrap().into_owned()),
                        _ => String::from(""),
                    };
                    vec.push(mod_name);
                }
            }
            Event::End(tag) => {
                if str::from_utf8(tag.name().as_ref()).unwrap() == end_field {
                    break;
                }
            }
            _ => continue,
        }
    }
}

//TODO: Parse ByVersion fields

//TODO: Need other info?
pub fn parse_dependencies(
    reader: &mut quick_xml::Reader<&[u8]>,
    vec: &mut Vec<String>,
    end_field: &str,
) {
    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) => {
                if str::from_utf8(tag.name().as_ref()).unwrap() == "packageId" {
                    let mod_name: String = match reader.read_event().unwrap() {
                        Event::Text(txt) => String::from(txt.unescape().unwrap().into_owned()),
                        _ => String::from(""),
                    };
                    if !vec.contains(&mod_name) {
                        vec.push(mod_name);
                    }
                }
            }
            Event::End(tag) => {
                if str::from_utf8(tag.name().as_ref()).unwrap() == end_field {
                    break;
                }
            }
            _ => continue,
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
    //reader.trim_text(true);

    //Find Beginning
    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) => {
                if str::from_utf8(tag.name().as_ref()).unwrap() == "ModMetaData" {
                    break;
                }
            }
            _ => continue,
        }
    }

    loop {
        match reader.read_event().unwrap() {
            Event::Start(tag) => {
                let text: String = match reader.read_event().unwrap() {
                    Event::Text(txt) => String::from(txt.unescape().unwrap().into_owned()),
                    _ => String::from(""),
                };

                //println!("{}", text);

                let tag_name = tag.name();
                let field = str::from_utf8(tag_name.as_ref()).unwrap();
                //println!("field {}", field);

                match field {
                    "name" => entry.name = text,
                    "author" => entry.author = text,
                    "packageId" => entry.packageid = text,
                    "supportedVersions" => {
                        parse_basic_list(&mut reader, &mut entry.versions, "supportedVersions")
                    }
                    "incompatibleWith" => {
                        parse_basic_list(&mut reader, &mut entry.incompatible, "incompatibleWith")
                    }
                    "loadBefore" => {
                        parse_basic_list(&mut reader, &mut entry.loadbefore, "loadBefore")
                    }
                    "loadAfter" => parse_basic_list(&mut reader, &mut entry.loadafter, "loadAfter"),
                    "modDependencies" => {
                        parse_dependencies(&mut reader, &mut entry.dependencies, "modDependencies")
                    }
                    //TODO: Create actual version function
                    "loadBeforeByVersion" => {
                        parse_basic_list(&mut reader, &mut entry.loadbefore, "loadBeforeByVersion")
                    }
                    "loadAfterByVersion" => {
                        parse_basic_list(&mut reader, &mut entry.loadafter, "loadAfterByVersion")
                    }
                    "modDependenciesByVersion" => parse_dependencies(
                        &mut reader,
                        &mut entry.dependencies,
                        "modDependenciesByVersion",
                    ),
                    //Drop unsupported tags
                    _ => loop {
                        match reader.read_event().unwrap() {
                            Event::End(tag) => {
                                if field == str::from_utf8(tag.name().as_ref()).unwrap() {
                                    break;
                                }
                            }
                            _ => continue,
                        }
                    },
                }
            }
            Event::Eof => break,
            _ => (),
        }
    }

    return entry;
}
