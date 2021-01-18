use lazy_static::lazy_static;
use std::sync::Mutex;
pub struct State {
    pub keygrep: Option<String>,
    pub passphrase: Option<String>,
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State {
        keygrep: None,
        passphrase: None
    });
}
