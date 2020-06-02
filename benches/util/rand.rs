use byte_set::ByteSet;
use hashbrown::HashSet as HashbrownSet;
use rand::{seq::SliceRandom, Rng};
use std::{
    collections::{BTreeSet, BinaryHeap, HashSet},
    hash::BuildHasher,
};

/// Returns an array of bytes that has been shuffled.
pub fn shuffled_bytes<R: Rng>(rng: &mut R) -> [u8; 256] {
    let mut input = [0u8; 256];
    for i in 0..=u8::max_value() {
        input[i as usize] = i;
    }
    input.shuffle(rng);
    input
}

/// A byte container that can be randomly generated.
pub trait Rand {
    /// Generates an instance containing `len` random bytes from `rng`.
    fn rand_len<R: Rng>(len: usize, rng: &mut R) -> Self;
}

impl Rand for ByteSet {
    fn rand_len<R: Rng>(len: usize, rng: &mut R) -> Self {
        let input = shuffled_bytes(rng);
        input[..len].iter().collect()
    }
}

// Standard library collections:

impl Rand for BTreeSet<u8> {
    fn rand_len<R: Rng>(len: usize, rng: &mut R) -> Self {
        let input = shuffled_bytes(rng);
        input[..len].iter().cloned().collect()
    }
}

impl Rand for BinaryHeap<u8> {
    fn rand_len<R: Rng>(len: usize, rng: &mut R) -> Self {
        let input = shuffled_bytes(rng);
        input[..len].iter().cloned().collect()
    }
}

impl<S: BuildHasher + Default> Rand for HashSet<u8, S> {
    fn rand_len<R: Rng>(len: usize, rng: &mut R) -> Self {
        let input = shuffled_bytes(rng);
        input[..len].iter().cloned().collect()
    }
}

impl Rand for Vec<u8> {
    fn rand_len<R: Rng>(len: usize, rng: &mut R) -> Self {
        let input = shuffled_bytes(rng);
        input[..len].into()
    }
}

// Hashbrown:

impl<S: BuildHasher + Default> Rand for HashbrownSet<u8, S> {
    fn rand_len<R: Rng>(len: usize, rng: &mut R) -> Self {
        let input = shuffled_bytes(rng);
        input[..len].iter().cloned().collect()
    }
}
