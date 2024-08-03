//! Benchmark the `RandomTournament::random_tournament` implementation for
//! different types.

use graaf::{
    adjacency_list,
    adjacency_matrix,
    gen::RandomTournament,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = adjacency_list::Digraph::random_tournament(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = adjacency_matrix::Digraph::random_tournament(n);
}
