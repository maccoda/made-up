// *****************************
// NOTE NOTE NOTE
// *****************************
// This file is just a copy of file_utils.rs and test_utils.rs
// Will hopefully work out a better way to do this but for now this will do.
// Perhaps make it part of the build step to keep them in sync
#![allow(dead_code)]
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

/// Writes the provided content to a file at the path provided.
pub fn write_to_file<P: AsRef<Path>>(file_name: P, content_to_write: String) {
    let mut file = File::create(file_name).unwrap();
    let content: &[u8] = &(content_to_write.into_bytes())[..];
    file.write_all(content).unwrap();
}

/// Reads and returns the contents of the file at the path provided.
pub fn read_from_file<P: AsRef<Path>>(file_name: P) -> String {
    let mut content = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut content).unwrap();
    content
}

/// Checks if a file exists. Will return false if it exists or it exists but is a directory.
pub fn check_file_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).map(|x| !x.is_dir()).unwrap_or(false)
}

/// Use to write to a file that is in a separate directory. This will ensure
/// that directory is created and create if required before writing to the file.
pub fn write_file_in_dir<P: AsRef<Path>>(
    file_name: P,
    content_to_write: String,
    dir: P,
) -> io::Result<()> {
    fs::create_dir_all(&dir)?;
    write_to_file(dir.as_ref().join(file_name), content_to_write);
    Ok(())
}

/// Copies `file_name` located in `source_dir` across to `dest_dir` under the same name.
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(
    source_dir: &P,
    dest_dir: &Q,
    file_name: &str,
) -> Result<(), io::Error> {
    let source = source_dir.as_ref().join(&file_name);
    let dest = dest_dir.as_ref().join(&file_name);
    let _ = fs::copy(source, dest)?;
    Ok(())
}

/// Strips all whitespace anywhere within the string. Useful for comparing
/// strings when only caring about content.
fn strip_all_whitespace(string: &str) -> String {
    string.chars().filter(|x| !x.is_whitespace()).collect()
}

/// Asserts the two strings provided have the same non-whitespace content.
pub fn compare_string_content(expected: &str, actual: &str) {
    let expected = strip_all_whitespace(expected);
    let actual = strip_all_whitespace(actual);

    assert_eq!(expected, actual);
}

use log::{LogLevel, LogMetadata, LogRecord};

pub struct SimpleLogger;

impl ::log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Debug
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}
