/// This struct is used to save CLI argument values passed into the program.
#[derive(Debug, Clone)]
pub struct BuildDockerImageParams {
    pub directory: String,
    pub docker_password: Option<String>,
    pub docker_username: Option<String>,
    pub dockerfile_path: String,
    pub ignore_build_directory: bool,
    pub image_name: String,
    pub latest: bool,
    pub main_version: bool,
    pub registry: String,
    pub tag: Vec<String>,
    pub version_file: String,
    pub watch_directory: Option<Vec<String>>,
    pub watch_file: Option<Vec<String>>,
}
