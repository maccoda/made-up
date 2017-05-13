use std::path::Path;
use serde_yaml;

use super::file_utils;
#[macro_use]
mod config_macro;

// Mirror of `RawConfiguration` but has resolved all `Option`s to their default values.
configuration!{
    stylesheet, String, String::new();
    gen_index, bool, false;
    out_dir, String, "out".to_string();
    copy_resources, bool, true;
    title, String, "Title".to_string()
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
