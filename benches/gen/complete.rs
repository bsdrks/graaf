//! Benchmark the `Complete::complete` implementation for different types.

use graaf::{
    adjacency_list,
    adjacency_matrix,
    gen::Complete,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_list(n: usize) {
    let _ = adjacency_list::Digraph::complete(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_matrix(n: usize) {
    let _ = adjacency_matrix::Digraph::complete(n);
}
