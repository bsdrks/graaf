#![macro_use]

use {
    divan::Bencher,
    graaf::{
        adjacency_list_weighted::fixture::{
            kattis_bryr_1_isize,
            kattis_bryr_1_usize,
        },
        algo::{
            bellman_ford_moore,
            dijkstra_dist::Dijkstra,
            floyd_warshall,
        },
    },
};

fn main() {
    divan::main();
}

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = kattis_bryr_1_isize();

    bencher.bench_local(|| {
        let _ = bellman_ford_moore::single_source_distances(&digraph, 0);
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = kattis_bryr_1_usize();

    bencher.bench_local(|| {
        let mut dijkstra = Dijkstra::new(&digraph, &[0]);
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = kattis_bryr_1_isize();

    bencher.bench_local(|| {
        let _ = floyd_warshall::distances(&digraph);
    });
}