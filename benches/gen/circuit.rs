//! Benchmarks of different implementations of `Circuit::circuit`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Circuit,
        EdgeList,
        Empty,
    },
    std::{
        alloc::{
            alloc,
            dealloc,
            Layout,
        },
        collections::{
            BTreeMap,
            BTreeSet,
            HashSet,
        },
        mem::ManuallyDrop,
        ptr,
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
struct AdjacencyListHashSet {
    pub arcs: Vec<HashSet<usize>>,
}

#[derive(Debug)]
#[allow(dead_code)]
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
fn circuit_adjacency_list_add_arc_empty(order: usize) -> AdjacencyList {
    let mut digraph = AdjacencyList::empty(order);

    if order == 1 {
        return digraph;
    }

    for u in 0..order - 1 {
        digraph.add_arc(u, u + 1);
    }

    digraph.add_arc(order - 1, 0);

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_list_hash_set_collect(
    order: usize,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListHashSet {
            arcs: vec![HashSet::new()],
        };
    }

    AdjacencyListHashSet {
        arcs: (0..order)
            .map(|u| HashSet::from([(u + 1) % order]))
            .collect::<Vec<_>>(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_map_btree_set_collect(
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
            .map(|u| (u, BTreeSet::from([(u + 1) % order])))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_map_btree_set_unsafe_vec(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let mut data: Vec<(usize, BTreeSet<usize>)> = Vec::with_capacity(order);

    for u in 0..order {
        let mut set = BTreeSet::new();
        let _ = set.insert((u + 1) % order);

        data.push((u, set));
    }

    let btree_map = unsafe { bulk_construct_btreemap_vec(data) };

    AdjacencyMapBTreeSet { arcs: btree_map }
}

unsafe fn bulk_construct_btreemap_vec<K: Ord, V>(
    data: Vec<(K, V)>,
) -> BTreeMap<K, V> {
    let mut map = BTreeMap::new();
    let mut data = ManuallyDrop::new(data);
    let ptr = data.as_mut_ptr();

    for i in 0..data.len() {
        let (key, value) = ptr::read(ptr.add(i));
        let _ = map.insert(key, value);
    }

    map
}

/// # Panics
///
/// Panics if `order` is zero.
#[allow(clippy::cast_ptr_alignment)]
#[allow(clippy::ptr_as_ptr)]
fn circuit_adjacency_map_btree_set_unsafe_raw(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let layout = Layout::array::<(usize, BTreeSet<usize>)>(order).unwrap();
    let raw_ptr = unsafe { alloc(layout) as *mut (usize, BTreeSet<usize>) };

    if raw_ptr.is_null() {
        std::alloc::handle_alloc_error(layout);
    }

    unsafe {
        for i in 0..order {
            let key = i;
            let mut set = BTreeSet::new();
            let _ = set.insert((i + 1) % order);

            ptr::write(raw_ptr.add(i), (key, set));
        }

        let btree_map = bulk_construct_btreemap_raw(raw_ptr, order);

        dealloc(raw_ptr as *mut u8, layout);

        AdjacencyMapBTreeSet { arcs: btree_map }
    }
}

/// # Safety
///
/// Constructs a `BTreeMap` from a raw pointer to an array of `(K, V)` pairs.
unsafe fn bulk_construct_btreemap_raw<K: Ord, V>(
    raw_ptr: *mut (K, V),
    len: usize,
) -> BTreeMap<K, V> {
    let mut map: BTreeMap<K, V> = BTreeMap::new();

    for i in 0..len {
        let (key, value) = ptr::read(raw_ptr.add(i));
        let _ = map.insert(key, value);
    }

    map
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_adjacency_map_btree_set_insert_collect(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let data = (0..order).map(|u| {
        let mut set = BTreeSet::new();
        let _ = set.insert((u + 1) % order);

        (u, set)
    });

    AdjacencyMapBTreeSet {
        arcs: data.collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_edge_list_btree_set_insert(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    let mut arcs = BTreeSet::new();

    for u in 0..order {
        let _ = arcs.insert((u, (u + 1) % order));
    }

    EdgeListBTreeSet { arcs, order }
}

/// # Panics
///
/// Panics if `order` is zero.
fn circuit_edge_list_btree_set_collect(order: usize) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return EdgeListBTreeSet {
            arcs: BTreeSet::new(),
            order,
        };
    }

    EdgeListBTreeSet {
        arcs: (0..order).map(|u| (u, (u + 1) % order)).collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_list_add_arc_empty(order: usize) {
    let _ = circuit_adjacency_list_add_arc_empty(order);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_list_hash_set_collect(order: usize) {
    let _ = circuit_adjacency_list_hash_set_collect(order);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_map_collect(n: usize) {
    let _ = circuit_adjacency_map_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_map_unsafe_vec(n: usize) {
    let _ = circuit_adjacency_map_btree_set_unsafe_vec(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_map_unsafe_raw(n: usize) {
    let _ = circuit_adjacency_map_btree_set_unsafe_raw(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_map_insert_collect(n: usize) {
    let _ = circuit_adjacency_map_btree_set_insert_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::circuit(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn edge_list_btree_set_insert(n: usize) {
    let _ = circuit_edge_list_btree_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000, 1_000_000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = circuit_edge_list_btree_set_collect(n);
}
