use {
    divan::Bencher,
    graaf::{
        repr::adjacency_list_weighted::fixture::bang_jensen_99,
        BellmanFordMoore,
        FloydWarshall,
    },
};

fn main() {
    divan::main();
}

#[divan::bench]
fn bellman_ford_moore_0(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_99();

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_99();

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
