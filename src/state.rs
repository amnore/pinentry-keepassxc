use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    pub static ref KEYGREP: Mutex<Option<String>> = Mutex::new(None);
    pub static ref DATABASE_ID: Mutex<Option<String>> = Mutex::new(None);
    pub static ref ID_KEY: Mutex<Option<String>> = Mutex::new(None);
}
