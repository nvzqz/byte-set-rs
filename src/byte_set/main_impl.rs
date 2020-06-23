use super::{chunk_index_and_shift, ByteSet, Chunk};
use crate::chunk;
use core::ops;

// Makes `ByteSet::{rand,try_rand}` simpler to express.
#[cfg(feature = "rand")]
use rand as rand_core;

impl ByteSet {
    /// Returns a set containing no bytes.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self([0; Self::NUM_SLOTS])
    }

    /// Returns a set containing all bytes (0-255).
    #[inline]
    #[must_use]
    pub const fn full() -> Self {
        Self([Chunk::max_value(); Self::NUM_SLOTS])
    }

    /// Returns a set containing only `byte`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use byte_set::ByteSet;
    /// let byte = 42;
    /// let set = ByteSet::from_byte(byte);
    ///
    /// assert_eq!(set.first(), Some(byte));
    /// assert_eq!(set.last(), Some(byte));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_byte(byte: u8) -> Self {
        byte_set![byte]
    }

    /// Construct a ByteSet from a `RangeTo` value, i.e. `..x`
    #[inline]
    #[must_use]
    pub const fn from_range_to(range: ops::RangeTo<u8>) -> Self {
        const fn chunk_for(
            this_chunk: usize,
            byte_chunk: usize,
            shift: usize,
        ) -> Chunk {
            // the following code is equivalent to
            // if this_chunk == byte_chunk {
            //     value
            // } else if this_chunk < byte_chunk {
            //     Chunk::max_value()
            // } else {
            //     0
            // }
            //
            // Once `if` works in const, this can be cleaned up
            // https://github.com/rust-lang/rust/pull/72437
            let value: Chunk = (1 << shift) - 1;
            let is_equal = (this_chunk == byte_chunk) as usize;
            let is_less_than = (this_chunk < byte_chunk) as usize;
            let if_unequal = [0, Chunk::max_value()][is_less_than];

            [if_unequal, value][is_equal]
        }
        let (index, shift) = chunk_index_and_shift(range.end);
        #[cfg(target_pointer_width = "64")]
        let array = [
            chunk_for(0, index, shift),
            chunk_for(1, index, shift),
            chunk_for(2, index, shift),
            chunk_for(3, index, shift),
        ];
        #[cfg(not(target_pointer_width = "64"))]
        let array = [
            chunk_for(0, index, shift),
            chunk_for(1, index, shift),
            chunk_for(2, index, shift),
            chunk_for(3, index, shift),
            chunk_for(4, index, shift),
            chunk_for(5, index, shift),
            chunk_for(6, index, shift),
            chunk_for(7, index, shift),
        ];
        ByteSet(array)
    }

    /// Construct a ByteSet from a `RangeToInclusive` value, i.e. `..=x`
    #[inline]
    #[must_use]
    pub const fn from_range_to_inclusive(
        range: ops::RangeToInclusive<u8>,
    ) -> Self {
        [
            Self::full(),
            Self::from_range_to(..(range.end.wrapping_add(1))),
        ][(range.end != 255) as usize]
    }

    /// Construct a ByteSet from a `RangeFrom` value, i.e. `x..`
    #[inline]
    #[must_use]
    pub const fn from_range_from(range: ops::RangeFrom<u8>) -> Self {
        Self::from_range_to(..range.start).not()
    }

    /// Construct a ByteSet from a `RangeToInclusive` value, i.e. `x..y`
    #[inline]
    #[must_use]
    pub const fn from_range(range: ops::Range<u8>) -> Self {
        Self::from_range_from(range.start..)
            .intersection(Self::from_range_to(..range.end))
    }

    /// Construct a ByteSet from a `RangeInclusive` value, i.e. `x..=y`
    #[inline]
    #[must_use]
    pub const fn from_range_inclusive(range: ops::RangeInclusive<u8>) -> Self {
        Self::from_range_from(*range.start()..)
            .intersection(Self::from_range_to_inclusive(..=*range.end()))
    }

    /// Returns a set containing uniformly-distributed random bytes from `rng`.
    ///
    /// This uses [`fill_bytes`] under the hood.
    ///
    /// [`fill_bytes`]: https://docs.rs/rand_core/0.5.*/rand_core/trait.RngCore.html#tymethod.fill_bytes
    #[cfg(any(feature = "rand", feature = "rand_core"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "rand", feature = "rand_core"))))]
    #[inline]
    pub fn rand<R: rand_core::RngCore>(mut rng: R) -> Self {
        let mut set = Self::new();
        rng.fill_bytes(set.as_raw_bytes_mut());
        set
    }

    /// Returns a set containing uniformly-distributed random bytes from `rng`,
    /// or `Err` if `rng` failed.
    ///
    /// This uses [`try_fill_bytes`] under the hood.
    ///
    /// [`try_fill_bytes`]: https://docs.rs/rand_core/0.5.*/rand_core/trait.RngCore.html#tymethod.try_fill_bytes
    #[cfg(any(feature = "rand", feature = "rand_core"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "rand", feature = "rand_core"))))]
    #[inline]
    pub fn try_rand<R: rand_core::RngCore>(
        mut rng: R,
    ) -> Result<Self, rand_core::Error> {
        let mut set = Self::new();
        rng.try_fill_bytes(set.as_raw_bytes_mut())?;
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
                let i = Self::LAST_SLOT_INDEX - i;
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
                let i = Self::LAST_SLOT_INDEX - i;
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
    pub(crate) const fn _is_subset(&self, other: &Self) -> bool {
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
