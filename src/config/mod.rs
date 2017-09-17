use std::path::Path;
use serde_yaml;

use super::file_utils;
#[macro_use]
mod config_macro;

// Mirror of `RawConfiguration` but has resolved all `Option`s to their default
// values.
configuration!{
    stylesheet, Vec<String>, vec![];
    index_template, Option<String>, None;
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
        assert_eq!(actual.stylesheet, Some(vec!["test_style.css".to_string()]));
        assert_eq!(actual.index_template, None);
        assert_eq!(actual.out_dir, None);
        assert_eq!(actual.copy_resources, None);
        assert_eq!(actual.title, Some("My Site".to_string()));
    }

    #[test]
    fn test_raw_read_all() {
        let actual = RawConfiguration::from("tests/resources/test_conf_all.yml").unwrap();
        assert_eq!(
            actual.stylesheet,
            Some(vec![
                "style.css".to_string(),
                "another.css".to_string(),
                "and_another.css".to_string(),
            ])
        );
        assert_eq!(
            actual.index_template,
            Some(Some("index_test.hbs".to_string()))
        );
        assert_eq!(actual.out_dir, Some("output".to_string()));
        assert_eq!(actual.copy_resources, Some(true));
        assert_eq!(actual.title, Some("My Site".to_string()));
    }

    #[test]
    fn test_read() {
        let actual = Configuration::from("tests/resources/test_conf.yml").unwrap();
        assert_eq!(actual.stylesheet, vec!["test_style.css".to_string()]);
        assert_eq!(actual.index_template, None);
        assert_eq!(actual.out_dir, "out".to_string());
        assert_eq!(actual.copy_resources, true);
        assert_eq!(actual.title, "My Site".to_string());
    }

    #[test]
    fn test_read_all() {
        let actual = Configuration::from("tests/resources/test_conf_all.yml").unwrap();
        assert_eq!(
            actual.stylesheet,
            vec![
                "style.css".to_string(),
                "another.css".to_string(),
                "and_another.css".to_string(),
            ]
        );
        assert_eq!(actual.index_template, Some("index_test.hbs".to_string()));
        assert_eq!(actual.out_dir, "output".to_string());
        assert_eq!(actual.copy_resources, true);
        assert_eq!(actual.title, "My Site".to_string());
    }
}
