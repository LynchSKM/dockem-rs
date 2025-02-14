/// Custom macro to conditionally exit or return an error based on environment.
#[macro_export]
macro_rules! assert_or_exit {
    ($condition:expr, $err_msg:expr) => {{
        if !$condition {
            if cfg!(test) {
                // In test mode, return an error
                return Err($err_msg.to_string());
            } else {
                // In application mode, exit with a status code of 1
                eprintln!("{}", $err_msg);
                std::process::exit(1);
            }
        }
    }};
}
