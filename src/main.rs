extern crate made_up;
use std::io::Write;
use std::fs::{self, File};
fn main() {
    let file_name = fs::canonicalize("./resources/all_test.md").unwrap();
    let result = made_up::create_html(file_name).unwrap();

    let mut file = File::create("output.html").unwrap();
    let content: &[u8] = &(result.into_bytes())[..];
    file.write_all(content).unwrap();
}
