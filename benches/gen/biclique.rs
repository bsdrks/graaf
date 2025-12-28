//! Benchmarks of different implementations of `Biclique::biclique`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Biclique,
        EdgeList,
        Empty,
    },
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
            HashSet,
        },
        iter::{
            repeat_n,
            repeat_with,
        },
        mem::{
            MaybeUninit,
            transmute,
        },
        num::NonZero,
        sync::Arc,
        thread::{
            available_parallelism,
            spawn,
        },
    },
};

fn main() {
    divan::main();
}

#[derive(Debug)]
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

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
struct EdgeListVec {
    pub arcs: Vec<(usize, usize)>,
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_add_arc_empty(m: usize, n: usize) -> AdjacencyList {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut digraph = AdjacencyList::empty(order);

    for u in 0..m {
        for v in m..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_clone_from(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    let mut digraph = AdjacencyListBTreeSet {
        arcs: vec![BTreeSet::new(); order],
    };

    for u in 0..m {
        unsafe { digraph.arcs.get_unchecked_mut(u).clone_from(&clique_2) };
    }

    for u in m..order {
        unsafe { digraph.arcs.get_unchecked_mut(u).clone_from(&clique_1) };
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_collect(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    AdjacencyListBTreeSet {
        arcs: (0..m)
            .map(|_| clique_2.clone())
            .chain((m..order).map(|_| clique_1.clone()))
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_repeat(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    AdjacencyListBTreeSet {
        arcs: repeat_n(clique_2, m).chain(repeat_n(clique_1, n)).collect(),
    }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_parallel(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = Arc::new((0..m).collect::<BTreeSet<usize>>());
    let clique_2 = Arc::new((m..order).collect::<BTreeSet<usize>>());

    let num_threads =
        order.min(available_parallelism().map_or(1, NonZero::get));

    let chunk_size = order.div_ceil(num_threads);

    let mut arcs: Vec<MaybeUninit<BTreeSet<usize>>> =
        repeat_with(MaybeUninit::uninit).take(order).collect();

    let arcs_ptr_usize = arcs.as_mut_ptr() as usize;
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let start = thread_id * chunk_size;
        let end = order.min(start + chunk_size);
        let clique_1 = Arc::clone(&clique_1);
        let clique_2 = Arc::clone(&clique_2);
        let thread_arcs_ptr_usize = arcs_ptr_usize;

        let handle = spawn(move || {
            let thread_arcs_ptr =
                thread_arcs_ptr_usize as *mut MaybeUninit<BTreeSet<usize>>;

            for u in start..end {
                unsafe {
                    if u < m {
                        *thread_arcs_ptr.add(u) =
                            MaybeUninit::new((*clique_2).clone());
                    } else {
                        *thread_arcs_ptr.add(u) =
                            MaybeUninit::new((*clique_1).clone());
                    }
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        unsafe {
            handle.join().unwrap_unchecked();
        }
    }

    let arcs = unsafe {
        arcs.into_iter()
            .map(|x| x.assume_init())
            .collect::<Vec<_>>()
    };

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_hash_set_clone_from(
    m: usize,
    n: usize,
) -> AdjacencyListHashSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<HashSet<_>>();
    let clique_2 = (m..order).collect::<HashSet<_>>();

    let mut digraph = AdjacencyListHashSet {
        arcs: vec![HashSet::new(); order],
    };

    for u in 0..m {
        digraph.arcs[u].clone_from(&clique_2);
    }

    for u in m..order {
        digraph.arcs[u].clone_from(&clique_1);
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_list_btree_set_extend(
    m: usize,
    n: usize,
) -> AdjacencyListBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();
    let mut arcs = Vec::with_capacity(order);

    arcs.extend(vec![clique_2; m]);
    arcs.extend(vec![clique_1; n]);

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_map_add_arc_empty(m: usize, n: usize) -> AdjacencyMap {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut digraph = AdjacencyMap::empty(order);

    for u in 0..m {
        for v in m..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_map_btree_set_clone_from(
    m: usize,
    n: usize,
) -> AdjacencyMapBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    let mut arcs = BTreeMap::new();

    for u in 0..m {
        let _ = arcs.insert(u, clique_2.clone());
    }

    for u in m..order {
        let _ = arcs.insert(u, clique_1.clone());
    }

    AdjacencyMapBTreeSet { arcs }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_map_btree_set_repeat(
    m: usize,
    n: usize,
) -> AdjacencyMapBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_1 = (0..m).collect::<BTreeSet<_>>();
    let clique_2 = (m..order).collect::<BTreeSet<_>>();

    let arcs = repeat_n(clique_2, m)
        .chain(repeat_n(clique_1, n))
        .enumerate()
        .collect();

    AdjacencyMapBTreeSet { arcs }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_adjacency_map_btree_set_loop(
    m: usize,
    n: usize,
) -> AdjacencyMapBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut arcs = BTreeMap::new();

    for u in 0..order {
        let mut set = BTreeSet::new();

        if u < m {
            // Vertices 0..m: their neighbors are all vertices in the
            // second partition: [m, m+n)
            for v in m..order {
                let _ = set.insert(v);
            }
        } else {
            // Vertices m..order: their neighbors are all vertices in the
            // first partition: [0, m)
            for v in 0..m {
                let _ = set.insert(v);
            }
        }

        let _ = arcs.insert(u, set);
    }

    AdjacencyMapBTreeSet { arcs }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
#[allow(clippy::transmute_undefined_repr)]
fn biclique_unsafe(m: usize, n: usize) -> AdjacencyMapBTreeSet {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut vec: Vec<MaybeUninit<(usize, BTreeSet<usize>)>> =
        Vec::with_capacity(order);

    unsafe {
        let ptr = vec.as_mut_ptr();

        for u in 0..order {
            let mut set = BTreeSet::new();

            if u < m {
                for v in m..order {
                    let _ = set.insert(v);
                }
            } else {
                for v in 0..m {
                    let _ = set.insert(v);
                }
            }

            ptr.add(u).write(MaybeUninit::new((u, set)));
        }

        vec.set_len(order);

        let vec = transmute::<
            Vec<MaybeUninit<(usize, BTreeSet<usize>)>>,
            Vec<(usize, BTreeSet<usize>)>,
        >(vec);

        let arcs: BTreeMap<usize, BTreeSet<usize>> = vec.into_iter().collect();

        AdjacencyMapBTreeSet { arcs }
    }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_edge_list_add_arc_empty(m: usize, n: usize) -> EdgeList {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let mut digraph = EdgeList::empty(order);

    for u in 0..m {
        for v in m..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_edge_list_flat_map(m: usize, n: usize) -> EdgeListVec {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;

    EdgeListVec {
        arcs: (0..m)
            .flat_map(|u| (m..order).map(move |v| (u, v)))
            .chain((m..order).flat_map(|u| (0..m).map(move |v| (u, v))))
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `m` is zero.
/// * Panics if `n` is zero.
fn biclique_edge_list_flat_map_clone(m: usize, n: usize) -> EdgeListVec {
    assert!(m > 0, "m = {m} must be greater than zero");
    assert!(n > 0, "n = {n} must be greater than zero");

    let order = m + n;
    let clique_2 = m..order;

    EdgeListVec {
        arcs: (0..m)
            .flat_map(|u| clique_2.clone().map(move |v| (u, v)))
            .chain(clique_2.clone().flat_map(|u| (0..m).map(move |v| (u, v))))
            .collect(),
    }
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list((m, n): (usize, usize)) {
    let _ = AdjacencyList::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_add_arc_empty((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_add_arc_empty(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_clone_from((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_clone_from(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_collect((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_collect(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_repeat((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_repeat(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_extend((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_extend(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_hash_set_clone_from((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_hash_set_clone_from(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_list_btree_set_parallel((m, n): (usize, usize)) {
    let _ = biclique_adjacency_list_btree_set_parallel(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map((m, n): (usize, usize)) {
    let _ = AdjacencyMap::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_add_arc_empty((m, n): (usize, usize)) {
    let _ = biclique_adjacency_map_add_arc_empty(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_btree_set_clone_from((m, n): (usize, usize)) {
    let _ = biclique_adjacency_map_btree_set_clone_from(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_btree_set_repeat((m, n): (usize, usize)) {
    let _ = biclique_adjacency_map_btree_set_repeat(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_btree_set_loop((m, n): (usize, usize)) {
    let _ = biclique_adjacency_map_btree_set_loop(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_map_unsafe((m, n): (usize, usize)) {
    let _ = biclique_unsafe(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn adjacency_matrix((m, n): (usize, usize)) {
    let _ = AdjacencyMatrix::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list((m, n): (usize, usize)) {
    let _ = EdgeList::biclique(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list_add_arc_empty((m, n): (usize, usize)) {
    let _ = biclique_edge_list_add_arc_empty(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list_flat_map((m, n): (usize, usize)) {
    let _ = biclique_edge_list_flat_map(m, n);
}

#[divan::bench(args = [
    (10, 10),
    (10, 100),
    (10, 1000),
    (10, 10000),
    (100, 100),
    (100, 1000),
    (100, 10000),
])]
fn edge_list_flat_map_clone((m, n): (usize, usize)) {
    let _ = biclique_edge_list_flat_map_clone(m, n);
}
