use std::io;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir, WalkDirIterator};


/// Wrapper of a list of Markdown files. With end goal to be able to convey the
/// hierarchy.
pub struct MarkdownFileList {
    // Considering maintaining directory structure by Map<Vec<>>
    files: Vec<MarkdownFile>,
}

impl MarkdownFileList {
    pub fn new(files: Vec<MarkdownFile>) -> MarkdownFileList {
        let mut sorted_files = files;
        sorted_files.sort_by(|a, b| a.get_file_name().cmp(&b.get_file_name()));
        MarkdownFileList { files: sorted_files }
    }
    /// Get all Markdown files
    pub fn get_files(&self) -> &Vec<MarkdownFile> {
        &self.files
    }
}

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

/// Determines if the provided entry should be excluded from the files to check.
/// The check just determines if the file or directory begins with an
/// underscore.
fn is_excluded(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('_'))
        .unwrap_or(false)
}

/// Walks the specified root directory to find all Markdown files and returns
/// the list of discovered files. A file is added if it has a file extension of
/// `*.md` or `*.MD`. Files will be ignored if they begin with an underscore,
/// this also includes any Markdown files beginning with an underscore.
pub fn find_markdown_files<P: AsRef<Path>>(root_dir: P) -> Result<Vec<MarkdownFile>, io::Error> {
    let mut files = vec![];
    let files_to_check = WalkDir::new(root_dir)
        .into_iter()
        .filter_entry(|file| !is_excluded(file));
    for entry in files_to_check {
        let entry = entry?;
        let path = entry.path();
        if is_accepted_markdown_file(path) {
            info!("Adding file {:?}", path);
            // info!("Parent: {:?}", path.parent());
            files.push(MarkdownFile::from(path));
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    #[test]
    fn test_get_file_name() {
        let file = super::MarkdownFile { path: PathBuf::from("resources/tester.md") };
        assert_eq!(file.get_file_name(), "tester");
    }

    #[test]
    fn test_find_markdown_files() {
        const ROOT_DIR: &str = "resources";
        let files = super::find_markdown_files(ROOT_DIR).unwrap();
        const SKIPPED_TOP: &str = "_ignored_top.md";
        const SKIPPED_NESTED: &str = "_ignored_nested.md";

        let file_names: Vec<String> = files.iter().map(|x| x.get_file_name()).collect();
        assert!(!file_names.contains(&SKIPPED_TOP.to_string()));
        assert!(!file_names.contains(&SKIPPED_NESTED.to_string()));
    }
}
