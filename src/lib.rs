//! Efficient sets of bytes.

#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

use core::{cmp, hash, iter::FromIterator, mem, ops, slice};

#[macro_use]
mod macros;

#[cfg(test)]
#[macro_use]
mod tests_macros;

#[cfg(test)]
mod tests;

mod iter;
pub use iter::Iter;

const SLOT_SIZE: usize = mem::size_of::<usize>();

const NUM_SLOTS: usize = 256 / 8 / SLOT_SIZE;

/// An efficient, general-purpose set of [`u8`]s.
///
/// # Implementation
///
/// This is a 256-bit mask where a byte is contained based on whether its bit is
/// enabled. The first (least significant) bit in the mask represents the first
/// byte in the set. Likewise, the last last (most significant) bit represents
/// the last byte.
///
/// The mask is composed a of [`usize`] "slot" array, and as a result this type
/// has the same alignment as [`usize`]. This will *never* change. The type is
/// even marked as [`#[repr(transparent)]`][transparent], so its ABI is the same
/// as that of `[usize; 4]`/`[usize; 8]` on 64/32-bit platforms respectively.
///
/// [`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
/// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
/// [transparent]: https://github.com/rust-lang/rfcs/blob/master/text/1758-repr-transparent.md
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ByteSet([usize; NUM_SLOTS]);

/// Returns the slot index for `byte` and the bit shift for that slot.
#[inline]
const fn slot_index_and_shift(byte: u8) -> (usize, usize) {
    let byte = byte as usize;

    #[cfg(target_pointer_width = "64")]
    let index = byte >> 6;
    #[cfg(target_pointer_width = "64")]
    let shift = byte & 0b0011_1111;

    #[cfg(target_pointer_width = "32")]
    let index = byte >> 5;
    #[cfg(target_pointer_width = "32")]
    let shift = byte & 0b0001_1111;

    (index, shift)
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
        Self([usize::max_value(); NUM_SLOTS])
    }

    /// Returns `true` if `self` contains no bytes.
    ///
    /// This is more efficient than checking `self.len() == 0`.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        let is_empty = (self.0[0] == 0)
            & (self.0[1] == 0)
            & (self.0[2] == 0)
            & (self.0[3] == 0);

        #[cfg(target_pointer_width = "32")]
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

    /// Returns the number of bytes contained in `self`.
    #[cfg_attr(target_feature = "popcnt", inline)]
    #[must_use]
    pub const fn len(&self) -> usize {
        let len = (self.0[0].count_ones() as usize)
            + (self.0[1].count_ones() as usize)
            + (self.0[2].count_ones() as usize)
            + (self.0[3].count_ones() as usize);

        #[cfg(target_pointer_width = "32")]
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
        let (index, shift) = slot_index_and_shift(byte);

        self.0[index] |= 1 << shift;
    }

    /// Inserts all bytes of `other` into `self` in-place.
    #[inline]
    pub fn insert_all(&mut self, other: Self) {
        self.0[0] |= other.0[0];
        self.0[1] |= other.0[1];
        self.0[2] |= other.0[2];
        self.0[3] |= other.0[3];

        #[cfg(target_pointer_width = "32")]
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
        let (index, shift) = slot_index_and_shift(byte);

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
        let (index, shift) = slot_index_and_shift(byte);

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
        let (index, shift) = slot_index_and_shift(byte);

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
        let (index, shift) = slot_index_and_shift(byte);
        let slot = self.0[index];

        self.0[index] = (slot & !(1 << shift)) | ((enabled as usize) << shift);
    }

    /// Returns a copy of `self` with `byte` set to `enabled`.
    #[inline]
    #[must_use]
    pub const fn setting(mut self, byte: u8, enabled: bool) -> Self {
        let (index, shift) = slot_index_and_shift(byte);
        let slot = self.0[index];

        self.0[index] = (slot & !(1 << shift)) | ((enabled as usize) << shift);
        self
    }

    /// Returns `true` if `byte` is contained in `self`.
    #[inline]
    #[must_use]
    pub const fn contains(&self, byte: u8) -> bool {
        let (index, shift) = slot_index_and_shift(byte);

        self.0[index] & (1 << shift) != 0
    }

    #[inline]
    #[must_use]
    const fn slot_and_or(&self, other: &Self) -> usize {
        map_reduce_slots!(self, other, &, |)
    }

    /// Returns `true` if `self` contains any bytes in `other`.
    #[inline]
    #[must_use]
    // Not `const` because it may be later improved with SIMD intrinsics.
    pub fn contains_any(&self, other: &Self) -> bool {
        self.slot_and_or(other) != 0
    }

    /// Returns `true` if `other` contains all bytes in `self`.
    #[inline]
    #[must_use]
    // Not `const` because it may be later improved with SIMD intrinsics.
    pub fn is_subset(&self, other: &Self) -> bool {
        self.intersection(*other).eq(self)
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
        map_slots!(self, ^, other)
    }

    /// Returns a set with the bytes contained both in `self` and `other`.
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        map_slots!(self, &, other)
    }

    /// Returns a new set with the bytes contained in `self` or `other`.
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        map_slots!(self, |, other)
    }

    /// Returns a new set with the bytes not contained in `self`.
    ///
    /// This exists because the [`Not`] trait cannot be used in `const` yet.
    ///
    /// [`Not`]: https://doc.rust-lang.org/std/ops/trait.Not.html
    #[inline]
    #[must_use]
    pub const fn not(self) -> Self {
        map_slots!(self, !)
    }

    /// Returns `self` with its bits reversed.
    ///
    /// This is equivalent to checking for `255 - b` in all subsequent searches
    /// of `b`.
    #[must_use]
    // Not inlined because lots of code is generated on x86.
    pub const fn reverse_bits(self) -> Self {
        Self([
            #[cfg(target_pointer_width = "32")]
            self.0[7].reverse_bits(),
            #[cfg(target_pointer_width = "32")]
            self.0[6].reverse_bits(),
            #[cfg(target_pointer_width = "32")]
            self.0[5].reverse_bits(),
            #[cfg(target_pointer_width = "32")]
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
    pub const fn eq(&self, other: &Self) -> bool {
        let eq = (self.0[0] == other.0[0])
            & (self.0[1] == other.0[1])
            & (self.0[2] == other.0[2])
            & (self.0[3] == other.0[3]);

        #[cfg(target_pointer_width = "32")]
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
    pub const fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
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

impl Extend<u8> for ByteSet {
    #[inline]
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        iter.into_iter().for_each(|byte| self.insert(byte));
    }
}

impl<'a> Extend<&'a u8> for ByteSet {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        self.extend(iter.into_iter().cloned());
    }
}

impl FromIterator<u8> for ByteSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        iter.into_iter().fold(Self::new(), Self::inserting)
    }
}

impl<'a> FromIterator<&'a u8> for ByteSet {
    #[inline]
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