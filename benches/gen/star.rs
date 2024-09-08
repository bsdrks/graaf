//! Benchmark the `Star::star` implementation for different types.

use graaf::{
    AdjacencyList,
    AdjacencyMatrix,
    Star,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::star(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::star(n);
}
