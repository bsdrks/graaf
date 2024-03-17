use {
    // divan::AllocProfiler,
    graph::AddEdge,
    std::collections::HashSet,
};

fn main() {
    divan::main();
}

// #[global_allocator]
// static ALLOC: AllocProfiler = AllocProfiler::system();

mod new {
    use super::*;

    #[divan::bench(args = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000])]
    fn vec(v: usize) {
        let _ = vec![Vec::<usize>::new(); v];
    }

    #[divan::bench(args = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000])]
    fn hash_set(v: usize) {
        let _ = vec![HashSet::<usize>::new(); v];
    }
}

mod add_edge {
    use super::*;

    #[divan::bench(args = [1, 10, 100, 1000, 10_000, 100_000])]
    fn vec(v: usize) {
        let mut adj = vec![Vec::<usize>::new(); v];

        for i in 0..v {
            adj.add_edge(i, (i + 1) % v);
        }
    }

    #[divan::bench(args = [1, 10, 100, 1000, 10_000, 100_000])]
    fn hash_set(v: usize) {
        let mut adj = vec![HashSet::<usize>::new(); v];

        for i in 0..v {
            adj.add_edge(i, (i + 1) % v);
        }
    }
}
