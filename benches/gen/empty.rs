//! Benchmark the `Empty::empty` implementation for different types.

use graaf::{
    adjacency_list,
    adjacency_matrix,
    gen::Empty,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = adjacency_list::Digraph::empty(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = adjacency_matrix::Digraph::empty(n);
}
