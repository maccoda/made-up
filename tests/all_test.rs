extern crate made_up;

use std::fs::{self, File};
use std::io::{Read};
use std::path::Path;
#[test]
fn test_it() {
    made_up::generate_site("resources").unwrap();
    check_file_exists("out/index.html");
    check_file_exists("out/all_test.html");
    check_file_exists("out/second-page.html");
    check_file_exists("out/style.css");

    let expected = include_str!("../tests/resources/all_test_good.html");
    let actual = read_from_file("out/all_test.html");
    compare_string_content(expected.to_string(), actual.to_string());
    let expected = include_str!("../tests/resources/index_good.html");
    let actual = read_from_file("out/index.html");
    compare_string_content(expected.to_string(), actual.to_string());
    let expected = include_str!("../tests/resources/second-page_good.html");
    let actual = read_from_file("out/second-page.html");
    compare_string_content(expected.to_string(), actual.to_string());

    fs::remove_dir_all("out").unwrap();
}

/// Reads and returns the contents of the file at the path provided.
fn read_from_file<P: AsRef<Path>>(file_name: P) -> String {
    let mut content = String::new();
    let mut file = File::open(file_name).unwrap();
    file.read_to_string(&mut content).unwrap();
    content
}

/// Checks if a file exists. Will return false if it exists or it exists but is a directory.
fn check_file_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).map(|x| !x.is_dir()).unwrap_or(false)
}

/// Strips all whitespace anywhere within the string. Useful for comparing
/// strings when only caring about content.
fn strip_all_whitespace(string: &String) -> String {
    string.chars().filter(|x| !x.is_whitespace()).collect()
}

/// Asserts the two strings provided have the same non-whitespace content.
fn compare_string_content(expected: String, actual: String) {
    let expected = strip_all_whitespace(&expected);
    let actual = strip_all_whitespace(&actual);

    assert_eq!(expected, actual);
}
