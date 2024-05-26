//! Benchmark [`graaf::algo::dijkstra::distances`] for different digraph
//! representations.

extern crate alloc;

use {
    alloc::collections::BinaryHeap,
    core::cmp::Reverse,
    divan::Bencher,
    graaf::{
        algo::{
            dijkstra::distances,
            fixture,
        },
        gen::EmptyConst,
        op::{
            AddWeightedArc,
            IterWeightedArcs,
        },
    },
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

const DIST: [usize; 4] = [0, usize::MAX, usize::MAX, usize::MAX];
const HEAP: [(Reverse<usize>, usize); 1] = [(Reverse(0), 0)];

#[divan::bench]
fn vec_vec(bencher: Bencher<'_, '_>) {
    let digraph = fixture::kattis_shortestpath1();
    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}

#[divan::bench]
fn vec_hash_set(bencher: Bencher<'_, '_>) {
    let digraph = fixture::kattis_shortestpath1()
        .into_iter()
        .map(HashSet::from_iter)
        .collect::<Vec<HashSet<(usize, usize)>>>();

    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}

#[divan::bench]
fn arr_vec(bencher: Bencher<'_, '_>) {
    let mut digraph = <[Vec<(usize, usize)>; 4]>::default();
    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    for (s, t, w) in fixture::kattis_shortestpath1().iter_weighted_arcs() {
        digraph.add_weighted_arc(s, t, *w)
    }

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}

#[divan::bench]
fn arr_hash_set(bencher: Bencher<'_, '_>) {
    let mut digraph = <[HashSet<(usize, usize)>; 4]>::empty();
    let mut dist = DIST;
    let mut heap = BinaryHeap::from(HEAP);

    for (s, t, w) in fixture::kattis_shortestpath1().iter_weighted_arcs() {
        digraph.add_weighted_arc(s, t, *w)
    }

    bencher.bench_local(|| {
        distances(&digraph, |acc, w| acc + w, &mut dist, &mut heap);

        dist
    });
}
