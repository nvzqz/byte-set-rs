<div align="center">
    <h1>
        <a href="https://github.com/nvzqz/byte-set-rs">
            ByteSet
        </a>
    </h1>
    <a href="https://crates.io/crates/byte_set">
        <img src="https://img.shields.io/crates/v/byte_set.svg" alt="Crates.io">
        <img src="https://img.shields.io/crates/d/byte_set.svg" alt="Downloads">
    </a>
    <a href="https://docs.rs/byte_set">
        <img src="https://docs.rs/byte_set/badge.svg" alt="docs.rs">
    </a>
    <a href="https://github.com/nvzqz/byte-set-rs/actions?query=workflow%3ACI">
        <img src="https://github.com/nvzqz/byte-set-rs/workflows/CI/badge.svg" alt="Build Status">
    </a>
    <br><br>
</div>

Efficient sets of bytes for Rust, brought to you by [@NikolaiVazquez]!

The star of the show is [`ByteSet`]: an allocation-free sorted set. It is a
*much faster* alternative to [`HashSet<u8>`], [`BTreeSet<u8>`], and other types
for a variety to scenarios. See ["Implementation"](#implementation) for a peek
under the hood.

If you found this library useful, please consider
[sponsoring me on GitHub](https://github.com/sponsors/nvzqz)!

## Table of Contents

1. [Usage](#usage)
2. [Examples](#examples)
   1. [`ByteSet` Type](#byteset-type)
      1. [Insert](#insert)
      2. [Extend](#extend)
      3. [Remove](#remove)
      4. [Iterate](#iterate)
      5. [Contains](#contains)
      6. [Subset](#subset)
      7. [Min and Max](#min-and-max)
   2. [`byte_set!` Macro](#byte_set-macro)
3. [Implementation](#implementation)
4. [Benchmarks](#benchmarks)
5. [License](#license)

## Usage

This library is available [on crates.io][crate] and can be used by adding the
following to your project's [`Cargo.toml`]:

```toml
[dependencies]
byte_set = "0.1"
```

To import the [`byte_set!`] macro, add this to your crate root (`main.rs` or
`lib.rs`):

```rust
use byte_set::byte_set;
```

If you're not using [Rust 2018 edition][2018], it must be imported differently:

```rust
#[macro_use]
extern crate byte_set;
```

## Examples

### `ByteSet` Type

First, let's import [`ByteSet`]:

```rust
use byte_set::ByteSet;
```

Here's how you create an empty set:

```rust
let bytes = ByteSet::new();
```

You can create a set filled with all bytes (0 through 255) just as easily:

```rust
let bytes = ByteSet::full();
```

Ok, let's see what we can do with this. Note that this isn't the only available
functionality. See [`ByteSet`] for a complete list.

#### Insert

Use [`insert`] to include a single byte, by mutating the [`ByteSet`] in-place:

```rust
let mut bytes = ByteSet::new();
bytes.insert(255);
```

Use [`inserting`] as an immutable alternative, by passing the calling
[`ByteSet`] by value:

```rust
let bytes = ByteSet::new().inserting(255);
```

Use [`insert_all`] to include all bytes of another [`ByteSet`], by mutating the
[`ByteSet`] in-place:

```rust
let mut alphabet = ByteSet::ASCII_UPPERCASE;
alphabet.insert_all(ByteSet::ASCII_LOWERCASE);

assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
```

Use [`inserting_all`] as an immutable alternative, by passing the calling
[`ByteSet`] by value:

```rust
let alphabet = ByteSet::ASCII_UPPERCASE.inserting_all(ByteSet::ASCII_LOWERCASE);

assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
```

#### Extend

Rather than call [`insert`] in a loop, [`extend`] simplifies inserting from an
iterator:

```rust
fn take_string(bytes: &mut ByteSet, s: &str) {
    bytes.extend(s.as_bytes());
}
```

Because this iterates over the entire input, it is *much* more efficient to use
[`insert_all`] instead of [`extend`] when inserting another [`ByteSet`].

#### Remove

Use [`remove`] to exclude a single byte by mutating the set in-place:

```rust
let mut bytes = ByteSet::full();
bytes.remove(255);
```

Use [`removing`] as an immutable alternative, by passing the calling [`ByteSet`]
by value:

```rust
let bytes = ByteSet::full().removing(255);
```

Use [`remove_all`] to exclude all bytes of another [`ByteSet`], by mutating the
[`ByteSet`] in-place:

```rust
let mut alphabet = ByteSet::ASCII_ALPHANUMERIC;
alphabet.remove_all(ByteSet::ASCII_DIGIT);

assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
```

Use [`removing_all`] as an immutable alternative, by passing the calling
[`ByteSet`] by value:

```rust
let alphabet = ByteSet::ASCII_ALPHANUMERIC.removing_all(ByteSet::ASCII_DIGIT);

assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
```

#### Iterate

Iterating can be done with just a `for` loop, and goes in order from least to
greatest:

```rust
fn small_to_big(bytes: ByteSet) {
    for byte in bytes {
        do_work(byte);
    }
}
```

Iterating in reverse is slightly more verbose, and goes in order from greatest
to least:

```rust
fn big_to_small(bytes: ByteSet) {
    for byte in bytes.into_iter().rev() {
        do_work(byte);
    }
}
```

#### Contains

It wouldn't really be a set if you couldn't check if it has specific items.

Use [`contains`] to check a single byte:

```rust
fn has_null(bytes: &ByteSet) -> bool {
    bytes.contains(0)
}
```

Use [`contains_any`] to check for any matches in another [`ByteSet`]:

```rust
fn intersects(a: &ByteSet, b: &ByteSet) -> bool {
    a.contains_any(b)
}
```

#### Subset

Use [`is_subset`] to check that all of the bytes in a [`ByteSet`] are contained
in another:

```rust
fn test(a: &ByteSet, b: &ByteSet) {
    assert!(a.is_subset(b));

    // Always passes because every set is a subset of itself.
    assert!(a.is_subset(a));
}
```

Use [`is_strict_subset`] to check [`is_subset`] *and* that the sets are not the
same:

```rust
fn test(a: &ByteSet, b: &ByteSet) {
    assert!(a.is_strict_subset(b));

    // `a` is equal to itself.
    assert!(!a.is_strict_subset(a));
}
```

For the sake of completion, there is also [`is_superset`] and
[`is_strict_superset`], which call these functions with `a` and `b` switched.

#### Min and Max

Use [`first`] to get the smallest byte and [`last`] to get the biggest byte:

```rust
fn sanity_check(bytes: &ByteSet) {
    if let (Some(first), Some(last)) = (bytes.first(), bytes.last()) {
        assert!(first <= last);
    } else {
        // `bytes` is empty.
    }
}
```

These are the first and last bytes returned when iterating.

### `byte_set!` Macro

[`byte_set!`] enables you to create a [`ByteSet`] with the same syntax as [`vec!`]
or array expressions:

```rust
let bytes = byte_set![1, 2, 3, b'x', b'y', b'z'];
```

It even works at compile-time in a `const` expression:

```rust
const WHOA: ByteSet = byte_set![b'w', b'h', b'o', b'a'];

static ABC: ByteSet = byte_set![b'a', b'c', b'c'];
```

## Implementation

[`ByteSet`] is implemented as a 256-bit mask where each bit corresponds to a
byte value. The first (least significant) bit in the mask represents the first
byte (0) in the set. Likewise, the last last (most significant) bit represents
the last byte (255).

Given the following [`ByteSet`]:

```rust
let bytes = byte_set![0, 1, 4, 5, 244];
```

The in-memory representation of `bytes` would look like:

```text
 Byte: 0 1 2 3 4 5 6 7 ... 253 244 255
Value: 1 1 0 0 1 1 0 0 ...  0   1   0
```

This bit mask is composed of either `[u64; 4]` or `[u32; 8]` depending on the
target CPU (see [#3]). Because this comes out to only 32 bytes, [`ByteSet`]
implements [`Copy`].

## Benchmarks

I will upload benchmarks run from my machine soon.

In the meantime, you can benchmark this library by running:

```sh
cargo bench
```

By default, this will benchmark [`ByteSet`] along with various other types to
compare performance. Note that this will take **a long time** (about 1 hour and
30 minutes).

Benchmark only [`ByteSet`] by running:

```sh
cargo bench ByteSet
```

This takes about 15 minutes, so maybe grab a coffee in the meantime.

Benchmark a specific [`ByteSet`] operation by running:

```sh
cargo bench $operation/ByteSet
```

See `/benches/benchmarks` for strings that can be used for `$operation`.

Note that `cargo bench` takes a regular expression, so `Contains (Random)` will
not work because the parentheses are treated as a capture group. To match
parentheses, escape them: `Contains \(Random\)`.

## License

This project is released under either:

- [MIT License](https://github.com/nvzqz/byte-set-rs/blob/master/LICENSE-MIT)

- [Apache License (Version 2.0)](https://github.com/nvzqz/byte-set-rs/blob/master/LICENSE-APACHE)

at your choosing.

[@NikolaiVazquez]: https://twitter.com/NikolaiVazquez

[`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
[2018]:         https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018
[crate]:        https://crates.io/crates/byte_set

[`BTreeSet<u8>`]:   https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
[`Copy`]:           https://doc.rust-lang.org/std/marker/trait.Copy.html
[`HashSet<u8>`]:    https://doc.rust-lang.org/std/collections/struct.HashSet.html
[`u8`]:             https://doc.rust-lang.org/std/primitive.u8.html
[`vec!`]:           https://doc.rust-lang.org/std/macro.vec.html

[#3]: https://github.com/nvzqz/byte-set-rs/issues/3

<!-- These links must be replaced with local ones when used in crate docs: -->
[`byte_set!`]:          https://docs.rs/byte_set/0.1.1/byte_set/macro.byte_set.html
[`ByteSet`]:            https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html
[`contains_any`]:       https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.contains_any
[`contains`]:           https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.contains
[`extend`]:             https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#impl-Extend%3Cu8%3E
[`first`]:              https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.first
[`insert_all`]:         https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.insert_all
[`insert`]:             https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.insert
[`inserting_all`]:      https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.inserting_all
[`inserting`]:          https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.inserting
[`last`]:               https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.last
[`remove_all`]:         https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.remove_all
[`remove`]:             https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.remove
[`removing_all`]:       https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.removing_all
[`removing`]:           https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.removing
[`is_strict_subset`]:   https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.is_strict_subset
[`is_subset`]:          https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.is_subset
[`is_strict_superset`]: https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.is_strict_superset
[`is_superset`]:        https://docs.rs/byte_set/0.1.1/byte_set/struct.ByteSet.html#method.is_superset
