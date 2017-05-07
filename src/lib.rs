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
use std::path::{Path, PathBuf};



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
    Fail(String),
    IO(std::io::Error),
    Template(handlebars::RenderError),
    Config(serde_yaml::Error),
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

impl From<serde_yaml::Error> for ConvError {
    fn from(error: serde_yaml::Error) -> ConvError {
        ConvError::Config(error)
    }
}

// TODO Have less functionality in this top level package.

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
    let root_dir: PathBuf = fs::canonicalize(&root_dir).unwrap();
    info!("Generating site from directory: {}", root_dir.display());
    let all_files = find_all_files(&root_dir)?;
    let configuration = read_config(&root_dir)?;
    handle_config(&root_dir, &configuration)?;
    let out_dir = configuration.out_dir();
    if configuration.gen_index() {
        debug!("Index to be generated");
        let index_content = templates::generate_index(&all_files, &configuration).unwrap();
        file_utils::write_file_in_dir("index.html", index_content, &out_dir)?;
    }
    for file in all_files.get_files() {
        let result = create_html(file.get_path(), &configuration).unwrap();
        file_utils::write_file_in_dir(format!("{}.html", file.get_file_name()),
                                      result,
                                      out_dir.to_owned())?;
    }

    // Copy across the stylesheet
    if configuration.copy_resources() {
        file_utils::copy_file(&root_dir, &out_dir, &configuration.stylesheet())?;

        // Copy across the images
        let images_source = root_dir.join("images");
        let images_dest = format!("{}/images", out_dir);
        fs::create_dir_all(&images_dest)?;
        for entry in fs::read_dir(format!("{}/images", root_dir.to_str().unwrap()))? {
            let entry = entry?;
            info!("Copying {:?}", entry.file_name());
            file_utils::copy_file(&images_source,
                                  &images_dest,
                                  &entry.file_name().into_string().unwrap())?;
        }
    }
    Ok(())
}



/// Starting at the root directory provided, find all Markdown files within in.
fn find_all_files<P: AsRef<Path>>(root_dir: P) -> Result<FileList, ConvError> {
    let files = walker::find_markdown_files(root_dir)?;
    for file in &files {
        debug!("{:?}", file);
    }
    Ok(FileList::new(files))
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

    templates::encapsulate_bare_html(html::consume(parser), config)
}


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
                return Ok(config::Configuration::from(full_path.join(file_name))?);
            }
        }
    }
    Err(ConvError::Fail(format!("Configuration file: {} not found in {}",
                                CONFIG_NAME,
                                fs::canonicalize(path).unwrap().display())))
}

/// Processes the configuration and produces a configuration addressing if
/// aspects are not present and other implications.
fn handle_config(root_dir: &AsRef<Path>, config: &config::Configuration) -> Result<(), ConvError> {
    // If not specified don't generate, if true generate
    if !config.gen_index() {
        let path = root_dir.as_ref().join("index.md");
        info!("Looking for {:?}", path);
        return if file_utils::check_file_exists(path) {
                   Ok(())
               } else {
                   Err(ConvError::Fail("Expected index.md in the root directory".into()))
               };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use test_utils;
    use std::env;
    use std::fs::File;
    #[test]
    fn test_create_html() {
        // Read expected
        let config = super::config::Configuration::from("resources/mdup.yml").unwrap();
        let expected = include_str!("../tests/resources/all_test_good.html");
        let actual = super::create_html("resources/all_test.md", &config).unwrap();
        test_utils::compare_string_content(expected, &actual);
    }

    // Ensure that will return an error when no configuration found
    #[test]
    fn test_fail_read_config() {
        assert!(super::read_config("src").is_err());
    }

    // Ensure that return error when no index found but specified it should not generate one
    #[test]
    fn test_fail_handle_config() {
        let config = super::config::Configuration::from("tests/resources/test_conf_all.yml")
            .unwrap();
        assert!(super::handle_config(&"resouces", &config).is_err());
    }

    // Ensure that return positive result when the index is not to be generated and one exists
    #[test]
    fn test_pass_handle_config() {
        let config = super::config::Configuration::from("tests/resources/test_conf_all.yml")
            .unwrap();
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push("index.md");

        File::create(tmp_dir).unwrap();
        assert!(super::handle_config(&env::temp_dir(), &config).is_ok());
    }
}
