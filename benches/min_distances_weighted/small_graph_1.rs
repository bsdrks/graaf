//! Benchmark [`graaf::algo::dijkstra::min_distances`] with different graph
//! representations.
fn main() {
    divan::main();
}

use core::cmp::Reverse;

const DIST: [usize; 9] = [
    0,
    usize::MAX,
    usize::MAX,
    usize::MAX,
    usize::MAX,
    usize::MAX,
    usize::MAX,
    usize::MAX,
    usize::MAX,
];

const HEAP: [(Reverse<usize>, usize); 1] = [(Reverse(0), 0)];

#[divan::bench_group]
mod dijkstra {
    extern crate alloc;

    use {
        super::{
            DIST,
            HEAP,
        },
        alloc::collections::BinaryHeap,
        divan::Bencher,
        graaf::algo::dijkstra::min_distances,
        std::collections::{
            HashMap,
            HashSet,
        },
    };

    #[divan::bench]
    fn vec_vec(bencher: Bencher<'_, '_>) {
        let graph = vec![
            vec![(1, 4), (7, 8)],
            vec![(0, 4), (2, 8), (7, 11)],
            vec![(1, 8), (3, 7), (5, 4), (8, 2)],
            vec![(2, 7), (4, 9), (5, 14)],
            vec![(3, 9), (5, 10)],
            vec![(2, 4), (3, 14), (4, 10), (6, 2)],
            vec![(5, 2), (7, 1), (8, 6)],
            vec![(0, 8), (1, 11), (6, 1), (8, 7)],
            vec![(2, 2), (6, 6), (7, 7)],
        ];

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }

    #[divan::bench]
    fn vec_hash_set(bencher: Bencher<'_, '_>) {
        let graph = vec![
            HashSet::from([(1, 4), (7, 8)]),
            HashSet::from([(0, 4), (2, 8), (7, 11)]),
            HashSet::from([(1, 8), (3, 7), (5, 4), (8, 2)]),
            HashSet::from([(2, 7), (4, 9), (5, 14)]),
            HashSet::from([(3, 9), (5, 10)]),
            HashSet::from([(2, 4), (3, 14), (4, 10), (6, 2)]),
            HashSet::from([(5, 2), (7, 1), (8, 6)]),
            HashSet::from([(0, 8), (1, 11), (6, 1), (8, 7)]),
            HashSet::from([(2, 2), (6, 6), (7, 7)]),
        ];

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }

    #[divan::bench]
    fn arr_vec(bencher: Bencher<'_, '_>) {
        let graph: [Vec<(usize, usize)>; 9] = [
            vec![(1, 4), (7, 8)],
            vec![(0, 4), (2, 8), (7, 11)],
            vec![(1, 8), (3, 7), (5, 4), (8, 2)],
            vec![(2, 7), (4, 9), (5, 14)],
            vec![(3, 9), (5, 10)],
            vec![(2, 4), (3, 14), (4, 10), (6, 2)],
            vec![(5, 2), (7, 1), (8, 6)],
            vec![(0, 8), (1, 11), (6, 1), (8, 7)],
            vec![(2, 2), (6, 6), (7, 7)],
        ];

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }

    #[divan::bench]
    fn arr_hash_set(bencher: Bencher<'_, '_>) {
        let graph = [
            HashSet::from([(1, 4), (7, 8)]),
            HashSet::from([(0, 4), (2, 8), (7, 11)]),
            HashSet::from([(1, 8), (3, 7), (5, 4), (8, 2)]),
            HashSet::from([(2, 7), (4, 9), (5, 14)]),
            HashSet::from([(3, 9), (5, 10)]),
            HashSet::from([(2, 4), (3, 14), (4, 10), (6, 2)]),
            HashSet::from([(5, 2), (7, 1), (8, 6)]),
            HashSet::from([(0, 8), (1, 11), (6, 1), (8, 7)]),
            HashSet::from([(2, 2), (6, 6), (7, 7)]),
        ];

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }

    #[divan::bench]
    fn hash_map_vec(bencher: Bencher<'_, '_>) {
        let graph = HashMap::from([
            (0, vec![(1, 4), (7, 8)]),
            (1, vec![(0, 4), (2, 8), (7, 11)]),
            (2, vec![(1, 8), (3, 7), (5, 4), (8, 2)]),
            (3, vec![(2, 7), (4, 9), (5, 14)]),
            (4, vec![(3, 9), (5, 10)]),
            (5, vec![(2, 4), (3, 14), (4, 10), (6, 2)]),
            (6, vec![(5, 2), (7, 1), (8, 6)]),
            (7, vec![(0, 8), (1, 11), (6, 1), (8, 7)]),
            (8, vec![(2, 2), (6, 6), (7, 7)]),
        ]);

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }

    #[divan::bench]
    fn hash_map_hash_set(bencher: Bencher<'_, '_>) {
        let graph = HashMap::from([
            (0, HashSet::from([(1, 4), (7, 8)])),
            (1, HashSet::from([(0, 4), (2, 8), (7, 11)])),
            (2, HashSet::from([(1, 8), (3, 7), (5, 4), (8, 2)])),
            (3, HashSet::from([(2, 7), (4, 9), (5, 14)])),
            (4, HashSet::from([(3, 9), (5, 10)])),
            (5, HashSet::from([(2, 4), (3, 14), (4, 10), (6, 2)])),
            (6, HashSet::from([(5, 2), (7, 1), (8, 6)])),
            (7, HashSet::from([(0, 8), (1, 11), (6, 1), (8, 7)])),
            (8, HashSet::from([(2, 2), (6, 6), (7, 7)])),
        ]);

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }

    #[divan::bench]
    fn hash_map_hash_map(bencher: Bencher<'_, '_>) {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 4), (7, 8)])),
            (1, HashMap::from([(0, 4), (2, 8), (7, 11)])),
            (2, HashMap::from([(1, 8), (3, 7), (5, 4), (8, 2)])),
            (3, HashMap::from([(2, 7), (4, 9), (5, 14)])),
            (4, HashMap::from([(3, 9), (5, 10)])),
            (5, HashMap::from([(2, 4), (3, 14), (4, 10), (6, 2)])),
            (6, HashMap::from([(5, 2), (7, 1), (8, 6)])),
            (7, HashMap::from([(0, 8), (1, 11), (6, 1), (8, 7)])),
            (8, HashMap::from([(2, 2), (6, 6), (7, 7)])),
        ]);

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }
}
