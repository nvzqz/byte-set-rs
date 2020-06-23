use super::ByteSet;
use core::{mem, slice};

/// Operations over the internal memory representation.
///
/// There are currently no stability guarantees over the internal bytes. This is
/// being tracked in [#8](https://github.com/nvzqz/byte-set-rs/issues/8).
impl ByteSet {
    const SIZE: usize = mem::size_of::<Self>();

    /// Returns the underlying bytes of `self`.
    #[inline]
    pub fn into_raw_bytes(self) -> [u8; Self::SIZE] {
        unsafe { mem::transmute(self) }
    }

    /// Returns a shared reference to the underlying bytes of `self`.
    #[inline]
    pub fn as_raw_bytes(&self) -> &[u8; Self::SIZE] {
        unsafe { &*self.0.as_ptr().cast() }
    }

    /// Returns a mutable reference to the underlying bytes of `self`.
    #[inline]
    pub fn as_raw_bytes_mut(&mut self) -> &mut [u8; Self::SIZE] {
        unsafe { &mut *self.0.as_mut_ptr().cast() }
    }

    /// Returns a shared reference to the underlying bytes of `slice`.
    #[inline]
    pub fn slice_as_raw_bytes(slice: &[Self]) -> &[u8] {
        let ptr = slice.as_ptr().cast::<u8>();
        let len = slice.len() * Self::SIZE;
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    /// Returns a mutable reference to the underlying bytes of `slice`.
    #[inline]
    pub fn slice_as_raw_bytes_mut(slice: &mut [Self]) -> &mut [u8] {
        let ptr = slice.as_mut_ptr().cast::<u8>();
        let len = slice.len() * Self::SIZE;
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }
}
