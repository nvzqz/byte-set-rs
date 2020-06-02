use std::{
    collections::HashSet,
    hash::{BuildHasher, Hasher},
};

pub use hashbrown::HashSet as HashbrownSet;

pub type NoHashSet<T> = HashSet<T, NoHashBuilder>;
pub type NoHashbrownSet<T> = HashbrownSet<T, NoHashBuilder>;

/// A hasher that passes the input byte directly.
#[derive(Clone, Copy, Default)]
pub struct NoHash {
    byte: u8,
}

impl Hasher for NoHash {
    fn finish(&self) -> u64 {
        self.byte as u64
    }

    fn write(&mut self, _: &[u8]) {
        panic!("Must use `write_u8` instead");
    }

    fn write_u8(&mut self, i: u8) {
        self.byte = i;
    }
}

#[derive(Clone, Copy, Default)]
pub struct NoHashBuilder;

impl BuildHasher for NoHashBuilder {
    type Hasher = NoHash;

    fn build_hasher(&self) -> Self::Hasher {
        Default::default()
    }
}
