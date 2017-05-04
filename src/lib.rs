extern crate pulldown_cmark;
extern crate handlebars;
extern crate walkdir;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

use std::fs::{self, File};
use std::io::Read;
use std::path::Path;



mod html;
mod walker;
mod file_utils;
mod config;
mod templates;

#[cfg(test)]
mod test_utils;

use walker::FileList;

/// Error type for the conversion of the markdown files to the static site.
#[derive(Debug)]
pub enum ConvError {
    Fail,
    IO(std::io::Error),
    Template(handlebars::RenderError),
}

impl From<std::io::Error> for ConvError {
    fn from(error: std::io::Error) -> ConvError {
        ConvError::IO(error)
    }
}

impl From<handlebars::RenderError> for ConvError {
    fn from(error: handlebars::RenderError) -> ConvError {
        ConvError::Template(error)
    }
}

// TODO Have less functionality in this top level package.

const DEF_OUT_DIR: &str = "./out";
/// Entry function which will perform the entire process for the static site
/// generation.
///
/// Through here it will:
///
/// * Find all markdown files to use
/// * Convert all to HTML
/// * Read the configuration to determine how the output should be produced
/// * Copy across required resources (stylesheet, referenced images, etc.)
pub fn generate_site<P: AsRef<Path>>(root_dir: P) -> Result<(), ConvError> {
    info!("Generating site from directory: {}", root_dir.as_ref().display());
    let all_files = find_all_files(&root_dir);
    let configuration = read_config(&root_dir)?;
    let out_dir = handle_config(&root_dir.as_ref(), &configuration)?;
    if configuration.gen_index().unwrap_or(false) {
        debug!("Index to be generated");
        let index_content = templates::generate_index(&all_files, &configuration).unwrap();
        file_utils::write_file_in_dir("index.html", index_content, &out_dir)?;
    }
    for file in all_files.get_files() {
        let result = create_html(file.get_path(), &configuration).unwrap();
        file_utils::write_file_in_dir(format!("{}.html", file.get_file_name()), result, out_dir.to_owned())?;
    }

    // Copy across the stylesheet
    configuration.stylesheet().and_then(|x| {
        let source = root_dir.as_ref().join(&x);
        debug!("Source stylesheet {:?}", source);
        let dest = out_dir.to_owned() + "/" + &x;
        debug!("Dest stylesheet: {:?}", dest);
        fs::copy(source, dest).ok()}
    ).unwrap();
    Ok(())
}



/// Starting at the root directory provided, find all Markdown files within in.
fn find_all_files<P: AsRef<Path>>(root_dir: P) -> FileList {
    // TODO Make this handle errors and document
    let files = walker::find_markdown_files(root_dir).unwrap();
    for file in &files {
        debug!("{:?}", file);
    }
    FileList::new(files)
}

/// Converts the provided Markdown file to it HTML equivalent. This ia a direct
/// mapping it does not add more tags, such as `<body>` or `<html>`.
fn create_html<P: AsRef<Path>>(file_name: P,
                               config: &config::RawConfiguration)
                               -> Result<String, ConvError> {
    let mut content = String::new();
    File::open(file_name)
        .and_then(|mut x| x.read_to_string(&mut content))?;
    let parser = pulldown_cmark::Parser::new_ext(&content, pulldown_cmark::OPTION_ENABLE_TABLES);

    templates::encapsulate_bare_html(html::consume(parser), config)
}


// TODO Add some testing on this
/// Finds the configuration file and deserializes it.
fn read_config<P: AsRef<Path>>(path: P) -> Result<config::RawConfiguration, ConvError> {
    const CONFIG_NAME: &'static str = "mdup.yml";
    let full_path = path.as_ref().to_path_buf();
    debug!("Starting search for configuration file at: {:?}",
           path.as_ref());
    let root_iter = fs::read_dir(&path)?.filter_map(|x| x.ok());
    for entry in root_iter {
        if let Ok(file_name) = entry.file_name().into_string() {
            if file_name.eq(CONFIG_NAME) {
                return Ok(config::RawConfiguration::from(full_path.join(file_name)));
            }
        }
    }
    warn!("Configuration file: {} not found in root directory",
          CONFIG_NAME);
    Err(ConvError::Fail)
}

// TODO Test this
/// Processes the configuration and produces a configuration addressing if
/// aspects are not present and other implications.
fn handle_config(root_dir: &AsRef<Path>, config: &config::RawConfiguration) -> Result<String, ConvError> {
    // If not specified don't generate, if true generate
    if !config.gen_index().unwrap_or(false) {
        info!("Looking for {:?}", root_dir.as_ref().join("index.md"));
        file_utils::check_file_exists(root_dir.as_ref().join("index.md"));
        warn!("Expected index.md in the root directory");
        return Err(ConvError::Fail)
    }
    // Check for presence of output directory
    Ok(config.out_dir().unwrap_or(DEF_OUT_DIR.to_owned()))
}

#[cfg(test)]
mod tests {
    // TODO Nowhere near enough tests here
    use test_utils;
    #[test]
    fn test_create_html() {
        // Read expected
        let config = super::config::RawConfiguration::from("resources/mdup.yml");
        let expected = include_str!("../tests/resources/all_test_good.html");
        let actual = super::create_html("resources/all_test.md", &config).unwrap();
        test_utils::compare_string_content(expected.to_string(), actual);
    }
}
