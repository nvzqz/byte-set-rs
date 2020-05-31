use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use std::collections::{BTreeSet, BinaryHeap, HashSet};

use crate::util::{self, Rand};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Min");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched_ref(
                || black_box(ByteSet::rand_len(size, &mut rng)),
                |byte_set| {
                    black_box(byte_set.first());
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
                    black_box(range_inclusive.start());
                })
            },
        );

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(HashSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| {
                    black_box(hash_set.iter().min());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(BTreeSet::<u8>::rand_len(size, &mut rng)),
                |btree_set| {
                    // The `first` method is nightly-only:
                    // https://github.com/rust-lang/rust/issues/62924
                    black_box(btree_set.iter().next());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(Vec::<u8>::rand_len(size, &mut rng)),
                |vec| {
                    black_box(vec.iter().min());
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BinaryHeap<u8>", size), |b| {
            b.iter_batched_ref(
                // `Reverse` is required here because `peek` returns the max value.
                || black_box(BinaryHeap::<u8>::rand_len(size, &mut rng)),
                |binary_heap| {
                    black_box(binary_heap.iter().min());
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
