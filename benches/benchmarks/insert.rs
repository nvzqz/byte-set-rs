use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use std::collections::{BTreeSet, BinaryHeap, HashSet};

use crate::util;
use byte_set::ByteSet;

pub fn benches() {
    let mut criterion = Criterion::default().configure_from_args();
    let mut group = criterion.benchmark_group("Insert");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = crate::rand::shuffled_bytes(&mut rng);
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

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = crate::rand::shuffled_bytes(&mut rng);
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
                    let bytes = crate::rand::shuffled_bytes(&mut rng);
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
                    let bytes = crate::rand::shuffled_bytes(&mut rng);
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

        group.bench_function(BenchmarkId::new("BinaryHeap<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = crate::rand::shuffled_bytes(&mut rng);
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
