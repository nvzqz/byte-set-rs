use super::ByteSet;
use crate::Iter;
use core::{cmp, fmt, hash, iter::FromIterator, ops};

#[cfg(any(test, feature = "std"))]
use std::collections::HashSet;

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;
#[cfg(any(test, feature = "alloc"))]
use alloc::collections::BTreeSet;

impl Default for ByteSet {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for ByteSet {
    #[inline]
    fn from(byte: u8) -> ByteSet {
        ByteSet::from_byte(byte)
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

impl From<&mut [u8]> for ByteSet {
    #[inline]
    fn from(bytes: &mut [u8]) -> Self {
        (bytes as &[u8]).into()
    }
}

impl From<&str> for ByteSet {
    #[inline]
    fn from(s: &str) -> Self {
        s.as_bytes().into()
    }
}

impl From<&mut str> for ByteSet {
    #[inline]
    fn from(s: &mut str) -> Self {
        (s as &str).into()
    }
}

impl From<ops::Range<u8>> for ByteSet {
    #[inline]
    fn from(range: ops::Range<u8>) -> Self {
        Self::from_range(range)
    }
}

impl From<ops::RangeTo<u8>> for ByteSet {
    #[inline]
    fn from(range: ops::RangeTo<u8>) -> Self {
        Self::from_range_to(range)
    }
}

impl From<ops::RangeFrom<u8>> for ByteSet {
    #[inline]
    fn from(range: ops::RangeFrom<u8>) -> Self {
        Self::from_range_from(range)
    }
}

impl From<ops::RangeInclusive<u8>> for ByteSet {
    #[inline]
    fn from(range: ops::RangeInclusive<u8>) -> Self {
        Self::from_range_inclusive(range)
    }
}

impl From<ops::RangeToInclusive<u8>> for ByteSet {
    #[inline]
    fn from(range: ops::RangeToInclusive<u8>) -> Self {
        Self::from_range_to_inclusive(range)
    }
}

impl From<ops::RangeFull> for ByteSet {
    #[inline]
    fn from(_: ops::RangeFull) -> Self {
        Self::full()
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

impl PartialOrd for ByteSet {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ByteSet {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // TODO: Optimize using internal representation while keeping semantics.
        // See https://github.com/nvzqz/byte-set-rs/issues/9.
        self.into_iter().cmp(other.into_iter())
    }
}

#[allow(clippy::derive_hash_xor_eq)]
impl hash::Hash for ByteSet {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_raw_bytes().hash(state)
    }

    #[inline]
    fn hash_slice<H: hash::Hasher>(data: &[Self], state: &mut H) {
        Self::slice_as_raw_bytes(data).hash(state)
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

#[cfg(any(test, feature = "std"))]
impl<S> PartialEq<HashSet<u8, S>> for ByteSet {
    fn eq(&self, other: &HashSet<u8, S>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        // Using `ByteSet::contains` instead of zipping the iterators because
        // it's much cheaper than iterating `ByteSet`.
        //
        // Although iterating over `HashSet` is slightly slower than `ByteSet`,
        // `HashSet::contains` is significantly slower.
        other.iter().all(|&byte| self.contains(byte))
    }
}

#[cfg(any(test, feature = "alloc"))]
impl PartialEq<BTreeSet<u8>> for ByteSet {
    fn eq(&self, other: &BTreeSet<u8>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        // Using `ByteSet::contains` instead of zipping the iterators because
        // it's much cheaper than iterating `ByteSet`.
        //
        // Although iterating over `BTreeSet` is slightly slower than `ByteSet`,
        // `BTreeSet::contains` is significantly slower.
        other.iter().all(|&byte| self.contains(byte))
    }
}

#[cfg(any(test, feature = "alloc"))]
impl PartialOrd<BTreeSet<u8>> for ByteSet {
    fn partial_cmp(&self, other: &BTreeSet<u8>) -> Option<cmp::Ordering> {
        Some(self.into_iter().cmp(other.iter().cloned()))
    }
}

// Enables `rand::random::<ByteSet>()`.
#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distributions::Distribution<ByteSet>
    for rand::distributions::Standard
{
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ByteSet {
        ByteSet::rand(rng)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl serde::Serialize for ByteSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for byte in *self {
            seq.serialize_element(&byte)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
impl<'de> serde::Deserialize<'de> for ByteSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ByteSetVisitor;

        impl<'de> serde::de::Visitor<'de> for ByteSetVisitor {
            type Value = ByteSet;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a set of bytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v.into())
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut set = ByteSet::new();
                while let Some(byte) = seq.next_element::<u8>()? {
                    set.insert(byte);
                }
                Ok(set)
            }
        }

        deserializer.deserialize_seq(ByteSetVisitor)
    }
}
