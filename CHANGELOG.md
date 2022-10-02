# genrepass's changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.1.1]
<!--BEGIN=1.1.1-->
Last 1.x release before refactor. Updated all dependencies except `clap`.

### Added

- GitHub Actions workflow for releasing new versions.
<!--END=1.1.1-->
## [1.1.0] - 2020-10-19

### Added

- More precise errors and clearer arguments.
- Coloured help message.

### Changed

- Extracted a library from the binary code.
- Let the library live at [genrepass](https://github.com/AlexChaplinBraz/genrepass).
- Created [genrepass-cli](https://github.com/AlexChaplinBraz/genrepass-cli) to host the CLI.
- Can now input more than one path as source for words.

## [1.0.1] - 2020-10-13

### Changed

- Refactored `Password::new()`.
- From `clipboard-ext` to `copypasta-ext`, adding support for Wayland clipboard
  [[PR1]](https://github.com/AlexChaplinBraz/genrepass/pull/1).

## [1.0.0] - 2020-09-23

Ported my [`genrepass.sh`](https://github.com/AlexChaplinBraz/shell-scripts/tree/master/genrepass) script to Rust.

[Unreleased]: https://github.com/AlexChaplinBraz/genrepass/compare/1.1.1...HEAD
[1.1.1]: https://github.com/AlexChaplinBraz/genrepass/compare/531efab...1.1.1
[1.1.0]: https://github.com/AlexChaplinBraz/genrepass/compare/bdbd989...531efab
[1.0.1]: https://github.com/AlexChaplinBraz/genrepass/compare/8908ce4...bdbd989
[1.0.0]: https://github.com/AlexChaplinBraz/genrepass/tree/8908ce4
