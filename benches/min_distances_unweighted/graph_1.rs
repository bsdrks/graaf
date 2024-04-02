use {
    divan::Bencher,
    graaf::algo::{
        bfs,
        dijkstra::unweighted as dijkstra,
    },
};

fn main() {
    divan::main();
}

fn graph() -> Vec<Vec<usize>> {
    vec![
        vec![1, 3],
        vec![0, 2],
        vec![1],
        vec![0, 4, 7],
        vec![3, 5, 6, 7],
        vec![4, 6],
        vec![4, 5, 7],
        vec![3, 4, 6],
    ]
}

#[divan::bench(min_time = 1)]
fn dijkstra(bencher: Bencher) {
    let graph = graph();

    bencher.bench_local(|| {
        dijkstra::min_distances_single_source(&graph, 0);
    });
}

#[divan::bench(min_time = 1)]
fn bfs(bencher: Bencher) {
    let graph = graph();

    bencher.bench_local(|| {
        bfs::min_distances_single_source(&graph, 0);
    });
}
