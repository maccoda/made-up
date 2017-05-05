use std::path::Path;
use serde_yaml;

use super::file_utils;

// TODO Look into what we actully need here
#[derive(Serialize, Deserialize, Debug)]
struct RawConfiguration {
    stylesheet: Option<String>,
    gen_index: Option<bool>,
    out_dir: Option<String>,
}

impl RawConfiguration {
    fn from<P: AsRef<Path>>(config_path: P) -> Result<RawConfiguration, serde_yaml::Error> {
        serde_yaml::from_str(&file_utils::read_from_file(config_path))
    }
}

// Mirror of `RawConfiguration` but has resolved all `Option`s.
#[derive(Debug)]
pub struct Configuration {
    stylesheet: String,
    gen_index: bool,
    out_dir: String,
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            stylesheet: String::new(),
            gen_index: false,
            out_dir: "out".to_string(),
        }
    }
}

impl Configuration {
    pub fn from<P: AsRef<Path>>(path: P) -> Configuration {
        // TODO Add error handling
        debug!("Reading configuration file");
        let raw_config = RawConfiguration::from(path).unwrap();
        let def_config = Configuration::default();
        let stylesheet = raw_config.stylesheet.unwrap_or(def_config.stylesheet);
        let gen_index = raw_config.gen_index.unwrap_or(def_config.gen_index);
        let out_dir = raw_config.out_dir.unwrap_or(def_config.out_dir);
        let config = Configuration {stylesheet, gen_index, out_dir};
        info!("{:?}", config);
        config
    }

    pub fn stylesheet(&self) -> String {
        self.stylesheet.clone()
    }
    pub fn gen_index(&self) -> bool {
        self.gen_index
    }
    pub fn out_dir(&self) -> String {
        self.out_dir.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Configuration,RawConfiguration};
    #[test]
    fn test_raw_read() {
        let actual = RawConfiguration::from("tests/resources/test_conf.yml").unwrap();
        assert_eq!(actual.stylesheet, Some("test_style.css".to_string()));
        assert_eq!(actual.gen_index, None);
        assert_eq!(actual.out_dir, None);
    }

    #[test]
    fn test_raw_read_all() {
        let actual = RawConfiguration::from("tests/resources/test_conf_all.yml").unwrap();
        assert_eq!(actual.stylesheet, Some("style.css".to_string()));
        assert_eq!(actual.gen_index, Some(false));
        assert_eq!(actual.out_dir, Some("output".to_string()));
    }

    #[test]
    fn test_read() {
        let actual = Configuration::from("tests/resources/test_conf.yml");
        assert_eq!(actual.stylesheet, "test_style.css".to_string());
        assert_eq!(actual.gen_index, false);
        assert_eq!(actual.out_dir, "out".to_string());
    }

    #[test]
    fn test_read_all() {
        let actual = Configuration::from("tests/resources/test_conf_all.yml");
        assert_eq!(actual.stylesheet, "style.css".to_string());
        assert_eq!(actual.gen_index, false);
        assert_eq!(actual.out_dir, "output".to_string());
    }
}
