# Changelog

All notable changes to this project will be documented in this file. See [commit-and-tag-version](https://github.com/absolute-version/commit-and-tag-version) for commit guidelines.

### [1.0.2](https://github.com/LynchSKM/dockem-rs/compare/v1.0.1...v1.0.2) (2025-02-22)


### Bug Fixes

* **scripts:** fix incorrect executable name in scripts ([1fdb519](https://github.com/LynchSKM/dockem-rs/commit/1fdb519e7a054048f7926fe9d310a85083446634))

### [1.0.1](https://github.com/LynchSKM/dockem-rs/compare/v1.0.0...v1.0.1) (2025-02-22)


### Bug Fixes

* **scripts:** fix scripts to use correct architecture name based on release files ([9956c69](https://github.com/LynchSKM/dockem-rs/commit/9956c69e6eeaae6ec6cae0093ccd9868a40e2c69))

## 1.0.0 (2025-02-22)


### Features

* **build-image:** This feature adds functionality to build the docker image using all the helper utils required ([#10](https://github.com/LynchSKM/dockem-rs/issues/10)) ([fa17d92](https://github.com/LynchSKM/dockem-rs/commit/fa17d9205da3b21c84c964c8b3f9f6f8cdd1b5e3))
* **build-image:** This feature adds the build image util function ([#7](https://github.com/LynchSKM/dockem-rs/issues/7)) ([14a336d](https://github.com/LynchSKM/dockem-rs/commit/14a336df72fa4da4dbebf97606b36de8fc60803d))
* **cli:** This feature adds a cli interface using clap ([#11](https://github.com/LynchSKM/dockem-rs/issues/11)) ([971ae61](https://github.com/LynchSKM/dockem-rs/commit/971ae61143b61d88f0b3703902172a45c3f0b3b5))
* **general:** update dependencies clap and tempfile ([72087f6](https://github.com/LynchSKM/dockem-rs/commit/72087f6c044162dba2c26a4573d13a6ccba770d0))
* **release:** This feature adds a build workflow to verify that the binaries still compile ([#13](https://github.com/LynchSKM/dockem-rs/issues/13)) ([149b8ca](https://github.com/LynchSKM/dockem-rs/commit/149b8ca8026388b1572ff86d45081db8421fd320))
* **release:** This feature adds functionality to build and release binaries ([#12](https://github.com/LynchSKM/dockem-rs/issues/12)) ([825c3a7](https://github.com/LynchSKM/dockem-rs/commit/825c3a7f04f1915b91243307ab28825483a09b4f))
* **tag-and-push-image:** This feature adds utils function to perform logic to tag and push images to a repository ([#8](https://github.com/LynchSKM/dockem-rs/issues/8)) ([8aae23c](https://github.com/LynchSKM/dockem-rs/commit/8aae23cdb7c7117215e8bd91bafc7a9402c43048))
* **testing:** This adds a test workflow that runs the implemented unit tests ([#14](https://github.com/LynchSKM/dockem-rs/issues/14)) ([e1a5d70](https://github.com/LynchSKM/dockem-rs/commit/e1a5d709aeec2f2c7fe73c85f211ace5fd6325f0))
* **utils:** This feature adds a assert string not empty util function that will be used to validate CLI arg values given ([#3](https://github.com/LynchSKM/dockem-rs/issues/3)) ([c703ddb](https://github.com/LynchSKM/dockem-rs/commit/c703ddbfe8fb2cdc78934e142055923ecdc7ff79))
* **utils:** This feature adds a check manifest head util that can determine if an image tag already exists in a repository ([#6](https://github.com/LynchSKM/dockem-rs/issues/6)) ([c103f78](https://github.com/LynchSKM/dockem-rs/commit/c103f7803f174e13a68e8e27eb4973f13411c5c7))
* **utils:** This feature adds a hash string util function ([#2](https://github.com/LynchSKM/dockem-rs/issues/2)) ([2907cb0](https://github.com/LynchSKM/dockem-rs/commit/2907cb07d783669eeb837b2cc51a56935a632bc4))
* **utils:** This feature adds copy image util function ([#9](https://github.com/LynchSKM/dockem-rs/issues/9)) ([a7d5ec9](https://github.com/LynchSKM/dockem-rs/commit/a7d5ec9527b37f30d81928bb20ae86b35b11d1fd))
* **utils:** This feature adds file operation utils that will be used by the CLI ([#1](https://github.com/LynchSKM/dockem-rs/issues/1)) ([cf6fe00](https://github.com/LynchSKM/dockem-rs/commit/cf6fe00832ea8b2ea94523691666ffd2047deb71))
* **utils:** This feature creates a docker client ([#5](https://github.com/LynchSKM/dockem-rs/issues/5)) ([06b5e07](https://github.com/LynchSKM/dockem-rs/commit/06b5e07a39642ede90bf197d0e593f2bcab5ce3e))
* **utils:** This feature creates the reg client using the oci-client crate ([#4](https://github.com/LynchSKM/dockem-rs/issues/4)) ([8ce5427](https://github.com/LynchSKM/dockem-rs/commit/8ce54279ffdab65278b730cf1f2af6ce78ba202a))


### Bug Fixes

* **build-image:** make struct public ([2711780](https://github.com/LynchSKM/dockem-rs/commit/271178007b512df6f8527167856dce08f203f0f0))
* **build:** add more println messages ([60fabe9](https://github.com/LynchSKM/dockem-rs/commit/60fabe92ca06821ed4649472bd2d3b9a4e327f26))
* **build:** change name to dockem-rs and default registry to Dockerhub registry ([28306ba](https://github.com/LynchSKM/dockem-rs/commit/28306ba326bd188cdac43f0a5fd1e705f8594160))
* **build:** fix tar_build_context not setting correct tar file headers required for Docker build to work ([4dea79e](https://github.com/LynchSKM/dockem-rs/commit/4dea79eafc2d897ddea48e2d08bec7771bf9dc31))
* **build:** This fixes the version file not being passed in to extract_version correctly. ([d85f1e2](https://github.com/LynchSKM/dockem-rs/commit/d85f1e26cc0dd540633b47d12fdfddf49483a03f))
* **documentation:** copy dockem mission and update roadmap to dockem-rs version ([0301db4](https://github.com/LynchSKM/dockem-rs/commit/0301db4614e43d8e5391b10a09231ee4bb50dbd8))
* **documentation:** fix linux types in documentation ([a158601](https://github.com/LynchSKM/dockem-rs/commit/a158601490707d7c7a5b4303f0ce0f64d0caad23))
* **documentation:** fix linux types in documentation ([344ae2b](https://github.com/LynchSKM/dockem-rs/commit/344ae2bf8288776ba90144258fd66187d8ecd768))
* **reg-client:** move default registry into a variable ([1d531e5](https://github.com/LynchSKM/dockem-rs/commit/1d531e54eba617156262fbabbf8c4addef675676))
* **release:** fix the release scripts to go to the correct folder to set the version ([5ad1a71](https://github.com/LynchSKM/dockem-rs/commit/5ad1a71ee0013f59199dbe6797528e8377331d58))
* **utils:** fix check manifest enum type check to just check the error message string instead of using the typeId function ([035f15a](https://github.com/LynchSKM/dockem-rs/commit/035f15a8db222ba08597dd5e887679e44a000de0))
* **utils:** fix create regclient when types were changes to options ([2a447a0](https://github.com/LynchSKM/dockem-rs/commit/2a447a084b3a58d661d20d580cfe9164f3bb0950))
* **utils:** update create_docker_client function to be public ([8b2f238](https://github.com/LynchSKM/dockem-rs/commit/8b2f238790857d95de0c3e1e27c3d0e6ac5716e2))
