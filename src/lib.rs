extern crate gio;
extern crate gtk;

pub struct ProcessConfig<'a> {
    name: &'a str,
    args: Option<Vec<&'a str>>,
}

pub mod ui {
    use super::config;
    use super::syscalls;
    use gtk::prelude::*;
    use gtk::{main_quit, Builder, ComboBoxText, ComboBoxTextExt, TextView};

    pub fn app_ui() {
        let main_window_path = "src/main.glade";

        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let builder = Builder::from_file(main_window_path);
        let window: gtk::Window = builder.get_object("main_window").unwrap();
        let choice_picker: ComboBoxText = builder.get_object("info_choices").unwrap();
        let categories = config::get_categories();
        let output_box: TextView = builder.get_object("info_box").unwrap();
        for item in categories.iter() {
            ComboBoxTextExt::append(&choice_picker, Some("info_choices"), item);
        }
        let process_config = config::get_process_config();
        choice_picker.connect_changed(move |item| {
            let choice = item.get_active_text().unwrap().as_str().to_string();
            println!("Changed {}", choice);
            let cfg =&process_config[&choice]; 
            let syscall_result = syscalls::exec_process(cfg.name, cfg.args.as_ref());
        });
        window.show_all();
        window.connect_destroy(|_| {
            main_quit();
        });
        syscalls::exec_process("lscpu", None);
        gtk::main();
    }
}

pub mod config {
    use super::ProcessConfig;
    use std::collections::HashMap;

    pub fn get_categories() -> std::vec::Vec<&'static str> {
        return vec!["CPU", "Hardware", "Network", "PCI", "SATA/SCSI", "Misc."];
    }

    fn create_config<'a>(name: &'a str, args: Option<Vec<&'a str>>) -> ProcessConfig<'a> {
        ProcessConfig {
            name: name,
            args: args,
        }
    }

    pub fn get_process_config<'a>() -> HashMap<String, ProcessConfig<'a>> {
        let mut config = HashMap::new();
        config.insert("CPU".to_string(), create_config("lscpu", None));
        config
    }
}

pub mod syscalls {
    use std::process::Command;
    use std::str;
    pub fn exec_process(name: &str, args: Option<Vec<&str>>) {
        let definite_args = match args {
            Some(x) => x,
            None => Vec::new(),
        };
        let buf = Command::new(name)
            .args(&definite_args)
            .output()
            .expect("Failed to execute command")
            .stdout;
        let output = match str::from_utf8(&buf) {
            Ok(x) => x,
            Err(e) => panic!("Faild to parse UTF-8 STDOUT to string slice {}", e),
        };
        println!("{}", output);
    }
}
