#![macro_use]
use {
    divan::Bencher,
    graaf::{
        adjacency_list::fixture::bang_jensen_94,
        adjacency_list_weighted::fixture::{
            bang_jensen_94_isize,
            bang_jensen_94_usize,
        },
        adjacency_matrix,
        algo::{
            bellman_ford_moore,
            bfs_dist::BfsDist,
            dijkstra_dist::DijkstraDist,
            floyd_warshall,
        },
    },
};

fn main() {
    divan::main();
}

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_94_isize();

    bencher.bench_local(|| {
        let _ = bellman_ford_moore::single_source_distances(&digraph, 0);
    });
}

#[divan::bench]
fn bfs_adjacency_matrix(bencher: Bencher<'_, '_>) {
    let digraph = adjacency_matrix::Digraph::from(bang_jensen_94());

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, &[0]);
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn bfs_adjacency_list(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_94();

    bencher.bench_local(|| {
        let mut bfs = BfsDist::new(&digraph, &[0]);
        let _ = bfs.distances();
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_94_usize();

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, &[0]);
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_94_isize();

    bencher.bench_local(|| {
        let _ = floyd_warshall::distances(&digraph);
    });
}
