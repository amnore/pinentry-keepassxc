use lazy_static::lazy_static;
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
}

pub fn handle_cmd(cmd: &String) -> String {
    // forward command to child
    write_child(cmd);
    read_child()
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

#[cfg(test)]
mod test {}
