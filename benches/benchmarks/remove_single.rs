use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use rand::{seq::SliceRandom, Rng};
use std::collections::{BTreeSet, HashSet};

use crate::util::{
    self,
    hash::{HashbrownSet, NoHashSet, NoHashbrownSet},
    rand::shuffled_bytes,
    Bool256,
};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Remove (Single)");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    let bytes = &bytes[..size];

                    if let Some(&byte) = bytes.choose(&mut rng) {
                        black_box((byte, bytes.iter().collect()))
                    } else {
                        black_box((rng.gen::<u8>(), ByteSet::new()))
                    }
                },
                |(byte, byte_set)| {
                    byte_set.remove(*byte);
                    black_box(byte_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("[bool; 256]", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    let bytes = &bytes[..size];

                    if let Some(&byte) = bytes.choose(&mut rng) {
                        black_box((byte, bytes.iter().collect()))
                    } else {
                        black_box((rng.gen::<u8>(), Bool256::new()))
                    }
                },
                |(byte, bool256)| {
                    bool256.remove(*byte);
                    black_box(bool256);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    let bytes = &bytes[..size];

                    if let Some(&byte) = bytes.choose(&mut rng) {
                        black_box((byte, bytes.iter().cloned().collect()))
                    } else {
                        black_box((rng.gen::<u8>(), HashSet::<u8>::new()))
                    }
                },
                |(byte, hash_set)| {
                    hash_set.remove(byte);
                    black_box(hash_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("HashSet<u8> (No Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        let bytes = shuffled_bytes(&mut rng);
                        let bytes = &bytes[..size];

                        if let Some(&byte) = bytes.choose(&mut rng) {
                            black_box((byte, bytes.iter().cloned().collect()))
                        } else {
                            black_box((
                                rng.gen::<u8>(),
                                NoHashSet::<u8>::default(),
                            ))
                        }
                    },
                    |(byte, hash_set)| {
                        hash_set.remove(byte);
                        black_box(hash_set);
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_function(BenchmarkId::new("HashbrownSet<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    let bytes = &bytes[..size];

                    if let Some(&byte) = bytes.choose(&mut rng) {
                        black_box((byte, bytes.iter().cloned().collect()))
                    } else {
                        black_box((rng.gen::<u8>(), HashbrownSet::<u8>::new()))
                    }
                },
                |(byte, hash_set)| {
                    hash_set.remove(byte);
                    black_box(hash_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(
            BenchmarkId::new("HashbrownSet<u8> (No Hash)", size),
            |b| {
                b.iter_batched_ref(
                    || {
                        let bytes = shuffled_bytes(&mut rng);
                        let bytes = &bytes[..size];

                        if let Some(&byte) = bytes.choose(&mut rng) {
                            black_box((byte, bytes.iter().cloned().collect()))
                        } else {
                            black_box((
                                rng.gen::<u8>(),
                                NoHashbrownSet::<u8>::default(),
                            ))
                        }
                    },
                    |(byte, hash_set)| {
                        hash_set.remove(byte);
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
                    let bytes = &bytes[..size];

                    if let Some(&byte) = bytes.choose(&mut rng) {
                        black_box((byte, bytes.iter().cloned().collect()))
                    } else {
                        black_box((rng.gen::<u8>(), BTreeSet::<u8>::new()))
                    }
                },
                |(byte, btree_set)| {
                    btree_set.remove(byte);
                    black_box(btree_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || {
                    let bytes = shuffled_bytes(&mut rng);
                    let bytes = &bytes[..size];

                    if let Some(&byte) = bytes.choose(&mut rng) {
                        black_box((byte, bytes.into()))
                    } else {
                        black_box((rng.gen::<u8>(), Vec::<u8>::new()))
                    }
                },
                |(byte, vec)| {
                    util::vec_remove_item(vec, byte);
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
                        let bytes = &bytes[..size];

                        if let Some(&byte) = bytes.choose(&mut rng) {
                            let mut vec = Vec::<u8>::from(bytes);
                            vec.sort_unstable();
                            black_box((byte, vec))
                        } else {
                            black_box((rng.gen::<u8>(), Vec::<u8>::new()))
                        }
                    },
                    |(byte, vec)| {
                        util::vec_remove_item_binary_search(vec, byte);
                        black_box(vec);
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}
