extern crate made_up;

use std::fs::{self, File};
use std::io::{Read};
use std::path::Path;
use std::env;

mod common;

#[test]
fn test_it() {
    const OUT_DIR: &str = "target/made_up_out/";
    made_up::generate_site("resources").unwrap();
    common::check_file_exists(OUT_DIR.to_string() + "index.html");
    common::check_file_exists(OUT_DIR.to_string() + "all_test.html");
    common::check_file_exists(OUT_DIR.to_string() + "second-page.html");
    common::check_file_exists(OUT_DIR.to_string() + "style.css");

    let expected = include_str!("../tests/resources/all_test_good.html");
    let actual = common::read_from_file(OUT_DIR.to_string() + "all_test.html");
    common::compare_string_content(expected, &actual.to_string());
    let expected = include_str!("../tests/resources/index_good.html");
    let actual = common::read_from_file(OUT_DIR.to_string() + "index.html");
    common::compare_string_content(expected, &actual.to_string());
    let expected = include_str!("../tests/resources/second-page_good.html");
    let actual = common::read_from_file(OUT_DIR.to_string() + "second-page.html");
    common::compare_string_content(expected, &actual.to_string());

    // Ensure the images were move across successfully
    assert!(common::check_file_exists(OUT_DIR.to_string() + "images/rustacean-orig-noshadow.png"));

    fs::remove_dir_all(OUT_DIR).unwrap();
}
