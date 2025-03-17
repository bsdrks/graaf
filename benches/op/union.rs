//! Benchmarks of different implementations of `Union::union`.
use {
    divan::Bencher,
    graaf::{
        AddArc,
        AdjacencyList,
        AdjacencyMap,
        Arcs,
        ErdosRenyi,
        Order,
        Union,
    },
    std::{
        cmp::Ordering,
        collections::{
            BTreeMap,
            BTreeSet,
        },
        mem::ManuallyDrop,
        num::NonZero,
        ptr::{
            self,
            read,
        },
        thread::{
            available_parallelism,
            scope,
        },
    },
};

fn main() {
    divan::main();
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct AdjacencyListBTreeSet {
    pub arcs: Vec<BTreeSet<usize>>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct AdjacencyMapBTreeSet {
    pub arcs: BTreeMap<usize, BTreeSet<usize>>,
}

fn union_adjacency_list_add_arc(
    lhs: &AdjacencyList,
    rhs: &AdjacencyList,
) -> AdjacencyList {
    let (mut union, other) = if lhs.order() > rhs.order() {
        (lhs.clone(), rhs)
    } else {
        (rhs.clone(), lhs)
    };

    for (u, v) in other.arcs() {
        union.add_arc(u, v);
    }

    union
}

fn union_adjacency_map_add_arc(
    lhs: &AdjacencyMap,
    rhs: &AdjacencyMap,
) -> AdjacencyMap {
    let (mut union, other) = if lhs.order() > rhs.order() {
        (lhs.clone(), rhs)
    } else {
        (rhs.clone(), lhs)
    };

    for (u, v) in other.arcs() {
        union.add_arc(u, v);
    }

    union
}

fn union_adjacency_map_merge(
    lhs: &AdjacencyMapBTreeSet,
    rhs: &AdjacencyMapBTreeSet,
) -> AdjacencyMapBTreeSet {
    let mut arcs = BTreeMap::new();
    let mut iter_self = lhs.arcs.iter();
    let mut iter_other = rhs.arcs.iter();
    let mut cur_self = iter_self.next();
    let mut cur_other = iter_other.next();

    while cur_self.is_some() || cur_other.is_some() {
        match (cur_self, cur_other) {
            (Some((&u, set_self)), Some((&v, set_other))) => match u.cmp(&v) {
                Ordering::Equal => {
                    let union_set = set_self
                        .union(set_other)
                        .copied()
                        .collect::<BTreeSet<_>>();

                    let _ = arcs.insert(u, union_set);

                    cur_self = iter_self.next();
                    cur_other = iter_other.next();
                }
                Ordering::Less => {
                    let _ = arcs.insert(u, set_self.clone());

                    cur_self = iter_self.next();
                }
                Ordering::Greater => {
                    let _ = arcs.insert(v, set_other.clone());

                    cur_other = iter_other.next();
                }
            },
            (Some((&u, set_self)), None) => {
                let _ = arcs.insert(u, set_self.clone());

                cur_self = iter_self.next();
            }
            (None, Some((&v, set_other))) => {
                let _ = arcs.insert(v, set_other.clone());

                cur_other = iter_other.next();
            }
            (None, None) => break,
        }
    }

    AdjacencyMapBTreeSet { arcs }
}

unsafe fn merge_two_sorted(lhs: &[usize], rhs: &[usize]) -> Vec<usize> {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let mut out = Vec::with_capacity(lhs_len + rhs_len);
    let mut i = 0;
    let mut j = 0;

    while i < lhs_len && j < rhs_len {
        let a_i = *lhs.get_unchecked(i);
        let b_j = *rhs.get_unchecked(j);

        match a_i.cmp(&b_j) {
            Ordering::Less => {
                out.push(a_i);
                i += 1;
            }
            Ordering::Greater => {
                out.push(b_j);
                j += 1;
            }
            Ordering::Equal => {
                out.push(a_i);
                i += 1;
                j += 1;
            }
        }
    }

    while i < lhs.len() {
        out.push(*lhs.get_unchecked(i));
        i += 1;
    }

    while j < rhs.len() {
        out.push(*rhs.get_unchecked(j));
        j += 1;
    }

    out
}

unsafe fn union_sets_unsafe(
    set_a: &BTreeSet<usize>,
    set_b: &BTreeSet<usize>,
) -> BTreeSet<usize> {
    let vec_a: Vec<usize> = set_a.iter().copied().collect();
    let vec_b: Vec<usize> = set_b.iter().copied().collect();
    let merged = merge_two_sorted(&vec_a, &vec_b);

    merged.into_iter().collect()
}

fn find_partition(
    r: usize,
    lhs: &[(usize, BTreeSet<usize>)],
    rhs: &[(usize, BTreeSet<usize>)],
) -> (usize, usize) {
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    let mut lo = r.saturating_sub(rhs_len);
    let mut hi = if r < lhs_len { r } else { lhs_len };

    while lo < hi {
        let mid = usize::midpoint(lo, hi);
        let j = r - mid;

        if j < rhs.len()
            && unsafe { lhs.get_unchecked(mid) }.0
                > unsafe { rhs.get_unchecked(j) }.0
        {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    (lo, r - lo)
}

#[allow(clippy::too_many_lines)]
fn union_adjacency_map_merge_parallel(
    lhs: &AdjacencyMapBTreeSet,
    rhs: &AdjacencyMapBTreeSet,
) -> AdjacencyMapBTreeSet {
    let lhs_vec = lhs
        .arcs
        .iter()
        .map(|(&k, v)| (k, v.clone()))
        .collect::<Vec<_>>();

    let rhs_vec = rhs
        .arcs
        .iter()
        .map(|(&k, v)| (k, v.clone()))
        .collect::<Vec<_>>();

    let n1 = lhs_vec.len();
    let n2 = rhs_vec.len();
    let order = n1 + n2;

    if order == 0 {
        return AdjacencyMapBTreeSet {
            arcs: BTreeMap::new(),
        };
    }

    let lhs_vec = ManuallyDrop::new(lhs_vec);
    let rhs_vec = ManuallyDrop::new(rhs_vec);
    let available = available_parallelism().map_or(1, NonZero::get);
    let t = available.min(order);
    let mut partitions = Vec::with_capacity(t + 1);

    for k in 0..=t {
        partitions.push(k * order / t);
    }

    let mut boundaries = Vec::with_capacity(t + 1);
    let lhs_slice = &lhs_vec;
    let rhs_slice = &rhs_vec;

    for &r in &partitions {
        boundaries.push(find_partition(r, lhs_slice, rhs_slice));
    }

    let mut merged_entries = Vec::with_capacity(order);

    scope(|s| {
        let mut handles = Vec::with_capacity(t);

        for k in 0..t {
            let (i_start, j_start) = unsafe { *boundaries.get_unchecked(k) };
            let (i_end, j_end) = unsafe { *boundaries.get_unchecked(k + 1) };

            handles.push(s.spawn(move || {
                let mut local = Vec::new();

                unsafe {
                    let lhs_ptr = lhs_slice.as_ptr();
                    let rhs_ptr = rhs_slice.as_ptr();
                    let mut i = i_start;
                    let mut j = j_start;

                    while (i < i_end) || (j < j_end) {
                        if i < i_end && j < j_end {
                            let a = &*lhs_ptr.add(i);
                            let b = &*rhs_ptr.add(j);

                            match a.0.cmp(&b.0) {
                                Ordering::Less => {
                                    local.push(read(lhs_ptr.add(i)));
                                    i += 1;
                                }
                                Ordering::Greater => {
                                    local.push(read(rhs_ptr.add(j)));
                                    j += 1;
                                }
                                Ordering::Equal => {
                                    let union_set =
                                        union_sets_unsafe(&a.1, &b.1);

                                    local.push((a.0, union_set));
                                    i += 1;
                                    j += 1;
                                }
                            }
                        } else if i < i_end {
                            local.push(ptr::read(lhs_ptr.add(i)));
                            i += 1;
                        } else {
                            local.push(ptr::read(rhs_ptr.add(j)));
                            j += 1;
                        }
                    }
                }

                local
            }));
        }

        for h in handles {
            merged_entries.extend(h.join().unwrap());
        }
    });

    merged_entries.sort_unstable_by_key(|&(k, _)| k);

    let mut final_entries = Vec::with_capacity(merged_entries.len());

    if !merged_entries.is_empty() {
        let mut current = merged_entries.remove(0);

        for entry in merged_entries {
            if entry.0 == current.0 {
                current.1 = unsafe { union_sets_unsafe(&current.1, &entry.1) };
            } else {
                final_entries.push(current);
                current = entry;
            }
        }

        final_entries.push(current);
    }

    AdjacencyMapBTreeSet {
        arcs: final_entries.into_iter().collect(),
    }
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let lhs = AdjacencyList::erdos_renyi(order * 80 / 100, 0.5, 0);
    let rhs = AdjacencyList::erdos_renyi(order, 0.6, 1);

    bencher.bench(|| {
        let _ = rhs.union(&lhs);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_add_arc(bencher: Bencher<'_, '_>, order: usize) {
    let lhs = AdjacencyList::erdos_renyi(order * 80 / 100, 0.5, 0);
    let rhs = AdjacencyList::erdos_renyi(order, 0.6, 1);

    bencher.bench(|| {
        let _ = union_adjacency_list_add_arc(&lhs, &rhs);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let lhs = AdjacencyMap::erdos_renyi(order * 80 / 100, 0.5, 0);
    let rhs = AdjacencyMap::erdos_renyi(order, 0.6, 1);

    bencher.bench(|| {
        let _ = rhs.union(&lhs);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_add_arc(bencher: Bencher<'_, '_>, order: usize) {
    let lhs = AdjacencyMap::erdos_renyi(order * 80 / 100, 0.5, 0);
    let rhs = AdjacencyMap::erdos_renyi(order, 0.6, 1);

    bencher.bench(|| {
        let _ = union_adjacency_map_add_arc(&lhs, &rhs);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_erdos_renyi_merge(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let lhs = AdjacencyMapBTreeSet {
        arcs: AdjacencyMap::erdos_renyi(order * 80 / 100, 0.5, 0)
            .arcs()
            .fold(BTreeMap::new(), |mut arcs, (u, v)| {
                let _ = arcs.entry(u).or_default().insert(v);

                arcs
            }),
    };

    let rhs = AdjacencyMapBTreeSet {
        arcs: AdjacencyMap::erdos_renyi(order, 0.6, 1).arcs().fold(
            BTreeMap::new(),
            |mut arcs, (u, v)| {
                let _ = arcs.entry(u).or_default().insert(v);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ = union_adjacency_map_merge(&lhs, &rhs);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_btree_set_erdos_renyi_parallel(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let lhs = AdjacencyMapBTreeSet {
        arcs: AdjacencyMap::erdos_renyi(order * 80 / 100, 0.5, 0)
            .arcs()
            .fold(BTreeMap::new(), |mut arcs, (u, v)| {
                let _ = arcs.entry(u).or_default().insert(v);

                arcs
            }),
    };

    let rhs = AdjacencyMapBTreeSet {
        arcs: AdjacencyMap::erdos_renyi(order, 0.6, 1).arcs().fold(
            BTreeMap::new(),
            |mut arcs, (u, v)| {
                let _ = arcs.entry(u).or_default().insert(v);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ = union_adjacency_map_merge_parallel(&lhs, &rhs);
    });
}
