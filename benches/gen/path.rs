//! Benchmark the `Path::path` implementation for different types.

use graaf::{
    AdjacencyList,
    AdjacencyMatrix,
    Path,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::path(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::path(n);
}
