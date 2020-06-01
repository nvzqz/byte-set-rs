use criterion::{black_box, BatchSize, BenchmarkId, Criterion, Throughput};
use hashbrown::HashSet as HashbrownSet;
use std::collections::{BTreeSet, HashSet};

use crate::util::{self, Bool256, Rand};
use byte_set::ByteSet;

pub fn benches(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("Length");

    let mut rng = rand::thread_rng();

    for &size in util::SIZES {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_function(BenchmarkId::new("ByteSet", size), |b| {
            b.iter_batched(
                || black_box(ByteSet::rand_len(size, &mut rng)),
                |byte_set| black_box(byte_set.len()),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("[bool; 256]", size), |b| {
            b.iter_batched(
                || black_box(Bool256::rand_len(size, &mut rng)),
                |bool256| black_box(bool256.len()),
                BatchSize::SmallInput,
            )
        });

        let range_inclusive = black_box(0u8..=util::saturating_cast(size));
        group.bench_with_input(
            BenchmarkId::new("RangeInclusive<u8>", size),
            &range_inclusive,
            |b, range_inclusive| b.iter(|| black_box(range_inclusive.len())),
        );

        group.bench_function(BenchmarkId::new("HashSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(HashSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| black_box(hash_set.len()),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("HashbrownSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(HashbrownSet::<u8>::rand_len(size, &mut rng)),
                |hash_set| black_box(hash_set.len()),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("BTreeSet<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(BTreeSet::<u8>::rand_len(size, &mut rng)),
                |btree_set| black_box(btree_set.len()),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(BenchmarkId::new("Vec<u8>", size), |b| {
            b.iter_batched_ref(
                || black_box(Vec::<u8>::rand_len(size, &mut rng)),
                |vec| black_box(vec.len()),
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}
