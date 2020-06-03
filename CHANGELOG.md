# Changelog [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][docs]

All notable changes to this project will be documented in this file. Please
update it with your changes when submitting a pull request.

This format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added

- `ByteSet::maybe_contains`: A [Bloom filter] function that returns `true` if a
  byte is maybe contained in `self`, with 1/8 (12.5%) probability.

## 0.1.0 - 2020-06-01

Initial release!

[crate]:       https://crates.io/crates/byte_set
[crate-badge]: https://img.shields.io/crates/v/byte_set.svg
[docs]:        https://docs.rs/byte_set
[docs-badge]:  https://docs.rs/byte_set/badge.svg

[Keep a Changelog]:    http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

[Bloom filter]: https://en.wikipedia.org/wiki/Bloom_filter

[Unreleased]: https://github.com/nvzqz/static-assertions-rs/compare/v0.1.0...HEAD
