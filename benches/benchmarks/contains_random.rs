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
    let mut group = criterion.benchmark_group("Contains (Random)");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched(
                || {
                    black_box((
                        rng.gen::<u8>(),
                        ByteSet::rand_len(size, &mut rng),
                    ))
                },
                |(byte, byte_set)| {
                    black_box(byte_set.contains(byte));
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("ByteSet (Bloom Filter)", size),
            |b| {
                b.iter_batched(
                    || {
                        black_box((
                            rng.gen::<u8>(),
                            ByteSet::rand_len(size, &mut rng),
                        ))
                    },
                    |(byte, byte_set)| {
                        black_box(byte_set.maybe_contains(byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("[bool; 256]", size), |b| {
            b.iter_batched(
                || {
                    black_box((
                        rng.gen::<u8>(),
                        Bool256::rand_len(size, &mut rng),
                    ))
                },
                |(byte, bool256)| {
                    black_box(bool256.contains(byte));
                },
                BatchSize::SmallInput,
            )
        });

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

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    black_box((
                        rng.gen::<u8>(),
                        HashSet::<u8>::rand_len(size, &mut rng),
                    ))
                },
                |(byte, hash_set)| {
                    black_box(hash_set.contains(byte));
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("HashSet<u8> (Identity Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        black_box((
                            rng.gen::<u8>(),
                            IdentityHashSet::<u8>::rand_len(size, &mut rng),
                        ))
                    },
                    |(byte, hash_set)| {
                        black_box(hash_set.contains(byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("HashbrownSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    black_box((
                        rng.gen::<u8>(),
                        HashbrownSet::<u8>::rand_len(size, &mut rng),
                    ))
                },
                |(byte, hash_set)| {
                    black_box(hash_set.contains(byte));
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("HashbrownSet<u8> (Identity Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        black_box((
                            rng.gen::<u8>(),
                            IdentityHashbrownSet::<u8>::rand_len(
                                size, &mut rng,
                            ),
                        ))
                    },
                    |(byte, hash_set)| {
                        black_box(hash_set.contains(byte));
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    black_box((
                        rng.gen::<u8>(),
                        BTreeSet::<u8>::rand_len(size, &mut rng),
                    ))
                },
                |(byte, btree_set)| {
                    black_box(btree_set.contains(byte));
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    black_box((
                        rng.gen::<u8>(),
                        Vec::<u8>::rand_len(size, &mut rng),
                    ))
                },
                |(byte, vec)| {
                    black_box(vec.contains(byte));
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
