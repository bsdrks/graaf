use std::collections::HashSet;

fn main() {
    divan::main();
}

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
