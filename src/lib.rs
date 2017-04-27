extern crate pulldown_cmark;
extern crate handlebars;
extern crate walkdir;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use handlebars::Handlebars;
use serde_json::Value;


mod html;
mod walker;
mod file_utils;
mod config;

#[cfg(test)]
mod test_utils;

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

/// Wrapper of a list of Markdown files. With end goal to be able to convey the
/// hierarchy.
pub struct FileList {
    files: Vec<walker::MarkdownFile>,
}

impl FileList {
    /// Get all Markdown files
    pub fn get_files(&self) -> &Vec<walker::MarkdownFile> {
        &self.files
    }
}

/// Entry function which will perform the entire process for the static site
/// generation.
///
/// Through here it will:
///
/// * Find all markdown files to use
/// * Convert all to HTML
/// * Read the configuration to determine how the output should beproduced
pub fn generate_site<P: AsRef<Path>>(root_dir: P) -> Result<(), ConvError> {
    let all_files = find_all_files(&root_dir);
    let configuration = read_config(&root_dir)?;
    let index_content = generate_index(&all_files, &configuration).unwrap();
    file_utils::write_to_file("index.html", index_content);
    for file in all_files.get_files() {
        let result = create_html(file.get_path(), &configuration).unwrap();

        file_utils::write_to_file(format!("{}.html", file.get_file_name()), result);
    }
    Ok(())
}

/// Construct a generated index page for the site from the list of files used.
fn generate_index(files: &FileList, config: &config::Configuration) -> Result<String, ConvError> {
    const TEMPLATE_NAME: &'static str = "index";
    // Build the page from the template just to make it easier for future us
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file(TEMPLATE_NAME,
                                &Path::new(&format!("templates/{}.hbs", TEMPLATE_NAME)))
        .unwrap();

    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    data.insert("stylesheet".to_string(),
                Value::String(config.stylesheet.clone()));
    // TODO Get the title perhaps from the configuration
    data.insert("title".to_string(),
                Value::String("Index Generated Title".to_string()));
    data.insert("element".to_string(),
                Value::Array(files
                                 .get_files()
                                 .iter()
                                 .map(|x| Value::String(x.get_file_name()))
                                 .collect()));

    let output = handlebars.render(TEMPLATE_NAME, &data)?;
    Ok(output)
}

/// Starting at the root directory provided, find all Markdown files within in.
fn find_all_files<P: AsRef<Path>>(root_dir: P) -> FileList {
    // TODO Make this handle errors and document
    let files = walker::find_markdown_files(root_dir).unwrap();
    for file in &files {
        debug!("{:?}", file);
    }
    FileList { files: files }
}

/// Converts the provided Markdown file to it HTML equivalent. This ia a direct
/// mapping it does not add more tags, such as `<body>` or `<html>`.
fn create_html<P: AsRef<Path>>(file_name: P,
                               config: &config::Configuration)
                               -> Result<String, ConvError> {
    let mut content = String::new();
    File::open(file_name)
        .and_then(|mut x| x.read_to_string(&mut content))?;
    let parser = pulldown_cmark::Parser::new_ext(&content, pulldown_cmark::OPTION_ENABLE_TABLES);

    encapsulate_bare_html(html::consume(parser), config)
}

/// Take a HTML string and encapsulate with the correct tags. Will also add the stylesheet.
fn encapsulate_bare_html(content: String,
                         config: &config::Configuration)
                         -> Result<String, ConvError> {
    const TEMPLATE_NAME: &'static str = "basic";
    // Build the page from the template just to make it easier for future us
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file(TEMPLATE_NAME,
                                &Path::new(&format!("templates/{}.hbs", TEMPLATE_NAME)))
        .unwrap();

    let mut data: BTreeMap<String, String> = BTreeMap::new();
    data.insert("stylesheet".to_string(), config.stylesheet.clone());
    // TODO Get the title from the first heading
    data.insert("title".to_string(), "Generated Title".to_string());
    data.insert("md_content".to_string(), content);

    let output = handlebars.render(TEMPLATE_NAME, &data)?;
    Ok(output)
}
// TODO Add some testing on this
/// Finds the configuration file and deserializes it.
fn read_config<P: AsRef<Path>>(path: P) -> Result<config::Configuration, ConvError> {
    const CONFIG_NAME: &'static str = "mdup.yml";
    let full_path = path.as_ref().to_path_buf();
    debug!("Starting search for configuration file at: {:?}",
           path.as_ref());
    let root_iter = fs::read_dir(&path)?.filter_map(|x| x.ok());
    for entry in root_iter {
        if let Ok(file_name) = entry.file_name().into_string() {
            if file_name.eq(CONFIG_NAME) {
                return Ok(config::Configuration::from(full_path.join(file_name)));
            }
        }
    }
    warn!("Configuration file: {} not found in root directory",
          CONFIG_NAME);
    Err(ConvError::Fail)
}

#[cfg(test)]
mod tests {
    use super::test_utils;
    #[test]
    fn test_create_html() {
        // Read expected
        let config = super::config::Configuration::from("resources/mdup.yml");
        let expected = include_str!("../tests/resources/all_test_good.html");
        let actual = super::create_html("resources/all_test.md", &config).unwrap();
        test_utils::compare_string_content(expected.to_string(), actual);
    }

    #[test]
    fn test_generate_index() {
        use std::path::Path;
        use super::walker::MarkdownFile;
        let config = super::config::Configuration::from("resources/mdup.yml");
        let expected = include_str!("../tests/resources/index_good.html");
        let actual = super::generate_index(&super::FileList {
                                                files: vec![MarkdownFile::from(&Path::new("all_test.md")),
                                                            MarkdownFile::from(&Path::new("second-page.md"))],
                                            }, &config).unwrap();
        test_utils::compare_string_content(expected.to_string(), actual);
    }
}
