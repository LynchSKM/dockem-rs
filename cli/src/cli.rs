use clap::{Arg, ArgAction, Command};

/// Builds the CLI structure using `clap`.
pub fn build_cli() -> Command {
    Command::new("dockem")
        .about("Build Docker images only when changes are detected")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("build")
                .about("Build the new Docker image")
                .long_about(
                    "Check the files or folders specified and compare the hash to what has already \
                    been built. If it has been built, then skip the build and copy the tag, \
                    otherwise, build the new image and push it to the specified tag(s).",
                )
                .arg(
                    Arg::new("directory")
                        .short('d')
                        .long("directory")
                        .value_name("DIR")
                        .default_value("./")
                        .help("(required) The directory that should be used as the context for the Docker build")
                        .required(true),
                )
                .arg(
                    Arg::new("dockerfile-path")
                        .short('f')
                        .long("dockerfile-path")
                        .value_name("FILE")
                        .default_value("./Dockerfile")
                        .help("(required) The path to the Dockerfile that should be used to build the image")
                        .required(true),
                )
                .arg(
                    Arg::new("image-name")
                        .short('i')
                        .long("image-name")
                        .value_name("NAME")
                        .help("(required) The name of the image you are building")
                        .required(true),
                )
                .arg(
                    Arg::new("version-file")
                        .short('F')
                        .long("version-file")
                        .value_name("FILE")
                        .default_value("./package.json")
                        .help("(required) The name of the JSON file that holds the version to be used in the build. This JSON file must have the 'version' key.")
                        .required(true),
                )
                .arg(
                    Arg::new("registry")
                        .short('r')
                        .long("registry")
                        .value_name("REGISTRY")
                        .help("The registry that should be used when pulling/pushing the image, Dockerhub is used by default"),
                )
                .arg(
                    Arg::new("tag")
                        .short('t')
                        .long("tag")
                        .value_name("TAG")
                        .action(ArgAction::Append)
                        .help("The tag or tags that should be attached to the image"),
                )
                .arg(
                    Arg::new("docker-username")
                        .short('u')
                        .long("docker-username")
                        .value_name("USERNAME")
                        .help("The username that should be used to authenticate the Docker client. Ignore if you have already logged in."),
                )
                .arg(
                    Arg::new("docker-password")
                        .short('p')
                        .long("docker-password")
                        .value_name("PASSWORD")
                        .help("The password that should be used to authenticate the Docker client. Ignore if you have already logged in."),
                )
                .arg(
                    Arg::new("latest")
                        .short('l')
                        .long("latest")
                        .action(ArgAction::SetTrue)
                        .help("Whether to push the latest tag with this image"),
                )
                .arg(
                    Arg::new("main-version")
                        .short('m')
                        .long("main-version")
                        .action(ArgAction::SetTrue)
                        .help("Whether to push this as the main version of the repository. This is done automatically if you do not specify tags or the latest flag."),
                )
                .arg(
                    Arg::new("ignore-build-directory")
                        .short('I')
                        .long("ignore-build-directory")
                        .action(ArgAction::SetTrue)
                        .help("Whether to ignore the build directory in the hashing process, this is useful when you are watching a specific file or directory."),
                )
                .arg(
                    Arg::new("watch-file")
                        .short('w')
                        .long("watch-file")
                        .value_name("FILE")
                        .action(ArgAction::Append)
                        .help("Watch for changes on a specific file or files"),
                )
                .arg(
                    Arg::new("watch-directory")
                        .short('W')
                        .long("watch-directory")
                        .value_name("DIR")
                        .action(ArgAction::Append)
                        .help("Watch for changes in a directory or directories"),
                )
                .after_help(
                    "Examples:\n\
                    $ dockem build --directory=./apps/backend --dockerfile-path=./devops/prod/backend/Dockerfile --image-name=my-repo/backend --tag=stable --main-version\n\
                    $ dockem build --directory=./apps/backend --watch-directory=./libs/shared --dockerfile-path=./apps/backend/Dockerfile --image-name=my-repo/backend --tag=dev --latest\n\
                    $ dockem build --image-name=my-repo/backend --registry=eu.reg.io --docker-username=uname --docker-password=1234 --tag=alpha --tag=test",
                ),
        )
}
