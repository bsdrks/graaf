//! Benchmark implementations of [`graaf::op::AddEdge`].

use {
    core::array::from_fn,
    divan::Bencher,
    graaf::op::AddEdge,
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

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

const ARGS: [usize; 3] = [10, 100, 1000];

#[divan::bench(args = ARGS)]
fn vec_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![Vec::<usize>::new(); v];

    bench_local_add_edge_complete_graph!(bencher, v, adj);

    let _ = adj;
}

#[divan::bench(args = ARGS)]
fn vec_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![BTreeSet::<usize>::new(); v];

    bench_local_add_edge_complete_graph!(bencher, v, adj);

    let _ = adj;
}

#[divan::bench(args = ARGS)]
fn vec_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = vec![HashSet::<usize>::new(); v];

    bench_local_add_edge_complete_graph!(bencher, v, adj);

    let _ = adj;
}

#[divan::bench(consts = ARGS)]
fn arr_vec<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<Vec<usize>, V, _>(|_| Vec::new());

    bench_local_add_edge_complete_graph!(bencher, V, adj);

    let _ = adj;
}

#[divan::bench(consts = ARGS)]
fn arr_btree_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<BTreeSet<usize>, V, _>(|_| BTreeSet::new());

    bench_local_add_edge_complete_graph!(bencher, V, adj);

    let _ = adj;
}

#[divan::bench(consts = ARGS)]
fn arr_hash_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = from_fn::<HashSet<usize>, V, _>(|_| HashSet::new());

    bench_local_add_edge_complete_graph!(bencher, V, adj);

    let _ = adj;
}

#[divan::bench(args = ARGS)]
fn btree_map_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeMap::<usize, Vec<usize>>::new();

    for s in 0..v {
        let _ = adj.insert(s, Vec::new());
    }

    bench_local_add_edge_complete_graph!(bencher, v, adj);

    let _ = adj;
}

#[divan::bench(consts = ARGS)]
fn btree_map_btree_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = BTreeMap::<usize, BTreeSet<usize>>::new();

    for s in 0..V {
        let _ = adj.insert(s, BTreeSet::new());
    }

    bench_local_add_edge_complete_graph!(bencher, V, adj);

    let _ = adj;
}

#[divan::bench(consts = ARGS)]
fn hash_map_vec<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = HashMap::<usize, Vec<usize>>::new();

    for s in 0..V {
        let _ = adj.insert(s, Vec::new());
    }

    bench_local_add_edge_complete_graph!(bencher, V, adj);

    let _ = adj;
}

#[divan::bench(consts = ARGS)]
fn hash_map_hash_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = HashMap::<usize, HashSet<usize>>::new();

    for s in 0..V {
        let _ = adj.insert(s, HashSet::new());
    }

    bench_local_add_edge_complete_graph!(bencher, V, adj);

    let _ = adj;
}