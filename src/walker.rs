use std::io;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct MarkdownFile {
    path: PathBuf,
}

impl MarkdownFile {
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

pub fn find_markdown_files<P: AsRef<Path>>(root_dir: P) -> Result<Vec<MarkdownFile>, io::Error> {
    let mut files = vec![];
    for entry in WalkDir::new(root_dir) {
        let entry = entry?;
        let path = entry.path();
        if is_accepted_markdown_file(&path) {
            files.push(MarkdownFile { path: path.to_owned() });
        }
    }

    Ok(files)
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
