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
        let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
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
            HashSet::from([(1, 2)]),
            HashSet::from([(2, 2)]),
            HashSet::new(),
            HashSet::from([(0, 2)]),
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
        let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
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
            HashSet::from([(1, 2)]),
            HashSet::from([(2, 2)]),
            HashSet::new(),
            HashSet::from([(0, 2)]),
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
            (0, vec![(1, 2)]),
            (1, vec![(2, 2)]),
            (2, Vec::new()),
            (3, vec![(0, 2)]),
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
            (0, HashSet::from([(1, 2)])),
            (1, HashSet::from([(2, 2)])),
            (2, HashSet::new()),
            (3, HashSet::from([(0, 2)])),
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
            (0, HashMap::from([(1, 2)])),
            (1, HashMap::from([(2, 2)])),
            (2, HashMap::new()),
            (3, HashMap::from([(0, 2)])),
        ]);

        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            min_distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }
}
