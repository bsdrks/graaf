//! Benchmarks of different implementations of
//! `GrowingNetwork::growing_network`.
use {
    graaf::{
        gen::prng::Xoshiro256StarStar, AddArc, AdjacencyList, AdjacencyMap,
        AdjacencyMatrix, EdgeList, Empty, RandomRecursiveTree,
    },
    std::{
        collections::{BTreeMap, BTreeSet, HashSet},
        iter::once,
        mem::{transmute, MaybeUninit},
        num::NonZero,
        ptr, thread,
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
fn growing_network_adjacency_list_btree_set_collect(
    order: usize,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyListBTreeSet {
        arcs: once(BTreeSet::new())
            .chain((1..order).map(|u| {
                BTreeSet::from([usize::try_from(rng.next().unwrap())
                    .expect("conversion failed")
                    % u])
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_btree_set_push(
    order: usize,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        arcs.push(BTreeSet::from([usize::try_from(v)
            .expect("conversion failed")
            % u]));
    }

    AdjacencyListBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_btree_set_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyList {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut digraph = AdjacencyList::empty(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        digraph.add_arc(u, usize::try_from(v).expect("conversion failed") % u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_hash_set_collect(
    order: usize,
    seed: u64,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyListHashSet {
        arcs: once(HashSet::new())
            .chain((1..order).map(|u| {
                HashSet::from([usize::try_from(rng.next().unwrap())
                    .expect("conversion failed")
                    % u])
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_list_hash_set_push(
    order: usize,
    seed: u64,
) -> AdjacencyListHashSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut arcs = Vec::with_capacity(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        arcs.push(HashSet::from([usize::try_from(v)
            .expect("conversion failed")
            % u]));
    }

    AdjacencyListHashSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_map_add_arc_empty(
    order: usize,
    seed: u64,
) -> AdjacencyMap {
    let mut digraph = AdjacencyMap::empty(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        digraph.add_arc(u, usize::try_from(v).expect("conversion failed") % u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_adjacency_map_collect(
    order: usize,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyMapBTreeSet {
        arcs: once((0, BTreeSet::new()))
            .chain((1..order).map(|u| {
                (
                    u,
                    BTreeSet::from([usize::try_from(rng.next().unwrap())
                        .expect("conversion failed")
                        % u]),
                )
            }))
            .collect(),
    }
}

/// # Panics
///
/// Panics if `order` is zero.
#[allow(clippy::cast_possible_truncation)]
fn growing_network_adjacency_map_btree_set_parallel(
    order: usize,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let num_threads = (order - 1)
        .min(std::thread::available_parallelism().map_or(1, NonZero::get));

    let chunk_size = (order - 1).div_ceil(num_threads);
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let start = 1 + thread_id * chunk_size;
        let end = order.min(1 + (thread_id + 1) * chunk_size);
        let thread_seed = seed.wrapping_add(thread_id as u64);

        let handle = thread::spawn(move || {
            let mut rng = Xoshiro256StarStar::new(thread_seed);
            let mut local_edges = Vec::with_capacity(end - start);

            for u in start..end {
                let parent =
                    unsafe { rng.next().unwrap_unchecked() as usize % u };

                local_edges.push((u, parent));
            }

            local_edges
        });

        handles.push(handle);
    }

    let mut parent_edges = Vec::with_capacity(order - 1);

    for handle in handles {
        let mut local = unsafe { handle.join().unwrap_unchecked() };

        parent_edges.append(&mut local);
    }

    parent_edges.sort_unstable_by_key(|&(u, _)| u);

    let mut arcs = BTreeMap::new();
    let _ = arcs.insert(0, BTreeSet::new());

    for (u, parent) in parent_edges {
        let _ = arcs.insert(u, BTreeSet::from([parent]));
    }

    AdjacencyMapBTreeSet { arcs }
}

/// # Panics
///
/// Panics if `order` is zero.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::transmute_undefined_repr)]
fn growing_network_adjacency_map_unsafe(
    order: usize,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let mut rng = Xoshiro256StarStar::new(seed);

    let mut vec: Vec<MaybeUninit<(usize, BTreeSet<usize>)>> =
        Vec::with_capacity(order);

    let ptr: *mut MaybeUninit<(usize, BTreeSet<usize>)> = vec.as_mut_ptr();

    unsafe {
        ptr::write(ptr, MaybeUninit::new((0, BTreeSet::new())));

        for u in 1..order {
            let r = rng.next().unwrap_unchecked();
            let parent = (r % (u as u64)) as usize;

            ptr::write(
                ptr.add(u),
                MaybeUninit::new((u, BTreeSet::from([parent]))),
            );
        }

        vec.set_len(order);

        let vec = transmute::<
            Vec<MaybeUninit<(usize, BTreeSet<usize>)>>,
            Vec<(usize, BTreeSet<usize>)>,
        >(vec);

        let arcs = vec.into_iter().collect();

        AdjacencyMapBTreeSet { arcs }
    }
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_edge_list_add_arc_empty(
    order: usize,
    seed: u64,
) -> EdgeList {
    let mut digraph = EdgeList::empty(order);
    let rng = Xoshiro256StarStar::new(seed);

    for (u, v) in (1..order).zip(rng) {
        digraph.add_arc(u, usize::try_from(v).expect("conversion failed") % u);
    }

    digraph
}

/// # Panics
///
/// Panics if `order` is zero.
fn growing_network_btree_set_edge_list_collect(
    order: usize,
    seed: u64,
) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");

    let mut rng = Xoshiro256StarStar::new(seed);

    EdgeListBTreeSet {
        arcs: (1..order)
            .map(|u| {
                (
                    u,
                    usize::try_from(rng.next().unwrap())
                        .expect("conversion failed")
                        % u,
                )
            })
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list(n: usize) {
    let _ = AdjacencyList::random_recursive_tree(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_collect(n: usize) {
    let _ = growing_network_adjacency_list_btree_set_collect(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_push(n: usize) {
    let _ = growing_network_adjacency_list_btree_set_push(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_btree_set_add_arc_empty(n: usize) {
    let _ = growing_network_adjacency_list_btree_set_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_collect(n: usize) {
    let _ = growing_network_adjacency_list_hash_set_collect(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_list_hash_set_push(n: usize) {
    let _ = growing_network_adjacency_list_hash_set_push(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_matrix(n: usize) {
    let _ = AdjacencyMatrix::random_recursive_tree(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map(n: usize) {
    let _ = AdjacencyMap::random_recursive_tree(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_add_arc_empty(n: usize) {
    let _ = growing_network_adjacency_map_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_collect(n: usize) {
    let _ = growing_network_adjacency_map_collect(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_parallel(n: usize) {
    let _ = growing_network_adjacency_map_btree_set_parallel(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_btree_set_pregenerate(n: usize) {
    let _ = growing_network_adjacency_map_unsafe(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn adjacency_map_unsafe_vec(n: usize) {
    let _ = growing_network_adjacency_map_unsafe(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list(n: usize) {
    let _ = EdgeList::random_recursive_tree(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_add_arc_empty(n: usize) {
    let _ = growing_network_edge_list_add_arc_empty(n, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000, 100_000])]
fn edge_list_btree_set_collect(n: usize) {
    let _ = growing_network_btree_set_edge_list_collect(n, 0);
}
