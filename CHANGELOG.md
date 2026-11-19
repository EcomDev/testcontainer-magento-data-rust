# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/EcomDev/testcontainer-magento-data-rust/compare/v0.1.0...v0.2.0) - 2026-11-19

### Added

- [**breaking**] support for multiple distributions
- bump minimum rust version, update deps

### Fixed

- ignore for now astral-tar usage until upstream fix
- warning for lifetime confusion

### Other

- update sqlx to latest version as older has active advisory
- downgrade dev dependency to pass MSRV
- fix formatting
- update action versions
- update msrv in ci build
- add proper badge urls for images
- change release plz to use trusted builds

## [0.1.0](https://github.com/EcomDev/testcontainer-magento-data-rust/releases/tag/v0.1.0) - 2025-01-07

### Added

- introduce `DbContainer` and `OpenSearchContainer`

### Other

- move to rust specific release-plz
- add release-please
- setup repo
