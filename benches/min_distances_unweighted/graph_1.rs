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

#[divan::bench(min_time = 1)]
fn dijkstra(bencher: Bencher) {
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
        dijkstra::min_distances_single_source(&graph, 0);
    });
}

#[divan::bench(min_time = 1)]
fn bfs(bencher: Bencher) {
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
        bfs::min_distances_single_source(&graph, 0);
    });
}
