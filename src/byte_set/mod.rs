use crate::Chunk;
use core::mem;

// These modules must appear in this order to make documentation easier to read.
// The space between ensures rustfmt does not reorder them.
mod main_impl;

mod ascii;

mod raw;

mod traits;

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
#[repr(C)]
pub struct ByteSet(pub(crate) [Chunk; Self::NUM_SLOTS]);

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

impl ByteSet {
    pub(crate) const SLOT_SIZE: usize = mem::size_of::<Chunk>();

    pub(crate) const NUM_SLOTS: usize = 256 / 8 / Self::SLOT_SIZE;

    pub(crate) const LAST_SLOT_INDEX: usize = Self::NUM_SLOTS - 1;
}

#[cfg(test)]
impl ByteSet {
    /// Returns a formatting proxy for the binary representation of `self`.
    ///
    /// `fmt::Binary` is not currently implemented for `ByteSet` because of the
    /// extra work to support formatting options.
    pub(crate) fn fmt_binary<'a>(&'a self) -> impl core::fmt::Display + 'a {
        struct Formatted<'a>(&'a ByteSet);

        impl core::fmt::Display for Formatted<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
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
