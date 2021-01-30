use std::env;
use std::io::{stdin, stdout, Write};

use log::{info, Log};

use pinentry_keepassxc::assuan;
use pinentry_keepassxc::config;
use pinentry_keepassxc::keepassxc;
use pinentry_keepassxc::logging;

fn main() {
    if cfg!(debug_assertions) {
        logging::init();
    }
    config::load();
    assuan::init();
    keepassxc::init();
    let stdin = stdin();
    let mut stdout = stdout();

    // log args and env variables
    info!("args:");
    for arg in env::args() {
        info!("{}", arg);
    }
    info!("environment variables:");
    for (k, v) in env::vars() {
        info!("{}={}", k, v);
    }

    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("Unable to read input");
        info!("agent: {}", line);
        let reply = assuan::handle_cmd(&line);
        info!("reply: {}", reply.as_str().replace("\n", "\\n"));
        stdout
            .write_all(reply.as_bytes())
            .expect("Unable to write to output");
        stdout.flush().expect("Unable to flush output");
        if Some("BYE") == line.split_whitespace().next() {
            break;
        }
    }
    config::store();
    logging::LOGFILE.flush();
}
