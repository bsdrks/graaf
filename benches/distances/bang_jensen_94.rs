use {
    divan::Bencher,
    graaf::{
        bellman_ford_moore,
        floyd_warshall,
        repr::{
            adjacency_list::fixture::bang_jensen_94,
            adjacency_list_weighted::fixture::{
                bang_jensen_94_isize,
                bang_jensen_94_usize,
            },
        },
        AdjacencyMatrix,
        BfsDist,
        DijkstraDist,
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
    let digraph = AdjacencyMatrix::from(bang_jensen_94());

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
