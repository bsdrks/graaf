//! Benchmark the `Biclique::biclique` implementation for different types.

use graaf::{
    adjacency_list,
    adjacency_matrix,
    gen::Biclique,
    op::AddArc,
    r#gen::Empty,
};

fn main() {
    divan::main();
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_naive(
    m: usize,
    n: usize,
) -> adjacency_list::Digraph {
    assert!(m > 0, "m must be greater than zero");
    assert!(n > 0, "n must be greater than zero");

    let order = m + n;
    let mut digraph = adjacency_list::Digraph::empty(order);

    for u in 0..m {
        for v in m..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_naive((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_naive(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list((m, n): (usize, usize)) {
    let _ = adjacency_list::Digraph::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_matrix((m, n): (usize, usize)) {
    let _ = adjacency_matrix::Digraph::biclique(m, n);
}
