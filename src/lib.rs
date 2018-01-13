#[macro_use]
extern crate error_chain;
extern crate handlebars;
#[macro_use]
extern crate log;
extern crate pulldown_cmark;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;
extern crate walkdir;

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

use walker::MarkdownFileList;

/// Error type for the conversion of the markdown files to the static site.
error_chain! {
    foreign_links {
        IO(std::io::Error);
        Template(handlebars::RenderError);
        TemplateCompile(handlebars::TemplateError);
        Config(serde_yaml::Error);
    }

    errors {
        Fail(t: String)
    }
}

#[derive(Debug)]
pub struct Convertor {
    configuration: config::Configuration,
    root_dir: PathBuf,
}

#[derive(Debug)]
pub struct ConvertedFile {
    path: PathBuf,
    content: String,
}

impl Convertor {
    /// Initialize a new convertor for the provided root directory.
    /// This will read and validate the configuration.
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Result<Convertor> {
        let root_dir: PathBuf = root_dir.as_ref().to_path_buf();
        info!(
            "Generating site from directory: {}",
            root_dir.canonicalize()?.display()
        );
        let configuration = read_config(&root_dir)?;
        handle_config(&root_dir, &configuration)?;
        Ok(Convertor {
            configuration,
            root_dir,
        })
    }

    /// Entry function which will perform the entire process for the static site
    /// generation.
    ///
    /// Through here it will:
    ///
    /// * Find all markdown files to use
    /// * Convert all to HTML
    pub fn generate_site(&self) -> Result<Vec<ConvertedFile>> {
        let mut converted_files = vec![];

        let all_files = find_all_files(&self.root_dir)?;

        let out_dir = self.configuration.out_dir();

        for file in all_files.get_files() {
            let result = create_html(file.get_path(), &self.configuration)?;
            converted_files.push(ConvertedFile {
                path: PathBuf::from(&out_dir).join(format!("{}.html", file.get_file_name())),
                content: result,
            })
        }
        let index_content = match self.configuration.index_template() {
            Some(index_path) => {
                // Generate it from what we have been given
                debug!("Using user defined index template");
                let template_path = self.root_dir.join(index_path);
                templates::render_index_with_template(
                    template_path,
                    &all_files,
                    &self.configuration,
                )?
            }
            None => {
                debug!("Using default index template");
                templates::generate_index(&all_files, &self.configuration)?
            }
        };

        converted_files.push(ConvertedFile {
            path: PathBuf::from(&out_dir).join("index.html"),
            content: index_content,
        });

        Ok(converted_files)
    }

    const IMAGE_DIR: &'static str = "images";
    /// Write the files provided to the file system
    ///
    /// The files provided will already be produced using `generate_site` and
    /// hence have all configuration information present
    pub fn write_files(&self, files: Vec<ConvertedFile>) -> Result<()> {
        if !file_utils::check_dir_exists(self.configuration.out_dir()) {
            fs::create_dir(self.configuration.out_dir())?;
        }
        for file in files {
            file_utils::write_to_file(file.path, file.content)?;
        }
        if self.configuration.copy_resources() {
            for stylesheet in &self.configuration.stylesheet() {
                // Copy across the stylesheet
                file_utils::copy_file(&self.root_dir, &self.configuration.out_dir(), stylesheet)?;
            }

            // Copy across the images
            let images_source = self.root_dir.join(Convertor::IMAGE_DIR);
            let images_dest =
                PathBuf::from(self.configuration.out_dir()).join(Convertor::IMAGE_DIR);
            fs::create_dir_all(&images_dest)?;
            for entry in fs::read_dir(&images_source)? {
                let entry = entry?;
                debug!("Copying {:?}", entry.file_name());
                file_utils::copy_file(
                    &images_source,
                    &images_dest,
                    &entry.file_name().into_string().unwrap(),
                )?;
            }
        }

        // Write across the default stylings. Need static string so can be
        // within the binary.
        let highlight_css = include_str!("../theme/highlight.css");
        let highlight_js = include_str!("../theme/highlight.js");
        let made_up_css = include_str!("../theme/made-up.css");
        let tomorrow_night_css = include_str!("../theme/tomorrow-night.css");

        file_utils::write_to_file(
            self.configuration.out_dir() + "/highlight.css",
            highlight_css.to_owned(),
        )?;
        file_utils::write_to_file(
            self.configuration.out_dir() + "/highlight.js",
            highlight_js.to_owned(),
        )?;
        file_utils::write_to_file(
            self.configuration.out_dir() + "/made-up.css",
            made_up_css.to_owned(),
        )?;
        file_utils::write_to_file(
            self.configuration.out_dir() + "/tomorrow-night.css",
            tomorrow_night_css.to_owned(),
        )?;

        Ok(())
    }
}

/// Starting at the root directory provided, find all Markdown files within in.
fn find_all_files<P: AsRef<Path>>(root_dir: P) -> Result<MarkdownFileList> {
    let files = walker::find_markdown_files(root_dir)?;
    for file in &files {
        debug!("{:?}", file);
    }
    Ok(MarkdownFileList::new(files))
}

/// Converts the provided Markdown file to it HTML equivalent. This ia a direct
/// mapping it does not add more tags, such as `<body>` or `<html>`.
fn create_html<P: AsRef<Path>>(file_name: P, config: &config::Configuration) -> Result<String> {
    let mut content = String::new();
    File::open(file_name).and_then(|mut x| x.read_to_string(&mut content))?;
    let parser = pulldown_cmark::Parser::new_ext(&content, pulldown_cmark::OPTION_ENABLE_TABLES);

    templates::encapsulate_bare_html(html::consume(parser), config)
}

/// Finds the configuration file and deserializes it.
fn read_config<P: AsRef<Path>>(path: P) -> Result<config::Configuration> {
    const CONFIG_NAME: &str = "mdup.yml";
    let full_path = path.as_ref().to_path_buf();
    debug!(
        "Starting search for configuration file at: {:?}",
        path.as_ref()
    );
    let root_iter = fs::read_dir(&path)?.filter_map(|x| x.ok());
    for entry in root_iter {
        if let Ok(file_name) = entry.file_name().into_string() {
            if file_name.eq(CONFIG_NAME) {
                return Ok(config::Configuration::from(full_path.join(file_name))?);
            }
        }
    }
    Err(ErrorKind::Fail(format!(
        "Configuration file: {} not found in {}",
        CONFIG_NAME,
        full_path.display()
    )).into())
}

/// Processes the configuration and ensure the environment is in a state
/// matching the definition in the configuration. This function will ensure:
/// * When the index template is specifed, that the specified file exists.
fn handle_config(root_dir: &AsRef<Path>, config: &config::Configuration) -> Result<()> {
    if config.index_template().is_some() {
        let path = root_dir.as_ref().join(config.index_template().unwrap());
        info!(
            "Checking that {:?} exists like the configuration says it will",
            path
        );
        if !file_utils::check_file_exists(path) {
            return Err(ErrorKind::Fail(
                "Did not find index template specified in configuration".into(),
            ).into());
        }
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
        let config =
            super::config::Configuration::from("tests/resources/input/site/mdup.yml").unwrap();
        let expected = include_str!("../tests/resources/output/all_test_good.html");
        let actual = super::create_html("tests/resources/input/site/all_test.md", &config).unwrap();
        test_utils::compare_string_content(expected, &actual);
    }

    // Ensure that will return an error when no configuration found
    #[test]
    fn test_fail_read_config() {
        assert!(super::read_config("src").is_err());
    }

    // Ensure that return error when no index found but specified it should not generate one
    #[test]
    fn test_fail_handle_config_no_index() {
        let config =
            super::config::Configuration::from("tests/resources/input/test_conf_all.yml").unwrap();
        assert!(super::handle_config(&"templates", &config).is_err());
    }

    // Ensure that return positive result when the index is not to be generated and one exists
    #[test]
    fn test_pass_handle_config() {
        let config =
            super::config::Configuration::from("tests/resources/input/test_conf_all.yml").unwrap();
        let mut tmp_dir = env::temp_dir();
        tmp_dir.push("index_test.hbs");

        File::create(tmp_dir).unwrap();
        assert!(super::handle_config(&env::temp_dir(), &config).is_ok());
    }
}
