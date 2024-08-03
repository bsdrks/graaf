//! Benchmark the `Star::star` implementation for different types.

use graaf::{
    adjacency_list,
    adjacency_matrix,
    gen::Star,
};

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_list(n: usize) {
    let _ = adjacency_list::Digraph::star(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100000])]
fn adjacency_matrix(n: usize) {
    let _ = adjacency_matrix::Digraph::star(n);
}
