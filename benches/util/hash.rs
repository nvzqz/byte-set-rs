use std::{
    collections::HashSet,
    hash::{BuildHasher, Hasher},
};

pub use hashbrown::HashSet as HashbrownSet;

pub type IdentityHashSet<T> = HashSet<T, IdentityHashBuilder>;
pub type IdentityHashbrownSet<T> = HashbrownSet<T, IdentityHashBuilder>;

/// A hasher that passes the input byte directly.
#[derive(Clone, Copy, Default)]
pub struct IdentityHash {
    byte: u8,
}

impl Hasher for IdentityHash {
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
pub struct IdentityHashBuilder;

impl BuildHasher for IdentityHashBuilder {
    type Hasher = IdentityHash;

    fn build_hasher(&self) -> Self::Hasher {
        Default::default()
    }
}
