use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use std::collections::{BTreeSet, BinaryHeap, HashSet};

use crate::util::{
    self,
    hash::{HashbrownSet, IdentityHashSet, IdentityHashbrownSet},
    rand::shuffled_bytes,
    Bool256,
};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Extend (Slice)");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, ByteSet::new()))
                },
                |(bytes, byte_set)| {
                    byte_set.extend(&bytes[..size]);
                    black_box(byte_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("[bool; 256]", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, Bool256::new()))
                },
                |(bytes, bool256)| {
                    bool256.extend(&bytes[..size]);
                    black_box(bool256);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, HashSet::<u8>::new()))
                },
                |(bytes, hash_set)| {
                    hash_set.extend(&bytes[..size]);
                    black_box(hash_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("HashSet<u8> (Identity Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        let bytes = shuffled_bytes(&mut rng);
                        black_box((bytes, IdentityHashSet::<u8>::default()))
                    },
                    |(bytes, hash_set)| {
                        hash_set.extend(&bytes[..size]);
                        black_box(hash_set);
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(
            BenchmarkId::new("hashbrown::HashSet<u8>", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        let bytes = shuffled_bytes(&mut rng);
                        black_box((bytes, HashbrownSet::<u8>::new()))
                    },
                    |(bytes, hash_set)| {
                        hash_set.extend(&bytes[..size]);
                        black_box(hash_set);
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(
            BenchmarkId::new("IdentityHashbrownSet<u8> (Identity Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        let bytes = shuffled_bytes(&mut rng);
                        black_box((
                            bytes,
                            IdentityHashbrownSet::<u8>::default(),
                        ))
                    },
                    |(bytes, hash_set)| {
                        hash_set.extend(&bytes[..size]);
                        black_box(hash_set);
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, BTreeSet::<u8>::new()))
                },
                |(bytes, btree_set)| {
                    btree_set.extend(&bytes[..size]);
                    black_box(btree_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, Vec::<u8>::new()))
                },
                |(bytes, vec)| {
                    vec.extend(&bytes[..size]);
                    black_box(vec);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BinaryHeap<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, BinaryHeap::<u8>::new()))
                },
                |(bytes, binary_heap)| {
                    binary_heap.extend(&bytes[..size]);
                    black_box(binary_heap);
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
