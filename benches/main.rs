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
        clear,
        contains_cached,
        contains_random,
        drop,
        extend_slice,
        insert,
        iter,
        len,
        max,
        min,
        remove_single,
    }

    criterion.final_summary();
}
