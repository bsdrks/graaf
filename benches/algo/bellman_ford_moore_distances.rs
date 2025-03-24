//! Benchmarks of different implementations of `BellmanFordMoore::distances`.
use {
    divan::Bencher,
    graaf::{
        AddArcWeighted,
        AdjacencyList,
        AdjacencyListWeighted,
        Arcs,
        ArcsWeighted,
        BellmanFordMoore,
        Empty,
        ErdosRenyi,
        Order,
    },
};

fn main() {
    divan::main();
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BellmanFordMooreForFor<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BellmanFordMooreIter<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BellmanFordMooreUnsafe<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
    dist_ptr: *mut isize,
}

impl<'a, D> BellmanFordMooreForFor<'a, D> {
    pub fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        Self {
            digraph,
            dist: (0..digraph.order())
                .map(|u| if u == s { 0 } else { isize::MAX })
                .collect(),
        }
    }

    #[must_use]
    pub fn distances(&mut self) -> Option<&[isize]>
    where
        D: ArcsWeighted<Weight = isize> + Order,
    {
        for _ in 1..self.digraph.order() {
            for (u, v, &w) in self.digraph.arcs_weighted() {
                self.dist[v] =
                    self.dist[v].min(self.dist[u].saturating_add(w));
            }
        }

        for (u, v, &w) in self.digraph.arcs_weighted() {
            if self.dist[v] > self.dist[u].saturating_add(w) {
                return None;
            }
        }

        Some(&self.dist)
    }
}

impl<'a, D> BellmanFordMooreIter<'a, D> {
    pub fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        Self {
            digraph,
            dist: (0..digraph.order())
                .map(|u| if u == s { 0 } else { isize::MAX })
                .collect(),
        }
    }

    #[must_use]
    pub fn distances(&mut self) -> Option<&[isize]>
    where
        D: ArcsWeighted<Weight = isize> + Order,
    {
        for _ in 1..self.digraph.order() {
            let mut updated = false;

            for (u, v, &w) in self.digraph.arcs_weighted() {
                let u = self.dist[u];

                if u == isize::MAX {
                    continue;
                }

                let w = u + w;

                if self.dist[v] > w {
                    self.dist[v] = w;

                    updated = true;
                }
            }

            if !updated {
                break;
            }
        }

        self.digraph
            .arcs_weighted()
            .all(|(u, v, &w)| {
                let u = self.dist[u];

                u == isize::MAX || self.dist[v] <= u + w
            })
            .then_some(&self.dist)
    }
}

impl<'a, D> BellmanFordMooreUnsafe<'a, D> {
    pub fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        let mut dist = (0..digraph.order())
            .map(|u| if u == s { 0 } else { isize::MAX })
            .collect::<Vec<_>>();

        let dist_ptr = dist.as_mut_ptr();

        Self {
            digraph,
            dist,
            dist_ptr,
        }
    }

    #[must_use]
    pub fn distances(&mut self) -> Option<&[isize]>
    where
        D: ArcsWeighted<Weight = isize> + Order,
    {
        let order = self.digraph.order();
        let arcs = self.digraph.arcs_weighted().collect::<Vec<_>>();
        let arcs_len = arcs.len();
        let arcs_ptr = arcs.as_ptr();

        for _ in 1..order {
            let mut updated = false;
            let dist_ptr = self.dist.as_mut_ptr();
            let mut i = 0;

            while i < arcs_len {
                unsafe {
                    {
                        let (u, v, w) = *arcs_ptr.add(i);
                        let dist_u = *dist_ptr.add(u);

                        if dist_u != isize::MAX {
                            let w = dist_u + w;
                            let dist_v = dist_ptr.add(v);

                            if *dist_v > w {
                                *dist_v = w;
                                updated = true;
                            }
                        }
                    }

                    i += 1;

                    if i < arcs_len {
                        let (u, v, w) = *arcs_ptr.add(i);
                        let dist_u = *dist_ptr.add(u);

                        if dist_u != isize::MAX {
                            let w = dist_u + w;
                            let dist_v = dist_ptr.add(v);

                            if *dist_v > w {
                                *dist_v = w;
                                updated = true;
                            }
                        }
                    }

                    i += 1;

                    if i < arcs_len {
                        let (u, v, w) = *arcs_ptr.add(i);
                        let dist_u = *dist_ptr.add(u);

                        if dist_u != isize::MAX {
                            let w = dist_u + w;
                            let dist_v = dist_ptr.add(v);

                            if *dist_v > w {
                                *dist_v = w;
                                updated = true;
                            }
                        }
                    }

                    i += 1;

                    if i < arcs_len {
                        let (u, v, w) = *arcs_ptr.add(i);
                        let dist_u = *dist_ptr.add(u);

                        if dist_u != isize::MAX {
                            let w = dist_u + w;
                            let dist_v = dist_ptr.add(v);

                            if *dist_v > w {
                                *dist_v = w;
                                updated = true;
                            }
                        }
                    }
                }

                i += 1;
            }

            if !updated {
                break;
            }
        }

        {
            let dist_ptr = self.dist.as_mut_ptr();
            let mut i = 0;

            while i < arcs_len {
                unsafe {
                    let (u, v, w) = *arcs_ptr.add(i);
                    let dist_u = *dist_ptr.add(u);

                    if dist_u != isize::MAX && *dist_ptr.add(v) > dist_u + w {
                        return None;
                    }
                }

                i += 1;
            }
        }

        Some(&self.dist[..])
    }
}

#[divan::bench(args = [10, 100])]
fn bellman_ford_moore(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench_local(|| {
        for u in 0..order {
            let mut bfm = BellmanFordMoore::new(&digraph, u);
            let _ = bfm.distances();
        }
    });
}

#[divan::bench(args = [10, 100])]
fn bellman_ford_moore_iter(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench_local(|| {
        for u in 0..order {
            let mut bfm = BellmanFordMooreIter::new(&digraph, u);
            let _ = bfm.distances();
        }
    });
}

#[divan::bench(args = [10, 100])]
fn bellman_ford_moore_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench_local(|| {
        for u in 0..order {
            let mut bfm = BellmanFordMooreUnsafe::new(&digraph, u);
            let _ = bfm.distances();
        }
    });
}

#[divan::bench(args = [10, 100])]
fn bellman_ford_moore_index_mut(bencher: Bencher<'_, '_>, order: usize) {
    let unweighted = AdjacencyList::erdos_renyi(order, 0.5, 0);
    let mut digraph = AdjacencyListWeighted::empty(order);

    for (u, v) in unweighted.arcs() {
        digraph.add_arc_weighted(u, v, 1);
    }

    bencher.bench_local(|| {
        for u in 0..order {
            let mut bfm = BellmanFordMooreForFor::new(&digraph, u);
            let _ = bfm.distances();
        }
    });
}
