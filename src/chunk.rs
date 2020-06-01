//! Operations over chunks (`usize`).

use core::mem;

#[cfg(not(byte_set_chunk_64))]
pub type Chunk = u32;

#[cfg(byte_set_chunk_64)]
pub type Chunk = u64;

const SLOT_NUM_BITS: usize = mem::size_of::<Chunk>() * 8;

/// Multiplied to get the byte offset for a given chunk index.
pub const INDEX_OFFSET: usize = SLOT_NUM_BITS;

/// Returns the first (least significant) bit of `chunk`, or `None` if `chunk`
/// is 0.
#[inline]
pub fn lsb(chunk: Chunk) -> Option<u8> {
    if chunk == 0 {
        None
    } else {
        Some(chunk.trailing_zeros() as u8)
    }
}

/// Returns the last (most significant) bit of `chunk`, or `None` if `chunk` is
/// 0.
#[inline]
pub fn msb(chunk: Chunk) -> Option<u8> {
    if chunk == 0 {
        None
    } else {
        let bits = SLOT_NUM_BITS - 1;
        Some((bits as u8) ^ chunk.leading_zeros() as u8)
    }
}

/// Removes the first (least significant) bit from `chunk` and returns it, or
/// `None` if `chunk` is 0.
#[inline]
pub fn pop_lsb(chunk: &mut Chunk) -> Option<u8> {
    let lsb = lsb(*chunk)?;
    *chunk ^= 1 << lsb;
    Some(lsb)
}

/// Removes the last (most significant) bit from `chunk` and returns it, or
/// `None` if `chunk` is 0.
#[inline]
pub fn pop_msb(chunk: &mut Chunk) -> Option<u8> {
    let msb = msb(*chunk)?;
    *chunk ^= 1 << msb;
    Some(msb)
}
