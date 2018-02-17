use std::path::Path;

use handlebars::{to_json, Handlebars};
use serde_json::{Map, Value as Json};

use MarkdownFileList;
use config::Configuration;
use Result;
use file_utils;

/// Construct a generated index page for the site from the list of files used.
pub fn generate_index(files: &MarkdownFileList, config: &Configuration) -> Result<String> {
    let data = populate_index_data(files, config);
    build_template(&data, include_str!("../templates/index.hbs"))
}

/// Render the index page using the provided template path filling the same
/// variables as expected for the generated template
pub fn render_index_with_template<P: AsRef<Path>>(
    template_path: P,
    files: &MarkdownFileList,
    config: &Configuration,
) -> Result<String> {
    let data = populate_index_data(files, config);
    let template_content = &file_utils::read_from_file(template_path)?;
    build_template(&data, template_content)
}

/// Element provided to the Handlebars template for creating the index page.
/// Each element represents a single Markdown document on the input.
#[derive(Serialize)]
pub struct Element {
    header: String,
    file_path: String,
}

/// Populate the data map used to populate the index page template
fn populate_index_data(files: &MarkdownFileList, config: &Configuration) -> Map<String, Json> {
    let mut data = Map::new();
    data.insert(
        "stylesheet".to_string(),
        Json::Array(
            config
                .stylesheet()
                .iter()
                .map(|x| Json::String(x.to_owned()))
                .collect(),
        ),
    );
    data.insert(
        "title".to_string(),
        Json::String(config.title() + " - Home"),
    );
    let elements: Vec<Element> = files
        .get_files()
        .iter()
        .map(|x| Element {
            header: x.get_heading().to_owned(),
            file_path: x.get_file_name(),
        })
        .collect();
    data.insert("element".to_string(), to_json(&elements));

    data
}

/// Take a HTML string and encapsulate with the correct tags. Will also add the stylesheet.
pub fn encapsulate_bare_html(
    content: String,
    config: &Configuration,
    title: String,
) -> Result<String> {
    let mut data = Map::new();
    data.insert(
        "stylesheet".to_string(),
        Json::Array(
            config
                .stylesheet()
                .iter()
                .map(|x| Json::String(x.to_owned()))
                .collect(),
        ),
    );
    data.insert(
        "title".to_string(),
        Json::String(config.title() + " - " + &title),
    );
    data.insert("md_content".to_string(), Json::String(content));

    build_template(&data, include_str!("../templates/basic.hbs"))
}

/// Constructs Handlebars template from the provided variable data. Uses partial templates
/// to produce consistent container. Returns error if the template failed to compile.
fn build_template(data: &Map<String, Json>, template_content: &str) -> Result<String> {
    let mut handlebars = Handlebars::new();
    // Render the partials
    handlebars.register_template_string("container", include_str!("../templates/container.hbs"))?;
    handlebars.register_partial("content", template_content)?;

    // That's all we need to build this thing
    let rendered = handlebars.render("container", &data)?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use test_utils;
    use std::path::Path;
    use walker::MarkdownFile;
    use config;
    #[test]
    fn test_generate_index() {
        let config = config::Configuration::from("tests/resources/input/site/mdup.yml").unwrap();
        let expected = include_str!("../tests/resources/output/index_good.html");
        let actual = super::generate_index(
            &super::MarkdownFileList::new(vec![
                MarkdownFile::from(&Path::new("tests/resources/input/site/second-page.md")),
                MarkdownFile::from(&Path::new("tests/resources/input/site/all_test.md")),
            ]),
            &config,
        ).unwrap();
        test_utils::compare_string_content(expected, &actual);
    }

    #[test]
    fn test_index_from_template() {
        let config = config::Configuration::from(
            "tests/resources/input/test_conf_user_template.yml",
        ).unwrap();
        let expected = include_str!("../tests/resources/output/user_index_good.html");
        let actual = super::render_index_with_template(
            "tests/resources/input/index_test.hbs",
            &super::MarkdownFileList::new(vec![
                MarkdownFile::from(&Path::new("tests/resources/input/site/second-page.md")),
                MarkdownFile::from(&Path::new("tests/resources/input/site/all_test.md")),
            ]),
            &config,
        ).unwrap();
        test_utils::compare_string_content(expected, &actual);
    }
}
