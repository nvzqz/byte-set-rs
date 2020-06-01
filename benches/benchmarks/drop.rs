use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use hashbrown::HashSet as HashbrownSet;
use std::collections::{BTreeSet, HashSet};

use crate::util::{self, Bool256, Rand};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Drop");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched(
                || black_box(ByteSet::rand_len(size, &mut rng)),
                |byte_set| {
                    drop(byte_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("[bool; 256]", size), |b| {
            b.iter_batched(
                || black_box(Bool256::rand_len(size, &mut rng)),
                |bool256| {
                    drop(bool256);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched(
                || black_box(HashSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| {
                    drop(hash_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("HashbrownSet<u8>", size), |b| {
            b.iter_batched(
                || black_box(HashbrownSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| {
                    drop(hash_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched(
                || black_box(BTreeSet::<u8>::rand_len(size, &mut rng)),
                |btree_set| {
                    drop(btree_set);
                },
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched(
                || black_box(Vec::<u8>::rand_len(size, &mut rng)),
                |vec| {
                    drop(vec);
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
