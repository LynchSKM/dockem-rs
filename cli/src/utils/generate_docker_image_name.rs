use std::format;
/// Generates a docker image name with the format `org/imageName:hash` or `imageName:hash`.
///
/// # Arguments
/// * `registry` - The registry where the image belongs to.
/// * `image_name` - The name to give the image.
/// * `tag` - The image tag to check for.
///
/// # Returns
/// * `String` Formatted as `org/imageName:hash` or `imageName:hash`
///
pub fn generate_docker_image_name(registry: &str, image_name: &str, tag: &str) -> String {
    if registry.is_empty() {
        format!("{}:{}", image_name, tag).to_string();
    }
    format!("{}/{}:{}", registry, image_name, tag)
}
