# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Updated internals to target rust 2021
- Comments are now valid within "cenv" blocks and will be ignored
- The keywords listed when an invalid choice is made are now de-deuplicated

## [0.0.6] - 2021-08-03
### Added
- Available keywords are now listed to the user when invalid or no keyword provided

## [0.0.5] - 2021-06-19
### Changed
- Performance improvements, now using [cenv_core](https://crates.io/crates/cenv_core) internals

## 0.0.4 - 2021-05-18
### Changed
-  Performance improvements, more work performed via. wasm

## 0.0.3 - 2021-05-12
### Added
- Alert and exit if keyword doesn't exist within file

## 0.0.2 - 2021-05-11
### Added
- MVP functionality

[Unreleased]: https://github.com/JonShort/cenv-wasm/compare/v0.0.6...HEAD
[0.0.6]: https://github.com/JonShort/cenv-wasm/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/JonShort/cenv-wasm/releases/tag/v0.0.5
