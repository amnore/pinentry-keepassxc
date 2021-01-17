use std::io::{stdout, BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

static mut CHILD: Option<Child> = None;
static mut CHILDOUT: Option<BufReader<ChildStdout>> = None;
static mut CHILDIN: Option<BufWriter<ChildStdin>> = None;

pub fn handle_cmd(cmd: &String) -> String {
    // forward command to child
    write_child(cmd);
    read_child()
}

pub fn init() {
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
    }

    // Forward hello message
    stdout()
        .write_all(read_child().as_bytes())
        .expect("Unable forward hello message");
}

/**
 * Write to child
 */
fn write_child(cmd: &String) {
    unsafe {
        let childin = CHILDIN.as_mut().unwrap();
        childin
            .write_all(cmd.as_bytes())
            .expect("Unable to write to child");
        childin.flush().expect("Unable to flush child");
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
            childout
                .read_line(&mut buf)
                .expect("Unable to read from child");
            let line = &buf[begin..];
            if line.starts_with("OK ") || line.starts_with("ERR ") || line == "OK\n" {
                break buf;
            }
            begin = buf.len();
        }
    }
}

#[cfg(test)]
mod test {}
