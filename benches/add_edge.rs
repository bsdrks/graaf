fn main() {
    divan::main();
}

macro_rules! bench_local_add_edge_complete_graph {
    ($bencher:ident, $v:ident, $adj:ident) => {
        $bencher.bench_local(|| {
            for s in 0..$v {
                for t in 0..$v {
                    if s != t {
                        $adj.add_edge(s, t);
                    }
                }
            }
        });
    };
}

// Vec

#[divan::bench_group(min_time = 1)]
mod add_edge {
    use {
        divan::Bencher,
        graph::AddEdge,
        std::{
            array::from_fn,
            collections::{
                HashMap,
                HashSet,
            },
        },
    };

    const ARGS: [usize; 3] = [10, 100, 1000];

    #[divan::bench(args = ARGS)]
    fn vec_vec(bencher: Bencher, v: usize) {
        let mut adj = vec![Vec::<usize>::new(); v];

        bench_local_add_edge_complete_graph!(bencher, v, adj);
    }

    #[divan::bench(args = ARGS)]
    fn vec_hash_set(bencher: Bencher, v: usize) {
        let mut adj = vec![HashSet::<usize>::new(); v];

        bench_local_add_edge_complete_graph!(bencher, v, adj);
    }

    // Arr

    #[divan::bench(consts = ARGS)]
    fn arr_vec<const V: usize>(bencher: Bencher) {
        let mut adj = from_fn::<Vec<usize>, V, _>(|_| Vec::new());

        bench_local_add_edge_complete_graph!(bencher, V, adj);
    }

    #[divan::bench(consts = ARGS)]
    fn arr_hash_set<const V: usize>(bencher: Bencher) {
        let mut adj = from_fn::<HashSet<usize>, V, _>(|_| HashSet::new());

        bench_local_add_edge_complete_graph!(bencher, V, adj);
    }

    // HashMap

    #[divan::bench(consts = ARGS)]
    fn hash_map_vec<const V: usize>(bencher: Bencher) {
        let mut adj = HashMap::<usize, Vec<usize>>::new();

        for s in 0..V {
            adj.insert(s, Vec::new());
        }

        bench_local_add_edge_complete_graph!(bencher, V, adj);
    }

    #[divan::bench(consts = ARGS)]
    fn hash_map_hash_set<const V: usize>(bencher: Bencher) {
        let mut adj = HashMap::<usize, HashSet<usize>>::new();

        for s in 0..V {
            adj.insert(s, HashSet::new());
        }

        bench_local_add_edge_complete_graph!(bencher, V, adj);
    }
}
