/// Generates a docker image name with the format `org/image-name:hash`.
///
/// # Arguments
/// `registry` - The registry where the image belongs to.
/// `image_name` - The name to give the image.
/// * `tag` - The image tag to check for.
///
/// # Returns
/// * `String` Formatted as `org/image-name:hash`.
///
pub fn generate_docker_image_name(registry: &str, image_name: &str, tag: &str) -> String {
    if registry.is_empty() {
        return format!("{}:{}", image_name, tag).to_string();
    }
}
