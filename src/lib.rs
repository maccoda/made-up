extern crate pulldown_cmark;

use std::fs::File;
use std::io::Read;
use std::path::Path;


mod html;

#[derive(Debug)]
pub enum ConvError {
    Fail,
    IO(std::io::Error),
}

impl From<std::io::Error> for ConvError {
    fn from(error: std::io::Error) -> ConvError {
        ConvError::IO(error)
    }
}

/// Converts the provided Markdown file to it HTML equivalent. This ia a direct
/// mapping it does not add more tags, such as `<body>` or `<html>`.
pub fn create_html<P: AsRef<Path>>(file_name: P) -> Result<String, ConvError> {
    let mut content = String::new();
    File::open(file_name)
        .and_then(|mut x| x.read_to_string(&mut content))?;
    let parser = pulldown_cmark::Parser::new_ext(&content, pulldown_cmark::OPTION_ENABLE_TABLES);

    // The below just simply prints the parsed markdown to the expected html
    // pulldown_cmark::html::push_html(&mut output, parser);
    // for elem in parser {
    //     println!("{:?} . ", elem);
    // }
    Ok(html::consume(parser))
    // println!("{}", output);

}

// Take a HTML string and encapsulate with the correct tags. Will also add the stylesheet.
fn encapsulate_bare_html(content: String) -> Result<String, ConvError> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_raw_html() {
        // Read expected
        let expected = include_str!("../tests/resources/all_test_raw_good.html");
        let actual = super::create_html("resources/all_test.md").unwrap();
        assert_eq!(expected, actual);
    }
}
