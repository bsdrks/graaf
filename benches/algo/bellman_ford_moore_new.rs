//! Becnhmarks of different implementations of `BellmanFordMoore::new`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        BellmanFordMoore,
        ErdosRenyi,
        Order,
    },
    std::ptr::write,
};

fn main() {
    divan::main();
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BellmanFordMooreIter<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BellmanFordMooreIndexMut<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BellmanFordMooreUnsafe<'a, D> {
    digraph: &'a D,
    dist: Vec<isize>,
    dist_ptr: *mut isize,
}

impl<'a, D> BellmanFordMooreIndexMut<'a, D> {
    fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        Self {
            digraph,
            dist: {
                let mut dist = vec![isize::MAX; digraph.order()];

                dist[s] = 0;

                dist
            },
        }
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
}

impl<'a, D> BellmanFordMooreUnsafe<'a, D> {
    pub fn new(digraph: &'a D, s: usize) -> Self
    where
        D: Order,
    {
        let order = digraph.order();
        let mut dist = Vec::with_capacity(order);

        unsafe {
            let dist_ptr: *mut isize = dist.as_mut_ptr();
            let mut i = 0;
            let n = order;

            while i + 4 <= n {
                write(dist_ptr.add(i), isize::MAX);
                write(dist_ptr.add(i + 1), isize::MAX);
                write(dist_ptr.add(i + 2), isize::MAX);
                write(dist_ptr.add(i + 3), isize::MAX);

                i += 4;
            }

            while i < n {
                write(dist_ptr.add(i), isize::MAX);

                i += 1;
            }

            write(dist_ptr.add(s), 0);

            dist.set_len(order);

            Self {
                digraph,
                dist,
                dist_ptr,
            }
        }
    }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bellman_ford_moore_iter(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        for u in 0..order {
            let _ = BellmanFordMooreIter::new(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bellman_ford_moore(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        for u in 0..order {
            let _ = BellmanFordMoore::new(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bellman_ford_moore_index_mut(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        for u in 0..order {
            let _ = BellmanFordMooreIndexMut::new(&digraph, u);
        }
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bellman_ford_moore_unsafe(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        for u in 0..order {
            let _ = BellmanFordMooreUnsafe::new(&digraph, u);
        }
    });
}
