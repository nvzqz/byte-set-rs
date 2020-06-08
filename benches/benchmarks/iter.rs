use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use std::collections::{BTreeSet, HashSet};

use crate::util::{self, hash::HashbrownSet, Bool256, Rand};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Iter");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched(
                || black_box(ByteSet::rand_len(size, &mut rng)),
                |byte_set| {
                    for byte in byte_set {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("[bool; 256]", size), |b| {
            b.iter_batched_ref(
                || black_box(Bool256::rand_len(size, &mut rng)),
                |bool256| {
                    for byte in &*bool256 {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        let range_inclusive = black_box(0u8..=util::saturating_cast(size));
        group.bench_with_input(
            BenchmarkId::new("RangeInclusive<u8>", size),
            &range_inclusive,
            |b, range_inclusive| {
                b.iter(|| {
                    for byte in range_inclusive.clone() {
                        black_box(byte);
                    }
                })
            },
        );

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(HashSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| {
                    for &byte in hash_set.iter() {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("hashbrown::HashSet<u8>", size),
            |b| {
                b.iter_batched_ref(
                    || black_box(HashbrownSet::<u8>::rand_len(size, &mut rng)),
                    |hash_set| {
                        for &byte in hash_set.iter() {
                            black_box(byte);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(
            BenchmarkId::new("fixedbitset::FixedBitSet", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        black_box(fixedbitset::FixedBitSet::rand_len(
                            size, &mut rng,
                        ))
                    },
                    |fixed_bit_set| {
                        // The `.ones()` iterator goes over all set bits, making
                        // it equivalent to the others.
                        for one in fixed_bit_set.ones() {
                            black_box(one);
                        }
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(BTreeSet::<u8>::rand_len(size, &mut rng)),
                |btree_set| {
                    for &byte in btree_set.iter() {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(Vec::<u8>::rand_len(size, &mut rng)),
                |vec| {
                    for &byte in vec.iter() {
                        black_box(byte);
                    }
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
