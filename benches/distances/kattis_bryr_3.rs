//! Benchmarks of different shortest path algorithms on the digraph from:
//!
//! Arnar Bjarni Arnarson and Unnar Freyr Erlendsson. 2019. Bridges (Sample
//! Input 3). Kattis. <https://open.kattis.com/problems/bryr>
//!
//! ![Kattis, bryr, sample 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_bryr_3.svg)
use {
    divan::Bencher,
    graaf::{
        BellmanFordMoore,
        DijkstraDist,
        FloydWarshall,
        repr::adjacency_list_weighted::fixture::{
            kattis_bryr_3_isize,
            kattis_bryr_3_usize,
        },
    },
    std::iter::once,
};

fn main() {
    divan::main();
}

const DISTANCES_ISIZE: [isize; 10] = [0, 0, 1, 0, 0, 0, 1, 0, 0, 1];
const DISTANCES_USIZE: [usize; 10] = [0, 0, 1, 0, 0, 0, 1, 0, 0, 1];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = kattis_bryr_3_isize();
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
    let digraph = kattis_bryr_3_usize();
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
    let digraph = kattis_bryr_3_isize();
    let mut floyd_warshall = FloydWarshall::new(&digraph);
    let dist = floyd_warshall.distances();

    assert!(
        dist[0..10].iter().eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
