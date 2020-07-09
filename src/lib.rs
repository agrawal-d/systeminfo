extern crate gio;
extern crate gtk;

pub mod ui {
    use super::config;
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::{Application, ApplicationWindow, Builder, Button, ComboBoxText, ComboBoxTextExt};

    pub fn app_ui() {
        let main_window_path = "src/main.glade";

        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let builder = gtk::Builder::from_file(main_window_path);
        let window: gtk::Window = builder.get_object("main_window").unwrap();
        let choice_picker: gtk::ComboBoxText = builder.get_object("info_choices").unwrap();
        let categories = config::get_categories();
        for item in categories.iter() {
            ComboBoxTextExt::append(&choice_picker, Some("info_choices"), item);
        }
        choice_picker.connect_changed(|item| {
            println!("Changed {}",item.get_active_text().unwrap());
        });
        window.show_all();
        gtk::main();
    }
}

pub mod config {
    pub fn get_categories() -> std::vec::Vec<&'static str> {
        return vec!["CPU", "Hardware", "Network", "PCI", "SATA/SCSI", "Misc."];
    }
}
