use std::convert::TryFrom;

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
