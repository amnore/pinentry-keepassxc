use crate::state::STATE;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::{stdout, BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;

lazy_static! {
    static ref CHILD: Mutex<Child> = Mutex::new(
        Command::new("pinentry")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    static ref CHILDOUT: Mutex<BufReader<ChildStdout>> =
        Mutex::new(BufReader::new(CHILD.lock().unwrap().stdout.take().unwrap()));
    static ref CHILDIN: Mutex<BufWriter<ChildStdin>> =
        Mutex::new(BufWriter::new(CHILD.lock().unwrap().stdin.take().unwrap()));
    static ref HANDLER: HashMap<&'static str, fn(&String) -> String> = {
        let mut m: HashMap<&'static str, fn(&String) -> String> = HashMap::new();
        m.insert("BYE", handle_bye);
        m.insert("SETKEYINFO", handle_setkeyinfo);
        m
    };
}

pub fn handle_cmd(cmd: &String) -> String {
    match cmd
        .split_whitespace()
        .next()
        .and_then(|cmd| HANDLER.get(cmd))
    {
        Some(handler) => handler(cmd),
        None => handle_default(cmd),
    }
}

pub fn init() {
    // Forward hello message
    stdout()
        .write_all(read_child().as_bytes())
        .expect("Unable forward hello message");
}

/**
 * Write to child
 */
fn write_child(cmd: &String) {
    let mut childin = CHILDIN.lock().unwrap();
    childin
        .write_all(cmd.as_bytes())
        .expect("Unable to write to child");
    childin.flush().expect("Unable to flush child");
}

/**
 * Read until reaching the end of a reply.
 */
fn read_child() -> String {
    let mut buf = String::new();
    let mut childout = CHILDOUT.lock().unwrap();
    loop {
        let len = buf.len();
        childout
            .read_line(&mut buf)
            .expect("Unable to read from child");
        let line = &buf[len..];
        if line.starts_with("OK ") || line.starts_with("ERR ") || line == "OK\n" {
            break buf;
        }
    }
}

fn handle_default(cmd: &String) -> String {
    // forward to child
    write_child(cmd);
    read_child()
}

fn handle_bye(cmd: &String) -> String {
    let reply = handle_default(cmd);
    CHILD.lock().unwrap().wait().expect("Child wasn't running");
    reply
}

fn handle_setkeyinfo(cmd: &String) -> String {
    let mut state = STATE.lock().unwrap();
    (*state).keygrep = Some(String::from(cmd.split_whitespace().nth(1).unwrap()));
    handle_default(cmd)
}
