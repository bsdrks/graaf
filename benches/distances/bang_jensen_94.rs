//! Benchmarks of shortest distance algorithms on the digraph from:
//!
//! Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
//! Algorithms and Applications (2nd ed.). Springer, London, 94.
//! <https://doi.org/10.1007/978-1-84800-998-1>
//!
//! ![Bang-Jensen, 94](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_94.svg)
use {
    divan::Bencher,
    graaf::{
        repr::{
            adjacency_list::fixture::bang_jensen_94 as fixture_adjacency_list,
            adjacency_list_weighted::fixture::{
                bang_jensen_94_isize as fixture_adjacency_list_weighted_isize,
                bang_jensen_94_usize as fixture_adjacency_list_weighted_usize,
            },
            adjacency_map::fixture::bang_jensen_94 as fixture_adjacency_map,
            adjacency_matrix::fixture::bang_jensen_94 as fixture_adjacency_matrix,
            edge_list::fixture::bang_jensen_94 as fixture_edge_list,
        },
        BellmanFordMoore,
        BfsDist,
        DijkstraDist,
        FloydWarshall,
    },
    std::iter::once,
};

fn main() {
    divan::main();
}

const DISTANCES_ISIZE: [isize; 7] = [0, 1, 1, 2, 2, 2, 3];
const DISTANCES_USIZE: [usize; 7] = [0, 1, 1, 2, 2, 2, 3];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = fixture_adjacency_list_weighted_isize();
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
fn bfs_adjacency_list(bencher: Bencher<'_, '_>) {
    let digraph = fixture_adjacency_list();
    let mut bfs = BfsDist::new(&digraph, once(0));

    assert!(
        bfs.distances().iter().eq(&DISTANCES_USIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn bfs_adjacency_matrix(bencher: Bencher<'_, '_>) {
    let digraph = fixture_adjacency_matrix();
    let mut bfs = BfsDist::new(&digraph, once(0));

    assert!(
        bfs.distances().iter().eq(&DISTANCES_USIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn bfs_adjacency_map(bencher: Bencher<'_, '_>) {
    let digraph = fixture_adjacency_map();
    let mut bfs = BfsDist::new(&digraph, once(0));

    assert!(
        bfs.distances().iter().eq(&DISTANCES_USIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn bfs_edge_list(bencher: Bencher<'_, '_>) {
    let digraph = fixture_edge_list();
    let mut bfs = BfsDist::new(&digraph, once(0));

    assert!(
        bfs.distances().iter().eq(&DISTANCES_USIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, once(0));
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = fixture_adjacency_list_weighted_usize();
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
    let digraph = fixture_adjacency_list_weighted_isize();
    let mut floyd_warshall = FloydWarshall::new(&digraph);

    assert!(
        floyd_warshall.distances()[0..7].iter().eq(&DISTANCES_ISIZE),
        "distances are incorrect"
    );

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
