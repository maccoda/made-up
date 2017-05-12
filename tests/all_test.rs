extern crate made_up;
extern crate log;
use std::fs;
use std::env;

mod common;
use common::SimpleLogger;

#[test]
fn test_it() {
    log::set_logger(|max_log_level| {
                        max_log_level.set(::log::LogLevelFilter::Debug);
                        Box::new(SimpleLogger)
                    })
            .unwrap();
    const CONFIG_FILE: &str = "resources/mdup.yml";
    // Need to use temporary directories for this
    // Ammend the configuration
    let tmp_dir = env::temp_dir();
    let tmp_dir = tmp_dir.join("made_up_out");
    println!("Temp dir: {:?}", tmp_dir.to_string_lossy());
    let mut config_content = common::read_from_file(CONFIG_FILE);
    let old_config_content = config_content.clone();
    config_content.push_str(&format!("out_dir: {:?}\n", tmp_dir.to_string_lossy()));

    println!("Writing config: {}", config_content);
    common::write_to_file(CONFIG_FILE, config_content);


    // Let's start the testing
    let convertor: made_up::Convertor =
        made_up::Convertor::new("resources").expect("Failed Convertor::new");
    let files = convertor.generate_site().expect("Failed generate_site");
    convertor.write_files(files).expect("Failed write_files");


    println!("Checking that files exist under {}",
             tmp_dir.to_string_lossy().to_string());
    assert!(common::check_file_exists(tmp_dir.to_string_lossy().to_string() + "/index.html"));
    assert!(common::check_file_exists(tmp_dir.to_string_lossy().to_string() + "/all_test.html"));
    assert!(common::check_file_exists(tmp_dir.to_string_lossy().to_string() + "/second-page.html"));
    assert!(common::check_file_exists(tmp_dir.to_string_lossy().to_string() + "/style.css"));

    println!("Checking all_test content");
    let expected = include_str!("../tests/resources/all_test_good.html");
    let actual = common::read_from_file(tmp_dir.to_string_lossy().to_string() + "/all_test.html");
    common::compare_string_content(expected, &actual.to_string());
    println!("Checking index content");
    let expected = include_str!("../tests/resources/index_good.html");
    let actual = common::read_from_file(tmp_dir.to_string_lossy().to_string() + "/index.html");
    common::compare_string_content(expected, &actual.to_string());
    println!("Checking second-page content");
    let expected = include_str!("../tests/resources/second-page_good.html");
    let actual = common::read_from_file(tmp_dir.to_string_lossy().to_string() +
                                        "/second-page.html");
    common::compare_string_content(expected, &actual.to_string());

    // Ensure the images were move across successfully
    assert!(common::check_file_exists(tmp_dir.to_string_lossy().to_string() +
                                      "/images/rustacean-orig-noshadow.png"));
    fs::remove_dir_all(tmp_dir).expect("Unable to delete tmp dir");
    common::write_to_file(CONFIG_FILE, old_config_content);
}
