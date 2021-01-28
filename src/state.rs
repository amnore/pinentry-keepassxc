use lazy_static::lazy_static;
use std::sync::Mutex;
use xsalsa20poly1305::generate_nonce;
use rand::thread_rng;
lazy_static! {
    pub static ref KEYGREP: Mutex<Option<String>> = Mutex::new(None);
    pub static ref ID_KEY: Mutex<String> = Mutex::new(generate_idkey());
}

fn generate_idkey() -> String {
    base64::encode(generate_nonce(&mut thread_rng()))
}
