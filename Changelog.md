# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added
- new features: `nightly_docs`, `unsafest`.
- impl conversions from scalars to strings & egcs.
- make `new` constructors const for strings and egcs.
- new const methods to convert from scalars to strings and egcs.
- new method `is_nul` for scalars.
- new scalar method `to_utf8_bytes`.
- new types `Egc`, `StaticNonNullEgc`, `StaticU8Egc`
    - new sized type aliases from 8 to 128 bits.
- new `iformat` macro.

### Removed
- remove `safe` from the `default` features.
- remove `int_buf` module, move to `devela`.

### Changed
- bump devela `0.8.0`.
- bump MSRV to `1.72.0`.
- deprecate and rename `no-std` feature to `no_std`.
- include in the crate any markdown files in `src/`
- check for `not("unsafe")` instead of `"safe"`.

### Fixed
- refactor manifest.
- update cargo categories and keywords.
- improve `check.sh` script, aliases, CI, docs.

## [0.0.2] - 2023-06-24

### Added
- new string types: `StaticNonNulString`, `StaticU8String`.
- new scalar types: `Char7`, `char8`, `Char16`, `Char24`, `Char32`.
- new fns: `indent`, `counter_string`, struct: `IntBuf` and trait: `IntBufAble`

## [0.0.1] - 2023-06-13

First release.

### Added
- new types: `BoxDrawing`, `String32`, `String64`, `String128`, `String256`, `String512`, `String1024`, `String2048`.
- new macro: `ascii_eq_uncased`.


[unreleased]: https://github.com/andamira/textos/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/andamira/textos/releases/tag/v0.0.2
[0.0.1]: https://github.com/andamira/textos/releases/tag/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
