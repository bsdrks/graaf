//! Benchmarks of different shortest distance algorithms on the digraph from:
//!
//! Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
//! Algorithms and Applications (2nd ed.). Springer, London, 99.
//! <https://doi.org/10.1007/978-1-84800-998-1>
//!
//! ![Bang-Jensen, 99](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_99.svg)
use {
    divan::Bencher,
    graaf::{
        repr::adjacency_list_weighted::fixture::bang_jensen_99 as fixture,
        BellmanFordMoore,
        FloydWarshall,
    },
};

fn main() {
    divan::main();
}

const DISTANCES_ISIZE: [isize; 6] = [0, 8, 3, 1, -4, -1];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = fixture();
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
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = fixture();
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
