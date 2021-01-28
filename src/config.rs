use crate::state::{ID, ID_KEY};
use directories::ProjectDirs;
use json::{object, parse};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Read};
use std::path::PathBuf;

lazy_static! {
    static ref CONFIG_PATH: PathBuf = PathBuf::from(
        ProjectDirs::from("", "", "pinentry-keepassxc")
            .unwrap()
            .config_dir()
    );
}

pub fn load() {
    match File::open(CONFIG_PATH.as_path()) {
        Err(_) => (),
        Ok(mut file) => {
            let mut conf = String::new();
            file.read_to_string(&mut conf)
                .expect("Cannot read config file");
            let obj = parse(&conf).unwrap();
            *ID.lock().unwrap() = obj["id"].as_str().map(|s| s.to_string());
            *ID_KEY.lock().unwrap() =
                obj["idKey"].as_str().map(|s| s.to_string()).unwrap();
        }
    }
}

pub fn store() {
    if ID.lock().unwrap().is_none() {
        return;
    }

    match File::create(CONFIG_PATH.as_path()) {
        Err(_) => (),
        Ok(mut file) => {
            let conf = object! {
                idKey: ID_KEY.lock().unwrap().as_str(),
                id: ID.lock().unwrap().as_ref().unwrap().as_str(),
            };
            conf.write(&mut file).expect("Cannot write to config file");
        }
    }
}
