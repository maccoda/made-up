use std::collections::BTreeMap;
use std::path::Path;

use handlebars::Handlebars;
// NOTE Looks like this needs to be here
use serde_json::Value;


use MarkdownFileList;
use config::Configuration;
use Result;

/// Construct a generated index page for the site from the list of files used.
pub fn generate_index(files: &MarkdownFileList, config: &Configuration) -> Result<String> {
    const TEMPLATE_NAME: &'static str = "index";
    // Build the page from the template just to make it easier for future us
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(TEMPLATE_NAME, include_str!("../templates/index.hbs"))
        .unwrap();

    let mut data: BTreeMap<String, Value> = BTreeMap::new();
    data.insert("stylesheet".to_string(), Value::String(config.stylesheet()));
    data.insert("title".to_string(),
                Value::String(config.title() + " - Home"));
    data.insert("element".to_string(),
                Value::Array(files
                                 .get_files()
                                 .iter()
                                 .map(|x| Value::String(x.get_file_name()))
                                 .collect()));

    let output = handlebars.render(TEMPLATE_NAME, &data)?;
    Ok(output)
}

/// Take a HTML string and encapsulate with the correct tags. Will also add the stylesheet.
pub fn encapsulate_bare_html(content: String, config: &Configuration) -> Result<String> {
    const TEMPLATE_NAME: &'static str = "basic";
    // Build the page from the template just to make it easier for future us
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(TEMPLATE_NAME, include_str!("../templates/basic.hbs"))
        .unwrap();

    let mut data: BTreeMap<String, String> = BTreeMap::new();
    data.insert("stylesheet".to_string(), config.stylesheet());
    data.insert("title".to_string(), config.title());
    data.insert("md_content".to_string(), content);

    let output = handlebars.render(TEMPLATE_NAME, &data)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use test_utils;
    #[test]
    fn test_generate_index() {
        use std::path::Path;
        use walker::MarkdownFile;
        use config;
        let config = config::Configuration::from("resources/mdup.yml").unwrap();
        let expected = include_str!("../tests/resources/index_good.html");
        let actual = super::generate_index(&super::MarkdownFileList::new(
                                        vec![MarkdownFile::from(&Path::new("second-page.md")),
                                             MarkdownFile::from(&Path::new("all_test.md"))],
        ), &config).unwrap();
        test_utils::compare_string_content(expected, &actual);
    }
}
