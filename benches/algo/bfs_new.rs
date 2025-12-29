//! Benchmarks of different implementations of `Bfs::new`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Bfs,
        EdgeList,
        ErdosRenyi,
    },
    std::collections::{
        HashSet,
        VecDeque,
    },
};

fn main() {
    divan::main();
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BfsPushBackInsert<'a, D> {
    digraph: &'a D,
    queue: VecDeque<usize>,
    visited: HashSet<usize>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BfsCollect<'a, D> {
    digraph: &'a D,
    queue: VecDeque<usize>,
    visited: HashSet<usize>,
}

impl<'a, D> BfsPushBackInsert<'a, D> {
    #[must_use]
    fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        T: IntoIterator<Item = usize>,
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

impl<'a, D> BfsCollect<'a, D> {
    #[must_use]
    pub fn new<T>(digraph: &'a D, sources: T) -> Self
    where
        T: Iterator<Item = usize> + Clone,
    {
        Self {
            digraph,
            queue: sources.clone().collect(),
            visited: sources.collect(),
        }
    }
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_list(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyList::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(Bfs::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_list_collect(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyList::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsCollect::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_list_push_back_insert(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyList::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsPushBackInsert::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_map(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMap::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(Bfs::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_map_collect(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMap::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsCollect::new(&digraph, sources.clone()));
    });
}
#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_map_push_back_insert(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMap::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsPushBackInsert::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_matrix(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(Bfs::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_matrix_collect(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsCollect::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_adjacency_matrix_push_back_insert(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsPushBackInsert::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_edge_list(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = EdgeList::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(Bfs::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_edge_list_collect(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = EdgeList::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsCollect::new(&digraph, sources.clone()));
    });
}

#[divan::bench(args = [10, 100, 1000, 10000])]
fn bfs_edge_list_push_back_insert(bencher: Bencher<'_, '_>, n: usize) {
    let digraph = EdgeList::erdos_renyi(n, 0.5, 0);
    let sources = 0..n / 10;

    bencher.bench_local(|| {
        drop(BfsPushBackInsert::new(&digraph, sources.clone()));
    });
}
