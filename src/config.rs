use std::path::Path;
use serde_yaml;

use super::file_utils;

// TODO Look into what we actully need here
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    stylesheet: Option<String>,
    index: Option<bool>,
    out_dir: Option<String>,
}

impl Configuration {
    pub fn from<P: AsRef<Path>>(path: P) -> Configuration {
        // TODO Add error handling
        debug!("Reading configuration file");
        let config = serde_yaml::from_str(&file_utils::read_from_file(path)).unwrap();
        info!("{:?}", config);
        config
    }

    pub fn stylesheet(&self) -> Option<String> {
        self.stylesheet.clone()
    }
    pub fn index(&self) -> Option<bool> {
        self.index
    }
    pub fn out_dir(&self) -> Option<String> {
        self.out_dir.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Configuration;
    #[test]
    fn test_read() {
        let actual = Configuration::from("tests/resources/test_conf.yml");
        assert_eq!(actual.stylesheet, Some("test_style.css".to_string()));
        assert_eq!(actual.index, None);
    }

    #[test]
    fn test_read_all() {
        let actual = Configuration::from("tests/resources/test_conf_all.yml");
        assert_eq!(actual.stylesheet, Some("style.css".to_string()));
        assert_eq!(actual.index, Some(false));
        assert_eq!(actual.out_dir, Some("output".to_string()));
    }
}
