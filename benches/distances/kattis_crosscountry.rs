use {
    divan::Bencher,
    graaf::{
        repr::adjacency_list_weighted::fixture::{
            kattis_crosscountry_isize,
            kattis_crosscountry_usize,
        },
        BellmanFordMoore,
        DijkstraDist,
        FloydWarshall,
    },
};

fn main() {
    divan::main();
}

const DISTANCES_ISIZE: [isize; 4] = [0, 1, 3, 10];
const DISTANCES_USIZE: [usize; 4] = [0, 1, 3, 10];

#[divan::bench]
fn bellman_ford_moore(bencher: Bencher<'_, '_>) {
    let digraph = kattis_crosscountry_isize();
    let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
    let dist = bellman_ford_moore.distances();

    assert!(dist.unwrap().iter().eq(&DISTANCES_ISIZE));

    bencher.bench_local(|| {
        let mut bellman_ford_moore = BellmanFordMoore::new(&digraph, 0);
        let _ = bellman_ford_moore.distances();
    });
}

#[divan::bench]
fn dijkstra(bencher: Bencher<'_, '_>) {
    let digraph = kattis_crosscountry_usize();
    let mut dijkstra = DijkstraDist::new(&digraph, &[0]);
    let dist = dijkstra.distances();

    assert!(dist.values().eq(&DISTANCES_USIZE));

    bencher.bench_local(|| {
        let mut dijkstra = DijkstraDist::new(&digraph, &[0]);
        let _ = dijkstra.distances();
    });
}

#[divan::bench]
fn floyd_warshall(bencher: Bencher<'_, '_>) {
    let digraph = kattis_crosscountry_isize();
    let mut floyd_warshall = FloydWarshall::new(&digraph);
    let dist = floyd_warshall.distances();

    assert!(dist[0].iter().eq(&DISTANCES_ISIZE));

    bencher.bench_local(|| {
        let mut floyd_warshall = FloydWarshall::new(&digraph);
        let _ = floyd_warshall.distances();
    });
}
