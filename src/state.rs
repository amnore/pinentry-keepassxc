use lazy_static::lazy_static;
use std::sync::Mutex;
pub struct State {
    pub keygrep: Option<String>,
    pub database_id: Option<String>,
    pub id_key: Option<String>,
}

lazy_static! {
    pub static ref STATE: Mutex<State> = Mutex::new(State {
        keygrep: None,
        database_id: None,
        id_key: None
    });
}
