//! Benchmarks of different shortest path algorithms on the digraph from:
//!
//! Karl Johan Sande Heimark. 2018. Cross Country. Kattis.
//! <https://open.kattis.com/problems/crosscountry>
//!
//! ![Kattis, crosscountry, sample](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_crosscountry.svg)
use {
    divan::Bencher,
    graaf::{
        BellmanFordMoore,
        DijkstraDist,
        FloydWarshall,
        repr::adjacency_list_weighted::fixture::{
            kattis_crosscountry_isize,
            kattis_crosscountry_usize,
        },
    },
    std::iter::once,
};

fn main() {
    divan::main();
}

const DISTANCES_ISIZE: [isize; 4] = [0, 1, 3, 10];
const DISTANCES_USIZE: [usize; 4] = [0, 1, 3, 10];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = kattis_crosscountry_isize();
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
    let digraph = kattis_crosscountry_usize();
    let mut dijkstra = DijkstraDist::new(&digraph, once(0));
    let dist = dijkstra.distances();

    assert!(dist.eq(&DISTANCES_USIZE), "distances are incorrect");

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, once(0));

        drop(dijkstra.distances());
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = kattis_crosscountry_isize();
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
