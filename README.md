# City Builder

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/city-builder/build.yml)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/mrivnak/city-builder/test.yml?label=tests)
[![Coverage Status](https://coveralls.io/repos/github/mrivnak/city-builder/badge.svg?branch=develop)](https://coveralls.io/github/mrivnak/city-builder?branch=develop)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/mrivnak/city-builder?display_name=tag&sort=semver)

![Bevy Engine](https://img.shields.io/badge/Bevy-%23FFFFFF.svg?style=for-the-badge&logo=Rust&logoColor=black)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)
![Coveralls](https://img.shields.io/badge/coveralls-%23b94947.svg?style=for-the-badge&logo=coveralls&logoColor=white)
![Renovate](https://img.shields.io/badge/renovate-%230281a1?style=for-the-badge&logo=renovatebot&logoColor=white)

City builder game where time progresses from the stone age into the future. Uses the [Bevy](https://bevyengine.org/) game engine

## Development

### Prerequisites

- [Rust nightly](https://www.rust-lang.org/tools/install)
- Bevy's [dependencies](https://bevyengine.org/learn/book/getting-started/setup/)
  - plus libwayland and libxkbcommon for Linux
- LLD (required in Debug mode for faster compile times)

### Running

There's a couple development features that can be enabled to speed up compile times/add debugging features. You can add `--features bevy/dynamic_linking,inspector` to the `cargo run` command to enable them.

```bash
cargo run
# or
cargo run --features bevy/dynamic_linking,inspector
```

<a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://i.creativecommons.org/l/by-nc-nd/4.0/88x31.png" /></a><br />This work is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License</a>.