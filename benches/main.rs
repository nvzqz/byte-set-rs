mod benchmarks;
mod rand;
mod util;

criterion::criterion_main! {
    benchmarks::contains_cached::benches,
    benchmarks::contains_random::benches,
    benchmarks::extend_slice::benches,
    benchmarks::insert::benches,
    benchmarks::iter::benches,
}
