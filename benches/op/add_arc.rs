//! Benchmark the implementations of [`AddArc`]
//!
//! [`AddArc`]: graaf::op::AddArc

use {
    divan::Bencher,
    graaf::{
        gen::{
            prng::Xoshiro256StarStar,
            Empty,
            EmptyConst,
        },
        op::AddArc,
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

fn gen_arcs(v: usize) -> Vec<(usize, usize)> {
    let mut prng = Xoshiro256StarStar::new(0);
    let mut arcs = Vec::<(usize, usize)>::empty();

    for _ in 0..v {
        let s = prng.next().unwrap() as usize % v;
        let mut t = prng.next().unwrap() as usize % v;

        while t == s {
            t = prng.next().unwrap() as usize % v;
        }

        arcs.add_arc(s, t);
    }

    arcs
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn vec_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<Vec<usize>>::empty(v);
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn vec_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<BTreeSet<usize>>::empty(v);
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn vec_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<HashSet<usize>>::empty(v);
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn slice_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<HashSet<usize>>::empty(v);
    let adj = adj.as_mut_slice();
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn slice_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<BTreeSet<usize>>::empty(v);
    let adj = adj.as_mut_slice();
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn slice_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<HashSet<usize>>::empty(v);
    let adj = adj.as_mut_slice();
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(consts = [10, 100, 1000, 10000])]
fn arr_vec<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[Vec<usize>; V]>::empty();
    let arcs = gen_arcs(V);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(consts = [10, 100, 1000, 10000])]
fn arr_btree_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[BTreeSet<usize>; V]>::empty();
    let arcs = gen_arcs(V);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(consts = [10, 100, 1000, 10000])]
fn arr_hash_set<const V: usize>(bencher: Bencher<'_, '_>) {
    let mut adj = <[HashSet<usize>; V]>::empty();
    let arcs = gen_arcs(V);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn btree_map_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeMap::<usize, Vec<usize>>::empty(v);
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn btree_map_btree_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeMap::<usize, BTreeSet<usize>>::empty(v);
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn hash_map_vec(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashMap::<usize, Vec<usize>>::empty(v);
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn hash_map_hash_set(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashMap::<usize, HashSet<usize>>::empty(v);
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn vec_tuple(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = Vec::<(usize, usize)>::empty();
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn btree_set_tuple(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = BTreeSet::<(usize, usize)>::empty();
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn hash_set_tuple(bencher: Bencher<'_, '_>, v: usize) {
    let mut adj = HashSet::<(usize, usize)>::new();
    let arcs = gen_arcs(v);

    bencher.bench_local(move || {
        for (s, t) in &arcs {
            adj.add_arc(*s, *t);
        }
    });
}
