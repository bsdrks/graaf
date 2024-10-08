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
        AdjacencyList,
        BellmanFordMoore,
        ErdosRenyi,
        Order,
    },
};

fn main() {
    divan::main();
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BellmanFordMooreIter<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BellmanFordMooreIndexMut<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

impl<'a, D> BellmanFordMooreIndexMut<'a, D> {
    pub fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        Self {
            digraph,
            dist: {
                let mut dist = vec![isize::MAX; digraph.order()];

                dist[s] = 0;

                dist
            },
        }
    }
}

impl<'a, D> BellmanFordMooreIter<'a, D> {
    pub fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        Self {
            digraph,
            dist: (0..digraph.order())
                .map(|u| if u == s { 0 } else { isize::MAX })
                .collect(),
        }
    }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bellman_ford_moore_iter(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        for u in 0..order {
            let _ = BellmanFordMooreIter::new(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bellman_ford_moore(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        for u in 0..order {
            let _ = BellmanFordMoore::new(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bellman_ford_moore_index_mut(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        for u in 0..order {
            let _ = BellmanFordMooreIndexMut::new(&digraph, u);
        }
    });
}
