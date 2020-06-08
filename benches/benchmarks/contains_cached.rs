use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use rand::Rng;
use std::collections::{BTreeSet, HashSet};

use crate::util::{
    self,
    hash::{HashbrownSet, IdentityHashSet, IdentityHashbrownSet},
    Bool256, Rand,
};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Contains (Cached)");

    let mut rng = rand::thread_rng();

    let mut input = [0u8; 256];
    for i in 0..=u8::max_value() {
        input[i as usize] = i;
    }

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        let byte_set = ByteSet::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("ByteSet", size),
            &byte_set,
            |b, byte_set| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(byte_set.contains(byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let bool256 = Bool256::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("[bool; 256]", size),
            &bool256,
            |b, bool256| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(bool256.contains(byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let range_inclusive = black_box(0u8..=util::saturating_cast(size));
        group.bench_with_input(
            BenchmarkId::new("RangeInclusive<u8>", size),
            &range_inclusive,
            |b, range_inclusive| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(range_inclusive.contains(&byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let hash_set = HashSet::<u8>::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("HashSet<u8>", size),
            &hash_set,
            |b, hash_set| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(hash_set.contains(&byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let hash_set = IdentityHashSet::<u8>::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("HashSet<u8> (Identity Hash)", size),
            &hash_set,
            |b, hash_set| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(hash_set.contains(&byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let hash_set = HashbrownSet::<u8>::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("hashbrown::HashSet<u8>", size),
            &hash_set,
            |b, hash_set| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(hash_set.contains(&byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let hash_set = IdentityHashbrownSet::<u8>::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("hashbrown::HashSet<u8> (Identity Hash)", size),
            &hash_set,
            |b, hash_set| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(hash_set.contains(&byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let fixed_bit_set = fixedbitset::FixedBitSet::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("fixedbitset::FixedBitSet", size),
            &fixed_bit_set,
            |b, fixed_bit_set| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(fixed_bit_set.contains(byte as usize));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let btree_set = BTreeSet::<u8>::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("BTreeSet<u8>", size),
            &btree_set,
            |b, btree_set| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(btree_set.contains(&byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let vec = Vec::<u8>::rand_len(size, &mut rng);
        group.bench_with_input(
            BenchmarkId::new("Vec<u8>", size),
            &vec,
            |b, vec| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(vec.contains(&byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        let vec = {
            let mut vec = Vec::<u8>::rand_len(size, &mut rng);
            vec.sort_unstable();
            vec
        };
        group.bench_with_input(
            BenchmarkId::new("Vec<u8> (Binary Search)", size),
            &vec,
            |b, vec| {
                b.iter_batched(
                    || rng.gen::<u8>(),
                    |byte| {
                        black_box(vec.binary_search(&byte).is_ok());
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}
