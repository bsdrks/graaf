use {
    divan::Bencher,
    graaf::{
        repr::{
            adjacency_list::fixture::bang_jensen_94,
            adjacency_list_weighted::fixture::{
                bang_jensen_94_isize,
                bang_jensen_94_usize,
            },
        },
        AdjacencyMatrix,
        BellmanFordMoore,
        BfsDist,
        DijkstraDist,
        FloydWarshall,
    },
};

fn main() {
    divan::main();
}

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_94_isize();

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances();
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
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
