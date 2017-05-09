extern crate made_up;
extern crate tempdir;

mod common;

#[test]
fn test_it() {
    const CONFIG_FILE: &str = "resources/mdup.yml";
    // Need to use temporary directories for this
    // Ammend the configuration
    let tmp_dir = tempdir::TempDir::new("made_up_out").expect("Failed to create temp dir");
    let mut config_content = common::read_from_file(CONFIG_FILE);
    let old_config_content = config_content.clone();
    config_content.push_str(&format!("out_dir: {:?}\n", tmp_dir.path().display()));

    println!("Writing config: {}", config_content);
    common::write_to_file(CONFIG_FILE, config_content);


    // Let's start the testing
    let convertor: made_up::Convertor =
        made_up::Convertor::new("resources").expect("Failed Convertor::new");
    let files = convertor.generate_site().expect("Failed generate_site");
    convertor.write_files(files).expect("Failed write_files");


    println!("Checking that files exist under {}",
             tmp_dir.path().to_string_lossy().to_string());
    assert!(common::check_file_exists(tmp_dir.path().to_string_lossy().to_string() +
                                      "/index.html"));
    assert!(common::check_file_exists(tmp_dir.path().to_string_lossy().to_string() +
                                      "/all_test.html"));
    assert!(common::check_file_exists(tmp_dir.path().to_string_lossy().to_string() +
                                      "/second-page.html"));
    assert!(common::check_file_exists(tmp_dir.path().to_string_lossy().to_string() + "/style.css"));

    println!("Checking all_test content");
    let expected = include_str!("../tests/resources/all_test_good.html");
    let actual = common::read_from_file(tmp_dir.path().to_string_lossy().to_string() +
                                        "/all_test.html");
    common::compare_string_content(expected, &actual.to_string());
    println!("Checking index content");
    let expected = include_str!("../tests/resources/index_good.html");
    let actual = common::read_from_file(tmp_dir.path().to_string_lossy().to_string() +
                                        "/index.html");
    common::compare_string_content(expected, &actual.to_string());
    println!("Checking second-page content");
    let expected = include_str!("../tests/resources/second-page_good.html");
    let actual = common::read_from_file(tmp_dir.path().to_string_lossy().to_string() +
                                        "/second-page.html");
    common::compare_string_content(expected, &actual.to_string());

    // Ensure the images were move across successfully
    assert!(common::check_file_exists(tmp_dir.path().to_string_lossy().to_string() +
                                      "/images/rustacean-orig-noshadow.png"));

    common::write_to_file(CONFIG_FILE, old_config_content);
}
