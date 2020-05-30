mod benchmarks;
mod rand;
mod util;

fn main() {
    let mut criterion = criterion::Criterion::default().configure_from_args();

    macro_rules! benchmarks {
        ($($module:ident,)+) => {
            $(benchmarks::$module::benches(&mut criterion);)+
        };
    }

    benchmarks! {
        contains_cached,
        contains_random,
        extend_slice,
        insert,
        iter,
    }

    criterion.final_summary();
}
