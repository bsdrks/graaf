fn main() {
    divan::main();
}

mod dijkstra {
    use {
        divan::Bencher,
        graph::DijkstraWeighted,
        std::{
            cmp::Reverse,
            collections::{
                BinaryHeap,
                HashMap,
                HashSet,
            },
        },
    };

    #[divan::bench]
    fn vec_vec(bencher: Bencher) {
        let graph: [Vec<(usize, usize)>; 4] =
            [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        bencher.bench_local(|| graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap));
    }

    #[divan::bench]
    fn vec_hash_set(bencher: Bencher) {
        let graph: Vec<HashSet<(usize, usize)>> = vec![
            HashSet::from([(1, 2)]),
            HashSet::from([(2, 2)]),
            HashSet::new(),
            HashSet::from([(0, 2)]),
        ];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        bencher.bench_local(|| graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap));
    }

    #[divan::bench]
    fn arr_vec(bencher: Bencher) {
        let graph: [Vec<(usize, usize)>; 4] =
            [vec![(1, 2)], vec![(2, 2)], Vec::new(), vec![(0, 2)]];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        bencher.bench_local(|| graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap));
    }

    #[divan::bench]
    fn arr_hash_set(bencher: Bencher) {
        let graph: [HashSet<(usize, usize)>; 4] = [
            HashSet::from([(1, 2)]),
            HashSet::from([(2, 2)]),
            HashSet::new(),
            HashSet::from([(0, 2)]),
        ];
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        bencher.bench_local(|| graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap));
    }

    #[divan::bench]
    fn hashmap_vec(bencher: Bencher) {
        let graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::from([
            (0, vec![(1, 2)]),
            (1, vec![(2, 2)]),
            (2, Vec::new()),
            (3, vec![(0, 2)]),
        ]);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        bencher.bench_local(|| graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap));
    }

    #[divan::bench]
    fn hashmap_hash_set(bencher: Bencher) {
        let graph: HashMap<usize, HashSet<(usize, usize)>> = HashMap::from([
            (0, HashSet::from([(1, 2)])),
            (1, HashSet::from([(2, 2)])),
            (2, HashSet::new()),
            (3, HashSet::from([(0, 2)])),
        ]);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        bencher.bench_local(|| graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap));
    }

    #[divan::bench]
    fn hashmap_hashmap(bencher: Bencher) {
        let graph: HashMap<usize, HashMap<usize, usize>> = HashMap::from([
            (0, HashMap::from([(1, 2)])),
            (1, HashMap::from([(2, 2)])),
            (2, HashMap::new()),
            (3, HashMap::from([(0, 2)])),
        ]);
        let mut dist = [0, usize::MAX, usize::MAX, usize::MAX];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);

        bencher.bench_local(|| graph.dijkstra(|acc, w| acc + w, &mut dist, &mut heap));
    }
}
