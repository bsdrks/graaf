use {
    divan::Bencher,
    graaf::{
        bellman_ford_moore,
        dijkstra_dist::DijkstraDist,
        floyd_warshall,
        repr::adjacency_list_weighted::fixture::{
            bang_jensen_96_isize,
            bang_jensen_96_usize,
        },
    },
};

fn main() {
    divan::main();
}

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_96_isize();

    bencher.bench_local(|| {
        let _ = bellman_ford_moore::single_source_distances(&digraph, 0);
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_96_usize();

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, &[0]);
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = bang_jensen_96_isize();

    bencher.bench_local(|| {
        let _ = floyd_warshall::distances(&digraph);
    });
}
