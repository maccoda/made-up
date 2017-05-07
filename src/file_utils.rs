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

/// Copies `file_name` located in `source_dir` across to `dest_dir` under the same name.
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(source_dir: &P,
                                                 dest_dir: &Q,
                                                 file_name: &str)
                                                 -> Result<(), io::Error> {
    let source = source_dir.as_ref().join(&file_name);
    let dest = dest_dir.as_ref().join(&file_name);
    info!("Performing copy {:?} -> {:?}", source, dest);
    let _ = fs::copy(source, dest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::env;

    const CONTENT: &'static str = "This is a test";
    const FILE_NAME: &'static str = "test_out.txt";
    // I do admit there is huge dependencies here but meh.
    #[test]
    fn test_write_and_read_file() {
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push(FILE_NAME);
        super::write_to_file(&tmp_dir, String::from(CONTENT));
        // Check it exists
        assert!(fs::metadata(&tmp_dir).is_ok());
        // Check is not a directory
        assert!(!fs::metadata(&tmp_dir).unwrap().is_dir());

        assert_eq!(CONTENT, super::read_from_file(&tmp_dir));

        fs::remove_file(&tmp_dir).unwrap();
    }

    #[test]
    fn test_check_file() {
        // Will check I exist
        assert!(super::check_file_exists("src/file_utils.rs"));
    }
}
