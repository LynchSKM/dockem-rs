/// This struct is used to save the process of the build and any variables as well.
/// It is used in testing to ensure that the expected outcomes are met.
#[derive(Debug, Default)]
pub struct BuildLog {
    pub custom_dockerfile: bool,
    pub custom_host: bool,
    pub docker_password: Option<String>,
    pub docker_registry: Option<String>,
    pub docker_username: Option<String>,
    pub hash_exists: bool,
    pub hashed_image_name: String,
    pub image_hash: String,
    pub local_tag: String,
    pub output_tags: Vec<String>,
    pub version: String,
}
