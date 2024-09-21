use {
    divan::Bencher,
    graaf::{
        repr::adjacency_list_weighted::fixture::{
            kattis_bryr_3_isize,
            kattis_bryr_3_usize,
        },
        BellmanFordMoore,
        DijkstraDist,
        FloydWarshall,
    },
};

fn main() {
    divan::main();
}
#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = kattis_bryr_3_isize();

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances();
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = kattis_bryr_3_usize();

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, &[0]);
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = kattis_bryr_3_isize();

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
