use gtk::prelude::*;
use gtk::{glib, Application};

mod manager;

const APP_ID: &str = "org.gtk_rs.rwManager";

pub fn main() -> glib::ExitCode {
    //let mut m = manager::Manager::new();
    //m.fetch_mods(String::from("/home/creami/Documents/rwmanager/test/input"));
    //m.load_active_from_file("/home/creami/Documents/rwmanager/test/input/ModsConfig.xml");
    //m.save_mods("/home/creami/Documents/rwmanager/test/");
    //m.save_mod_list("/home/creami/Documents/rwmanager/test/");
    //m.load_mod_list("/home/creami/Documents/rwmanager/test/");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
