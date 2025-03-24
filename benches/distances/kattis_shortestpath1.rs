//! Benchmarks of different shortest path algorithms on the digraph from:
//!
//! Per Austrin. 2005. Single source shortest path, non-negative weights.
//! Kattis. <https://open.kattis.com/problems/shortestpath1>
//!
//! ![Kattis, shortestpath1, sample](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_shortestpath1.svg)
use {
    divan::Bencher,
    graaf::{
        repr::adjacency_list_weighted::fixture::{
            kattis_shortestpath1_isize,
            kattis_shortestpath1_usize,
        },
        BellmanFordMoore,
        DijkstraDist,
        FloydWarshall,
    },
    std::iter::once,
};

fn main() {
    divan::main();
}

const DISTANCES_ISIZE: [isize; 4] = [0, 2, 4, isize::MAX];
const DISTANCES_USIZE: [usize; 4] = [0, 2, 4, usize::MAX];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = kattis_shortestpath1_isize();
    let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
    let dist = bellman_ford_moore.distances();

    assert!(
        dist.unwrap().iter().eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances();
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = kattis_shortestpath1_usize();
    let mut dijkstra = DijkstraDist::new(&digraph, once(0));
    let dist = dijkstra.distances();

    assert!(dist.eq(&DISTANCES_USIZE), "distances are incorrect");

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, once(0));
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = kattis_shortestpath1_isize();
    let mut floyd_warshall = FloydWarshall::new(&digraph);
    let dist = floyd_warshall.distances();

    assert!(
        dist[0..4].iter().eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
