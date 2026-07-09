# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [Unreleased]

## [0.1.2] - 2026-07-xx

### Changed

## [Unreleased]

## [0.1.1] - 2026-07-09

### Changed
- Improved README with installation and usage instructions.
- Simplified error handling and propagation with anyhow.

## [0.1.0] - 2026-07-08

### Added
- Initial release.
- Count lines, words, and characters in a file.
- Read from standard input via pipes (e.g. `cat file | weez`).
- Additive, stackable flags: `-l`/`--lines`, `-w`/`--words`, `-c`/`--chars`.
- Filenames included in error messages when a file cannot be read.

[Unreleased]: https://github.com/Lightx0x/weez/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Lightx0x/weez/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Lightx0x/weez/releases/tag/v0.1.0
