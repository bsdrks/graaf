//! Benchmark the `Circuit::circuit` implementation for different types.

use graaf::{
    adjacency_list,
    adjacency_matrix,
    gen::{
        Circuit,
        Empty,
    },
    op::AddArc,
};

fn main() {
    divan::main();
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_list_naive(order: usize) -> adjacency_list::Digraph {
    let mut digraph = adjacency_list::Digraph::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        digraph.add_arc(u, u + 1);
    }

    digraph.add_arc(order - 1, 0);

    digraph
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_naive(order: usize) {
    let _ = circuit_adjacency_list_naive(order);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = adjacency_list::Digraph::circuit(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = adjacency_matrix::Digraph::circuit(n);
}
