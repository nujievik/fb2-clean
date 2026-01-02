# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2026-01-02

### Added

- Parallel run via `--jobs`.

### Deprecated

- API: Input::iter (use Input::new_iter instead).

### Fixed

- Remove temp directories while `--force`.


## [0.2.1] - 2025-12-18

### Changed

- Flag `--force` overwrite input files, removing temp files.

## [0.2.0] - 2025-12-11

### Changed

- Minor version to 2.


## [0.1.1] - 2025-12-08 [YANKED]

Yanked due Config API incompatibility.

### Added

- Recursive search via --recursive.

### Changed

- Conflict while parse mutually exclusive --zip and --unzip.
- Warning if not found any fb2.

## [0.1.0] - 2025-12-07

### Added

- Initial release.
