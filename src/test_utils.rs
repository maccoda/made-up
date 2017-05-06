/// Strips all whitespace anywhere within the string. Useful for comparing
/// strings when only caring about content.
fn strip_all_whitespace(string: &str) -> String {
    string.chars().filter(|x| !x.is_whitespace()).collect()
}

/// Asserts the two strings provided have the same non-whitespace content.
pub fn compare_string_content(expected: &str, actual: &str) {
    let expected = strip_all_whitespace(&expected);
    let actual = strip_all_whitespace(&actual);

    assert_eq!(expected, actual);
}
