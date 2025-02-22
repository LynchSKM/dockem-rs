/// Custom macro to conditionally return early or exit based on environment.
///
/// - In tests, it returns an `Ok(())` if the condition is met, or an `Err(String)` otherwise.
/// - In application mode, it prints the error message and exits with a status code of 1 if the condition is not met.
///
/// # Arguments
/// * `$condition` - The condition to check.
/// * `$err_msg` - The error message to display or return.
///
/// # Example
/// ```rust
/// assert_or_exit!(file_exists, "ERROR: File does not exist.");
/// ```
#[macro_export]
macro_rules! assert_or_exit {
    ($condition:expr, $err_msg:expr) => {{
        if $condition {
            return Ok(());
        }

        if cfg!(test) {
            // In test mode, return an error
            return Err($err_msg.to_string());
        } else {
            // In application mode, exit with a status code of 1
            eprintln!("{}", $err_msg);
            std::process::exit(1);
        }
    }};
}
