# dockem-rust
A Rust CLI that helps optimise the build process of Docker containers, if the image has been built before, it will just copy the tag across on the registry which is much faster than pulling and pushing to the new image tag.
