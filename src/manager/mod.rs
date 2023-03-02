use glob::glob;

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

pub struct Manager {
    inactive: Vec<parse::Entry>,
    active: Vec<parse::Entry>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            inactive: Vec::new(),
            active: Vec::new(),
        }
    }
    pub fn fetch_mods(&self, path: String) {
        for dir in glob(&(path + "/*")).expect("Failed to read directory") {
            //TODO: Only glob directories
            //TODO actually parse path
            match dir {
                Err(e) => println!("{:?}", e),
                Ok(path) => {
                    if (path.to_str().unwrap()
                        == "/home/creami/.steam/steam/steamapps/workshop/content/294100/839005762")
                    {
                        parse::parse_mod(path);
                    }
                }
            }
        }
    }
}
