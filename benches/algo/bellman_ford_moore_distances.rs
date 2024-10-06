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
        AddArcWeighted,
        AdjacencyList,
        AdjacencyListWeighted,
        Arcs,
        ArcsWeighted,
        BellmanFordMoore,
        Empty,
        ErdosRenyi,
        Order,
    },
};

fn main() {
    divan::main();
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BellmanFordMooreForFor<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BellmanFordMooreIter<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

impl<'a, D> BellmanFordMooreForFor<'a, D> {
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

    #[must_use]
    pub fn distances(&mut self) -> Option<&[isize]>
    where
        D: ArcsWeighted<Weight = isize> + Order,
    {
        for _ in 1..self.digraph.order() {
            for (u, v, &w) in self.digraph.arcs_weighted() {
                self.dist[v] =
                    self.dist[v].min(self.dist[u].saturating_add(w));
            }
        }

        for (u, v, &w) in self.digraph.arcs_weighted() {
            if self.dist[v] > self.dist[u].saturating_add(w) {
                return None;
            }
        }

        Some(&self.dist)
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

    #[must_use]
    pub fn distances(&mut self) -> Option<&[isize]>
    where
        D: ArcsWeighted<Weight = isize> + Order,
    {
        for _ in 1..self.digraph.order() {
            let mut updated = false;

            for (u, v, &w) in self.digraph.arcs_weighted() {
                let u = self.dist[u];

                if u == isize::MAX {
                    continue;
                }

                let w = u + w;

                if self.dist[v] > w {
                    self.dist[v] = w;

                    updated = true;
                }
            }

            if !updated {
                break;
            }
        }

        self.digraph
            .arcs_weighted()
            .all(|(u, v, &w)| {
                let u = self.dist[u];

                u == isize::MAX || self.dist[v] <= u + w
            })
            .then_some(&self.dist)
    }
}

#[divan::bench(args = [10, 100])]
fn bellman_ford_moore(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench_local(|| {
        for u in 0..order {
            let mut bfm = BellmanFordMoore::new(&digraph, u);
            let _ = bfm.distances();
        }
    });
}

#[divan::bench(args = [10, 100])]
fn bellman_ford_moore_iter(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench_local(|| {
        for u in 0..order {
            let mut bfm = BellmanFordMooreIter::new(&digraph, u);
            let _ = bfm.distances();
        }
    });
}

#[divan::bench(args = [10, 100])]
fn bellman_ford_moore_index_mut(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench_local(|| {
        for u in 0..order {
            let mut bfm = BellmanFordMooreForFor::new(&digraph, u);
            let _ = bfm.distances();
        }
    });
}
