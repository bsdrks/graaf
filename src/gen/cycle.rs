//! A trait to generate cycle graphs
//!
//! Cycle graphs are also known as circular graphs.
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Cycle;
//!
//! //
//! assert_eq!(Vec::<Vec<usize>>::cycle(0), Vec::<Vec<usize>>::new());
//!
//! // 0 → 0
//! assert_eq!(Vec::<Vec<usize>>::cycle(1), vec![vec![0]]);
//!
//! // 0 → 1 → 0
//! assert_eq!(Vec::<Vec<usize>>::cycle(2), vec![vec![1], vec![0]]);
//!
//! // 0 → 1 → 2 → 0
//! assert_eq!(Vec::<Vec<usize>>::cycle(3), vec![vec![1], vec![2], vec![0]]);
//! ```

extern crate alloc;

use {
    alloc::collections::BTreeSet,
    core::hash::BuildHasher,
    std::collections::HashSet,
};

/// A trait to generate cycle graphs
///
/// # How can I implement `Cycle`?
///
/// Provide an implementation of `cycle` that generates a cycle graph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Cycle,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Cycle for Graph {
///     fn cycle(v: usize) -> Self {
///         let mut edges = (0..v - 1).map(|s| (s, s + 1)).collect::<HashSet<_>>();
///
///         edges.insert((v - 1, 0));
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::cycle(3);
///
/// assert_eq!(graph.edges, HashSet::from([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait Cycle {
    /// Generate a cycle graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn cycle(v: usize) -> Self;
}

impl Cycle for Vec<Vec<usize>> {
    fn cycle(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![Vec::new(); v];

        for (s, vec) in graph.iter_mut().enumerate().take(v - 1) {
            vec.push(s + 1);
        }

        graph[v - 1].push(0);

        graph
    }
}

impl Cycle for Vec<BTreeSet<usize>> {
    fn cycle(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![BTreeSet::new(); v];

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = graph[v - 1].insert(0);

        graph
    }
}

impl<H> Cycle for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn cycle(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![HashSet::with_hasher(H::default()); v];

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = graph[v - 1].insert(0);

        graph
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::{
            CountAllVertices,
            Indegree,
            IterVertices,
            Outdegree,
        },
        proptest::prelude::*,
    };

    proptest! {
        #[test]
        fn count_all_vertices_vec_vec(v in 0..100_usize) {
            assert_eq!(Vec::<Vec<usize>>::cycle(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set(v in 0..100_usize) {
            assert_eq!(Vec::<BTreeSet<usize>>::cycle(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set(v in 0..100_usize) {
            assert_eq!(Vec::<HashSet<usize>>::cycle(v).count_all_vertices(), v);
        }

        #[test]
        fn indegree_vec_btree_set(v in 0..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::cycle(v);

            for s in graph.iter_vertices() {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn indegree_vec_hash_set(v in 0..100_usize) {
            let graph = Vec::<HashSet<usize>>::cycle(v);

            for s in graph.iter_vertices() {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn outdegree_vec_vec(v in 0..100_usize) {
            let graph = Vec::<Vec<usize>>::cycle(v);

            for s in graph.iter_vertices() {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_vec_btree_set(v in 0..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::cycle(v);

            for s in graph.iter_vertices() {
                assert_eq!(graph.outdegree(s), 1);
            }
        }

        #[test]
        fn outdegree_vec_hash_set(v in 0..100_usize) {
            let graph = Vec::<HashSet<usize>>::cycle(v);

            for s in graph.iter_vertices() {
                assert_eq!(graph.outdegree(s), 1);
            }
        }
    }

    #[test]
    fn vec_vec() {
        for (v, g) in [
            //
            Vec::new(),
            // 0 → 0
            vec![vec![0]],
            // 0 → 1 → 0
            vec![vec![1], vec![0]],
            // 0 → 1 → 2 → 0
            vec![vec![1], vec![2], vec![0]],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn vec_btree_set() {
        for (v, g) in [
            //
            Vec::new(),
            // 0 → 0
            vec![BTreeSet::from([0])],
            // 0 → 1 → 0
            vec![BTreeSet::from([1]), BTreeSet::from([0])],
            // 0 → 1 → 2 → 0
            vec![
                BTreeSet::from([1]),
                BTreeSet::from([2]),
                BTreeSet::from([0]),
            ],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::cycle(v), g);
        }
    }

    #[test]
    fn vec_hash_set() {
        for (v, g) in [
            //
            Vec::new(),
            // 0 → 0
            vec![HashSet::from([0])],
            // 0 → 1 → 0
            vec![HashSet::from([1]), HashSet::from([0])],
            // 0 → 1 → 2 → 0
            vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([0])],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::cycle(v), g);
        }
    }
}
