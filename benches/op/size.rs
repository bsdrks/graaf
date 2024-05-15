//! Benchmark implementations of [`graaf::op::Size`].

extern crate alloc;

use {
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
    core::array::from_fn,
    divan::Bencher,
    graaf::op::{
        AddEdge,
        AddWeightedEdge,
        Size,
    },
    std::collections::{
        HashMap,
        HashSet,
    },
};

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

const ARGS: [usize; 3] = [10, 100, 1000];

#[divan::bench(args = ARGS)]
fn vec_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![Vec::<usize>::new(); v];

    complete_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn vec_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![BTreeSet::<usize>::new(); v];

    complete_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn vec_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![HashSet::<usize>::new(); v];

    complete_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn vec_btree_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![BTreeMap::<usize, usize>::new(); v];

    complete_weighted_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn vec_hash_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![HashMap::<usize, usize>::new(); v];

    complete_weighted_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = ARGS)]
fn arr_vec<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<_, V, _>(|_| Vec::new());

    complete_graph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = ARGS)]
fn arr_btree_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<_, V, _>(|_| BTreeSet::new());

    complete_graph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = ARGS)]
fn arr_hash_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<_, V, _>(|_| HashSet::new());

    complete_graph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = ARGS)]
fn arr_btree_map<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<_, V, _>(|_| BTreeMap::new());

    complete_weighted_graph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = ARGS)]
fn arr_hash_map<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<_, V, _>(|_| HashMap::new());

    complete_weighted_graph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn btree_map_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeMap::<usize, Vec<usize>>::new();

    for s in 0..v {
        let _ = adj.insert(s, Vec::new());
    }

    complete_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn hash_map_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashMap::<usize, Vec<usize>>::new();

    for s in 0..v {
        let _ = adj.insert(s, Vec::new());
    }

    complete_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn btree_map_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeMap::<usize, BTreeSet<usize>>::new();

    for s in 0..v {
        let _ = adj.insert(s, BTreeSet::new());
    }

    complete_graph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn hash_map_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashMap::<usize, HashSet<usize>>::new();

    for s in 0..v {
        let _ = adj.insert(s, HashSet::new());
    }

    complete_graph!(v, adj);

    bencher.bench(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn btree_map_btree_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeMap::<usize, BTreeMap<usize, usize>>::new();

    for s in 0..v {
        let _ = adj.insert(s, BTreeMap::new());
    }

    complete_weighted_graph!(v, adj);

    bencher.bench(|| adj.size());
}

#[divan::bench(args = ARGS)]
fn hash_map_hash_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashMap::<usize, HashMap<usize, usize>>::new();

    for s in 0..v {
        let _ = adj.insert(s, HashMap::new());
    }

    complete_weighted_graph!(v, adj);

    bencher.bench(|| adj.size());
}
