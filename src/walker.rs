use std::io;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir, WalkDirIterator};
use pulldown_cmark::{Event, Parser, Tag};
use file_utils;

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
        MarkdownFileList {
            files: sorted_files,
        }
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
        MarkdownFile {
            path: path.to_path_buf(),
        }
    }
    /// Return the path of the Markdown file
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

    /// Return the main heading of the Markdown file
    pub fn get_heading(&self) -> String {
        // Obtain the heading
        let content = file_utils::read_from_file(&self.path)
            .expect(&format!("Unable to read Markdown file: {:?}", self.path));
        let parser = Parser::new(&content);
        let mut iter = parser.into_iter();
        let mut opt_header = None;
        let mut in_header = false;
        while let Some(event) = iter.next() {
            // Look for a start event for a heading
            if let Event::Start(tag) = event {
                // Check the tag
                if let Tag::Header(num) = tag {
                    if num == 1 {
                        in_header = true;
                    }
                }
            } else if let Event::Text(text) = event {
                if in_header {
                    opt_header = Some(text.to_string());
                    break;
                }
            }
        }
        opt_header.expect(&format!("No header 1 found for {:?}", self.path))
    }
}

/// Checks that the file extension matches the expected _md_.
fn is_accepted_markdown_file(path: &Path) -> bool {
    const FILE_EXT: &str = "md";
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
            debug!("Adding file {:?}", path);
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
        let file = super::MarkdownFile {
            path: PathBuf::from("resources/tester.md"),
        };
        assert_eq!(file.get_file_name(), "tester");
    }

    #[test]
    fn test_find_markdown_files() {
        const ROOT_DIR: &str = "tests/resources/input/site";
        let files = super::find_markdown_files(ROOT_DIR).unwrap();
        const SKIPPED_TOP: &str = "_ignored_top.md";
        const SKIPPED_NESTED: &str = "_ignored_nested.md";

        let file_names: Vec<String> = files.iter().map(|x| x.get_file_name()).collect();
        assert!(!file_names.contains(&SKIPPED_TOP.to_string()));
        assert!(!file_names.contains(&SKIPPED_NESTED.to_string()));
    }
}
