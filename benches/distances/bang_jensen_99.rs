use {
    divan::Bencher,
    graaf::{
        bellman_ford_moore,
        floyd_warshall,
        repr::adjacency_list_weighted::fixture::bang_jensen_99,
    },
};

fn main() {
    divan::main();
}

#[divan::bench]
fn bellman_ford_moore_0(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_99();

    bencher.bench_local(|| {
        let _ = bellman_ford_moore::single_source_distances(&digraph, 0);
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_99();

    bencher.bench_local(|| {
        let _ = floyd_warshall::distances(&digraph);
    });
}
