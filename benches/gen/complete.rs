//! Benchmark the `Complete::complete` implementation for different types.

use graaf::{
    AdjacencyList,
    AdjacencyMatrix,
    Complete,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::complete(n);
}
