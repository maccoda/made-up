extern crate made_up;
extern crate log;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use made_up::{Error, ErrorKind};

fn main() {
    log::set_logger(|max_log_level| {
                        max_log_level.set(::log::LogLevelFilter::Debug);
                        Box::new(SimpleLogger)
                    })
            .unwrap();
    let matches = App::new("Made-Up Static Site Generator")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("root_dir")
                 .help("Root directory of Markdown files")
                 .index(1)
                 .required(false))
        .get_matches();

    let dir = matches.value_of("root_dir").unwrap_or(".");
    let convertor: made_up::Convertor = handle_error(made_up::Convertor::new(dir));
    let files = handle_error(convertor.generate_site());
    handle_error(convertor.write_files(files));

}
use std::fmt::Debug;
fn handle_error<T: Debug>(possible_error: Result<T, Error>) -> T {
    if possible_error.is_err() {
        match possible_error.unwrap_err() {
            made_up::Error(ErrorKind::Config(e), _) => println!("Configuration Error: {:?}", e),
            made_up::Error(ErrorKind::Fail(e), _) => println!("Error: {}", e),
            made_up::Error(ErrorKind::IO(e), _) => println!("IO Error: {:?}", e),
            made_up::Error(ErrorKind::Template(e), _) => {
                println!("Template Generation Error: {:?}", e)
            }
            made_up::Error(ErrorKind::Msg(msg), _) => println!("{}", msg),
        };
        std::process::exit(1);
    } else {
        possible_error.unwrap()
    }
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
