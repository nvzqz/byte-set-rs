//! <div align="center">
//!     <h1>
//!         <a href="https://github.com/nvzqz/byte-set-rs">
//!             ByteSet
//!         </a>
//!     </h1>
//!     <a href="https://crates.io/crates/byte_set">
//!         <img src="https://img.shields.io/crates/v/byte_set.svg" alt="Crates.io">
//!         <img src="https://img.shields.io/crates/d/byte_set.svg" alt="Downloads">
//!     </a>
//!     <a href="https://docs.rs/byte_set">
//!         <img src="https://docs.rs/byte_set/badge.svg" alt="docs.rs">
//!     </a>
//!     <a href="https://github.com/nvzqz/byte-set-rs/actions?query=workflow%3ACI">
//!         <img src="https://github.com/nvzqz/byte-set-rs/workflows/CI/badge.svg" alt="Build Status">
//!     </a>
//!     <br><br>
//! </div>
//!
//! Efficient sets of bytes for Rust, brought to you by [@NikolaiVazquez]!
//!
//! The star of the show is [`ByteSet`]: an allocation-free sorted set. It is a
//! *much faster* alternative to [`HashSet<u8>`], [`BTreeSet<u8>`], and other
//! types for a variety to scenarios. See ["Implementation"](#implementation)
//! for a peek under the hood.
//!
//! If you found this library useful, please consider [sponsoring me on
//! GitHub](https://github.com/sponsors/nvzqz)!
//!
//! ## Table of Contents
//!
//! 1. [Usage](#usage)
//! 2. [Examples](#examples)
//!    1. [`ByteSet` Type](#byteset-type)
//!       1. [Insert](#insert)
//!       2. [Extend](#extend)
//!       3. [Remove](#remove)
//!       4. [Iterate](#iterate)
//!       5. [Contains](#contains)
//!       6. [Subset](#subset)
//!       7. [Min and Max](#min-and-max)
//!    2. [`byte_set!` Macro](#byte_set-macro)
//! 3. [Implementation](#implementation)
//! 4. [Benchmarks](#benchmarks)
//! 5. [License](#license)
//!
//! ## Usage
//!
//! This library is available [on crates.io][crate] and can be used by adding
//! the following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! byte_set = "0.1"
//! ```
//!
//! To import the [`byte_set!`] macro, add this to your crate root (`main.rs` or
//! `lib.rs`):
//!
//! ```rust
//! use byte_set::byte_set;
//! ```
//!
//! If you're not using [Rust 2018 edition][2018], it must be imported
//! differently:
//!
//! ```rust
//! #[macro_use]
//! extern crate byte_set;
//! # fn main() {}
//! ```
//!
//! ## Examples
//!
//! ### `ByteSet` Type
//!
//! First, let's import [`ByteSet`]:
//!
//! ```rust
//! use byte_set::ByteSet;
//! ```
//!
//! Here's how you create an empty set:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let bytes = ByteSet::new();
//! ```
//!
//! You can create a set filled with all bytes (0 through 255) just as easily:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let bytes = ByteSet::full();
//! ```
//!
//! Ok, let's see what we can do with this. Note that this isn't the only
//! available functionality. See [`ByteSet`] for a complete list.
//!
//! #### Insert
//!
//! Use [`insert`] to include a single byte, by mutating the [`ByteSet`]
//! in-place:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let mut bytes = ByteSet::new();
//! bytes.insert(255);
//! ```
//!
//! Use [`inserting`] as an immutable alternative, by passing the calling
//! [`ByteSet`] by value:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let bytes = ByteSet::new().inserting(255);
//! ```
//!
//! Use [`insert_all`] to include all bytes of another [`ByteSet`], by mutating
//! the [`ByteSet`] in-place:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let mut alphabet = ByteSet::ASCII_UPPERCASE;
//! alphabet.insert_all(ByteSet::ASCII_LOWERCASE);
//!
//! assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
//! ```
//!
//! Use [`inserting_all`] as an immutable alternative, by passing the calling
//! [`ByteSet`] by value:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let alphabet = ByteSet::ASCII_UPPERCASE.inserting_all(ByteSet::ASCII_LOWERCASE);
//!
//! assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
//! ```
//!
//! #### Extend
//!
//! Rather than call [`insert`] in a loop, [`extend`] simplifies inserting from
//! an iterator:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! fn take_string(bytes: &mut ByteSet, s: &str) {
//!     bytes.extend(s.as_bytes());
//! }
//! ```
//!
//! Because this iterates over the entire input, it is *much* more efficient to
//! use [`insert_all`] instead of [`extend`] when inserting another [`ByteSet`].
//!
//! #### Remove
//!
//! Use [`remove`] to exclude a single byte by mutating the set in-place:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let mut bytes = ByteSet::full();
//! bytes.remove(255);
//! ```
//!
//! Use [`removing`] as an immutable alternative, by passing the calling
//! [`ByteSet`] by value:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let bytes = ByteSet::full().removing(255);
//! ```
//!
//! Use [`remove_all`] to exclude all bytes of another [`ByteSet`], by mutating
//! the [`ByteSet`] in-place:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let mut alphabet = ByteSet::ASCII_ALPHANUMERIC;
//! alphabet.remove_all(ByteSet::ASCII_DIGIT);
//!
//! assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
//! ```
//!
//! Use [`removing_all`] as an immutable alternative, by passing the calling
//! [`ByteSet`] by value:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! let alphabet = ByteSet::ASCII_ALPHANUMERIC.removing_all(ByteSet::ASCII_DIGIT);
//!
//! assert_eq!(alphabet, ByteSet::ASCII_ALPHABETIC);
//! ```
//!
//! #### Iterate
//!
//! Iterating can be done with just a `for` loop, and goes in order from least
//! to greatest:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! # fn do_work(_: u8) {}
//! fn small_to_big(bytes: ByteSet) {
//!     for byte in bytes {
//!         do_work(byte);
//!     }
//! }
//! ```
//!
//! Iterating in reverse is slightly more verbose, and goes in order from
//! greatest to least:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! # fn do_work(_: u8) {}
//! fn big_to_small(bytes: ByteSet) {
//!     for byte in bytes.into_iter().rev() {
//!         do_work(byte);
//!     }
//! }
//! ```
//!
//! #### Contains
//!
//! It wouldn't really be a set if you couldn't check if it has specific items.
//!
//! Use [`contains`] to check a single byte:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! fn has_null(bytes: &ByteSet) -> bool {
//!     bytes.contains(0)
//! }
//! ```
//!
//! Use [`contains_any`] to check for any matches in another [`ByteSet`]:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! fn intersects(a: &ByteSet, b: &ByteSet) -> bool {
//!     a.contains_any(b)
//! }
//! ```
//!
//! #### Subset
//!
//! Use [`is_subset`] to check that all of the bytes in a [`ByteSet`] are
//! contained in another:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! fn test(a: &ByteSet, b: &ByteSet) {
//!     assert!(a.is_subset(b));
//!
//!     // Always passes because every set is a subset of itself.
//!     assert!(a.is_subset(a));
//! }
//! ```
//!
//! Use [`is_strict_subset`] to check [`is_subset`] *and* that the sets are not
//! the same:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! fn test(a: &ByteSet, b: &ByteSet) {
//!     assert!(a.is_strict_subset(b));
//!
//!     // `a` is equal to itself.
//!     assert!(!a.is_strict_subset(a));
//! }
//! ```
//!
//! For the sake of completion, there is also [`is_superset`] and
//! [`is_strict_superset`], which call these functions with `a` and `b`
//! switched.
//!
//! #### Min and Max
//!
//! Use [`first`] to get the smallest byte and [`last`] to get the biggest byte:
//!
//! ```rust
//! # use byte_set::ByteSet;
//! fn sanity_check(bytes: &ByteSet) {
//!     if let (Some(first), Some(last)) = (bytes.first(), bytes.last()) {
//!         assert!(first <= last);
//!     } else {
//!         // `bytes` is empty.
//!     }
//! }
//! ```
//!
//! These are the first and last bytes returned when iterating.
//!
//! ### `byte_set!` Macro
//!
//! [`byte_set!`] enables you to create a [`ByteSet`] with the same syntax as
//! [`vec!`] or array expressions:
//!
//! ```rust
//! # use byte_set::byte_set;
//! let bytes = byte_set![1, 2, 3, b'x', b'y', b'z'];
//! ```
//!
//! It even works at compile-time in a `const` expression:
//!
//! ```rust
//! # use byte_set::{ByteSet, byte_set};
//! const WHOA: ByteSet = byte_set![b'w', b'h', b'o', b'a'];
//!
//! static ABC: ByteSet = byte_set![b'a', b'c', b'c'];
//! ```
//!
//! ## Implementation
//!
//! [`ByteSet`] is implemented as a 256-bit mask where each bit corresponds to a
//! byte value. The first (least significant) bit in the mask represents the
//! first byte (0) in the set. Likewise, the last last (most significant) bit
//! represents the last byte (255).
//!
//! Given the following [`ByteSet`]:
//!
//! ```rust
//! # use byte_set::byte_set;
//! let bytes = byte_set![0, 1, 4, 5, 244];
//! ```
//!
//! The in-memory representation of `bytes` would look like:
//!
//! ```text
//!  Byte: 0 1 2 3 4 5 6 7 ... 253 244 255
//! Value: 1 1 0 0 1 1 0 0 ...  0   1   0
//! ```
//!
//! This bit mask is composed of either `[u64; 4]` or `[u32; 8]` depending on
//! the target CPU (see [#3]). Because this comes out to only 32 bytes,
//! [`ByteSet`] implements [`Copy`].
//!
//! ## Benchmarks
//!
//! I will upload benchmarks run from my machine soon.
//!
//! In the meantime, you can benchmark this library by running:
//!
//! ```sh
//! cargo bench
//! ```
//!
//! By default, this will benchmark [`ByteSet`] along with various other types
//! to compare performance. Note that this will take **a long time** (about 1
//! hour and 30 minutes).
//!
//! Benchmark only [`ByteSet`] by running:
//!
//! ```sh
//! cargo bench ByteSet
//! ```
//!
//! This takes about 15 minutes, so maybe grab a coffee in the meantime.
//!
//! Benchmark a specific [`ByteSet`] operation by running:
//!
//! ```sh
//! cargo bench $operation/ByteSet
//! ```
//!
//! See `/benches/benchmarks` for strings that can be used for `$operation`.
//!
//! Note that `cargo bench` takes a regular expression, so `Contains (Random)`
//! will not work because the parentheses are treated as a capture group. To
//! match parentheses, escape them: `Contains \(Random\)`.
//!
//! ## License
//!
//! This project is released under either:
//!
//! - [MIT License](https://github.com/nvzqz/byte-set-rs/blob/master/LICENSE-MIT)
//!
//! - [Apache License (Version 2.0)](https://github.com/nvzqz/byte-set-rs/blob/master/LICENSE-APACHE)
//!
//! at your choosing.
//!
//! [@NikolaiVazquez]: https://twitter.com/NikolaiVazquez
//!
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [2018]:         https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018
//! [crate]:        https://crates.io/crates/byte_set
//!
//! [`BTreeSet<u8>`]:   https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
//! [`Copy`]:           https://doc.rust-lang.org/std/marker/trait.Copy.html
//! [`HashSet<u8>`]:    https://doc.rust-lang.org/std/collections/struct.HashSet.html
//! [`u8`]:             https://doc.rust-lang.org/std/primitive.u8.html
//! [`vec!`]:           https://doc.rust-lang.org/std/macro.vec.html
//!
//! [#3]: https://github.com/nvzqz/byte-set-rs/issues/3
//!
//! [`byte_set!`]:          macro.byte_set.html
//! [`ByteSet`]:            struct.ByteSet.html
//! [`contains_any`]:       struct.ByteSet.html#method.contains_any
//! [`contains`]:           struct.ByteSet.html#method.contains
//! [`extend`]:             struct.ByteSet.html#impl-Extend%3Cu8%3E
//! [`first`]:              struct.ByteSet.html#method.first
//! [`insert_all`]:         struct.ByteSet.html#method.insert_all
//! [`insert`]:             struct.ByteSet.html#method.insert
//! [`inserting_all`]:      struct.ByteSet.html#method.inserting_all
//! [`inserting`]:          struct.ByteSet.html#method.inserting
//! [`last`]:               struct.ByteSet.html#method.last
//! [`remove_all`]:         struct.ByteSet.html#method.remove_all
//! [`remove`]:             struct.ByteSet.html#method.remove
//! [`removing_all`]:       struct.ByteSet.html#method.removing_all
//! [`removing`]:           struct.ByteSet.html#method.removing
//! [`is_strict_subset`]:   struct.ByteSet.html#method.is_strict_subset
//! [`is_subset`]:          struct.ByteSet.html#method.is_subset
//! [`is_strict_superset`]: struct.ByteSet.html#method.is_strict_superset
//! [`is_superset`]:        struct.ByteSet.html#method.is_superset

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

use core::{cmp, fmt, hash, iter::FromIterator, mem, ops, slice};

// Makes `ByteSet::{rand,try_rand}` simpler to express.
#[cfg(any(test, feature = "rand"))]
use rand as rand_core;

#[macro_use]
mod macros;

#[cfg(test)]
#[macro_use]
mod tests_macros;

#[cfg(test)]
mod tests;

pub(crate) mod chunk;
pub(crate) use chunk::Chunk;

mod iter;
pub use iter::Iter;

const SLOT_SIZE: usize = mem::size_of::<Chunk>();

const NUM_SLOTS: usize = 256 / 8 / SLOT_SIZE;

const LAST_SLOT_INDEX: usize = NUM_SLOTS - 1;

/// An efficient, general-purpose set of [`u8`]s.
///
/// # Implementation
///
/// This is a 256-bit mask where a byte is contained based on whether its bit is
/// enabled. The first (least significant) bit in the mask represents the first
/// byte in the set. Likewise, the last last (most significant) bit represents
/// the last byte.
///
/// The mask is composed a of "chunk" array. Each chunk is either 64 or 32 bits
/// wide, depending on the target architecture. As of right now, this is based
/// on native register size. This may change in the future based on target
/// features that enable better performance.
///
/// [`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ByteSet([Chunk; NUM_SLOTS]);

/// Returns the chunk index for `byte` and the bit shift for that chunk.
#[inline]
const fn chunk_index_and_shift(byte: u8) -> (usize, usize) {
    let byte = byte as usize;

    #[cfg(target_pointer_width = "64")]
    let index = byte >> 6;
    #[cfg(target_pointer_width = "64")]
    let shift = byte & 0b0011_1111;

    #[cfg(not(target_pointer_width = "64"))]
    let index = byte >> 5;
    #[cfg(not(target_pointer_width = "64"))]
    let shift = byte & 0b0001_1111;

    (index, shift)
}

#[cfg(test)]
impl ByteSet {
    /// Returns a formatting proxy for the binary representation of `self`.
    ///
    /// `fmt::Binary` is not currently implemented for `ByteSet` because of the
    /// extra work to support formatting options.
    pub(crate) fn fmt_binary<'a>(&'a self) -> impl fmt::Display + 'a {
        struct Formatted<'a>(&'a ByteSet);

        impl fmt::Display for Formatted<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for chunk in &(self.0).0 {
                    #[cfg(target_pointer_width = "64")]
                    write!(f, "{:064b}", chunk)?;

                    #[cfg(not(target_pointer_width = "64"))]
                    write!(f, "{:032b}", chunk)?;
                }
                Ok(())
            }
        }

        Formatted(self)
    }
}

impl ByteSet {
    /// Returns a set containing no bytes.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self([0; NUM_SLOTS])
    }

    /// Returns a set containing all bytes (0-255).
    #[inline]
    #[must_use]
    pub const fn full() -> Self {
        Self([Chunk::max_value(); NUM_SLOTS])
    }

    /// Returns a set containing uniformly-distributed random bytes from `rng`.
    ///
    /// This uses [`fill_bytes`] under the hood.
    ///
    /// [`fill_bytes`]: https://docs.rs/rand_core/0.5.*/rand_core/trait.RngCore.html#tymethod.fill_bytes
    #[cfg(any(test, feature = "rand", feature = "rand_core"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "rand", feature = "rand_core"))))]
    #[inline]
    pub fn rand<R: rand_core::RngCore>(mut rng: R) -> Self {
        let mut set = Self::new();
        rng.fill_bytes(set.as_bytes_mut());
        set
    }

    /// Returns a set containing uniformly-distributed random bytes from `rng`,
    /// or `Err` if `rng` failed.
    ///
    /// This uses [`try_fill_bytes`] under the hood.
    ///
    /// [`try_fill_bytes`]: https://docs.rs/rand_core/0.5.*/rand_core/trait.RngCore.html#tymethod.try_fill_bytes
    #[cfg(any(test, feature = "rand", feature = "rand_core"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "rand", feature = "rand_core"))))]
    #[inline]
    pub fn try_rand<R: rand_core::RngCore>(
        mut rng: R,
    ) -> Result<Self, rand_core::Error> {
        let mut set = Self::new();
        rng.try_fill_bytes(set.as_bytes_mut())?;
        Ok(set)
    }

    /// Returns `true` if `self` contains no bytes.
    ///
    /// This is more efficient than checking `self.len() == 0`.
    #[inline]
    #[must_use]
    #[allow(clippy::let_and_return)]
    pub const fn is_empty(&self) -> bool {
        let is_empty = (self.0[0] == 0)
            & (self.0[1] == 0)
            & (self.0[2] == 0)
            & (self.0[3] == 0);

        #[cfg(not(target_pointer_width = "64"))]
        {
            is_empty
                & (self.0[4] == 0)
                & (self.0[5] == 0)
                & (self.0[6] == 0)
                & (self.0[7] == 0)
        }

        #[cfg(target_pointer_width = "64")]
        is_empty
    }

    /// Returns `true` if `self` contains all bytes.
    ///
    /// This is more efficient than checking `self.len() == 256`.
    #[inline]
    #[must_use]
    #[allow(clippy::let_and_return)]
    pub const fn is_full(&self) -> bool {
        let is_full = (self.0[0] == !0)
            & (self.0[1] == !0)
            & (self.0[2] == !0)
            & (self.0[3] == !0);

        #[cfg(not(target_pointer_width = "64"))]
        {
            is_full
                & (self.0[4] == !0)
                & (self.0[5] == !0)
                & (self.0[6] == !0)
                & (self.0[7] == !0)
        }

        #[cfg(target_pointer_width = "64")]
        is_full
    }

    /// Returns the number of bytes contained in `self`.
    #[cfg_attr(target_feature = "popcnt", inline)]
    #[must_use]
    #[allow(clippy::let_and_return)]
    pub const fn len(&self) -> usize {
        let len = (self.0[0].count_ones() as usize)
            + (self.0[1].count_ones() as usize)
            + (self.0[2].count_ones() as usize)
            + (self.0[3].count_ones() as usize);

        #[cfg(not(target_pointer_width = "64"))]
        {
            len + (self.0[4].count_ones() as usize)
                + (self.0[5].count_ones() as usize)
                + (self.0[6].count_ones() as usize)
                + (self.0[7].count_ones() as usize)
        }

        #[cfg(target_pointer_width = "64")]
        len
    }

    /// Removes all bytes from `self`.
    #[inline]
    pub fn clear(&mut self) {
        *self = ByteSet::new();
    }

    /// Returns the first (least) byte in `self`, or `None` if `self` is empty.
    pub fn first(&self) -> Option<u8> {
        for (i, &chunk) in self.0.iter().enumerate() {
            if let Some(lsb) = chunk::lsb(chunk) {
                return Some(lsb + (i * chunk::INDEX_OFFSET) as u8);
            }
        }
        None
    }

    /// Removes the first (least) byte in `self` and returns it, or `None` if
    /// `self` is empty.
    pub fn pop_first(&mut self) -> Option<u8> {
        for (i, chunk) in self.0.iter_mut().enumerate() {
            if let Some(lsb) = chunk::pop_lsb(chunk) {
                return Some(lsb + (i * chunk::INDEX_OFFSET) as u8);
            }
        }
        None
    }

    /// Returns the last (greatest) byte in `self`, or `None` if `self` is
    /// empty.
    pub fn last(&self) -> Option<u8> {
        for (i, &chunk) in self.0.iter().rev().enumerate() {
            if let Some(msb) = chunk::msb(chunk) {
                let i = LAST_SLOT_INDEX - i;
                return Some(msb + (i * chunk::INDEX_OFFSET) as u8);
            }
        }
        None
    }

    /// Removes the last (least) byte in `self` and returns it, or `None` if
    /// `self` is empty.
    pub fn pop_last(&mut self) -> Option<u8> {
        for (i, chunk) in self.0.iter_mut().rev().enumerate() {
            if let Some(msb) = chunk::pop_msb(chunk) {
                let i = LAST_SLOT_INDEX - i;
                return Some(msb + (i * chunk::INDEX_OFFSET) as u8);
            }
        }
        None
    }

    /// Inserts `byte` into `self` in-place.
    ///
    /// Unlike [`HashSet::insert`] and [`BTreeSet::insert`], this does not
    /// return a `bool` for whether `byte` was not present. This is because it's
    /// just as efficient to call [`contains`](#method.contains) before.
    ///
    /// [`HashSet::insert`]:  https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.insert
    /// [`BTreeSet::insert`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.insert
    #[inline]
    pub fn insert(&mut self, byte: u8) {
        let (index, shift) = chunk_index_and_shift(byte);

        self.0[index] |= 1 << shift;
    }

    /// Inserts all bytes of `other` into `self` in-place.
    #[inline]
    pub fn insert_all(&mut self, other: Self) {
        self.0[0] |= other.0[0];
        self.0[1] |= other.0[1];
        self.0[2] |= other.0[2];
        self.0[3] |= other.0[3];

        #[cfg(not(target_pointer_width = "64"))]
        {
            self.0[4] |= other.0[4];
            self.0[5] |= other.0[5];
            self.0[6] |= other.0[6];
            self.0[7] |= other.0[7];
        }
    }

    /// Returns a copy of `self` with `byte` inserted.
    #[inline]
    #[must_use]
    pub const fn inserting(mut self, byte: u8) -> Self {
        let (index, shift) = chunk_index_and_shift(byte);

        self.0[index] |= 1 << shift;
        self
    }

    /// Returns a copy of `self` with all of `other` inserted.
    ///
    /// This is equivalent to the [`union`](#method.union) method.
    #[inline]
    #[must_use]
    pub const fn inserting_all(self, other: Self) -> Self {
        self.union(other)
    }

    /// Removes `byte` from `self` in-place.
    ///
    /// Unlike [`HashSet::remove`] and [`BTreeSet::remove`], this does not
    /// return a `bool` for whether `byte` was present. This is because it's
    /// just as efficient to call [`contains`](#method.contains) before.
    ///
    /// [`HashSet::remove`]:  https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.remove
    /// [`BTreeSet::remove`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html#method.remove
    #[inline]
    pub fn remove(&mut self, byte: u8) {
        let (index, shift) = chunk_index_and_shift(byte);

        self.0[index] &= !(1 << shift);
    }

    /// Removes all bytes of `other` from `self` in-place.
    #[inline]
    pub fn remove_all(&mut self, other: Self) {
        *self &= !other;
    }

    /// Returns a copy of `self` with `byte` removed.
    #[inline]
    #[must_use]
    pub const fn removing(mut self, byte: u8) -> Self {
        let (index, shift) = chunk_index_and_shift(byte);

        self.0[index] &= !(1 << shift);
        self
    }

    /// Returns a copy of `self` with `byte` removed.
    #[inline]
    #[must_use]
    pub const fn removing_all(self, other: Self) -> Self {
        self.difference(other)
    }

    /// Sets `byte` in `self` to `enabled` in-place.
    #[inline]
    pub fn set(&mut self, byte: u8, enabled: bool) {
        let (index, shift) = chunk_index_and_shift(byte);
        let chunk = self.0[index];

        self.0[index] = (chunk & !(1 << shift)) | ((enabled as Chunk) << shift);
    }

    /// Returns a copy of `self` with `byte` set to `enabled`.
    #[inline]
    #[must_use]
    pub const fn setting(mut self, byte: u8, enabled: bool) -> Self {
        let (index, shift) = chunk_index_and_shift(byte);
        let chunk = self.0[index];

        self.0[index] = (chunk & !(1 << shift)) | ((enabled as Chunk) << shift);
        self
    }

    /// Returns `true` if `byte` is contained in `self`.
    #[inline]
    #[must_use]
    pub const fn contains(&self, byte: u8) -> bool {
        let (index, shift) = chunk_index_and_shift(byte);

        self.0[index] & (1 << shift) != 0
    }

    #[inline]
    #[must_use]
    const fn chunk_and_or(&self, other: &Self) -> Chunk {
        map_reduce_chunks!(self, other, &, |)
    }

    /// Returns `true` if `self` contains any bytes in `other`.
    #[inline]
    #[must_use]
    // Not `const` because it may be later improved with SIMD intrinsics.
    pub fn contains_any(&self, other: &Self) -> bool {
        self.chunk_and_or(other) != 0
    }

    #[inline]
    const fn _is_subset(&self, other: &Self) -> bool {
        self.intersection(*other).eq(self)
    }

    /// Returns `true` if `other` contains all bytes in `self`.
    #[inline]
    #[must_use]
    // Not `const` because it may be later improved with SIMD intrinsics.
    pub fn is_subset(&self, other: &Self) -> bool {
        self._is_subset(other)
    }

    /// Returns `true` if `other` contains all bytes in `self` and at least one
    /// other byte not contained in `self`.
    ///
    /// This is also known as a "proper subset".
    #[must_use]
    // Not inlined because lots of code is generated on x86.
    // Not `const` because it may be later improved with SIMD intrinsics.
    pub fn is_strict_subset(&self, other: &Self) -> bool {
        // On x86, checking inequality first produces less code and uses fewer
        // registers.
        self.ne(other) && self.is_subset(other)
    }

    /// Returns `true` if `self` contains all bytes in `other`.
    #[inline]
    #[must_use]
    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    /// Returns `true` if `self` contains all bytes in `other` and at least one
    /// other byte not contained in `other`.
    ///
    /// This is also known as a "proper superset".
    #[inline]
    #[must_use]
    pub fn is_strict_superset(&self, other: &Self) -> bool {
        other.is_strict_subset(self)
    }

    /// Returns `true` if `self` and `other` have no bytes in common.
    #[inline]
    #[must_use]
    // Not `const` because it may be later improved with SIMD intrinsics.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(*other).is_empty()
    }

    /// Returns a set with the bytes contained in `self`, but not in `other`.
    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        self.intersection(other.not())
    }

    /// Returns a set with the bytes contained in `self` or `other`, but not in
    /// both.
    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        map_chunks!(self, ^, other)
    }

    /// Returns a set with the bytes contained both in `self` and `other`.
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        map_chunks!(self, &, other)
    }

    /// Returns a new set with the bytes contained in `self` or `other`.
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        map_chunks!(self, |, other)
    }

    /// Returns a new set with the bytes not contained in `self`.
    ///
    /// This exists because the [`Not`] trait cannot be used in `const` yet.
    ///
    /// [`Not`]: https://doc.rust-lang.org/std/ops/trait.Not.html
    #[inline]
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub const fn not(self) -> Self {
        map_chunks!(self, !)
    }

    /// Returns `self` with its bits reversed.
    ///
    /// This is equivalent to checking for `255 - b` in all subsequent searches
    /// of `b`.
    #[must_use]
    // The `rbit` instruction makes this reasonable to inline.
    #[cfg_attr(target_arch = "aarch64", inline)]
    // Not inlined because lots of code is generated on x86.
    pub const fn reverse_bits(self) -> Self {
        Self([
            #[cfg(not(target_pointer_width = "64"))]
            self.0[7].reverse_bits(),
            #[cfg(not(target_pointer_width = "64"))]
            self.0[6].reverse_bits(),
            #[cfg(not(target_pointer_width = "64"))]
            self.0[5].reverse_bits(),
            #[cfg(not(target_pointer_width = "64"))]
            self.0[4].reverse_bits(),
            self.0[3].reverse_bits(),
            self.0[2].reverse_bits(),
            self.0[1].reverse_bits(),
            self.0[0].reverse_bits(),
        ])
    }

    /// Returns `true` if `self` and `other` contain the same bytes.
    ///
    /// This exists because `PartialEq` is currently unstable in `const`.
    #[inline]
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    #[allow(clippy::let_and_return)]
    pub const fn eq(&self, other: &Self) -> bool {
        let eq = (self.0[0] == other.0[0])
            & (self.0[1] == other.0[1])
            & (self.0[2] == other.0[2])
            & (self.0[3] == other.0[3]);

        #[cfg(not(target_pointer_width = "64"))]
        {
            eq & (self.0[4] == other.0[4])
                & (self.0[5] == other.0[5])
                & (self.0[6] == other.0[6])
                & (self.0[7] == other.0[7])
        }

        #[cfg(target_pointer_width = "64")]
        eq
    }

    /// Returns `true` if `self` and `other` do not contain the same bytes.
    ///
    /// This exists because `PartialEq` is currently unstable in `const`.
    #[inline]
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub const fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

/// Operations related to the ASCII character set.
impl ByteSet {
    /// The set of all ASCII characters: U+0000 NULL ..= U+007F DELETE.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii`] returns `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII {
    ///     assert!(byte.is_ascii());
    /// }
    ///
    /// for byte in !ByteSet::ASCII {
    ///     assert!(!byte.is_ascii());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii
    pub const ASCII: Self = {
        #[cfg(target_pointer_width = "64")]
        {
            Self([!0, !0, 0, 0])
        }

        #[cfg(not(target_pointer_width = "64"))]
        {
            Self([!0, !0, !0, !0, 0, 0, 0, 0])
        }
    };

    /// The set of all ASCII alphabetic characters:
    ///
    /// - U+0041 'A' ..= U+005A 'Z'
    /// - U+0061 'a' ..= U+007A 'z'
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_alphabetic`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_ALPHABETIC {
    ///     assert!(byte.is_ascii_alphabetic());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_ALPHABETIC {
    ///     assert!(!byte.is_ascii_alphabetic());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_alphabetic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
    pub const ASCII_ALPHABETIC: Self =
        Self::ASCII_LOWERCASE.inserting_all(Self::ASCII_UPPERCASE);

    /// The set of all ASCII uppercase characters: U+0041 'A' ..= U+005A 'Z'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_uppercase`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_UPPERCASE {
    ///     assert!(byte.is_ascii_uppercase());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_UPPERCASE {
    ///     assert!(!byte.is_ascii_uppercase());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_uppercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_uppercase
    pub const ASCII_UPPERCASE: Self = byte_set![
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
        b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z',
    ];

    /// The set of all ASCII lowercase characters: U+0061 'a' ..= U+007A 'z'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_lowercase`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_LOWERCASE {
    ///     assert!(byte.is_ascii_lowercase());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_LOWERCASE {
    ///     assert!(!byte.is_ascii_lowercase());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_lowercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_lowercase
    pub const ASCII_LOWERCASE: Self = byte_set![
        b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l',
        b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x',
        b'y', b'z',
    ];

    /// The set of all ASCII alphanumeric characters:
    ///
    /// - U+0041 'A' ..= U+005A 'Z'
    /// - U+0061 'a' ..= U+007A 'z'
    /// - U+0030 '0' ..= U+0039 '9'
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_alphanumeric`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_ALPHANUMERIC {
    ///     assert!(byte.is_ascii_alphanumeric());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_ALPHANUMERIC {
    ///     assert!(!byte.is_ascii_alphanumeric());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_alphanumeric`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphanumeric
    pub const ASCII_ALPHANUMERIC: Self =
        Self::ASCII_ALPHABETIC.inserting_all(Self::ASCII_DIGIT);

    /// The set of all ASCII decimal digits: U+0030 '0' ..= U+0039 '9'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_digit`] returns `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_DIGIT {
    ///     assert!(byte.is_ascii_digit());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_DIGIT {
    ///     assert!(!byte.is_ascii_digit());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_digit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_digit
    pub const ASCII_DIGIT: Self =
        byte_set![b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];

    /// The set of all ASCII hexadecimal digits:
    ///
    /// - U+0030 '0' ..= U+0039 '9'
    /// - U+0041 'A' ..= U+0046 'F'
    /// - U+0061 'a' ..= U+0066 'f'
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_hexdigit`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_HEXDIGIT {
    ///     assert!(byte.is_ascii_hexdigit());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_HEXDIGIT {
    ///     assert!(!byte.is_ascii_hexdigit());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_hexdigit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_hexdigit
    pub const ASCII_HEXDIGIT: Self = Self::ASCII_DIGIT
        .inserting_all(byte_set![b'A', b'B', b'C', b'D', b'E', b'F'])
        .inserting_all(byte_set![b'a', b'b', b'c', b'd', b'e', b'f']);

    /// The set of all ASCII punctuation characters:
    ///
    /// - U+0021 ..= U+002F `! " # $ % & ' ( ) * + , - . /`
    /// - U+003A ..= U+0040 `: ; < = > ? @`
    /// - U+005B ..= U+0060 ``[ \ ] ^ _ ` ``
    /// - U+007B ..= U+007E `{ | } ~`
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_punctuation`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_PUNCTUATION {
    ///     assert!(byte.is_ascii_punctuation());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_PUNCTUATION {
    ///     assert!(!byte.is_ascii_punctuation());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_punctuation`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_punctuation
    pub const ASCII_PUNCTUATION: Self = byte_set![
        b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+',
        b',', b'-', b'.', b'/', b':', b';', b'<', b'=', b'>', b'?', b'@', b'[',
        b'\\', b']', b'^', b'_', b'`', b'{', b'|', b'}', b'~',
    ];

    /// The set of all ASCII graphic characters: U+0021 '!' ..= U+007E '~'.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_graphic`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_GRAPHIC {
    ///     assert!(byte.is_ascii_graphic());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_GRAPHIC {
    ///     assert!(!byte.is_ascii_graphic());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_graphic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_graphic
    pub const ASCII_GRAPHIC: Self =
        Self::ASCII_ALPHANUMERIC.inserting_all(Self::ASCII_PUNCTUATION);

    /// The set of all ASCII whitespace characters:
    ///
    /// - U+0020 SPACE
    /// - U+0009 HORIZONTAL TAB
    /// - U+000A LINE FEED
    /// - U+000C FORM FEED
    /// - U+000D CARRIAGE RETURN
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_whitespace`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_WHITESPACE {
    ///     assert!(byte.is_ascii_whitespace());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_WHITESPACE {
    ///     assert!(!byte.is_ascii_whitespace());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_whitespace`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
    pub const ASCII_WHITESPACE: Self =
        byte_set![b'\t', b'\n', 0x0C, b'\r', b' '];

    /// The set of all ASCII control characters:
    ///
    /// - U+0000 NUL ..= U+001F UNIT SEPARATOR
    /// - U+007F DELETE.
    ///
    /// Note that most ASCII whitespace characters are control characters, but
    /// SPACE is not.
    ///
    /// # Examples
    ///
    /// This contains all bytes for which [`u8::is_ascii_control`] returns
    /// `true`:
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// for byte in ByteSet::ASCII_CONTROL {
    ///     assert!(byte.is_ascii_control());
    /// }
    ///
    /// for byte in !ByteSet::ASCII_CONTROL {
    ///     assert!(!byte.is_ascii_control());
    /// }
    /// ```
    ///
    /// [`u8::is_ascii_whitespace`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
    pub const ASCII_CONTROL: Self = byte_set![
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, b'\t', b'\n',
        0x0B, 0x0C, b'\r', 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15,
        0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x7F,
    ];

    /// Returns `true` if [`u8::is_ascii`] returns `true` for all bytes in
    /// `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii`]:
    /// https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii
    #[inline]
    #[must_use]
    pub const fn is_ascii(&self) -> bool {
        self._is_subset(&Self::ASCII)
    }

    /// Returns `true` if [`u8::is_ascii_alphabetic`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_alphabetic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphabetic
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphabetic(&self) -> bool {
        self._is_subset(&Self::ASCII_ALPHABETIC)
    }

    /// Returns `true` if [`u8::is_ascii_uppercase`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_uppercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_uppercase
    #[inline]
    #[must_use]
    pub const fn is_ascii_uppercase(&self) -> bool {
        self._is_subset(&Self::ASCII_UPPERCASE)
    }

    /// Returns `true` if [`u8::is_ascii_lowercase`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_lowercase`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_lowercase
    #[inline]
    #[must_use]
    pub const fn is_ascii_lowercase(&self) -> bool {
        self._is_subset(&Self::ASCII_LOWERCASE)
    }

    /// Returns `true` if [`u8::is_ascii_alphanumeric`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_alphanumeric`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_alphanumeric
    #[inline]
    #[must_use]
    pub const fn is_ascii_alphanumeric(&self) -> bool {
        self._is_subset(&Self::ASCII_ALPHANUMERIC)
    }

    /// Returns `true` if [`u8::is_ascii_digit`] returns `true` for all bytes in
    /// `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_digit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_digit
    #[inline]
    #[must_use]
    pub const fn is_ascii_digit(&self) -> bool {
        self._is_subset(&Self::ASCII_DIGIT)
    }

    /// Returns `true` if [`u8::is_ascii_hexdigit`] returns `true` for all bytes
    /// in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_hexdigit`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_hexdigit
    #[inline]
    #[must_use]
    pub const fn is_ascii_hexdigit(&self) -> bool {
        self._is_subset(&Self::ASCII_HEXDIGIT)
    }

    /// Returns `true` if [`u8::is_ascii_punctuation`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_punctuation`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_punctuation
    #[inline]
    #[must_use]
    pub const fn is_ascii_punctuation(&self) -> bool {
        self._is_subset(&Self::ASCII_PUNCTUATION)
    }

    /// Returns `true` if [`u8::is_ascii_graphic`] returns `true` for all bytes
    /// in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_graphic`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_graphic
    #[inline]
    #[must_use]
    pub const fn is_ascii_graphic(&self) -> bool {
        self._is_subset(&Self::ASCII_GRAPHIC)
    }

    /// Returns `true` if [`u8::is_ascii_whitespace`] returns `true` for all
    /// bytes in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_whitespace`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_whitespace
    #[inline]
    #[must_use]
    pub const fn is_ascii_whitespace(&self) -> bool {
        self._is_subset(&Self::ASCII_WHITESPACE)
    }

    /// Returns `true` if [`u8::is_ascii_control`] returns `true` for all bytes
    /// in `self`.
    ///
    /// This is significantly more efficient than checking each byte in `self`
    /// individually.
    ///
    /// [`u8::is_ascii_control`]: https://doc.rust-lang.org/std/primitive.u8.html#method.is_ascii_control
    #[inline]
    #[must_use]
    pub const fn is_ascii_control(&self) -> bool {
        self._is_subset(&Self::ASCII_CONTROL)
    }
}

/// Operations over the internal memory representation.
impl ByteSet {
    const SIZE: usize = mem::size_of::<Self>();

    /// Returns a shared reference to the underlying bytes of `self`.
    #[inline]
    pub fn as_bytes(&self) -> &[u8; Self::SIZE] {
        unsafe { &*self.0.as_ptr().cast() }
    }

    /// Returns a mutable reference to the underlying bytes of `self`.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8; Self::SIZE] {
        unsafe { &mut *self.0.as_mut_ptr().cast() }
    }

    /// Returns a shared reference to the underlying bytes of `slice`.
    #[inline]
    pub fn slice_as_bytes(slice: &[Self]) -> &[u8] {
        let ptr = slice.as_ptr().cast::<u8>();
        let len = slice.len() * Self::SIZE;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Returns a mutable reference to the underlying bytes of `slice`.
    #[inline]
    pub fn slice_as_bytes_mut(slice: &mut [Self]) -> &mut [u8] {
        let ptr = slice.as_mut_ptr().cast::<u8>();
        let len = slice.len() * Self::SIZE;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }
}

impl Default for ByteSet {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for ByteSet {
    #[inline]
    fn from(byte: u8) -> ByteSet {
        byte_set![byte]
    }
}

impl From<&[u8]> for ByteSet {
    #[inline]
    fn from(bytes: &[u8]) -> Self {
        let mut set = ByteSet::new();
        set.extend(bytes);
        set
    }
}

impl From<&str> for ByteSet {
    #[inline]
    fn from(s: &str) -> Self {
        s.as_bytes().into()
    }
}

impl Extend<u8> for ByteSet {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        iter.into_iter().for_each(|byte| self.insert(byte));
    }
}

impl<'a> Extend<&'a u8> for ByteSet {
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        self.extend(iter.into_iter().cloned());
    }
}

impl FromIterator<u8> for ByteSet {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        // Make sure to use `insert` over `inserting` to not copy so many bytes
        // on each iteration.
        let mut set = ByteSet::new();
        set.extend(iter);
        set
    }
}

impl<'a> FromIterator<&'a u8> for ByteSet {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
        iter.into_iter().cloned().collect()
    }
}

impl IntoIterator for ByteSet {
    type Item = u8;
    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl fmt::Debug for ByteSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_set().entries(*self).finish()
    }
}

// This is implemented manually over `[u8; 32]` because it seems to produce
// better code than `[usize; N]`.
impl PartialOrd for ByteSet {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.as_bytes().lt(other.as_bytes())
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.as_bytes().le(other.as_bytes())
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.as_bytes().gt(other.as_bytes())
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.as_bytes().ge(other.as_bytes())
    }
}

impl Ord for ByteSet {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for ByteSet {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state)
    }

    #[inline]
    fn hash_slice<H: hash::Hasher>(data: &[Self], state: &mut H) {
        Self::slice_as_bytes(data).hash(state)
    }
}

impl ops::Sub for ByteSet {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.removing_all(rhs)
    }
}

impl ops::SubAssign for ByteSet {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.remove_all(rhs);
    }
}

impl ops::BitAnd for ByteSet {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl ops::BitAndAssign for ByteSet {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl ops::BitOr for ByteSet {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        self.inserting_all(rhs)
    }
}

impl ops::BitOrAssign for ByteSet {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.insert_all(rhs);
    }
}

impl ops::BitXor for ByteSet {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl ops::BitXorAssign for ByteSet {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl ops::Not for ByteSet {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        ByteSet::not(self)
    }
}

// Enables `rand::random::<ByteSet>()`.
#[cfg(any(test, feature = "rand"))]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distributions::Distribution<ByteSet>
    for rand::distributions::Standard
{
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ByteSet {
        ByteSet::rand(rng)
    }
}
