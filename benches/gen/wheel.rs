//! Benchmarks of different implementations of `Wheel::wheel`.
use {
    graaf::{
        AddArc, AdjacencyList, AdjacencyMap, AdjacencyMatrix, EdgeList, Empty,
        Wheel,
    },
    std::{
        collections::{BTreeMap, BTreeSet},
        iter::once,
        mem::MaybeUninit,
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
/// Panics if `order` is less than `4`.
fn wheel_adjacency_list_add_arc_empty(order: usize) -> AdjacencyList {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    let mut digraph = AdjacencyList::empty(order);

    for u in 1..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 1);
    digraph.add_arc(1, u);

    for u in 1..order {
        digraph.add_arc(0, u);
        digraph.add_arc(u, 0);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_adjacency_list_btree_set_collect(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    AdjacencyListBTreeSet {
        arcs: once((1..order).collect())
            .chain((1..order).map(|u| {
                let last = order - 1;

                BTreeSet::from([
                    0,
                    if u == 1 { last } else { u - 1 },
                    if u == last { 1 } else { u + 1 },
                ])
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_adjacency_map_add_arc_empty(order: usize) -> AdjacencyMap {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    let mut digraph = AdjacencyMap::empty(order);

    for u in 1..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 1);
    digraph.add_arc(1, u);

    for u in 1..order {
        digraph.add_arc(0, u);
        digraph.add_arc(u, 0);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_adjacency_map_btree_set_collect_from(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    AdjacencyMapBTreeSet {
        arcs: once((0, (1..order).collect()))
            .chain((1..order).map(|u| {
                (
                    u,
                    BTreeSet::from([
                        0,
                        (u + order - 1) % order,
                        (u + 1) % order,
                    ]),
                )
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_adjacency_map_btree_set_collect_collect(
    order: usize,
) -> AdjacencyMapBTreeSet {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    AdjacencyMapBTreeSet {
        arcs: once((0, (1..order).collect()))
            .chain((1..order).map(|u| {
                let last = order - 1;

                (
                    u,
                    once(0)
                        .chain(once(if u == 1 { last } else { u - 1 }))
                        .chain(once(if u == last { 1 } else { u + 1 }))
                        .collect(),
                )
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_adjacency_matrix_add_arc_empty(order: usize) -> AdjacencyMatrix {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    let mut digraph = AdjacencyMatrix::empty(order);

    for u in 1..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 1);
    digraph.add_arc(1, u);

    for u in 1..order {
        digraph.add_arc(0, u);
        digraph.add_arc(u, 0);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is less than `4`.
#[allow(clippy::ptr_as_ptr)]
#[allow(clippy::as_ptr_cast_mut)]
fn wheel_adjacency_map_btree_set_unsafe(order: usize) -> AdjacencyMapBTreeSet {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    let mut arcs: Vec<MaybeUninit<(usize, BTreeSet<usize>)>> =
        std::iter::repeat_with(MaybeUninit::uninit)
            .take(order)
            .collect();
    let arcs_ptr = arcs.as_mut_ptr();

    unsafe {
        *arcs_ptr = MaybeUninit::new((0, (1..order).collect()));
    }

    let last = order - 1;
    for u in 1..order {
        let neighbor1 = if u == 1 { last } else { u - 1 };
        let neighbor2 = if u == last { 1 } else { u + 1 };
        let mut arr = [0, neighbor1, neighbor2];
        arr.sort_unstable();
        unsafe {
            *arcs_ptr.add(u) = MaybeUninit::new((u, BTreeSet::from(arr)));
        }
    }

    AdjacencyMapBTreeSet {
        arcs: arcs
            .into_iter()
            .map(|x| unsafe { x.assume_init() })
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_edge_list_add_arc_empty(order: usize) -> EdgeList {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    let mut digraph = EdgeList::empty(order);

    for u in 1..order - 1 {
        let v = u + 1;

        digraph.add_arc(u, v);
        digraph.add_arc(v, u);
    }

    let u = order - 1;

    digraph.add_arc(u, 1);
    digraph.add_arc(1, u);

    for u in 1..order {
        digraph.add_arc(0, u);
        digraph.add_arc(u, 0);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_edge_list_btree_set_collect(order: usize) -> EdgeListBTreeSet {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    EdgeListBTreeSet {
        arcs: (1..order)
            .map(|v| (0, v))
            .chain((1..order).flat_map(|u| {
                let last = order - 1;

                once((u, 0))
                    .chain(once((u, if u == 1 { last } else { u - 1 })))
                    .chain(once((u, if u == last { 1 } else { u + 1 })))
            }))
            .collect(),
        order,
    }
}

/// # Panics
///
/// Panics if `order` is less than `4`.
fn wheel_edge_list_btree_set_collect_map(order: usize) -> EdgeListBTreeSet {
    assert!(order >= 4, "a wheel digraph has at least four vertices");

    EdgeListBTreeSet {
        arcs: (1..order)
            .map(|v| (0, v))
            .chain((1..order).flat_map(|u| {
                let last = order - 1;

                once(0)
                    .chain(once(if u == 1 { last } else { u - 1 }))
                    .chain(once(if u == last { 1 } else { u + 1 }))
                    .map(move |v| (u, v))
            }))
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::wheel(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_add_arc_empty(n: usize) {
    let _ = wheel_adjacency_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = wheel_adjacency_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::wheel(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = wheel_adjacency_map_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_collect_from(n: usize) {
    let _ = wheel_adjacency_map_btree_set_collect_from(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_collect_collect(n: usize) {
    let _ = wheel_adjacency_map_btree_set_collect_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_unsafe(n: usize) {
    let _ = wheel_adjacency_map_btree_set_unsafe(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::wheel(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix_add_arc_empty(n: usize) {
    let _ = wheel_adjacency_matrix_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::wheel(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = wheel_edge_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = wheel_edge_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_collect_map(n: usize) {
    let _ = wheel_edge_list_btree_set_collect_map(n);
}
