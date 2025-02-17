use crate::utils::{
    generate_docker_image_name, tag_and_push_image, BuildDockerImageParams, BuildLog,
};
use bollard::auth::DockerCredentials;
use bollard::errors::Error as BollardError;
use bollard::Docker;

/// Tags and pushes a Docker image to multiple tags based on the provided parameters.
///
/// # Arguments
/// * `docker` - A connected Docker client.
/// * `params` - Parameters for building and tagging the Docker image.
/// * `version` - The version of the image to tag and push.
/// * `local_tag` - The local tag of the image to push.
/// * `credentials` - Docker credentials for authentication.
/// * `build_log` - A shared, mutable reference to the build log.
///
/// # Returns
/// A `Result` indicating success or failure of the tag and push operations.
pub async fn tag_and_push_new_images(
    docker: &Docker,
    params: &BuildDockerImageParams,
    version: &str,
    local_tag: &str,
    credentials: DockerCredentials,
    build_log: &mut BuildLog,
) -> Result<(), BollardError> {
    // Push to versioned tags (e.g., `tag-version`)
    for tag in &params.tag {
        let version_tag = format!("{}-{}", tag, version);
        let target_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, &version_tag);
        println!("Pushing the image to the new tag: {}", target_image_name);
        tag_and_push_image(docker, local_tag, &target_image_name, credentials.clone()).await?;
        build_log.output_tags.push(target_image_name);
    }

    // If no tags are specified and neither `latest` nor `main_version` is set,
    // push to the main version (e.g., `image-name:version`)
    if params.tag.is_empty() && !params.latest && !params.main_version {
        let main_version_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, version);
        println!(
            "WARN: No tags were specified and you have not selected the --latest flag, \
            so the image will be deployed to the main version: {}",
            main_version_image_name
        );
        tag_and_push_image(
            docker,
            local_tag,
            &main_version_image_name,
            credentials.clone(),
        )
        .await?;
        build_log.output_tags.push(main_version_image_name);
    }

    // Push to the `latest` tag if specified
    if params.latest {
        let latest_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, "latest");
        println!(
            "You have selected the --latest flag, so the image will be deployed to the latest tag: {}",
            latest_image_name
        );
        tag_and_push_image(docker, local_tag, &latest_image_name, credentials.clone()).await?;
        build_log.output_tags.push(latest_image_name);
    }

    // Push to the main version tag if specified
    if params.main_version {
        let main_version_image_name =
            generate_docker_image_name(&params.registry, &params.image_name, version);
        println!(
            "You have selected the --main-version flag, so the image will be deployed to the main version: {}",
            main_version_image_name
        );
        tag_and_push_image(
            docker,
            local_tag,
            &main_version_image_name,
            credentials.clone(),
        )
        .await?;
        build_log.output_tags.push(main_version_image_name);
    }

    Ok(())
}
