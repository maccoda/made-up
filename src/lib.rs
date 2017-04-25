extern crate pulldown_cmark;
extern crate handlebars;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use handlebars::Handlebars;


mod html;

/// Error type for the conversion of the markdown files to the static site.
#[derive(Debug)]
pub enum ConvError {
    Fail,
    IO(std::io::Error),
    Template(handlebars::RenderError),
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

/// Converts the provided Markdown file to it HTML equivalent. This ia a direct
/// mapping it does not add more tags, such as `<body>` or `<html>`.
pub fn create_html<P: AsRef<Path>>(file_name: P) -> Result<String, ConvError> {
    let mut content = String::new();
    File::open(file_name)
        .and_then(|mut x| x.read_to_string(&mut content))?;
    let parser = pulldown_cmark::Parser::new_ext(&content, pulldown_cmark::OPTION_ENABLE_TABLES);

    encapsulate_bare_html(html::consume(parser))
}

// Take a HTML string and encapsulate with the correct tags. Will also add the stylesheet.
fn encapsulate_bare_html(content: String) -> Result<String, ConvError> {
    // Build the page from the template just to make it easier for future us
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("basic", &Path::new("templates/basic.hbs"))
        .unwrap();

    let mut data: BTreeMap<String, String> = BTreeMap::new();
    data.insert("stylesheet".to_string(), "".to_string());
    // TODO Get the title from the first heading
    data.insert("title".to_string(), "Generated Title".to_string());
    data.insert("md_content".to_string(), content);

    let output = handlebars.render("basic", &data)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_html() {
        // Read expected
        let expected = include_str!("../tests/resources/all_test_good.html");
        let actual = super::create_html("resources/all_test.md").unwrap();
        assert_eq!(expected, actual);
    }
}
