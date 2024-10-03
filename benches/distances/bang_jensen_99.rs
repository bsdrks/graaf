// Clippy lint groups
#![deny(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
// Clippy restriction lints
#![deny(
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::missing_assert_message,
    clippy::multiple_inherent_impl,
    clippy::panic_in_result_fn,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::self_named_module_files,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result
)]
// Rustc lint groups
#![deny(rust_2018_idioms)]
// Rustc lints
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences
)]
// Rustdoc lints
#![deny(rustdoc::all)]
// Overwrites
#![allow(clippy::large_stack_frames)]

use {
    divan::Bencher,
    graaf::{
        repr::adjacency_list_weighted::fixture::bang_jensen_99 as fixture,
        BellmanFordMoore,
        FloydWarshall,
    },
};

fn main() {
    divan::main();
}

const DISTANCES_ISIZE: [isize; 6] = [0, 8, 3, 1, -4, -1];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = fixture();
    let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);

    assert!(
        bellman_ford_moore
            .distances()
            .unwrap()
            .iter()
            .eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = fixture();
    let mut floyd_warshall = FloydWarshall::new(&digraph);

    assert!(
        floyd_warshall.distances()[0].iter().eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
