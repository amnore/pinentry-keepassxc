use crate::state::{DATABASE_ID, ID_KEY, KEYGREP};
use base64::{decode, encode};
use crypto_box::{aead::Aead, generate_nonce, PublicKey, SecretKey};
use directories::ProjectDirs;
use json::{object, JsonValue};
use lazy_static::lazy_static;
use rand::thread_rng;
use std::convert::TryInto;
use std::error::Error;
use std::io::Read;
use std::os::unix::net::UnixStream;
use std::sync::Mutex;
use xsalsa20poly1305::Nonce;

#[derive(Debug, Clone)]
struct KeepassXCError(String);

impl std::fmt::Display for KeepassXCError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for KeepassXCError {
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    static ref KEYBOX: Mutex<crypto_box::Box> = Mutex::new(exchange_key());
    static ref PRIVKEY: Mutex<SecretKey> = Mutex::new(SecretKey::generate(&mut thread_rng()));
    static ref CLIENT_ID: Mutex<String> = Mutex::new(encode(generate_nonce(&mut thread_rng())));
    static ref STREAM: Mutex<UnixStream> =
        Mutex::new(UnixStream::connect(get_socketpath()).unwrap());
}

fn get_socketpath() -> std::path::PathBuf {
    ProjectDirs::from("org", "keepassxc", "KeePassXC.BrowserServer")
        .and_then(|dirs| dirs.runtime_dir().map(|dir| std::path::PathBuf::from(dir)))
        .unwrap()
}

fn exchange_key() -> crypto_box::Box {
    let secret_key = PRIVKEY.lock().unwrap();
    // try to connect to keepassxc and exchange keys
    let message = object! {
        action: "change-public-keys",
        publicKey: encode(secret_key.public_key().as_bytes()),
        nonce: encode(generate_nonce(&mut thread_rng())),
        clientID: CLIENT_ID.lock().unwrap().clone(),
    };

    let response = send_clear(&message).unwrap();
    let key = decode(response["publicKey"].as_str().unwrap()).unwrap();
    let key = TryInto::<[u8; 32]>::try_into(key).unwrap();

    let keepassxc_pubkey = PublicKey::from(key);
    crypto_box::Box::new(&keepassxc_pubkey, &secret_key)
}

fn send_clear(message: &JsonValue) -> Result<JsonValue> {
    lazy_static! {
        static ref BUF: Mutex<[u8; 1024]> = Mutex::new([0; 1024]);
    }
    let mut stream = STREAM.lock()?;
    let mut buf = BUF.lock()?;

    message.write(&mut *stream)?;
    let size = stream.read(&mut *buf)?;
    Ok(json::parse(std::str::from_utf8(&buf[..size])?)?)
}

fn send_encrypt(message: &JsonValue) -> Result<JsonValue> {
    let nonce = generate_nonce(&mut thread_rng());
    let keybox = KEYBOX.lock()?;
    let message = object! {
        action: message["action"].as_str().unwrap(),
        message: encode(keybox.encrypt(&nonce, message.dump().as_bytes()).unwrap()),
        nonce: encode(nonce),
        clientID: CLIENT_ID.lock()?.as_str()
    };
    let response = send_clear(&message)?;
    let response = keybox
        .decrypt(
            Nonce::from_slice(decode(response["nonce"].as_str().unwrap())?.as_slice()),
            decode(response["message"].as_str().unwrap())?.as_slice(),
        )
        .unwrap();
    Ok(json::parse(std::str::from_utf8(response.as_slice())?)?)
}

fn generate_idkey() {
    *ID_KEY.lock().unwrap() = Some(encode(generate_nonce(&mut thread_rng())));
}

pub fn associate() {
    // check database id to see if we have already associated
    let mut database_id = DATABASE_ID.lock().unwrap();

    if database_id.is_some() {
        return;
    }

    // if we don't yet have an id key, generate one
    if ID_KEY.lock().unwrap().is_none() {
        generate_idkey();
    }

    // try to associate with current database
    let message = object! {
        action:"associate",
        key: encode(PRIVKEY.lock().unwrap().public_key().as_bytes()),
        idKey: ID_KEY.lock().unwrap().as_ref().unwrap().as_str(),
    };
    let response = send_encrypt(&message).unwrap();
    *database_id = Some(String::from(response["hash"].as_str().unwrap()));
}

pub fn get_passphrase() -> Result<String> {
    let keygrep = KEYGREP.lock().unwrap();
    let database_id = DATABASE_ID.lock().unwrap();
    let id_key = ID_KEY.lock().unwrap();

    if keygrep.is_none() || database_id.is_none() || id_key.is_none() {
        return Err(Box::new(KeepassXCError("Did not connect to keepassxc".to_string())))
    }

    let message = object! {
        action: "get-logins",
        url: (String::from("gpg://") + keygrep.as_ref().unwrap()).as_str(),
        keys: [
            {
                id: database_id.as_ref().unwrap().as_str(),
                key: id_key.as_ref().unwrap().as_str(),
            }
        ]
    };

    let entries = &send_encrypt(&message)?["entries"];
    if entries.len() == 0 {
        Err(Box::new(KeepassXCError("No matching entry found".to_string())))
    } else {
       Ok(String::from(entries[0]["password"].as_str().unwrap()))
    }
}

pub fn init() {
    if ID_KEY.lock().unwrap().is_none(){
        generate_idkey();
    }
    associate();
}
