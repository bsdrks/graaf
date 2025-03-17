//! Benchmarks of different implementations of `Arcs::arcs`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        Arcs,
        ErdosRenyi,
    },
    std::collections::{
        btree_set,
        BTreeSet,
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

struct ArcsIterator<'a> {
    arcs_ptr: *const BTreeSet<usize>,
    len: usize,
    u: usize,
    inner: Option<btree_set::Iter<'a, usize>>,
}

impl Iterator for ArcsIterator<'_> {
    type Item = (usize, usize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                if let Some(ref mut inner) = self.inner {
                    if let Some(&v) = inner.next() {
                        return Some((self.u - 1, v));
                    }
                }

                if self.u >= self.len {
                    return None;
                }

                self.inner = Some((*self.arcs_ptr.add(self.u)).iter());
                self.u += 1;
            }
        }
    }
}

fn arcs_adjacency_list_btree_set_unsafe(
    digraph: &AdjacencyListBTreeSet,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    ArcsIterator {
        arcs_ptr: digraph.arcs.as_ptr(),
        len: digraph.arcs.len(),
        u: 0,
        inner: None,
    }
}

fn arcs_adjacency_list_btree_set(
    digraph: &AdjacencyListBTreeSet,
) -> impl Iterator<Item = (usize, usize)> + use<'_> {
    digraph
        .arcs
        .iter()
        .enumerate()
        .flat_map(|(u, set)| set.iter().map(move |&v| (u, v)))
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let digraph = AdjacencyListBTreeSet {
        arcs: digraph.arcs().fold(
            vec![BTreeSet::new(); order],
            |mut arcs, (u, v)| {
                let _ = arcs[u].insert(v);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ = arcs_adjacency_list_btree_set(&digraph).count();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn adjacency_list_erdos_renyi_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    let digraph = AdjacencyListBTreeSet {
        arcs: digraph.arcs().fold(
            vec![BTreeSet::new(); order],
            |mut arcs, (u, v)| {
                let _ = arcs[u].insert(v);

                arcs
            },
        ),
    };

    bencher.bench(|| {
        let _ = arcs_adjacency_list_btree_set_unsafe(&digraph).count();
    });
}
