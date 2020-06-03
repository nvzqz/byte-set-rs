use std::{
    collections::HashSet,
    hash::{BuildHasher, Hasher},
};

pub use hashbrown::HashSet as HashbrownSet;

pub type IdentityHashSet<T> = HashSet<T, IdentityHasherBuilder>;
pub type IdentityHashbrownSet<T> = HashbrownSet<T, IdentityHasherBuilder>;

/// A hasher that passes the input byte directly.
#[derive(Clone, Copy, Default)]
pub struct IdentityHasher {
    byte: u8,
}

impl Hasher for IdentityHasher {
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
pub struct IdentityHasherBuilder;

impl BuildHasher for IdentityHasherBuilder {
    type Hasher = IdentityHasher;

    fn build_hasher(&self) -> Self::Hasher {
        Default::default()
    }
}
