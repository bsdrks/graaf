//! Benchmark different algorithms for finding the distances from a single
//! source vertex to all vertices in a graph.

use {
    divan::Bencher,
    graaf::algo::{
        bfs,
        dijkstra,
    },
};

fn main() {
    divan::main();
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let graph = vec![
        vec![(1, 1), (3, 1)],
        vec![(0, 1), (2, 1)],
        vec![(1, 1)],
        vec![(0, 1), (4, 1), (7, 1)],
        vec![(3, 1), (5, 1), (6, 1), (7, 1)],
        vec![(4, 1), (6, 1)],
        vec![(4, 1), (5, 1), (7, 1)],
        vec![(3, 1), (4, 1), (6, 1)],
    ];

    bencher.bench(|| {
        let _ = dijkstra::single_source_distances(&graph, 0);
    });
}

#[divan::bench]
fn bfs(bencher: Bencher<'_, '_>) {
    let graph = vec![
        vec![1, 3],
        vec![0, 2],
        vec![1],
        vec![0, 4, 7],
        vec![3, 5, 6, 7],
        vec![4, 6],
        vec![4, 5, 7],
        vec![3, 4, 6],
    ];

    bencher.bench(|| {
        let _ = bfs::single_source_distances(&graph, 0);
    });
}
