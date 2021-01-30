use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use directories::BaseDirs;
use lazy_static::lazy_static;
use log::{Log, Metadata, Record};

lazy_static! {
    pub static ref LOGFILE: FileLogger = FileLogger::new();
}

pub struct FileLogger(Mutex<File>);

impl Log for FileLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, rec: &Record) {
        writeln!(self.0.lock().unwrap(), "{} - {}", rec.level(), rec.args())
            .expect("could not write log");
    }

    fn flush(&self) {
        self.0.lock().unwrap().flush().expect("could not flush log");
    }
}

impl FileLogger {
    fn new() -> FileLogger {
        let logfile = File::create(
            BaseDirs::new()
                .unwrap()
                .cache_dir()
                .join("pinentry-keepassxc.log"),
        )
        .unwrap();
        FileLogger(Mutex::new(logfile))
    }
}

pub fn init() {
    log::set_logger(&*LOGFILE).expect("could not set logger");
    log::set_max_level(log::LevelFilter::Info);
}
