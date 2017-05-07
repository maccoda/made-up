use std::path::Path;
use serde_yaml;

use super::file_utils;

/// Configuration type to be serialized. Allows for keys to not be present in
/// the configuration file.
#[derive(Serialize, Deserialize, Debug)]
struct RawConfiguration {
    stylesheet: Option<String>,
    gen_index: Option<bool>,
    out_dir: Option<String>,
    copy_resources: Option<bool>,
    title: Option<String>,
}

impl RawConfiguration {
    /// Construct a `RawConfiguration` from the provided path. Will return an
    /// error if unable to parse the YAML file.
    fn from<P: AsRef<Path>>(config_path: P) -> Result<RawConfiguration, serde_yaml::Error> {
        serde_yaml::from_str(&file_utils::read_from_file(config_path))
    }
}

// Mirror of `RawConfiguration` but has resolved all `Option`s to their default values.
#[derive(Debug)]
pub struct Configuration {
    stylesheet: String,
    gen_index: bool,
    out_dir: String,
    copy_resources: bool,
    title: String,
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            stylesheet: String::new(),
            gen_index: false,
            out_dir: "out".to_string(),
            copy_resources: true,
            title: "Title".to_string(),
        }
    }
}

impl Configuration {
    /// Obtain a `Configuration` from the path provided.
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Configuration, serde_yaml::Error> {
        debug!("Reading configuration file");
        let raw_config = RawConfiguration::from(path)?;
        let def_config = Configuration::default();
        let stylesheet = raw_config.stylesheet.unwrap_or(def_config.stylesheet);
        let gen_index = raw_config.gen_index.unwrap_or(def_config.gen_index);
        let out_dir = raw_config.out_dir.unwrap_or(def_config.out_dir);
        let copy_resources = raw_config
            .copy_resources
            .unwrap_or(def_config.copy_resources);
        let title = raw_config.title.unwrap_or(def_config.title);
        let config = Configuration {
            stylesheet,
            gen_index,
            out_dir,
            copy_resources,
            title,
        };
        debug!("{:?}", config);
        Ok(config)
    }
    /// Returns the stylesheet value of the configuration
    pub fn stylesheet(&self) -> String {
        self.stylesheet.clone()
    }
    /// Returns the gen_index value of the configuration
    pub fn gen_index(&self) -> bool {
        self.gen_index
    }
    /// Returns the out_dir value of the configuration
    pub fn out_dir(&self) -> String {
        self.out_dir.clone()
    }
    /// Returns the copy_resources value of the configuration
    pub fn copy_resources(&self) -> bool {
        self.copy_resources
    }
    /// Returns the title value of the configuration
    pub fn title(&self) -> String {
        self.title.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Configuration, RawConfiguration};
    #[test]
    fn test_raw_read() {
        let actual = RawConfiguration::from("tests/resources/test_conf.yml").unwrap();
        assert_eq!(actual.stylesheet, Some("test_style.css".to_string()));
        assert_eq!(actual.gen_index, None);
        assert_eq!(actual.out_dir, None);
        assert_eq!(actual.copy_resources, None);
        assert_eq!(actual.title, Some("My Site".to_string()));
    }

    #[test]
    fn test_raw_read_all() {
        let actual = RawConfiguration::from("tests/resources/test_conf_all.yml").unwrap();
        assert_eq!(actual.stylesheet, Some("style.css".to_string()));
        assert_eq!(actual.gen_index, Some(false));
        assert_eq!(actual.out_dir, Some("output".to_string()));
        assert_eq!(actual.copy_resources, Some(true));
        assert_eq!(actual.title, Some("My Site".to_string()));
    }

    #[test]
    fn test_read() {
        let actual = Configuration::from("tests/resources/test_conf.yml").unwrap();
        assert_eq!(actual.stylesheet, "test_style.css".to_string());
        assert_eq!(actual.gen_index, false);
        assert_eq!(actual.out_dir, "out".to_string());
        assert_eq!(actual.copy_resources, true);
        assert_eq!(actual.title, "My Site".to_string());
    }

    #[test]
    fn test_read_all() {
        let actual = Configuration::from("tests/resources/test_conf_all.yml").unwrap();
        assert_eq!(actual.stylesheet, "style.css".to_string());
        assert_eq!(actual.gen_index, false);
        assert_eq!(actual.out_dir, "output".to_string());
        assert_eq!(actual.copy_resources, true);
        assert_eq!(actual.title, "My Site".to_string());
    }
}
