# DOCKEM-RS

![Dockem](docs/logo.png)

A Rust CLI that helps optimise the build process of Docker containers, if the image has
been built before, it will just
copy the tag across on the registry which is much faster than pulling and pushing to the
new image tag.

## Development

#### Pre-requisites

* [rustup](https://rustup.rs/#)
* [cross](https://github.com/cross-rs/cross)
* [cargo-set-version](https://crates.io/crates/cargo-set-version)

### Release

To tag a new version run the following on the main branch:

```shell
task release
```

or

```shell
task release-major
```

### Build

Make sure [cross](https://github.com/cross-rs/cross) is installed to help with
cross-platform compilation. When ready, run:

```shell
task build
```

#### Supported Platforms

* Linux AMD64
* Linux ARM64 / Linux AARCH64
* Windows

I wanted to support Darwin (macOS) Targets, but it seemed like too much effort for now.
Maybe in a future release I will work on figuring it out by following
this [article](https://blog.crafteo.io/2024/02/29/my-rust-cross-compilation-journey/).
