use log::{error, info};
use state::Storage;
use std::process::{Child, Command, Stdio};

static CHILD: Storage<Child> = Storage::new();

pub fn handle_cmd(cmd: &String) -> String {
    unimplemented!();
}

pub fn init() {
    info!("Starting child process.");
    let ok = CHILD.set(
        Command::new("pinentry")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap(),
    );
    if !ok {
        error!("Could not start child process.");
        panic!();
    }
}

#[cfg(test)]
mod test {}
