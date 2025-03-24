//! Benchmarks of different shortest path algorithms on Erdős-Rényi digraphs.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        AdjacencyListWeighted,
        AdjacencyMap,
        AdjacencyMatrix,
        BellmanFordMoore,
        BfsDist,
        DijkstraDist,
        EdgeList,
        ErdosRenyi,
        FloydWarshall,
    },
    std::iter::once,
};

fn main() {
    divan::main();
}

const ORDER: usize = 1000;
const PROBABILITY: f64 = 0.003;
const SEED: u64 = 0;

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyListWeighted::from(AdjacencyList::erdos_renyi(
        ORDER,
        PROBABILITY,
        SEED,
    ));

    let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
    let dist = bellman_ford_moore.distances().unwrap();

    assert_eq!(dist[0], 0, "expected 0, got {}", dist[0]);
    assert_eq!(dist[999], 6, "expected 6, got {}", dist[999]);

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances().unwrap();
    });
}

#[divan::bench]
fn bfs_adjacency_list(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyList::erdos_renyi(ORDER, PROBABILITY, SEED);
    let mut bfs = BfsDist::new(&digraph, once(0));
    let dist = bfs.distances();

    assert_eq!(dist[0], 0, "expected 0, got {}", dist[0]);
    assert_eq!(dist[999], 6, "expected 6, got {}", dist[999]);

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn bfs_adjacency_map(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyMap::erdos_renyi(ORDER, PROBABILITY, SEED);
    let mut bfs = BfsDist::new(&digraph, once(0));
    let dist = bfs.distances();

    assert_eq!(dist[0], 0, "expected 0, got {}", dist[0]);
    assert_eq!(dist[999], 6, "expected 6, got {}", dist[999]);

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn bfs_adjacency_matrix(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyMatrix::erdos_renyi(ORDER, PROBABILITY, SEED);
    let mut bfs = BfsDist::new(&digraph, once(0));
    let dist = bfs.distances();

    assert_eq!(dist[0], 0, "expected 0, got {}", dist[0]);
    assert_eq!(dist[999], 6, "expected 6, got {}", dist[999]);

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn bfs_edge_list(bencher: Bencher<'_, '_>) {
    let digraph = EdgeList::erdos_renyi(ORDER, PROBABILITY, SEED);
    let mut bfs = BfsDist::new(&digraph, once(0));
    let dist = bfs.distances();

    assert_eq!(dist[0], 0, "expected 0, got {}", dist[0]);
    assert_eq!(dist[999], 6, "expected 6, got {}", dist[999]);

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyListWeighted::from(AdjacencyList::erdos_renyi(
        ORDER,
        PROBABILITY,
        SEED,
    ));

    let mut dijkstra = DijkstraDist::new(&digraph, once(0));
    let dist = dijkstra.distances();

    assert_eq!(dist[0], 0, "expected 0, got {}", dist[0]);
    assert_eq!(dist[999], 6, "expected 6, got {}", dist[999]);

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, once(0));
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = AdjacencyListWeighted::from(AdjacencyList::erdos_renyi(
        ORDER,
        PROBABILITY,
        SEED,
    ));

    let mut floyd_warshall = FloydWarshall::new(&digraph);
    let dist = floyd_warshall.distances();

    assert_eq!(dist[(0, 0)], 0, "expected 0, got {}", dist[(0, 0)]);
    assert_eq!(dist[(0, 999)], 6, "expected 6, got {}", dist[(0, 999)]);

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
