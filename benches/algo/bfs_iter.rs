//! Benchmarks of different implementations of `Bfs::iter`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Bfs,
        EdgeList,
        ErdosRenyi,
        OutNeighbors,
    },
    std::{
        collections::{
            HashSet,
            VecDeque,
        },
        iter::once,
    },
};

fn main() {
    divan::main();
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BfsPushBack<'a, D> {
    digraph: &'a D,
    queue: VecDeque<usize>,
    visited: HashSet<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BfsExtend<'a, D> {
    digraph: &'a D,
    queue: VecDeque<usize>,
    visited: HashSet<usize>,
}

impl<'a, D> BfsPushBack<'a, D> {
    #[must_use]
    fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        T: Iterator<Item = usize>,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        for source in sources {
            queue.push_back(source);

            let _ = visited.insert(source);
        }

        Self {
            digraph,
            queue,
            visited,
        }
    }
}

impl<D> Iterator for BfsPushBack<'_, D>
where
    D: OutNeighbors,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.queue.pop_front()?;

        for v in self
            .digraph
            .out_neighbors(u)
            .filter(|v| self.visited.insert(*v))
        {
            self.queue.push_back(v);
        }

        Some(u)
    }
}

impl<'a, D> BfsExtend<'a, D> {
    #[must_use]
    pub fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        T: Iterator<Item = usize>,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        for source in sources {
            queue.push_back(source);

            let _ = visited.insert(source);
        }

        Self {
            digraph,
            queue,
            visited,
        }
    }
}

impl<D> Iterator for BfsExtend<'_, D>
where
    D: OutNeighbors,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let u = self.queue.pop_front()?;

        self.queue.extend(
            self.digraph
                .out_neighbors(u)
                .filter(|v| self.visited.insert(*v)),
        );

        Some(u)
    }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_list(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = Bfs::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_list_push_back(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsPushBack::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_list_extend(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyList::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsExtend::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_map(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMap::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = Bfs::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_map_push_back(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMap::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsPushBack::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_map_extend(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMap::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsExtend::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_matrix(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = Bfs::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_matrix_push_back(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsPushBack::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_matrix_extend(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsExtend::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_edge_list(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = EdgeList::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = Bfs::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_edge_list_push_back(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = EdgeList::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsPushBack::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_edge_list_extend(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = EdgeList::erdos_renyi(n, 0.5, 0);

    bencher.bench_local(|| {
        let bfs = BfsExtend::new(&digraph, once(0));
        let _ = bfs.collect::<Vec<_>>();
    });
}
