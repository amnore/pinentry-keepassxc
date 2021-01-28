use crate::state::{ID_KEY, KEYGREP};
use base64::{decode, encode};
use crypto_box::{aead::Aead, generate_nonce, PublicKey, SecretKey};
use directories::BaseDirs;
use json::{object, JsonValue};
use lazy_static::lazy_static;
use rand::thread_rng;
use std::convert::TryInto;
use std::error::Error;
use std::io::{Read, Write};
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

impl Error for KeepassXCError {}

impl KeepassXCError {
    fn new(response: &JsonValue) -> KeepassXCError {
        if let Some(err) = response["error"].as_str() {
            KeepassXCError(err.to_string())
        } else {
            KeepassXCError(format!("invalid field in {}", response))
        }
    }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

lazy_static! {
    static ref PRIVKEY: Mutex<SecretKey> = Mutex::new(SecretKey::generate(&mut thread_rng()));
    static ref CLIENT_ID: Mutex<String> = Mutex::new(encode(generate_nonce(&mut thread_rng())));
    static ref KEYBOX: Mutex<Option<crypto_box::Box>> = Mutex::new(exchange_key().ok());
    static ref STREAM: Mutex<Option<UnixStream>> =
        Mutex::new(UnixStream::connect(get_socketpath()).ok());
    static ref DATABASE_ID: Mutex<Option<String>> = Mutex::new(get_databasehash().ok());
}

fn get_socketpath() -> std::path::PathBuf {
    let socket = std::path::Path::new("org.keepassxc.KeePassXC.BrowserServer");
    let path = BaseDirs::new().unwrap().runtime_dir().unwrap().join(socket);
    eprintln!("connect: {:?}", path);
    path
}

fn exchange_key() -> Result<crypto_box::Box> {
    let secret_key = PRIVKEY.lock()?;
    let message = object! {
        action: "change-public-keys",
        publicKey: encode(secret_key.public_key().as_bytes()),
        nonce: encode(generate_nonce(&mut thread_rng())),
        clientID: CLIENT_ID.lock()?.as_str(),
    };

    // try to connect to keepassxc and exchange keys
    let response = send_clear(&message)?;
    let key = decode(response["publicKey"].as_str().ok_or_else(|| KeepassXCError::new(&response))?)?;
    let key = TryInto::<[u8; 32]>::try_into(key).or_else(|_| Err(KeepassXCError("wrong format of public key".to_string())))?;

    let keepassxc_pubkey = PublicKey::from(key);
    Ok(crypto_box::Box::new(&keepassxc_pubkey, &secret_key))
}

fn send_clear(message: &JsonValue) -> Result<JsonValue> {
    let mut stream = STREAM.lock()?;
    let stream = stream.as_mut().ok_or_else(|| KeepassXCError("could not connect to keepassxc".to_string()))?;
    let mut buf = [0u8; 1024];

    writeln!(stream, "{}", message)?;
    eprintln!("sent: {}", message);
    let size = stream.read(&mut buf)?;
    eprintln!("read: {}", std::str::from_utf8(&buf[..size])?);
    Ok(json::parse(std::str::from_utf8(&buf[..size])?)?)
}

fn send_encrypt(message: &JsonValue) -> Result<JsonValue> {
    let keybox = KEYBOX.lock()?;
    let keybox = keybox.as_ref().ok_or_else(|| KeepassXCError("could not exchange key with keepassxc".to_string()))?;
    let nonce = generate_nonce(&mut thread_rng());
    let message = object! {
        action: message["action"].as_str().unwrap(),
        message: encode(keybox.encrypt(&nonce, message.dump().as_bytes()).unwrap()),
        nonce: encode(nonce),
        clientID: CLIENT_ID.lock()?.as_str()
    };
    let response = send_clear(&message)?;
    let response = keybox
        .decrypt(
            Nonce::from_slice(
                decode(
                    response["nonce"]
                        .as_str()
                        .ok_or_else(|| KeepassXCError::new(&response))?,
                )?.as_slice(),
            ),
            decode(response["message"].as_str().ok_or_else(|| KeepassXCError::new(&response))?)?.as_slice(),
        ).or_else(|_| Err(KeepassXCError("could not decrypt message".to_string())))?;
    Ok(json::parse(std::str::from_utf8(response.as_slice())?)?)
}

fn associate() -> Result<()> {
    // check if we have already associated
    if test_associate().is_ok() {
        return Ok(());
    }

    // try to associate with current database
    let message = object! {
        action:"associate",
        key: encode(PRIVKEY.lock()?.public_key().as_bytes()),
        idKey: ID_KEY.lock()?.as_str(),
    };
    send_encrypt(&message)?;

    // if no error occurs, then we succeeded
    Ok(())
}

fn test_associate() -> Result<()> {
    let message = object! {
        action: "test-associate",
        id: DATABASE_ID.lock()?.as_ref().ok_or_else(|| KeepassXCError("did not connect to keepassxc".to_string()))?.as_str(),
        key: ID_KEY.lock()?.as_str(),
    };

    send_encrypt(&message)?;
    Ok(())
}

// get database hash from keepassxc, and store it in DATABASE_ID
fn get_databasehash() -> Result<String> {
    let message = object! {
        action: "get-databasehash"
    };

    let response = send_encrypt(&message)?;
    Ok(response["hash"].as_str().ok_or_else(|| KeepassXCError::new(&response))?.to_string())
}

pub fn get_passphrase() -> Result<String> {
    let keygrep = KEYGREP.lock()?;
    let keygrep = keygrep.as_ref().ok_or_else(|| KeepassXCError("did not set keygrep".to_string()))?;
    let database_id = DATABASE_ID.lock()?;
    let database_id = database_id.as_ref().ok_or_else(|| KeepassXCError("did not associate".to_string()))?;
    let id_key = ID_KEY.lock()?;

    let message = object! {
        action: "get-logins",
        url: "gpg://".to_string() + &keygrep,
        keys: [
            {
                id: database_id.as_str(),
                key: id_key.as_str(),
            }
        ]
    };

    let entries = &send_encrypt(&message)?["entries"];
    Ok(String::from(entries[0]["password"].as_str().ok_or_else(|| KeepassXCError("no matching entry found".to_string()))?))
}

pub fn init() {
    if let Err(e) = associate() {
        eprintln!("{}", e);
    }
}
