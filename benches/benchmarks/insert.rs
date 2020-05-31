use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use std::collections::{BTreeSet, BinaryHeap, HashSet};

use crate::util::{self, rand::shuffled_bytes, Bool256};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Insert");

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
                    for &byte in &bytes[..size] {
                        byte_set.insert(byte);
                    }
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
                    for &byte in &bytes[..size] {
                        bool256.insert(byte);
                    }
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
                    for &byte in &bytes[..size] {
                        hash_set.insert(byte);
                    }
                    black_box(hash_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, BTreeSet::<u8>::new()))
                },
                |(bytes, btree_set)| {
                    for &byte in &bytes[..size] {
                        btree_set.insert(byte);
                    }
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
                    for &byte in &bytes[..size] {
                        vec.push(byte);
                    }
                    black_box(vec);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("Vec<u8> (Binary Search)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        let bytes = shuffled_bytes(&mut rng);
                        black_box((bytes, Vec::<u8>::new()))
                    },
                    |(bytes, vec)| {
                        for byte in &bytes[..size] {
                            if let Err(index) = vec.binary_search(byte) {
                                vec.insert(index, *byte);
                            }
                        }
                        black_box(vec);
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("BinaryHeap<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    black_box((bytes, BinaryHeap::<u8>::new()))
                },
                |(bytes, binary_heap)| {
                    for &byte in &bytes[..size] {
                        binary_heap.push(byte);
                    }
                    black_box(binary_heap);
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
