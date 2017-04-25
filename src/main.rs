extern crate made_up;
use std::io::Write;
use std::fs::{self, File};
fn main() {
    let all_files = made_up::find_all_files("./resources");
    // let file_name = fs::canonicalize("./resources/all_test.md").unwrap();
    for file in all_files {

        let result = made_up::create_html(file.get_path()).unwrap();

        let mut file = File::create(format!("{:?}.html",
                                            file.get_path().as_path().file_name().unwrap()))
                .unwrap();
        let content: &[u8] = &(result.into_bytes())[..];
        file.write_all(content).unwrap();
    }

}
