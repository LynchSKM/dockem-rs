/// Removes empty strings from a vector of strings in place.
///
/// # Arguments
/// * `strings` - A mutable vector of strings to filter.
pub fn remove_empty_strings(strings: &mut Vec<String>) {
    strings.retain(|s| !s.is_empty()); // Keep only non-empty strings
}
