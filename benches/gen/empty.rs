//! Benchmark the `Empty::empty` implementation for different types.

use graaf::{
    AdjacencyList,
    AdjacencyMatrix,
    Empty,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::empty(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::empty(n);
}
