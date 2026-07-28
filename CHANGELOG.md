# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2026-07-28

### Changed
- GUI structure.
- Single input file to many files.
- Log messages.

### Fixed
- Remove ANSI-codes in windows logs.


## [0.4.1] - 2026-04-27

### Fixed
- Hide debug logs.

## [0.4.0] - 2026-04-26

### Changed
- Default recursive search to 16 (was 0).
- Default jobs to the number of CPU threads.
- Rename --force to --overwrite.
- CLI: --recursive must contains a value.


## [0.3.1] - 2026-03-12

### Added
- Russian translate.

### Changed
- GUI: display tags.
- GUI: dynamic disable unused options.
- GUI: increase font size.
- Log & help messages.

## [0.3.0] - 2026-03-01

### Added
- GUI interface.
- Color log prefixes in CLI.

### Changed
- API.
- Logging via log crate.


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
