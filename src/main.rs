use log::{error, info};
use pinentry_keepassxc::assuan;
use std::io::stdin;

fn main() {
    assuan::init();
    let stdin = stdin();
    let mut line = String::new();
    loop {
        if let Err(e) = stdin.read_line(&mut line) {
            error!("Unable to read input: {}", &e);
        }
        info!("Agent: {}", &line);
        let reply = assuan::handle_cmd(&line);
        info!("Pinentry: {}", &reply);
        println!("{}", reply);
    }
}
