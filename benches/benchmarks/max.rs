use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use std::collections::{BTreeSet, BinaryHeap, HashSet};

use crate::util::{
    self,
    hash::{HashbrownSet, NoHashSet, NoHashbrownSet},
    Bool256, Rand,
};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Max");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched_ref(
                || black_box(ByteSet::rand_len(size, &mut rng)),
                |byte_set| {
                    black_box(byte_set.last());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("[bool; 256]", size), |b| {
            b.iter_batched_ref(
                || black_box(Bool256::rand_len(size, &mut rng)),
                |bool256| {
                    black_box(bool256.max());
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
                    black_box(range_inclusive.end());
                })
            },
        );

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(HashSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| {
                    black_box(hash_set.iter().max());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("HashSet<u8> (No Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || black_box(NoHashSet::<u8>::rand_len(size, &mut rng)),
                    |hash_set| {
                        black_box(hash_set.iter().max());
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("HashbrownSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(HashbrownSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| {
                    black_box(hash_set.iter().max());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("HashbrownSet<u8> (No Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        black_box(NoHashbrownSet::<u8>::rand_len(
                            size, &mut rng,
                        ))
                    },
                    |hash_set| {
                        black_box(hash_set.iter().max());
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(BTreeSet::<u8>::rand_len(size, &mut rng)),
                |btree_set| {
                    // The `last` method is nightly-only:
                    // https://github.com/rust-lang/rust/issues/62924
                    black_box(btree_set.iter().next_back());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(Vec::<u8>::rand_len(size, &mut rng)),
                |vec| {
                    black_box(vec.iter().max());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BinaryHeap<u8>", size), |b| {
            b.iter_batched_ref(
                // `Reverse` is required here because `peek` returns the max value.
                || black_box(BinaryHeap::<u8>::rand_len(size, &mut rng)),
                |binary_heap| {
                    black_box(binary_heap.peek());
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
