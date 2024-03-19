use {
    divan::Bencher,
    graph::{
        AddEdge,
        AddWeightedEdge,
        CountAllEdges,
    },
    std::{
        array::from_fn,
        collections::{
            HashMap,
            HashSet,
        },
    },
};

fn main() {
    divan::main();
}

const ARGS: [usize; 4] = [1, 10, 100, 1000];

// Vec

#[divan::bench(args = ARGS)]
fn vec_vec(bencher: Bencher, v: usize) {
    let mut adj = vec![Vec::<usize>::new(); v];

    for s in 0..v {
        for t in 0..v {
            adj.add_edge(s, t);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

#[divan::bench(args = ARGS)]
fn vec_hash_set(bencher: Bencher, v: usize) {
    let mut adj = vec![HashSet::<usize>::new(); v];

    for s in 0..v {
        for t in 0..v {
            adj.add_edge(s, t);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

#[divan::bench(args = ARGS)]
fn vec_hash_map(bencher: Bencher, v: usize) {
    let mut adj = vec![HashMap::<usize, usize>::new(); v];

    for s in 0..v {
        for t in 0..v {
            adj.add_weighted_edge(s, t, 1);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

// Arr

#[divan::bench(consts = ARGS)]
fn arr_vec<const V: usize>(bencher: Bencher) {
    let mut adj = from_fn::<_, V, _>(|_| Vec::new());

    for s in 0..V {
        for t in 0..V {
            adj.add_edge(s, t);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

#[divan::bench(consts = ARGS)]
fn arr_hash_set<const V: usize>(bencher: Bencher) {
    let mut adj = from_fn::<_, V, _>(|_| HashSet::new());

    for s in 0..V {
        for t in 0..V {
            adj.add_edge(s, t);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

#[divan::bench(consts = ARGS)]
fn arr_hash_map<const V: usize>(bencher: Bencher) {
    let mut adj = from_fn::<_, V, _>(|_| HashMap::new());

    for s in 0..V {
        for t in 0..V {
            adj.add_weighted_edge(s, t, 1);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

// HashMap

#[divan::bench(args = ARGS)]
fn hash_map_vec(bencher: Bencher, v: usize) {
    let mut adj = HashMap::<usize, Vec<usize>>::new();

    for s in 0..v {
        for t in 0..v {
            adj.add_edge(s, t);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

#[divan::bench(args = ARGS)]
fn hash_map_hash_set(bencher: Bencher, v: usize) {
    let mut adj = HashMap::<usize, HashSet<usize>>::new();

    for s in 0..v {
        for t in 0..v {
            adj.add_edge(s, t);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}

#[divan::bench(args = ARGS)]
fn hash_map_hash_map(bencher: Bencher, v: usize) {
    let mut adj = HashMap::<usize, HashMap<usize, usize>>::new();

    for s in 0..v {
        for t in 0..v {
            adj.add_weighted_edge(s, t, 1);
        }
    }

    bencher.bench(|| adj.count_all_edges());
}
