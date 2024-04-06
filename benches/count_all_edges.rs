//! Benchmark implementations of [`graaf::op::CountAllEdges`].
fn main() {
    divan::main();
}

macro_rules! complete_graph {
    ($v:ident, $adj:ident) => {
        for s in 0..$v {
            for t in 0..$v {
                if s != t {
                    $adj.add_edge(s, t);
                }
            }
        }
    };
}

macro_rules! complete_weighted_graph {
    ($v:ident, $adj:ident) => {
        for s in 0..$v {
            for t in 0..$v {
                if s != t {
                    $adj.add_weighted_edge(s, t, 1);
                }
            }
        }
    };
}

// Vec

#[divan::bench_group(min_time = 1)]
mod count_all_edges {
    use {
        core::array::from_fn,
        divan::Bencher,
        graaf::op::{
            AddEdge,
            AddWeightedEdge,
            CountAllEdges,
        },
        std::collections::{
            HashMap,
            HashSet,
        },
    };

    const ARGS: [usize; 3] = [10, 100, 1000];

    #[divan::bench(args = ARGS)]
    fn vec_vec(bencher: Bencher<'_, '_>, v: usize) {
        let mut adj = vec![Vec::<usize>::new(); v];

        complete_graph!(v, adj);

        bencher.bench_local(|| adj.count_all_edges());
    }

    #[divan::bench(args = ARGS)]
    fn vec_hash_set(bencher: Bencher<'_, '_>, v: usize) {
        let mut adj = vec![HashSet::<usize>::new(); v];

        complete_graph!(v, adj);

        bencher.bench_local(|| adj.count_all_edges());
    }

    #[divan::bench(args = ARGS)]
    fn vec_hash_map(bencher: Bencher<'_, '_>, v: usize) {
        let mut adj = vec![HashMap::<usize, usize>::new(); v];

        complete_weighted_graph!(v, adj);

        bencher.bench_local(|| adj.count_all_edges());
    }

    // Arr

    #[divan::bench(consts = ARGS)]
    fn arr_vec<const V: usize>(bencher: Bencher<'_, '_>) {
        let mut adj = from_fn::<_, V, _>(|_| Vec::new());

        complete_graph!(V, adj);

        bencher.bench_local(|| adj.count_all_edges());
    }

    #[divan::bench(consts = ARGS)]
    fn arr_hash_set<const V: usize>(bencher: Bencher<'_, '_>) {
        let mut adj = from_fn::<_, V, _>(|_| HashSet::new());

        complete_graph!(V, adj);

        bencher.bench_local(|| adj.count_all_edges());
    }

    #[divan::bench(consts = ARGS)]
    fn arr_hash_map<const V: usize>(bencher: Bencher<'_, '_>) {
        let mut adj = from_fn::<_, V, _>(|_| HashMap::new());

        complete_weighted_graph!(V, adj);

        bencher.bench_local(|| adj.count_all_edges());
    }

    // HashMap

    #[divan::bench(args = ARGS)]
    fn hash_map_vec(bencher: Bencher<'_, '_>, v: usize) {
        let mut adj = HashMap::<usize, Vec<usize>>::new();

        for s in 0..v {
            let _ = adj.insert(s, Vec::new());
        }

        complete_graph!(v, adj);

        bencher.bench_local(|| adj.count_all_edges());
    }

    #[divan::bench(args = ARGS)]
    fn hash_map_hash_set(bencher: Bencher<'_, '_>, v: usize) {
        let mut adj = HashMap::<usize, HashSet<usize>>::new();

        for s in 0..v {
            let _ = adj.insert(s, HashSet::new());
        }

        complete_graph!(v, adj);

        bencher.bench(|| adj.count_all_edges());
    }

    #[divan::bench(args = ARGS)]
    fn hash_map_hash_map(bencher: Bencher<'_, '_>, v: usize) {
        let mut adj = HashMap::<usize, HashMap<usize, usize>>::new();

        complete_weighted_graph!(v, adj);

        bencher.bench(|| adj.count_all_edges());
    }
}
