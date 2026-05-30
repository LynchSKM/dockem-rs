use crate::utils::{tar_build_context, BuildDockerImageParams, BuildLog};
use anyhow::{anyhow, Result};
use bollard::auth::DockerCredentials;
use bollard::image::{BuildImageOptions, BuilderVersion};
use bollard::Docker;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn build_image(
    docker: &Docker,
    params: &BuildDockerImageParams,
    image_hash: &str,
    build_log: Arc<Mutex<BuildLog>>,
    credentials: &DockerCredentials,
) -> Result<String> {
    let build_context = {
        let build_log = Arc::clone(&build_log);
        let params_clone = Arc::from(params.clone());
        tokio::task::spawn_blocking(move || {
            let mut build_log = build_log.lock().unwrap();
            tar_build_context(&params_clone, &mut build_log)
        })
        .await??
    };

    let local_tag = format!("local:{}", image_hash);
    let session_id = generate_session_id();
    let build_options = BuildImageOptions {
        dockerfile: build_context
            .dockerfile_path
            .as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("Dockerfile"),
        t: &local_tag,
        rm: true,
        version: BuilderVersion::BuilderBuildKit,
        session: Some(session_id),
        ..Default::default()
    };

    let mut creds_map = HashMap::new();
    creds_map.insert(
        credentials
            .serveraddress
            .clone()
            .unwrap_or_else(|| "docker.io".to_string()),
        credentials.clone(),
    );

    println!("Building image: {}", local_tag);
    let mut build_stream = docker.build_image(
        build_options,
        Some(creds_map),
        Some(build_context.tarball.into()),
    );

    while let Some(output) = build_stream.next().await {
        match output {
            Ok(output) => {
                if let Some(message) = output.stream {
                    print!("{}", message);
                }
            }
            Err(e) => return Err(anyhow!("Build failed: {}", e)),
        }
    }

    Ok(local_tag)
}

fn generate_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.gen();
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_session_id_format() {
        let id = generate_session_id();
        assert_eq!(id.len(), 32);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_generate_session_id_uniqueness() {
        let id1 = generate_session_id();
        let id2 = generate_session_id();
        assert_ne!(id1, id2);
    }
}
