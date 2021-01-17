use log::{error, info};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

static mut CHILD: Option<Child> = None;
static mut CHILDOUT: Option<BufReader<ChildStdout>> = None;
static mut CHILDIN: Option<BufWriter<ChildStdin>> = None;

pub fn handle_cmd(cmd: &String) -> String {
    // forward command to child
    info!("Forwarding '{}' to child.", cmd);
    write_child(cmd);
    read_child()
}

pub fn init() {
    info!("Starting child process.");
    unsafe {
        CHILD = Some(
            Command::new("pinentry")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap(),
        );
        CHILDOUT = CHILD
            .as_mut()
            .map(|child| BufReader::new(child.stdout.take().unwrap()));
        CHILDIN = CHILD
            .as_mut()
            .map(|child| BufWriter::new(child.stdin.take().unwrap()));
        if !CHILDOUT.is_some() || !CHILDIN.is_some() {
            error!("Failed to start child process.");
            panic!();
        }
    }

    // Forward hello message
    print!("{}", read_child());
}

/**
 * Write to child
 */
fn write_child(cmd: &String) {
    unsafe {
        let childin = CHILDIN.as_mut().unwrap();
        let ok = write!(childin, "{}", cmd).is_ok() && childin.flush().is_ok();
        if !ok {
            error!("Failed to write to child.");
            panic!();
        }
    }
}

/**
 * Read until we reach the end of a reply.
 */
fn read_child() -> String {
    let mut buf = String::new();
    let mut begin = buf.len();
    unsafe {
        let childout = CHILDOUT.as_mut().unwrap();
        loop {
            let ok = childout.read_line(&mut buf).is_ok();
            if !ok {
                error!("Failed to read from child.");
                panic!();
            }
            let line = &buf[begin..];
            if line.starts_with("OK ")
                || line.starts_with("ERR ")
                || line == "OK\n"
            {
                break buf;
            }
            begin = buf.len();
        }
    }
}

#[cfg(test)]
mod test {}
