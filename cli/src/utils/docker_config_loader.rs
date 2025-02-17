use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug, Default)]
struct DockerConfig {
    auths: Option<HashMap<String, AuthConfig>>,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
struct AuthConfig {
    username: Option<String>,
    password: Option<String>,
    serveraddress: Option<String>,
    identitytoken: Option<String>,
}

impl DockerConfig {
    /// Loads the Docker configuration file (`config.json`) from the `.docker` directory.
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the Docker configuration file, usually `~/.docker/config.json`.
    ///
    /// # Returns
    ///
    /// Returns a result containing the loaded `DockerConfig`, or an error if the operation fails.
    pub fn load(config_path: Option<String>) -> Result<DockerConfig, Box<dyn Error>> {
        let config_file_path = match config_path {
            Some(path) => Path::new(&path).to_path_buf(),
            None => {
                // Check if a DOCKER_CONFIG env is set and use that first
                let env_config_path_str = env::var("DOCKER_CONFIG");
                match env_config_path_str {
                    Ok(path) => Path::new(&path).to_path_buf(),
                    Err(_) => {
                        println!("DOCKER_CONFIG is not set, trying default path");
                        // Default to ~/.docker/config.json if no path is provided
                        let home_dir = env::var("HOME")?;
                        Path::new(&home_dir).join(".docker/config.json")
                    }
                }
            }
        };

        if !config_file_path.exists() {
            return Err("Docker config file does not exist.".into());
        }

        let config_data = fs::read_to_string(config_file_path)?;
        let docker_config: DockerConfig = serde_json::from_str(&config_data)?;

        Ok(docker_config)
    }

    /// Retrieves the authentication configuration for a specific registry.
    ///
    /// # Arguments
    ///
    /// * `registry_name` - The name of the registry (e.g., "docker.io").
    ///
    /// # Returns
    ///
    /// Returns an option containing the `AuthConfig` if found, or `None` if not.
    pub fn get_auth_config_for_registry(&self, registry_name: &str) -> Option<AuthConfig> {
        self.auths
            .as_ref()
            .and_then(|auths| auths.get(registry_name).cloned())
    }
}
