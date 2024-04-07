//! Benchmark [`graaf::algo::dijkstra::min_distances`] with different graph
//! representations.
fn main() {
    divan::main();
}

use core::cmp::Reverse;

const DIST: [usize; 4] = [0, usize::MAX, usize::MAX, usize::MAX];
const HEAP: [(Reverse<usize>, usize); 1] = [(Reverse(0), 0)];

#[divan::bench_group(min_time = 1)]
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
        let graph: [Vec<(usize, usize)>; 4] = [
            vec![(1, 1), (2, 3), (3, 14)],
            vec![(0, 2), (2, 4), (3, 22)],
            vec![(0, 3), (1, 10), (3, 7)],
            vec![(0, 13), (1, 8), (2, 2)],
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
        let graph: Vec<HashSet<(usize, usize)>> = vec![
            HashSet::from([(1, 1), (2, 3), (3, 14)]),
            HashSet::from([(0, 2), (2, 4), (3, 22)]),
            HashSet::from([(0, 3), (1, 10), (3, 7)]),
            HashSet::from([(0, 13), (1, 8), (2, 2)]),
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
        let graph: [Vec<(usize, usize)>; 4] = [
            vec![(1, 1), (2, 3), (3, 14)],
            vec![(0, 2), (2, 4), (3, 22)],
            vec![(0, 3), (1, 10), (3, 7)],
            vec![(0, 13), (1, 8), (2, 2)],
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
        let graph: [HashSet<(usize, usize)>; 4] = [
            HashSet::from([(1, 1), (2, 3), (3, 14)]),
            HashSet::from([(0, 2), (2, 4), (3, 22)]),
            HashSet::from([(0, 3), (1, 10), (3, 7)]),
            HashSet::from([(0, 13), (1, 8), (2, 2)]),
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
        let graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::from([
            (0, vec![(1, 1), (2, 3), (3, 14)]),
            (1, vec![(0, 2), (2, 4), (3, 22)]),
            (2, vec![(0, 3), (1, 10), (3, 7)]),
            (3, vec![(0, 13), (1, 8), (2, 2)]),
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
        let graph: HashMap<usize, HashSet<(usize, usize)>> = HashMap::from([
            (0, HashSet::from([(1, 1), (2, 3), (3, 14)])),
            (1, HashSet::from([(0, 2), (2, 4), (3, 22)])),
            (2, HashSet::from([(0, 3), (1, 10), (3, 7)])),
            (3, HashSet::from([(0, 13), (1, 8), (2, 2)])),
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
        let graph: HashMap<usize, HashMap<usize, usize>> = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 3), (3, 14)])),
            (1, HashMap::from([(0, 2), (2, 4), (3, 22)])),
            (2, HashMap::from([(0, 3), (1, 10), (3, 7)])),
            (3, HashMap::from([(0, 13), (1, 8), (2, 2)])),
        ]);

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }
}