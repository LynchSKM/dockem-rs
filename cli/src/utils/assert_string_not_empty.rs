use crate::assert_or_exit;

/// Asserts that a string is not empty. If it is empty, either exits or returns an error message.
///
/// # Arguments
/// * `string` - The string to check the length of
/// * `flag` - The flag the string value belongs to
/// * `error_message` - The error message template. It can contain a `%s` for the flag.
///
/// # Returns
/// * `Ok(())` if the string is not empty.
/// * `Err(String)` containing the error message if the string is empty (only in test mode).
pub fn assert_string_not_empty(
    string: &str,
    flag: &str,
    error_message: Option<&str>,
) -> Result<(), String> {
    if !string.trim().is_empty() {
        return Ok(());
    }
    let default_message = "ERROR: The string for flag '%s' does not exist.";
    let error_message = error_message.unwrap_or(default_message);

    let output_message = error_message.replace("%s", &flag);

    assert_or_exit!(!string.trim().is_empty(), output_message);
}

#[cfg(test)]
mod tests {
    use crate::utils::assert_string_not_empty;

    #[test]
    fn test_assert_string_not_empty_passes() {
        let result = assert_string_not_empty("some value", "--test-flag", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_string_not_empty_fails_default_message() {
        let result = assert_string_not_empty("", "--test-flag", None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "ERROR: The string for flag '--test-flag' does not exist."
        );
    }

    #[test]
    fn test_assert_string_not_empty_fails_custom_message() {
        let result = assert_string_not_empty("", "--test-flag", Some("Custom error for '%s'"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Custom error for '--test-flag'");
    }

    #[test]
    fn test_assert_string_not_empty_passes_with_whitespace() {
        let result = assert_string_not_empty("   some value   ", "--test-flag", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assert_string_not_empty_fails_with_whitespace_only() {
        let result = assert_string_not_empty("     ", "--test-flag", None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "ERROR: The string for flag '--test-flag' does not exist."
        );
    }
}
