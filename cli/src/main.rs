mod cli;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "dockem")]
#[command(about = "Build Docker images only when changes are detected", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(BuildArgs),
}

#[derive(Parser)]
struct BuildArgs {
    #[arg(short, long, default_value = "./")]
    directory: String,

    #[arg(short, long, default_value = "./Dockerfile")]
    dockerfile_path: String,

    #[arg(short, long)]
    image_name: String,

    #[arg(short, long, default_value = "./package.json")]
    version_file: String,

    #[arg(short, long)]
    registry: Option<String>,

    #[arg(short, long)]
    tag: Vec<String>,

    #[arg(short, long)]
    docker_username: Option<String>,

    #[arg(short, long)]
    docker_password: Option<String>,

    #[arg(short, long)]
    latest: bool,

    #[arg(short, long)]
    main_version: bool,

    #[arg(short = 'I', long)]
    ignore_build_directory: bool,

    #[arg(short, long)]
    watch_file: Vec<String>,

    #[arg(short = 'W', long)]
    watch_directory: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(args) => {
            // Validate required paths
            utils::assert_directory_exists(&args.directory, Some("ERROR: The directory '%s' does not exist. Please specify the path to the directory you would like to build.")).expect("");
            utils::assert_file_exists(&args.dockerfile_path, Some("ERROR: The file '%s' does not exist. Please specify the path to the Dockerfile you would like to use to build the image.")).expect("");
            utils::assert_file_exists(&args.version_file, Some("ERROR: The image-name flag is required. Please specify the name of the image you would like to build, this usually includes the organisation or group as well eg. your-org/image-name.")).expect("");

            // Build the Docker image
            let build_params = utils::BuildDockerImageParams {
                directory: args.directory,
                dockerfile_path: args.dockerfile_path,
                image_name: args.image_name,
                version_file: args.version_file,
                registry: args.registry.unwrap_or("".parse()?),
                tag: args.tag,
                docker_username: args.docker_username,
                docker_password: args.docker_password,
                latest: args.latest,
                main_version: args.main_version,
                ignore_build_directory: args.ignore_build_directory,
                watch_file: Some(args.watch_file),
                watch_directory: Some(args.watch_directory),
            };

            utils::build_docker_image(Arc::from(build_params)).await?;
        }
    }

    Ok(())
}
