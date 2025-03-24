//! Benchmarks of shortest distance algorithms on the digraph from:
//!
//! Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
//! Algorithms and Applications (2nd ed.). Springer, London, 96.
//! <https://doi.org/10.1007/978-1-84800-998-1>
//!
//! ![Bang-Jensen, 96](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_96.svg)
use {
    divan::Bencher,
    graaf::{
        repr::adjacency_list_weighted::fixture::{
            bang_jensen_96_isize as fixture_isize,
            bang_jensen_96_usize as fixture_usize,
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

const DISTANCES_ISIZE: [isize; 6] = [0, 5, 3, 6, 4, 7];
const DISTANCES_USIZE: [usize; 6] = [0, 5, 3, 6, 4, 7];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = fixture_isize();
    let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);

    assert!(
        bellman_ford_moore
            .distances()
            .unwrap()
            .iter()
            .eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances();
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = fixture_usize();
    let mut dijkstra = DijkstraDist::new(&digraph, once(0));

    assert!(
        dijkstra.distances().eq(&DISTANCES_USIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, once(0));
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = fixture_isize();
    let mut floyd_warshall = FloydWarshall::new(&digraph);

    assert!(
        floyd_warshall.distances()[0..6].iter().eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
