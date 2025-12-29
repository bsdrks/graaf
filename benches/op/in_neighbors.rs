//! Benchmarks of different implementations of `InNeighbors::in_neighbors`.
use {
    divan::Bencher,
    graaf::{
        AddArcWeighted,
        AdjacencyList,
        AdjacencyListWeighted,
        AdjacencyMap,
        AdjacencyMatrix,
        Arcs,
        EdgeList,
        Empty,
        ErdosRenyi,
        InNeighbors,
    },
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
        },
        marker::PhantomData,
        num::NonZero,
        thread::{
            available_parallelism,
            scope,
        },
    },
};

fn main() {
    divan::main();
}

struct AdjacencyListBTreeSet {
    arcs: Vec<BTreeSet<usize>>,
}

struct AdjacencyListBTreeMap {
    arcs: Vec<BTreeMap<usize, usize>>,
}

struct AdjacencyMapBTreeMap {
    arcs: BTreeMap<usize, BTreeSet<usize>>,
}

fn in_neighbors_arcs_filter_map_eq<D>(
    digraph: &D,
    v: usize,
) -> impl Iterator<Item = usize>
where
    D: Arcs,
{
    digraph
        .arcs()
        .filter_map(move |(x, y)| (v == y).then_some(x))
}

fn in_neighbors_adjacency_list_contains(
    digraph: &AdjacencyListBTreeSet,
    v: usize,
) -> impl Iterator<Item = usize> {
    digraph
        .arcs
        .iter()
        .enumerate()
        .filter_map(move |(x, set)| set.contains(&v).then_some(x))
}

#[derive(Clone)]
struct InNeighborsIterator<'a> {
    ptr: *const BTreeSet<usize>,
    len: usize,
    i: usize,
    v: usize,
    _marker: PhantomData<&'a BTreeSet<usize>>,
}

impl Iterator for InNeighborsIterator<'_> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.i < self.len {
                let idx = self.i;
                let set = &*self.ptr.add(idx);

                self.i += 1;

                if set.contains(&self.v) {
                    return Some(idx);
                }
            }
        }

        None
    }
}

fn in_neighbors_adjacency_list_unsafe(
    digraph: &AdjacencyListBTreeSet,
    v: usize,
) -> impl Iterator<Item = usize> {
    InNeighborsIterator {
        ptr: digraph.arcs.as_ptr(),
        len: digraph.arcs.len(),
        i: 0,
        v,
        _marker: PhantomData,
    }
}

fn in_neighbors_adjacency_list_parallel(
    digraph: &AdjacencyListBTreeSet,
    v: usize,
) -> impl Iterator<Item = usize> + use<> {
    let arcs = &digraph.arcs;
    let len = arcs.len();
    let num_threads = available_parallelism().map(NonZero::get).unwrap_or(1);
    let chunk_size = len.div_ceil(num_threads);
    let mut indices = Vec::new();

    if num_threads == 1 || len < chunk_size * 2 {
        for (i, set) in arcs.iter().enumerate() {
            if set.contains(&v) {
                indices.push(i);
            }
        }
    } else {
        scope(|s| {
            let mut handles = Vec::new();

            for chunk_start in (0..len).step_by(chunk_size) {
                let chunk_end = len.min(chunk_start + chunk_size);
                let slice = &arcs[chunk_start..chunk_end];

                let handle = s.spawn(move || {
                    let mut local_indices = Vec::new();

                    for (i, set) in slice.iter().enumerate() {
                        if set.contains(&v) {
                            local_indices.push(chunk_start + i);
                        }
                    }

                    local_indices
                });

                handles.push(handle);
            }

            for handle in handles {
                indices.append(&mut handle.join().unwrap());
            }
        });
    }

    indices.into_iter()
}

fn in_neighbors_adjacency_list_weighted_contains(
    digraph: &AdjacencyListBTreeMap,
    v: usize,
) -> impl Iterator<Item = usize> {
    digraph
        .arcs
        .iter()
        .enumerate()
        .filter_map(move |(x, map)| map.contains_key(&v).then_some(x))
}

fn in_neighbors_adjacency_map_contains(
    digraph: &AdjacencyMapBTreeMap,
    v: usize,
) -> impl Iterator<Item = usize> {
    digraph
        .arcs
        .iter()
        .filter_map(move |(x, set)| set.contains(&v).then_some(*x))
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_arcs_filter_map_eq(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_list_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in digraph.arcs() {
        let _ = arcs[u].insert(v);
    }

    let digraph = AdjacencyListBTreeSet { arcs };

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_list_unsafe(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_list_parallel(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let mut arcs = vec![BTreeSet::new(); order];

    for (u, v) in digraph.arcs() {
        let _ = arcs[u].insert(v);
    }

    let digraph = AdjacencyListBTreeSet { arcs };

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_list_parallel(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_contains(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyListBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = digraph.arcs[u].insert(v);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_list_contains(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_arcs_filter_map_eq(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_weighted_contains(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let mut digraph = AdjacencyListBTreeMap {
        arcs: vec![BTreeMap::new(); order],
    };

    for (u, v) in unweighted.arcs() {
        let _ = digraph.arcs[u].insert(v, 1);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_list_weighted_contains(&digraph, v)
                .count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_arcs_filter_map_eq(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_contains(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = AdjacencyMapBTreeMap {
        arcs: BTreeMap::new(),
    };

    for (u, v) in AdjacencyList::erdos_renyi(order, 0.5, 0).arcs() {
        let _ = digraph.arcs.entry(v).or_default().insert(u);
    }

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_adjacency_map_contains(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_arcs_filter_map_eq(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = digraph.in_neighbors(v).count();
        }
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_arcs_filter_map_eq(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        for v in 0..order {
            let _ = in_neighbors_arcs_filter_map_eq(&digraph, v).count();
        }
    });
}
