//! Benchmark [`graaf::algo::dijkstra::distances`] for different digraph
//! representations.

extern crate alloc;

use {
    alloc::collections::BinaryHeap,
    core::cmp::Reverse,
    divan::Bencher,
    graaf::algo::dijkstra::distances,
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

const DIST: [usize; 4] = [0, usize::MAX, usize::MAX, usize::MAX];
const HEAP: [(Reverse<usize>, usize); 1] = [(Reverse(0), 0)];

#[divan::bench]
fn vec_vec(bencher: Bencher<'_, '_>) {
    let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}

#[divan::bench]
fn vec_hash_set(bencher: Bencher<'_, '_>) {
    let digraph = vec![
        HashSet::from([(1, 2)]),
        HashSet::from([(2, 2)]),
        HashSet::new(),
        HashSet::from([(0, 2)]),
    ];

    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}

#[divan::bench]
fn arr_vec(bencher: Bencher<'_, '_>) {
    let digraph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}

#[divan::bench]
fn arr_hash_set(bencher: Bencher<'_, '_>) {
    let digraph = [
        HashSet::from([(1, 2)]),
        HashSet::from([(2, 2)]),
        HashSet::new(),
        HashSet::from([(0, 2)]),
    ];

    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}
