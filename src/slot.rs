//! Operations over slots (`usize`).

use core::mem;

const SLOT_NUM_BITS: usize = mem::size_of::<usize>() * 8;

/// Multiplied to get the byte offset for a given slot index.
pub const INDEX_OFFSET: usize = SLOT_NUM_BITS;

/// Returns the first (least significant) bit of `slot`, or `None` if `slot` is
/// 0.
#[inline]
pub fn lsb(slot: usize) -> Option<u8> {
    if slot == 0 {
        None
    } else {
        Some(slot.trailing_zeros() as u8)
    }
}

/// Returns the last (most significant) bit of `slot`, or `None` if `slot` is 0.
#[inline]
pub fn msb(slot: usize) -> Option<u8> {
    if slot == 0 {
        None
    } else {
        let bits = SLOT_NUM_BITS - 1;
        Some((bits as u8) ^ slot.leading_zeros() as u8)
    }
}

/// Removes the first (least significant) bit from `slot` and returns it, or
/// `None` if `slot` is 0.
#[inline]
pub fn pop_lsb(slot: &mut usize) -> Option<u8> {
    let lsb = lsb(*slot)?;
    *slot ^= 1 << lsb;
    Some(lsb)
}

/// Removes the last (most significant) bit from `slot` and returns it, or
/// `None` if `slot` is 0.
#[inline]
pub fn pop_msb(slot: &mut usize) -> Option<u8> {
    let msb = msb(*slot)?;
    *slot ^= 1 << msb;
    Some(msb)
}
