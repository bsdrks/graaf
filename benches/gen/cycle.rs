//! Benchmarks of different implementations of `Cycle::cycle`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Cycle,
        EdgeList,
        Empty,
    },
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
            HashSet,
        },
        iter::once,
        ptr::write,
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct EdgeListBTreeSet {
    pub arcs: BTreeSet<(usize, usize)>,
    pub order: usize,
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_list_btree_set_collect(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListBTreeSet {
            arcs: vec![BTreeSet::new()],
        };
    }

    AdjacencyListBTreeSet {
        arcs: (0..order)
            .map(|u| {
                let u = u + order;

                BTreeSet::from([(u - 1) % order, (u + 1) % order])
            })
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_list_btree_set_add_arc_empty(
    order: usize,
) -> AdjacencyList {
    let mut digraph = AdjacencyList::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 0);
    digraph.add_arc(0, u);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
#[allow(clippy::uninit_vec)]
fn cycle_adjacency_list_btree_set_unsafe(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListBTreeSet {
            arcs: vec![BTreeSet::new()],
        };
    }

    let mut arcs = Vec::with_capacity(order);
    let last = order - 1;

    unsafe {
        let ptr: *mut BTreeSet<usize> = arcs.as_mut_ptr();

        ptr.write(BTreeSet::from([last, 1]));

        for u in 1..last {
            ptr.add(u).write(BTreeSet::from([u - 1, u + 1]));
        }

        ptr.add(last).write(BTreeSet::from([order - 2, 0]));

        arcs.set_len(order);
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_list_hash_set_insert(order: usize) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");
    let mut arcs = vec![HashSet::new(); order];

    if order == 1 {
        return AdjacencyListHashSet { arcs };
    }

    for u in 0..order - 1 {
        let v = u + 1;

        let _ = arcs[u].insert(v);
        let _ = arcs[v].insert(u);
    }

    let u = order - 1;

    let _ = arcs[u].insert(0);
    let _ = arcs[0].insert(u);

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_map_btree_set_collect(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    AdjacencyMapBTreeSet {
        arcs: (0..order)
            .map(|u| {
                (
                    u,
                    BTreeSet::from([(u + order - 1) % order, (u + 1) % order]),
                )
            })
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_map_btree_set_add_arc_empty(order: usize) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 0);
    digraph.add_arc(0, u);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_map_btree_set_insert(order: usize) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let mut arcs = BTreeMap::new();
    let _ = arcs.insert(0, BTreeSet::from([order - 1, 1]));

    for u in 1..order - 1 {
        let _ = arcs.insert(u, BTreeSet::from([u - 1, u + 1]));
    }

    let _ = arcs.insert(order - 1, BTreeSet::from([order - 2, 0]));

    AdjacencyMapBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_adjacency_map_btree_set_vec_collect(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let mut vec = Vec::with_capacity(order);
    let ptr: *mut (usize, BTreeSet<usize>) = vec.as_mut_ptr();

    for u in 0..order {
        unsafe {
            write(
                ptr.add(u),
                (
                    u,
                    BTreeSet::from([(u + order - 1) % order, (u + 1) % order]),
                ),
            );
        }
    }

    unsafe {
        vec.set_len(order);
    }

    AdjacencyMapBTreeSet {
        arcs: vec.into_iter().collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_edge_list_add_arc_empty(order: usize) -> EdgeList {
    let mut digraph = EdgeList::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 0);
    digraph.add_arc(0, u);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_edge_list_btree_set_flat_map(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                once((u, (u + order - 1) % order))
                    .chain(once((u, (u + 1) % order)))
            })
            .collect(),
        order,
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn cycle_edge_list_btree_set_flat_map_map(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                once((u + order - 1) % order)
                    .chain(once((u + 1) % order))
                    .map(move |v| (u, v))
            })
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = cycle_adjacency_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_add_arc_empty(n: usize) {
    let _ = cycle_adjacency_list_btree_set_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_unsafe(n: usize) {
    let _ = cycle_adjacency_list_btree_set_unsafe(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set(n: usize) {
    let _ = cycle_adjacency_list_hash_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = cycle_adjacency_map_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_add_arc_empty(n: usize) {
    let _ = cycle_adjacency_map_btree_set_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_insert(n: usize) {
    let _ = cycle_adjacency_map_btree_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_vec_collect(n: usize) {
    let _ = cycle_adjacency_map_btree_set_vec_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::cycle(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = cycle_edge_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_flat_map(n: usize) {
    let _ = cycle_edge_list_btree_set_flat_map(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_flat_map_map(n: usize) {
    let _ = cycle_edge_list_btree_set_flat_map_map(n);
}
