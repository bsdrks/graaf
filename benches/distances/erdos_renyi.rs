#![macro_use]

use {
    divan::Bencher,
    graaf::{
        self,
        adjacency_list,
        adjacency_list_weighted,
        adjacency_matrix,
        algo::{
            bellman_ford_moore,
            bfs_dist::BfsDist,
            dijkstra_dist::DijkstraDist,
        },
        gen::ErdosRenyi,
    },
};

fn main() {
    divan::main();
}

const ORDER: usize = 1000;
const PROBABILITY: f64 = 0.003;
const SEED: u64 = 0;

#[divan::bench]
fn bfs_adjacency_list(bencher: Bencher<'_, '_>) {
    let digraph =
        adjacency_list::Digraph::erdos_renyi(ORDER, PROBABILITY, SEED);

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, &[0]);
        let dist = bfs.distances();

        assert_eq!(dist[0], 0);
        assert_eq!(dist[999], 6);
    });
}

#[divan::bench]
fn bfs_adjacency_matrix(bencher: Bencher<'_, '_>) {
    let digraph = adjacency_matrix::Digraph::from(
        adjacency_list::Digraph::erdos_renyi(ORDER, PROBABILITY, SEED),
    );

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, &[0]);
        let dist = bfs.distances();

        assert_eq!(dist[0], 0);
        assert_eq!(dist[999], 6);
    });
}

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = adjacency_list_weighted::Digraph::from(
        adjacency_list::Digraph::erdos_renyi(ORDER, PROBABILITY, SEED),
    );

    bencher.bench_local(|| {
        let dist =
            bellman_ford_moore::single_source_distances(&digraph, 0).unwrap();

        assert_eq!(dist[0], 0);
        assert_eq!(dist[999], 6);
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = adjacency_list_weighted::Digraph::from(
        adjacency_list::Digraph::erdos_renyi(ORDER, PROBABILITY, SEED),
    );

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, &[0]);
        let dist = dijkstra.distances();

        assert_eq!(dist[&0], 0);
        assert_eq!(dist[&999], 6);
    });
}
