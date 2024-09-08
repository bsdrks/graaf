//! Benchmark the `RandomTournament::random_tournament` implementation for
//! different types.

use graaf::{
    AdjacencyList,
    AdjacencyMatrix,
    RandomTournament,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::random_tournament(n, 0);
}
