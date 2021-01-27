use crate::state::ID_KEY;
use directories::ProjectDirs;
use json::{object, parse};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{Read};
use std::path::PathBuf;

lazy_static! {
    static ref CONFIG_PATH: PathBuf = PathBuf::from(
        ProjectDirs::from("com", "czg", "pinentry-keepassxc")
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
            *ID_KEY.lock().unwrap() =
                Some(obj["idKey"].as_str().map(|str| String::from(str)).unwrap());
        }
    }
}
pub fn store() {
    let idkey = ID_KEY.lock().unwrap();
    if idkey.is_none() {
        return;
    }
    match File::open(CONFIG_PATH.as_path()) {
        Err(_) => (),
        Ok(mut file) => {
            let conf = object! {
                "idKey": idkey.clone().unwrap(),
            };
            conf.write(&mut file).expect("Connot write to config file");
        }
    }
}
