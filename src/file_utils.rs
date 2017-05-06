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
pub fn write_file_in_dir<P: AsRef<Path>>(file_name: P,
                                         content_to_write: String,
                                         dir: P)
                                         -> io::Result<()> {
    fs::create_dir_all(&dir)?;
    write_to_file(dir.as_ref().join(file_name), content_to_write);
    Ok(())
}

/// Copies `file_name` located in `source_dir` across to `dest_dir` under the same name.
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(source_dir: &P,
                                                 dest_dir: &Q,
                                                 file_name: &String)
                                                 -> Result<(), io::Error> {
    let source = source_dir.as_ref().join(&file_name);
    let dest = dest_dir.as_ref().join(&file_name);
    info!("Performing copy {:?} -> {:?}", source, dest);
    let _ = fs::copy(source, dest)?;
    Ok(())
}

#[cfg(not(feature = "ci"))]
#[cfg(test)]
mod tests {
    use std::fs;

    const CONTENT: &'static str = "This is a test";
    const FILE_NAME: &'static str = "test_out.txt";
    // I do admit there is huge dependencies here but meh.
    #[test]
    fn test_write_and_read_file() {
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

    #[test]
    fn test_dir_exists_write() {
        const DIR: &str = "src";
        super::write_file_in_dir(FILE_NAME, CONTENT.to_string(), DIR).unwrap();
        assert_eq!(CONTENT,
                   super::read_from_file(DIR.to_owned() + "/" + FILE_NAME));

        fs::remove_file(DIR.to_owned() + "/" + FILE_NAME).unwrap();
    }

    #[test]
    fn test_dir_write() {
        const DIR: &str = "awoogaa";
        super::write_file_in_dir(FILE_NAME, CONTENT.to_string(), DIR).unwrap();
        assert_eq!(CONTENT,
                   super::read_from_file(DIR.to_owned() + "/" + FILE_NAME));

        fs::remove_file(DIR.to_owned() + "/" + FILE_NAME).unwrap();
        fs::remove_dir(DIR).unwrap();
    }
}
