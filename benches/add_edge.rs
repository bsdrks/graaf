use {
    graph::AddEdge,
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
fn vec_vec(v: usize) {
    let mut adj = vec![Vec::<usize>::new(); v];

    for s in 0..v {
        for t in 0..v {
            adj.add_edge(s, t);
        }
    }
}

#[divan::bench(args = ARGS)]
fn vec_hash_set(v: usize) {
    let mut adj = vec![HashSet::<usize>::new(); v];

    for s in 0..v {
        for t in 0..v {
            adj.add_edge(s, t);
        }
    }
}

// Arr

#[divan::bench(consts = ARGS)]
fn arr_vec<const V: usize>() {
    let mut adj = from_fn::<Vec<usize>, V, _>(|_| Vec::new());

    for s in 0..V {
        for t in 0..V {
            adj.add_edge(s, t);
        }
    }
}

#[divan::bench(consts = ARGS)]
fn arr_hash_set<const V: usize>() {
    let mut adj = from_fn::<HashSet<usize>, V, _>(|_| HashSet::new());

    for s in 0..V {
        for t in 0..V {
            adj.add_edge(s, t);
        }
    }
}

// HashMap

#[divan::bench(consts = ARGS)]
fn hash_map_vec<const V: usize>() {
    let mut adj = HashMap::<usize, Vec<usize>>::new();

    for s in 0..V {
        for t in 0..V {
            adj.add_edge(s, t);
        }
    }
}

#[divan::bench(consts = ARGS)]
fn hash_map_hash_set<const V: usize>() {
    let mut adj = HashMap::<usize, HashSet<usize>>::new();

    for s in 0..V {
        for t in 0..V {
            adj.add_edge(s, t);
        }
    }
}
