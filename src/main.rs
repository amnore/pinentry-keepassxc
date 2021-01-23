use pinentry_keepassxc::assuan;
use std::io::{stdin, stdout, Write};
use pinentry_keepassxc::config;

fn main() {
    config::load();
    assuan::init();
    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Unable to read input");
        let reply = assuan::handle_cmd(&line);
        stdout
            .write_all(reply.as_bytes())
            .expect("Unable to write to output");
        stdout.flush().expect("Unable to flush output");
        if Some("BYE") == line.split_whitespace().next() {
            break;
        }
    }
    config::store();
}
