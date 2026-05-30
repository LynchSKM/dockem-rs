# Changelog

All notable changes to this project will be documented in this file. See [commit-and-tag-version](https://github.com/absolute-version/commit-and-tag-version) for commit guidelines.

## [1.2.0](https://github.com/LynchSKM/dockem-rs/compare/v1.1.1...v1.2.0) (2026-05-30)


### Features

* **build:** enable BuildKit for Docker image builds ([#18](https://github.com/LynchSKM/dockem-rs/issues/18)) ([cd7bcf3](https://github.com/LynchSKM/dockem-rs/commit/cd7bcf36c9d637f8d5d808b30cad0f22e15e7460))
* **build:** enable BuildKit for Docker image builds and add config.json tests ([e82ac55](https://github.com/LynchSKM/dockem-rs/commit/e82ac55eff6f7ff150501b42481f1e11ace02b2c))
* **ci:** add ci:release tasks with --no-sign for unsigned CI releases ([2088141](https://github.com/LynchSKM/dockem-rs/commit/2088141146ef99e6013f37fcb509792bc3af8a32))
* **ci:** add ci:release tasks with --no-sign for unsigned CI releases ([#23](https://github.com/LynchSKM/dockem-rs/issues/23)) ([0f6b6a4](https://github.com/LynchSKM/dockem-rs/commit/0f6b6a4c581ad8c366ed2a3d4cf799cbcb41d5ea))
* **devops:** Refactor CI workflows and improve build tooling ([#16](https://github.com/LynchSKM/dockem-rs/issues/16)) ([a7f3a1f](https://github.com/LynchSKM/dockem-rs/commit/a7f3a1f5e1405690140d3bb283a3b67eba63aac9))
* **setup:** Add Claude session initialization hooks and build setup ([#15](https://github.com/LynchSKM/dockem-rs/issues/15)) ([b1342dd](https://github.com/LynchSKM/dockem-rs/commit/b1342ddd81774b7f820c52bc5715fb01c4ec6c0a))


### Bug Fixes

* **auth:** decode config.json base64 auth into username/password for push ([269aa3e](https://github.com/LynchSKM/dockem-rs/commit/269aa3efb4c9d164ca4e81049a70cc239bac0f9e))
* **auth:** fall through to config.json when credentials are empty ([902a8c9](https://github.com/LynchSKM/dockem-rs/commit/902a8c9782c7aff1b58446b6b848704edea6b857))
* **ci:** create local main branch without fetching into checked-out develop ([a1b9fbb](https://github.com/LynchSKM/dockem-rs/commit/a1b9fbbf91812b7a3a33578be16d3c67e2b1e5f6))
* **ci:** disable GPG signing in release workflow ([20fd0a4](https://github.com/LynchSKM/dockem-rs/commit/20fd0a4aa8e0c51d332d7bb93f09791ac96d92ea))
* **ci:** disable GPG signing in release workflow ([#22](https://github.com/LynchSKM/dockem-rs/issues/22)) ([64c2980](https://github.com/LynchSKM/dockem-rs/commit/64c29804aa69a9dec1b774569918bc3bd8ecfb07))
* **ci:** fix build workflow and add artifact upload ([13282f8](https://github.com/LynchSKM/dockem-rs/commit/13282f89fbcd9ef3356d137aa8add888698d6f55))
* **ci:** fix release workflow branch setup ([#21](https://github.com/LynchSKM/dockem-rs/issues/21)) ([e63c1fb](https://github.com/LynchSKM/dockem-rs/commit/e63c1fb7085ebbe99cc4eaa1471ba7584f1a07f4))
* **cli:** resolve short option conflicts and improve CI config test ([219d4e1](https://github.com/LynchSKM/dockem-rs/commit/219d4e1fb2539b186ce927c6023048d4f6f1c647))
* **config:** treat DOCKER_CONFIG env var as directory path per Docker spec ([4282404](https://github.com/LynchSKM/dockem-rs/commit/4282404d80e5710cf1768f39a81b664daafcb5fd))
* **devops-build:** add Zig setup step and cancel-in-progress for workflows ([b5ba635](https://github.com/LynchSKM/dockem-rs/commit/b5ba6351633db8324e1d9df237c0804c7d15a403))
* **devops-build:** build Windows target on native runner instead of cross ([015aefa](https://github.com/LynchSKM/dockem-rs/commit/015aefaea57525dfd2da27708d6fa1fc4dca453c))
* **devops-build:** fix CI build workflow and add artifact uploads ([#17](https://github.com/LynchSKM/dockem-rs/issues/17)) ([4a07830](https://github.com/LynchSKM/dockem-rs/commit/4a0783050652caa62d0c44b6b951181f05e6bb16))
* **devops-build:** fix cross-compilation for Windows target ([7d0ddaa](https://github.com/LynchSKM/dockem-rs/commit/7d0ddaaaf9aab6fe47ff90d492f49c98e0508703))
* **devops-build:** revert to cross for Linux and revert macOS roadmap/docs ([cf53078](https://github.com/LynchSKM/dockem-rs/commit/cf5307837c9b30ea6cb284724c6343970655f86a))
* **devops-build:** use cargo-zigbuild for Linux/macOS and native runner for Windows ([a635415](https://github.com/LynchSKM/dockem-rs/commit/a635415aaad9e80e80d6daedb062828cfb8e439b))
* **devops-build:** use matrix strategy for cross-compilation targets ([a314569](https://github.com/LynchSKM/dockem-rs/commit/a314569e29211f0bcb3044988a2fd43e6d519797))
* **error-handling:** updated certain functions to use anyhow for errors that were Box<dyn Error> ([412fa3a](https://github.com/LynchSKM/dockem-rs/commit/412fa3acf38f4580843aa4417a377301e8c77e51))

### [1.1.1](https://github.com/LynchSKM/dockem-rs/compare/v1.1.0...v1.1.1) (2025-02-23)

## [1.1.0](https://github.com/LynchSKM/dockem-rs/compare/v1.0.7...v1.1.0) (2025-02-23)


### Features

* **build:** remove manual dockerfile add of Dockerfile into archive ([547c8a8](https://github.com/LynchSKM/dockem-rs/commit/547c8a8c5605cba1b51baddcd8a34e057cdda2db))

### [1.0.7](https://github.com/LynchSKM/dockem-rs/compare/v1.0.6...v1.0.7) (2025-02-23)


### Bug Fixes

* **build:** attempt to fix error with long directory names by using built-in append_dir_all function and remove println that was printing every file name ([646920f](https://github.com/LynchSKM/dockem-rs/commit/646920fe7342f61f1df597b82c7e89246000f4dc))

### [1.0.6](https://github.com/LynchSKM/dockem-rs/compare/v1.0.5...v1.0.6) (2025-02-23)


### Bug Fixes

* **build:** update tar build context to recursively crawl build context folder correctly ([e50fa09](https://github.com/LynchSKM/dockem-rs/commit/e50fa09455ea1ef8b78dc9915a8ceeff9386300e))

### [1.0.5](https://github.com/LynchSKM/dockem-rs/compare/v1.0.4...v1.0.5) (2025-02-23)


### Bug Fixes

* **build:** fix issue with copy of dockerfile into build context ([f404461](https://github.com/LynchSKM/dockem-rs/commit/f40446159412fc010606a0778452b28235b2c9ab))

### [1.0.4](https://github.com/LynchSKM/dockem-rs/compare/v1.0.3...v1.0.4) (2025-02-23)


### Bug Fixes

* **cli:** add missing version command to cli ([0735706](https://github.com/LynchSKM/dockem-rs/commit/0735706927eefc4b3b6de1d203abd489eeabb2e3))
* **documentation:** update to version 1.0.4 ([f92139b](https://github.com/LynchSKM/dockem-rs/commit/f92139b667d02759a413f821ea0c8cb454c41b0c))

### [1.0.3](https://github.com/LynchSKM/dockem-rs/compare/v1.0.2...v1.0.3) (2025-02-22)


### Bug Fixes

* **build:** add missing openssl dependency in build ([6d89836](https://github.com/LynchSKM/dockem-rs/commit/6d898364c45916bb5a91de510b148bbfbf7002c7))

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
