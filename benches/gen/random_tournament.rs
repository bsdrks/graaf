//! Benchmark the `RandomTournament::random_tournament` implementation for
//! different types.

use {
    graaf::{
        gen::prng::Xoshiro256StarStar,
        AdjacencyList,
        AdjacencyMatrix,
        RandomTournament,
    },
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

pub struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

fn random_tournament_adjacency_list_hash_set(
    order: usize,
    seed: u64,
) -> AdjacencyListHashSet {
    let mut digraph = AdjacencyListHashSet {
        arcs: vec![HashSet::new(); order],
    };

    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (u + 1)..order {
            if rng.next_bool() {
                digraph.arcs[u].insert(v);
            } else {
                digraph.arcs[v].insert(u);
            }
        }
    }

    digraph
}
#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::random_tournament(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = random_tournament_adjacency_list_hash_set(n, 0);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::random_tournament(n, 0);
}
