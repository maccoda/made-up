extern crate made_up;
use std::io::Write;
use std::fs::File;
fn main() {
    made_up::generate_site("./resources").unwrap();
}
