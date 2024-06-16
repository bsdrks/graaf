//! Benchmark the implementations of [`Size`].
//!
//! [`Size`]: graaf::op::Size

use {
    divan::Bencher,
    graaf::{
        gen::{
            Complete,
            CompleteConst,
            Empty,
            EmptyConst,
        },
        op::{
            AddWeightedArc,
            Size,
        },
    },
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

macro_rules! complete_weighted_digraph {
    ($v:ident, $adj:ident) => {
        for s in 0..$v {
            for t in (s + 1)..$v {
                $adj.add_weighted_arc(s, t, 1);
                $adj.add_weighted_arc(t, s, 1);
            }
        }
    };
}

#[divan::bench(args = [10, 100, 1000])]
fn vec_vec(bencher: Bencher<'_, '_>, v: usize) {
    let adj = Vec::<Vec<usize>>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn vec_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let adj = Vec::<BTreeSet<usize>>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn vec_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let adj = Vec::<HashSet<usize>>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn vec_btree_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<BTreeMap<usize, usize>>::empty(v);

    complete_weighted_digraph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn vec_hash_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<HashMap<usize, usize>>::empty(v);

    complete_weighted_digraph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn slice_vec<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[Vec<usize>; V]>::complete();
    let adj = adj.as_mut_slice();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn slice_btree_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[BTreeSet<usize>; V]>::complete();
    let adj = adj.as_mut_slice();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn slice_hash_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[HashSet<usize>; V]>::complete();
    let adj = adj.as_mut_slice();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn slice_btree_map<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[BTreeMap<usize, usize>; V]>::empty();
    let adj = adj.as_mut_slice();

    complete_weighted_digraph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn slice_hash_map<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[HashMap<usize, usize>; V]>::empty();
    let adj = adj.as_mut_slice();

    complete_weighted_digraph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn arr_vec<const V: usize>(bencher: Bencher<'_, '_>) {
    let adj = <[Vec<usize>; V]>::complete();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn arr_btree_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let adj = <[BTreeSet<usize>; V]>::complete();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn arr_hash_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let adj = <[HashSet<usize>; V]>::complete();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn arr_btree_map<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[BTreeMap<usize, usize>; V]>::empty();

    complete_weighted_digraph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(consts = [10, 100, 1000])]
fn arr_hash_map<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[HashMap<usize, usize>; V]>::empty();

    complete_weighted_digraph!(V, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn btree_map_vec(bencher: Bencher<'_, '_>, v: usize) {
    let adj = BTreeMap::<usize, Vec<usize>>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn btree_map_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let adj = BTreeMap::<usize, BTreeSet<usize>>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn btree_map_btree_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(v);

    complete_weighted_digraph!(v, adj);

    bencher.bench(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn hash_map_vec(bencher: Bencher<'_, '_>, v: usize) {
    let adj = HashMap::<usize, Vec<usize>>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn hash_map_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let adj = HashMap::<usize, HashSet<usize>>::complete(v);

    bencher.bench(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn hash_map_hash_map(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashMap::<usize, HashMap<usize, usize>>::empty(v);

    complete_weighted_digraph!(v, adj);

    bencher.bench(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn vec_tuple_unweighted(bencher: Bencher<'_, '_>, v: usize) {
    let adj = Vec::<(usize, usize)>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn slice_tuple_unweighted(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<(usize, usize)>::complete(v);
    let adj = adj.as_mut_slice();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn btree_set_tuple_unweighted(bencher: Bencher<'_, '_>, v: usize) {
    let adj = BTreeSet::<(usize, usize)>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn hash_set_tuple_unweighted(bencher: Bencher<'_, '_>, v: usize) {
    let adj = HashSet::<(usize, usize)>::complete(v);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn vec_tuple_weighted(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<(usize, usize, usize)>::empty();

    complete_weighted_digraph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn slice_tuple_weighted(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<(usize, usize, usize)>::empty();

    complete_weighted_digraph!(v, adj);

    let adj = adj.as_mut_slice();

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn btree_set_tuple_weighted(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeSet::<(usize, usize, usize)>::empty();

    complete_weighted_digraph!(v, adj);

    bencher.bench_local(|| adj.size());
}

#[divan::bench(args = [10, 100, 1000])]
fn hash_set_tuple_weighted(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashSet::<(usize, usize, usize)>::empty();

    complete_weighted_digraph!(v, adj);

    bencher.bench_local(|| adj.size());
}
