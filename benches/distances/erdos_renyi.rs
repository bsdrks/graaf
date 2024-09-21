use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        AdjacencyListWeighted,
        AdjacencyMatrix,
        BellmanFordMoore,
        BfsDist,
        DijkstraDist,
        ErdosRenyi,
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
    let digraph = AdjacencyList::erdos_renyi(ORDER, PROBABILITY, SEED);

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, &[0]);
        let dist = bfs.distances();

        assert_eq!(dist[0], 0);
        assert_eq!(dist[999], 6);
    });
}

#[divan::bench]
fn bfs_adjacency_matrix(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyMatrix::from(AdjacencyList::erdos_renyi(
        ORDER,
        PROBABILITY,
        SEED,
    ));

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, &[0]);
        let dist = bfs.distances();

        assert_eq!(dist[0], 0);
        assert_eq!(dist[999], 6);
    });
}

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyListWeighted::from(AdjacencyList::erdos_renyi(
        ORDER,
        PROBABILITY,
        SEED,
    ));

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let dist = bellman_ford_moore.distances().unwrap();

        assert_eq!(dist[0], 0);
        assert_eq!(dist[999], 6);
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyListWeighted::from(AdjacencyList::erdos_renyi(
        ORDER,
        PROBABILITY,
        SEED,
    ));

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, &[0]);
        let dist = dijkstra.distances();

        assert_eq!(dist[&0], 0);
        assert_eq!(dist[&999], 6);
    });
}
