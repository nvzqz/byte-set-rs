use super::rand::{shuffled_bytes, Rand};
use std::{iter, ops::Range};

/// A wrapper around `[bool; 256]` for comparing performance.
pub struct Bool256(pub [bool; 256]);

impl Default for Bool256 {
    fn default() -> Self {
        Self([false; 256])
    }
}

impl Rand for Bool256 {
    fn rand_len<R: rand::Rng>(len: usize, rng: &mut R) -> Self {
        let input = shuffled_bytes(rng);
        input[..len].iter().collect()
    }
}

impl Bool256 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn len(&self) -> usize {
        self.0
            .iter()
            .fold(0, |n, &is_contained| n + is_contained as usize)
    }

    pub fn insert(&mut self, byte: u8) {
        self.0[byte as usize] = true;
    }

    pub fn remove(&mut self, byte: u8) {
        self.0[byte as usize] = false;
    }

    pub fn contains(&self, byte: u8) -> bool {
        self.0[byte as usize]
    }

    pub fn min(&self) -> Option<u8> {
        self.into_iter().next()
    }

    pub fn max(&self) -> Option<u8> {
        self.into_iter().next_back()
    }
}

impl Extend<u8> for Bool256 {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        iter.into_iter().for_each(|byte| self.insert(byte));
    }
}

impl<'a> Extend<&'a u8> for Bool256 {
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        self.extend(iter.into_iter().cloned());
    }
}

impl iter::FromIterator<u8> for Bool256 {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut value = Self::default();
        value.extend(iter);
        value
    }
}

impl<'a> iter::FromIterator<&'a u8> for Bool256 {
    fn from_iter<T: IntoIterator<Item = &'a u8>>(iter: T) -> Self {
        iter.into_iter().cloned().collect()
    }
}

impl<'a> IntoIterator for &'a Bool256 {
    type IntoIter = Iter<'a>;
    type Item = u8;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            bool256: self,
            indexes: 0..256,
        }
    }
}

pub struct Iter<'a> {
    bool256: &'a Bool256,
    indexes: Range<usize>,
}

impl Iterator for Iter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        for i in self.indexes.by_ref() {
            let byte = i as u8;
            if self.bool256.contains(byte) {
                return Some(byte);
            }
        }
        None
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        for i in self.indexes.by_ref().rev() {
            let byte = i as u8;
            if self.bool256.contains(byte) {
                return Some(byte);
            }
        }
        None
    }
}
