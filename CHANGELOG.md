# Changelog [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][docs]

All notable changes to this project will be documented in this file. Please
update it with your changes when submitting a pull request.

This format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Changed

- **\[Breaking\]** Comparison functions in [`PartialOrd`] and [`Ord`] are based
  on lexicographical order of the contained bytes. Previously they were just a
  `memcmp`, regardless of architecture.

## [0.1.3] - 2020-06-12

### Added

- `ByteSet::from_byte`: creates a set from a single byte.
- `serde::Serialize` into a `u8` sequence.
- `serde::Deserialize` from `&[u8]` or a `u8` sequence.
- Conversions from range types that can be used in `const` ([#5] by [@Manishearth]).
- Manual implementation of `Iter::for_each` that's slightly more optimized than
  the default.

## [0.1.2] - 2020-06-03

### Removed

- `build.rs`, which improves compile time and allows this to be used in external
  build systems like Bazel and Buck.

## [0.1.1] - 2020-06-03

### Added

- `ByteSet::is_ascii_alphabetic`: returns `true` if [`u8::is_ascii_alphabetic`] returns `true` for all bytes.
- `ByteSet::is_ascii_uppercase`: returns `true` if [`u8::is_ascii_uppercase`] returns `true` for all bytes.
- `ByteSet::is_ascii_lowercase`: returns `true` if [`u8::is_ascii_lowercase`] returns `true` for all bytes.
- `ByteSet::is_ascii_alphanumeric`: returns `true` if [`u8::is_ascii_alphanumeric`] returns `true` for all bytes.
- `ByteSet::is_ascii_digit`: returns `true` if [`u8::is_ascii_digit`] returns `true` for all bytes.
- `ByteSet::is_ascii_hexdigit`: returns `true` if [`u8::is_ascii_hexdigit`] returns `true` for all bytes.
- `ByteSet::is_ascii_punctuation`: returns `true` if [`u8::is_ascii_punctuation`] returns `true` for all bytes.
- `ByteSet::is_ascii_graphic`: returns `true` if [`u8::is_ascii_graphic`] returns `true` for all bytes.
- `ByteSet::is_ascii_whitespace`: returns `true` if [`u8::is_ascii_whitespace`] returns `true` for all bytes.
- `ByteSet::is_ascii_control`: returns `true` if [`u8::is_ascii_control`] returns `true` for all bytes.

## 0.1.0 - 2020-06-01

Initial release!

[crate]:       https://crates.io/crates/byte_set
[crate-badge]: https://img.shields.io/crates/v/byte_set.svg
[docs]:        https://docs.rs/byte_set
[docs-badge]:  https://docs.rs/byte_set/badge.svg

[Keep a Changelog]:    http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

[`PartialOrd`]:                https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
[`Ord`]:                       https://doc.rust-lang.org/std/cmp/trait.Ord.html
[`u8::is_ascii_alphabetic`]:   https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
[`u8::is_ascii_uppercase`]:    https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_uppercase
[`u8::is_ascii_lowercase`]:    https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_lowercase
[`u8::is_ascii_alphanumeric`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphanumeric
[`u8::is_ascii_digit`]:        https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_digit
[`u8::is_ascii_hexdigit`]:     https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_hexdigit
[`u8::is_ascii_punctuation`]:  https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_punctuation
[`u8::is_ascii_graphic`]:      https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_graphic
[`u8::is_ascii_whitespace`]:   https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
[`u8::is_ascii_control`]:      https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_control

[`serde::Serialize`]:   https://docs.rs/serde/1.*/serde/trait.Serialize.html
[`serde::Deserialize`]: https://docs.rs/serde/1.*/serde/trait.Deserialize.html

[Unreleased]: https://github.com/nvzqz/byte-set-rs/compare/v0.1.3...HEAD
[0.1.3]:      https://github.com/nvzqz/byte-set-rs/compare/v0.1.2...v0.1.3
[0.1.2]:      https://github.com/nvzqz/byte-set-rs/compare/v0.1.1...v0.1.2
[0.1.1]:      https://github.com/nvzqz/byte-set-rs/compare/v0.1.0...v0.1.1

[#5]: https://github.com/nvzqz/byte-set-rs/pull/5

[@Manishearth]: https://github.com/Manishearth
