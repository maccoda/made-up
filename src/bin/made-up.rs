extern crate made_up;
extern crate log;
use std::env;
fn main() {
    log::set_logger(|max_log_level| {
                        max_log_level.set(::log::LogLevelFilter::Debug);
                        Box::new(SimpleLogger)
                    })
            .unwrap();
    // NOTE Not a very robust way to do it
    // TODO Add in help display on -h or --help
    // .expect("Usage: made-up <root_directory>");
    let dir = env::args().nth(1).unwrap_or(".".to_string());
    made_up::generate_site(dir).unwrap_or_else(|x| println!("{:?}", x));
}

use log::{LogLevel, LogRecord, LogMetadata};

struct SimpleLogger;

impl ::log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}
