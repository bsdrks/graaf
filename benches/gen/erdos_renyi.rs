//! Benchmarks of different implementations of `ErdosRenyi::erdos_renyi`.
use {
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        EdgeList,
        Empty,
        ErdosRenyi,
        r#gen::prng::Xoshiro256StarStar,
    },
    std::{
        cmp,
        collections::{
            BTreeMap,
            BTreeSet,
        },
        num::NonZero,
        thread,
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
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_list_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyList {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = AdjacencyList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_list_btree_set_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyListBTreeSet {
        arcs: (0..order)
            .map(|u| {
                (0..u)
                    .chain((u + 1)..order)
                    .filter(|_| rng.next_f64() < p)
                    .collect()
            })
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_map_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMap {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = AdjacencyMap::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_map_btree_set_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyMapBTreeSet {
        arcs: (0..order)
            .map(|u| {
                (
                    u,
                    (0..u)
                        .chain((u + 1)..order)
                        .filter(|_| rng.next_f64() < p)
                        .collect(),
                )
            })
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
#[allow(clippy::many_single_char_names)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn erdos_renyi_adjacency_map_btree_set_skip(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let mut rng = Xoshiro256StarStar::new(seed);
    let edges = order * (order - 1);
    let mut i = 0;
    let mut vec = vec![BTreeSet::new(); order];

    while i < edges {
        let r = loop {
            let r = rng.next_f64();

            if r > 0.0 {
                break r;
            }
        };

        let skip = r.log(1.0 - p).floor() as usize;

        i += skip;

        if i < edges {
            let order_minus_1 = order - 1;
            let u = i / order_minus_1;
            let j = i % order_minus_1;
            let v = if j >= u { j + 1 } else { j };
            let _ = unsafe { vec.get_unchecked_mut(u).insert(v) };

            i += 1;
        }
    }

    AdjacencyMapBTreeSet {
        arcs: vec.into_iter().enumerate().collect(),
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` isn't in `[0, 1]`.
fn erdos_renyi_adjacency_map_btree_set_for(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    if p == 0.0 {
        return AdjacencyMapBTreeSet {
            arcs: (0..order).map(|u| (u, BTreeSet::new())).collect(),
        };
    }

    let mut rng = Xoshiro256StarStar::new(seed);

    AdjacencyMapBTreeSet {
        arcs: (0..order)
            .map(|u| {
                let mut out_neighbors = BTreeSet::new();

                for v in 0..order {
                    if v != u && rng.next_f64() < p {
                        let _ = out_neighbors.insert(v);
                    }
                }

                (u, out_neighbors)
            })
            .collect(),
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` isn't in `[0, 1]`.
fn erdos_renyi_adjacency_map_btree_set_parallel(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMapBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    if order == 1 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::from([(0, BTreeSet::new())]),
        };
    }

    let num_threads = cmp::min(
        order,
        thread::available_parallelism().map_or(1, NonZero::get),
    );

    let chunk_size = order.div_ceil(num_threads);
    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let start = thread_id * chunk_size;
        let end = order.min(start + chunk_size);
        let thread_seed = seed.wrapping_add(thread_id as u64);

        let handle = thread::spawn(move || {
            let mut rng = Xoshiro256StarStar::new(thread_seed);
            let mut local_results = Vec::with_capacity(end - start);

            for u in start..end {
                let mut out_neighbors = BTreeSet::new();

                for v in 0..order {
                    if v != u && rng.next_f64() < p {
                        let _ = out_neighbors.insert(v);
                    }
                }

                local_results.push((u, out_neighbors));
            }

            local_results
        });

        handles.push(handle);
    }

    let mut results = Vec::with_capacity(order);

    for handle in handles {
        let local = handle.join().expect("Thread panicked");

        results.extend(local);
    }

    results.sort_by_key(|&(u, _)| u);

    AdjacencyMapBTreeSet {
        arcs: results.into_iter().collect::<BTreeMap<_, _>>(),
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_adjacency_matrix_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> AdjacencyMatrix {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = AdjacencyMatrix::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_edge_list_add_arc_empty(
    order: usize,
    p: f64,
    seed: u64,
) -> EdgeList {
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut digraph = EdgeList::empty(order);
    let mut rng = Xoshiro256StarStar::new(seed);

    for u in 0..order {
        for v in (0..order).filter(|&v| u != v) {
            if rng.next_f64() < p {
                digraph.add_arc(u, v);
            }
        }
    }

    digraph
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_edge_list_btree_set_collect_btree_set_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                (0..u)
                    .chain((u + 1)..order)
                    .filter(|_| rng.next_f64() < p)
                    .map(|v| (u, v))
                    .collect::<BTreeSet<_>>()
            })
            .collect(),
        order,
    }
}

/// # Panics
///
/// * Panics if `order` is zero.
/// * Panics if `p` is not in the interval `[0, 1]`.
fn erdos_renyi_edge_list_btree_set_collect_vec_collect(
    order: usize,
    p: f64,
    seed: u64,
) -> EdgeListBTreeSet {
    assert!(order > 0, "a digraph has at least one vertex");
    assert!((0.0..=1.0).contains(&p), "p = {p} must be in [0, 1]");

    let mut rng = Xoshiro256StarStar::new(seed);

    EdgeListBTreeSet {
        arcs: (0..order)
            .flat_map(|u| {
                (0..u)
                    .chain((u + 1)..order)
                    .filter(|_| rng.next_f64() < p)
                    .map(|v| (u, v))
                    .collect::<Vec<_>>()
            })
            .collect(),
        order,
    }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_0_5(n: usize) {
    let _ = AdjacencyList::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_add_arc_empty_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_list_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_btree_set_collect_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_list_btree_set_collect(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_0_5(n: usize) {
    let _ = AdjacencyMap::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_add_arc_empty_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_map_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_collect_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_collect(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_skip_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_skip(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_for_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_for(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_parallel_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_parallel(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix_0_5(n: usize) {
    let _ = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix_add_arc_empty_0_5(n: usize) {
    let _ = erdos_renyi_adjacency_matrix_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_0_5(n: usize) {
    let _ = EdgeList::erdos_renyi(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_add_arc_empty_0_5(n: usize) {
    let _ = erdos_renyi_edge_list_add_arc_empty(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_btree_list_collect_btree_set_collect_0_5(n: usize) {
    let _ =
        erdos_renyi_edge_list_btree_set_collect_btree_set_collect(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_btree_list_collect_vec_collect_0_5(n: usize) {
    let _ = erdos_renyi_edge_list_btree_set_collect_vec_collect(n, 0.5, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_0_1(n: usize) {
    let _ = AdjacencyList::erdos_renyi(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_add_arc_empty_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_list_add_arc_empty(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_btree_set_collect_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_list_btree_set_collect(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_0_1(n: usize) {
    let _ = AdjacencyMap::erdos_renyi(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_add_arc_empty_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_map_add_arc_empty(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_collect_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_collect(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_skip_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_skip(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_for_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_for(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_map_btree_set_parallel_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_map_btree_set_parallel(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix_0_1(n: usize) {
    let _ = AdjacencyMatrix::erdos_renyi(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_matrix_add_arc_empty_0_1(n: usize) {
    let _ = erdos_renyi_adjacency_matrix_add_arc_empty(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_0_1(n: usize) {
    let _ = EdgeList::erdos_renyi(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_add_arc_empty_0_1(n: usize) {
    let _ = erdos_renyi_edge_list_add_arc_empty(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_btree_list_collect_btree_set_collect_0_1(n: usize) {
    let _ =
        erdos_renyi_edge_list_btree_set_collect_btree_set_collect(n, 0.1, 0);
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn edge_list_btree_list_collect_vec_collect_0_1(n: usize) {
    let _ = erdos_renyi_edge_list_btree_set_collect_vec_collect(n, 0.1, 0);
}
