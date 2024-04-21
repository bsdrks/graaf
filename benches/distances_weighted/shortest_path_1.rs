//! Benchmark [`graaf::algo::dijkstra::distances`] with different graph
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
        graaf::algo::dijkstra::distances,
        std::collections::HashSet,
    };

    #[divan::bench]
    fn vec_vec(bencher: Bencher<'_, '_>) {
        let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

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
            distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }

    #[divan::bench]
    fn arr_vec(bencher: Bencher<'_, '_>) {
        let graph = [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
        let mut dist = DIST;
        let mut heap = BinaryHeap::from(HEAP);

        bencher.bench_local(|| {
            distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

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
            distances(&graph, |acc, w| acc + w, &mut dist, &mut heap);

            dist
        });
    }
}
