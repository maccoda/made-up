use std::io;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct MarkdownFile {
    path: PathBuf,
}

impl MarkdownFile {
    /// Creates a `MarkdownFile` from the provided path
    pub fn from(path: &Path) -> MarkdownFile {
        MarkdownFile { path: path.to_path_buf() }
    }
    // Return the path of the Markdown file
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Return the name of the Markdown file
    pub fn get_file_name(&self) -> String {
        self.path
            .as_path()
            .file_stem()
            .and_then(|x| x.to_str())
            .unwrap()
            .to_string()
    }
}

fn is_accepted_markdown_file(path: &Path) -> bool {
    const FILE_EXT: &'static str = "md";
    if let Some(extension) = path.extension().and_then(|x| x.to_str()) {
        if extension.to_lowercase().eq(FILE_EXT) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    #[test]
    fn test_get_file_name() {
        let file = super::MarkdownFile { path: PathBuf::from("resources/tester.md") };
        assert_eq!(file.get_file_name(), "tester");
    }
}

pub fn find_markdown_files<P: AsRef<Path>>(root_dir: P) -> Result<Vec<MarkdownFile>, io::Error> {
    let mut files = vec![];
    for entry in WalkDir::new(root_dir) {
        let entry = entry?;
        let path = entry.path();
        if is_accepted_markdown_file(path) {
            files.push(MarkdownFile { path: path.to_owned() });
        }
    }

    Ok(files)
}
