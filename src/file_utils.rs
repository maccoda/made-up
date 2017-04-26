use std::fs::{self, File};
use std::io::{Read, Write};
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

#[cfg(test)]
mod tests {
    use std::fs;
    // I do admit there is huge dependencies here but meh.
    #[test]
    fn test_write_and_read_file() {
        const CONTENT: &'static str = "This is a test";
        const FILE_NAME: &'static str = "test_out.txt";
        super::write_to_file(FILE_NAME, String::from(CONTENT));
        // Check it exists
        assert!(fs::metadata(FILE_NAME).is_ok());
        // Check is not a directory
        assert!(!fs::metadata(FILE_NAME).unwrap().is_dir());

        assert_eq!(CONTENT, super::read_from_file(FILE_NAME));

        fs::remove_file(FILE_NAME).unwrap();
    }

    #[test]
    fn test_check_file() {
        // Will check I exist
        assert!(super::check_file_exists("src/file_utils.rs"));
    }
}
