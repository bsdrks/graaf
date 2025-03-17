//! Benchmarks of different implementations of `Complete::complete`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Complete,
        EdgeList,
        Empty,
    },
    std::{
        collections::{
            BTreeMap,
            BTreeSet,
            HashSet,
        },
        num::NonZero,
        thread::available_parallelism,
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
struct AdjacencyMapTreeSet {
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
fn complete_adjacency_list_btree_set_push(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);

    for u in 0..order {
        arcs.push((0..u).chain((u + 1)..order).collect());
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_collect(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListBTreeSet {
        arcs: (0..order)
            .map(|u| (0..u).chain((u + 1)..order).collect())
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_insert(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![BTreeSet::new(); order];

    for u in 0..order {
        for v in (u + 1)..order {
            let _ = arcs[u].insert(v);
            let _ = arcs[v].insert(u);
        }
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_remove(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let neighbors = (0..order).collect::<BTreeSet<usize>>();
    let mut arcs = vec![neighbors; order];

    for (u, neighbors) in arcs.iter_mut().enumerate().take(order) {
        let _ = neighbors.remove(&u);
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_parallel_for_for(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListBTreeSet {
            arcs: vec![BTreeSet::new()],
        };
    }

    let num_threads =
        order.min(available_parallelism().map_or(1, NonZero::get));

    let chunk_size = order.div_ceil(num_threads);
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let start = thread_id * chunk_size;
        let end = ((thread_id + 1) * chunk_size).min(order);

        let handle = std::thread::spawn(move || {
            let mut local = Vec::with_capacity(end - start);

            for u in start..end {
                let mut set = BTreeSet::new();

                for v in 0..u {
                    let _ = set.insert(v);
                }

                for v in (u + 1)..order {
                    let _ = set.insert(v);
                }

                local.push((u, set));
            }

            local
        });

        handles.push(handle);
    }

    let mut arcs = Vec::with_capacity(order);

    for handle in handles {
        unsafe {
            arcs.extend(handle.join().unwrap_unchecked());
        }
    }

    arcs.sort_unstable_by_key(|&(u, _)| u);

    AdjacencyListBTreeSet {
        arcs: arcs.into_iter().map(|(_, s)| s).collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_btree_set_parallel_remove(
    order: usize,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyListBTreeSet {
            arcs: vec![BTreeSet::new()],
        };
    }

    let num_threads =
        order.min(available_parallelism().map_or(1, NonZero::get));

    let chunk_size = order.div_ceil(num_threads);
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let start = thread_id * chunk_size;
        let end = ((thread_id + 1) * chunk_size).min(order);

        let handle = std::thread::spawn(move || {
            let mut local = Vec::with_capacity(end - start);
            let vertices = (0..order).collect::<BTreeSet<_>>();

            for u in start..end {
                let mut out_neighbors = vertices.clone();
                let _ = out_neighbors.remove(&u);

                local.push((u, out_neighbors));
            }

            local
        });

        handles.push(handle);
    }

    let mut arcs = Vec::with_capacity(order);

    for handle in handles {
        unsafe {
            arcs.extend(handle.join().unwrap_unchecked());
        }
    }

    arcs.sort_unstable_by_key(|&(u, _)| u);

    AdjacencyListBTreeSet {
        arcs: arcs.into_iter().map(|(_, s)| s).collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_hash_set_push(
    order: usize,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);

    for u in 0..order {
        arcs.push((0..u).chain((u + 1)..order).collect());
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_hash_set_collect(
    order: usize,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyListHashSet {
        arcs: (0..order)
            .map(|u| (0..u).chain((u + 1)..order).collect())
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_list_hash_set_insert(
    order: usize,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = vec![HashSet::new(); order];

    for u in 0..order {
        for v in (u + 1)..order {
            let _ = arcs[u].insert(v);
            let _ = arcs[v].insert(u);
        }
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_map_add_arc_empty(order: usize) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);

    for u in 0..order {
        for v in (u + 1)..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_map_btree_set_collect(
    order: usize,
) -> AdjacencyMapTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    AdjacencyMapTreeSet {
        arcs: (0..order)
            .map(|u| (u, (0..u).chain((u + 1)..order).collect()))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_map_btree_set_insert(
    order: usize,
) -> AdjacencyMapTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let mut arcs = BTreeMap::new();

    for u in 0..order {
        let neighbors = (0..order).filter(|&v| v != u).collect();
        let _ = arcs.insert(u, neighbors);
    }

    AdjacencyMapTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_map_btree_set_remove(
    order: usize,
) -> AdjacencyMapTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let vertices = (0..order).collect::<BTreeSet<_>>();
    let mut arcs = BTreeMap::new();

    for u in 0..order {
        let mut out_neighbors = vertices.clone();
        let _ = out_neighbors.remove(&u);
        let _ = arcs.insert(u, out_neighbors);
    }

    AdjacencyMapTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_adjacency_map_btree_set_split_off(
    order: usize,
) -> AdjacencyMapTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let vertices = (0..order).collect::<BTreeSet<_>>();
    let mut arcs = BTreeMap::new();

    for u in 0..order {
        let mut left = vertices.clone();
        let right = left.split_off(&(u + 1));
        let _ = left.remove(&u);
        let vertices = left.into_iter().chain(right).collect();
        let _ = arcs.insert(u, vertices);
    }

    AdjacencyMapTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_edge_list_add_arc_empty(order: usize) -> EdgeList {
    let mut digraph = EdgeList::empty(order);

    for u in 0..order {
        for v in (u + 1)..order {
            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn complete_edge_btree_set_list_collect(order: usize) -> EdgeListBTreeSet {
    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| (0..u).chain((u + 1)..order).map(move |v| (u, v)))
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_push(n: usize) {
    let _ = complete_adjacency_list_btree_set_push(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = complete_adjacency_list_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_insert(n: usize) {
    let _ = complete_adjacency_list_btree_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_remove(n: usize) {
    let _ = complete_adjacency_list_btree_set_remove(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_parallel_for_for(n: usize) {
    let _ = complete_adjacency_list_btree_set_parallel_for_for(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_btree_set_parallel_remove(n: usize) {
    let _ = complete_adjacency_list_btree_set_parallel_remove(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_push(n: usize) {
    let _ = complete_adjacency_list_hash_set_push(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_collect(n: usize) {
    let _ = complete_adjacency_list_hash_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_hash_set_insert(n: usize) {
    let _ = complete_adjacency_list_hash_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = complete_adjacency_map_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = complete_adjacency_map_btree_set_collect(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_insert(n: usize) {
    let _ = complete_adjacency_map_btree_set_insert(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_remove(n: usize) {
    let _ = complete_adjacency_map_btree_set_remove(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_split_off(n: usize) {
    let _ = complete_adjacency_map_btree_set_split_off(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list(n: usize) {
    let _ = EdgeList::complete(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = complete_edge_list_add_arc_empty(n);
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = complete_edge_btree_set_list_collect(n);
}
