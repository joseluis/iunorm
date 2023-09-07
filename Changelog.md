# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added
- readd katex header.

### Changed
- remove `std` from default features.
- update dependencies.

### Fixed
- remove unnecessary `std` dependency for impl `Display`.
- fix docs for ranges, make them inclusive.
- remove unnecessary clippy allowances.
- update categories, licenses, readme.
- general docs improvements.
- refactor internal macro.
- misc. fixes.
- update CI.

## [0.2.1] - 2022-12-17

### Changed
- update MSRV to `v1.43.1`.
- update `paste` dependency.
- update maintenance to `passively-maintained`.

## [0.2.0] - 2022-10-05

### Added
- add methods `from_f32_minmax`, `to_f32_minmax`, `from_f64_minmax`, `to_f64_minmax`.
- add functions `scale32`, `scale64`.
- add default `std` feature & implement `Display`.

## [0.1.2] - 2022-09-27

### Added
- add conversions to floating point.

## [0.1.1] - 2022-03-09

### Added

- make `from_f32` & `from_f64` methods public.
- make the newtypes inner type public.
- add a simple example.

## [0.1.0] - 2022-03-08

First commit.


[unreleased]: https://github.com/joseluis/iunorm/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/joseluis/iunorm/releases/tag/v0.2.1
[0.2.0]: https://github.com/joseluis/iunorm/releases/tag/v0.2.0
[0.1.2]: https://github.com/joseluis/iunorm/releases/tag/v0.1.2
[0.1.1]: https://github.com/joseluis/iunorm/releases/tag/v0.1.1
[0.1.0]: https://github.com/joseluis/iunorm/releases/tag/v0.1.0

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
