use std::convert::TryFrom;

pub mod bool256;
pub mod hash;
pub mod rand;

pub use self::{bool256::Bool256, rand::Rand};

/// Input sizes for benchmarks.
pub const SIZES: &[usize] = &[0, 8, 16, 32, 64, 128, 192, 256];

/// A type that can be casted to `T`.
pub trait Cast<T> {
    /// Casts `self` to `T`, saturating at the max value.
    fn saturating_cast(self) -> T;
}

impl Cast<u8> for usize {
    fn saturating_cast(self) -> u8 {
        u8::try_from(self).unwrap_or(u8::max_value())
    }
}

/// Casts `value` to `T`, saturating at the max value.
pub fn saturating_cast<T: Cast<U>, U>(value: T) -> U {
    value.saturating_cast()
}

/// Finds and removes `item` from `vec`.
///
/// This is a stable implementation of the nightly-only `Vec::remove_item`.
pub fn vec_remove_item<T>(vec: &mut Vec<T>, item: &T) -> Option<T>
where
    T: PartialEq,
{
    let pos = vec.iter().position(|x| *x == *item)?;
    Some(vec.remove(pos))
}

/// Finds and removes `item` from `vec`, using binary search.
pub fn vec_remove_item_binary_search<T>(vec: &mut Vec<T>, item: &T) -> Option<T>
where
    T: Ord,
{
    let pos = vec.binary_search(item).ok()?;
    Some(vec.remove(pos))
}
