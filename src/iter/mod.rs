use super::{ByteSet, NUM_SLOTS};
use core::{iter, mem};

#[cfg(test)]
mod tests;

const LAST_SLOT_INDEX: usize = NUM_SLOTS - 1;

/// An iterator over a [`ByteSet`].
///
/// [`ByteSet`]: struct.ByteSet.html
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Iter {
    /// The set being iterated over. It is mutated in-place as bits are popped
    /// from each slot.
    byte_set: ByteSet,

    /// The current slot index when iterating forwards.
    forward_index: usize,

    /// The current slot index when iterating backwards.
    backward_index: usize,
}

// Multiplied to get the byte offset for a given slot index.
const INDEX_OFFSET: usize = mem::size_of::<usize>() * 8;

/// Removes the least significant bit from `slot` and returns it, or `None` if
/// `slot` is 0.
#[inline]
fn pop_lsb(slot: &mut usize) -> Option<u8> {
    if *slot == 0 {
        return None;
    }

    let lsb = slot.trailing_zeros() as u8;
    *slot ^= 1 << lsb;

    Some(lsb)
}

/// Removes the most significant bit from `slot` and returns it, or `None` if
/// `slot` is 0.
#[inline]
fn pop_msb(slot: &mut usize) -> Option<u8> {
    if *slot == 0 {
        return None;
    }

    let bits = mem::size_of::<usize>() * 8 - 1;
    let msb = bits ^ slot.leading_zeros() as usize;
    *slot ^= 1 << msb;

    Some(msb as u8)
}

impl Iter {
    #[inline]
    pub(crate) const fn new(byte_set: ByteSet) -> Self {
        Self {
            byte_set,
            forward_index: 0,
            backward_index: LAST_SLOT_INDEX,
        }
    }

    /// Returns the underlying [`ByteSet`].
    ///
    /// Note that iteration mutates the byteset in-place.
    #[inline]
    pub const fn into_byte_set(self) -> ByteSet {
        self.byte_set
    }
}

impl From<ByteSet> for Iter {
    #[inline]
    fn from(byte_set: ByteSet) -> Self {
        Self::new(byte_set)
    }
}

impl Iterator for Iter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let range = self.forward_index..NUM_SLOTS;

        for index in range {
            self.forward_index = index;

            let slot = &mut self.byte_set.0[index];

            if let Some(lsb) = pop_lsb(slot) {
                return Some(lsb + (index * INDEX_OFFSET) as u8);
            }
        }

        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn last(mut self) -> Option<u8> {
        self.next_back()
    }

    #[inline]
    fn min(mut self) -> Option<u8> {
        self.next()
    }

    #[inline]
    fn max(self) -> Option<u8> {
        self.last()
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<u8> {
        // `Range` (`a..b`) is faster than `InclusiveRange` (`a..=b`).
        let range = 0..(self.backward_index + 1);

        for index in range.rev() {
            self.backward_index = index;

            // SAFETY: This invariant is tested.
            let slot = unsafe { self.byte_set.0.get_unchecked_mut(index) };

            if let Some(msb) = pop_msb(slot) {
                return Some(msb + (index * INDEX_OFFSET) as u8);
            }
        }

        None
    }
}

impl ExactSizeIterator for Iter {
    #[inline]
    fn len(&self) -> usize {
        self.byte_set.len()
    }
}

// `Iter` does not produce more values after `None` is reached.
impl iter::FusedIterator for Iter {}
