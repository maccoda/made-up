use std::path::Path;
use serde_yaml;

use super::file_utils;

// TODO Look into what we actully need here
// TODO Determine if can have optional values that are set to default if not specifed
// TODO Look into if we don't want all the fields to be public
#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub stylesheet: String,
}

impl Configuration {
    pub fn from<P: AsRef<Path>>(path: P) -> Configuration {
        // TODO Add error handling
        debug!("Reading configuration file");
        serde_yaml::from_str(&file_utils::read_from_file(path)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Configuration;
    #[test]
    fn test_read() {
        let actual = Configuration::from("tests/resources/test_conf.yml");
        assert_eq!(actual.stylesheet, "test_style.css");
    }
}
