//! A trait to generate linear graphs for variable-sized graphs
//!
//! Linear graphs are also known as path graphs. To generate const-sized linear
//! graphs, see the [`LinearConst`](crate::gen::LinearConst) trait.
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Linear;
//!
//! //
//! assert_eq!(Vec::<Vec<usize>>::linear(0), Vec::<Vec<usize>>::new());
//!
//! // 0
//! assert_eq!(Vec::<Vec<usize>>::linear(1), vec![Vec::new()]);
//!
//! // 0 → 1
//! assert_eq!(Vec::<Vec<usize>>::linear(2), vec![vec![1], Vec::new()]);
//!
//! // 0 → 1 → 2
//! assert_eq!(
//!     Vec::<Vec<usize>>::linear(3),
//!     vec![vec![1], vec![2], Vec::new()]
//! );
//! ```

extern crate alloc;

use {
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to generate linear graphs for variable-sized graphs
///
/// # How can I implement `Linear`?
///
/// Provide an implementation of `linear` that generates a linear graph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Linear,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Linear for Graph {
///     fn linear(v: usize) -> Self {
///         Graph {
///             edges: (0..v - 1).map(|s| (s, s + 1)).collect(),
///         }
///     }
/// }
///
/// let graph = Graph::linear(3);
///
/// assert_eq!(graph.edges, HashSet::from([(0, 1), (1, 2)]));
/// ```
pub trait Linear {
    /// Generate a linear graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn linear(v: usize) -> Self;
}

impl Linear for Vec<Vec<usize>> {
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![Vec::new(); v];

        for (s, vec) in graph.iter_mut().enumerate().take(v - 1) {
            vec.push(s + 1);
        }

        graph
    }
}

impl Linear for Vec<BTreeSet<usize>> {
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![BTreeSet::new(); v];

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        graph
    }
}

impl<H> Linear for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = vec![HashSet::with_hasher(H::default()); v];

        for (s, set) in graph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        graph
    }
}

impl Linear for BTreeMap<usize, Vec<usize>> {
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = Self::new();

        for s in 0..v - 1 {
            let _ = graph.insert(s, vec![s + 1]);
        }

        let _ = graph.insert(v - 1, Vec::new());

        graph
    }
}

impl Linear for BTreeMap<usize, BTreeSet<usize>> {
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::new();
        }

        let mut graph = Self::new();

        for s in 0..v - 1 {
            let _ = graph.insert(s, BTreeSet::from([s + 1]));
        }

        let _ = graph.insert(v - 1, BTreeSet::new());

        graph
    }
}

impl<H> Linear for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
    Vec<usize>: Clone,
{
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::with_hasher(H::default());
        }

        let mut graph = Self::with_hasher(H::default());

        for s in 0..v - 1 {
            let _ = graph.insert(s, vec![s + 1]);
        }

        let _ = graph.insert(v - 1, Vec::new());

        graph
    }
}

impl<H> Linear for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    fn linear(v: usize) -> Self {
        if v == 0 {
            return Self::with_hasher(H::default());
        }

        let mut graph = Self::with_hasher(H::default());

        for s in 0..v - 1 {
            let _ = graph.insert(s, HashSet::with_hasher(H::default()));
            let _ = graph.get_mut(&s).unwrap().insert(s + 1);
        }

        let _ = graph.insert(v - 1, HashSet::with_hasher(H::default()));

        graph
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::op::{
            CountAllEdges,
            CountAllVertices,
            Indegree,
            Outdegree,
        },
        proptest::prelude::*,
    };

    proptest! {
        #[test]
        fn count_all_edges_vec_vec(v in 0..100_usize) {
            assert_eq!(Vec::<Vec<usize>>::linear(v).count_all_edges(), v.saturating_sub(1));
        }

        #[test]
        fn count_all_edges_vec_btree_set(v in 0..100_usize) {
            assert_eq!(Vec::<BTreeSet<usize>>::linear(v).count_all_edges(), v.saturating_sub(1));
        }

        #[test]
        fn count_all_edges_vec_hash_set(v in 0..100_usize) {
            assert_eq!(Vec::<HashSet<usize>>::linear(v).count_all_edges(), v.saturating_sub(1));
        }

        #[test]
        fn count_all_edges_btree_map_vec(v in 0..100_usize) {
            assert_eq!(BTreeMap::<usize, Vec<usize>>::linear(v).count_all_edges(), v.saturating_sub(1));
        }

        #[test]
        fn count_all_edges_btree_map_btree_set(v in 0..100_usize) {
            assert_eq!(BTreeMap::<usize, BTreeSet<usize>>::linear(v).count_all_edges(), v.saturating_sub(1));
        }

        #[test]
        fn count_all_edges_hash_map_vec(v in 0..100_usize) {
            assert_eq!(HashMap::<usize, Vec<usize>>::linear(v).count_all_edges(), v.saturating_sub(1));
        }

        #[test]
        fn count_all_edges_hash_map_hash_set(v in 0..100_usize) {
            assert_eq!(HashMap::<usize, HashSet<usize>>::linear(v).count_all_edges(), v.saturating_sub(1));
        }

        #[test]
        fn count_all_vertices_vec_vec(v in 0..100_usize) {
            assert_eq!(Vec::<Vec<usize>>::linear(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set(v in 0..100_usize) {
            assert_eq!(Vec::<BTreeSet<usize>>::linear(v).count_all_vertices(), v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set(v in 0..100_usize) {
            assert_eq!(Vec::<HashSet<usize>>::linear(v).count_all_vertices(), v);
        }

        #[test]
        fn indegree_vec_btree_set(v in 1..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::linear(v);

            assert_eq!(graph.indegree(0), 0);

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn indegree_vec_hash_set(v in 1..100_usize) {
            let graph = Vec::<HashSet<usize>>::linear(v);

            assert_eq!(graph.indegree(0), 0);

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn indegree_btree_map_btree_set(v in 1..100_usize) {
            let graph = BTreeMap::<usize, BTreeSet<usize>>::linear(v);

            assert_eq!(graph.indegree(0), 0);

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn indegree_hash_map_hash_set(v in 1..100_usize) {
            let graph = HashMap::<usize, HashSet<usize>>::linear(v);

            assert_eq!(graph.indegree(0), 0);

            for s in 1..v {
                assert_eq!(graph.indegree(s), 1);
            }
        }

        #[test]
        fn outdegree_vec_vec(v in 1..100_usize) {
            let graph = Vec::<Vec<usize>>::linear(v);

            for s in 0..v - 1 {
                assert_eq!(graph.outdegree(s), 1);
            }

            if v != 0 {
                assert_eq!(graph.outdegree(v - 1), 0);
            }
        }

        #[test]
        fn outdegree_vec_btree_set(v in 1..100_usize) {
            let graph = Vec::<BTreeSet<usize>>::linear(v);

            for s in 0..v - 1 {
                assert_eq!(graph.outdegree(s), 1);
            }

            if v != 0 {
                assert_eq!(graph.outdegree(v - 1), 0);
            }
        }

        #[test]
        fn outdegree_vec_hash_set(v in 1..100_usize) {
            let graph = Vec::<HashSet<usize>>::linear(v);

            for s in 0..v - 1 {
                assert_eq!(graph.outdegree(s), 1);
            }

            if v != 0 {
                assert_eq!(graph.outdegree(v - 1), 0);
            }
        }

        #[test]
        fn outdegree_btree_map_vec(v in 1..100_usize) {
            let graph = BTreeMap::<usize, Vec<usize>>::linear(v);

            for s in 0..v - 1 {
                assert_eq!(graph.outdegree(s), 1);
            }

            if v != 0 {
                assert_eq!(graph.outdegree(v - 1), 0);
            }
        }

        #[test]
        fn outdegree_btree_map_btree_set(v in 1..100_usize) {
            let graph = BTreeMap::<usize, BTreeSet<usize>>::linear(v);

            for s in 0..v - 1 {
                assert_eq!(graph.outdegree(s), 1);
            }

            if v != 0 {
                assert_eq!(graph.outdegree(v - 1), 0);
            }
        }

        #[test]
        fn outdegree_hash_map_vec(v in 1..100_usize) {
            let graph = HashMap::<usize, Vec<usize>>::linear(v);

            for s in 0..v - 1 {
                assert_eq!(graph.outdegree(s), 1);
            }

            if v != 0 {
                assert_eq!(graph.outdegree(v - 1), 0);
            }
        }

        #[test]
        fn outdegree_hash_map_hash_set(v in 1..100_usize) {
            let graph = HashMap::<usize, HashSet<usize>>::linear(v);

            for s in 0..v - 1 {
                assert_eq!(graph.outdegree(s), 1);
            }

            if v != 0 {
                assert_eq!(graph.outdegree(v - 1), 0);
            }
        }
    }

    #[test]
    fn vec_vec() {
        for (v, g) in [
            //
            Vec::new(),
            // 0
            vec![Vec::new()],
            // 0 → 1
            vec![vec![1], Vec::new()],
            // 0 → 1 → 2
            vec![vec![1], vec![2], Vec::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<Vec<usize>>::linear(v), g);
        }
    }

    #[test]
    fn vec_btree_set() {
        for (v, g) in [
            //
            Vec::new(),
            // 0
            vec![BTreeSet::new()],
            // 0 → 1
            vec![BTreeSet::from([1]), BTreeSet::new()],
            // 0 → 1 → 2
            vec![BTreeSet::from([1]), BTreeSet::from([2]), BTreeSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<BTreeSet<usize>>::linear(v), g);
        }
    }

    #[test]
    fn vec_hash_set() {
        for (v, g) in [
            //
            Vec::new(),
            // 0
            vec![HashSet::new()],
            // 0 → 1
            vec![HashSet::from([1]), HashSet::new()],
            // 0 → 1 → 2
            vec![HashSet::from([1]), HashSet::from([2]), HashSet::new()],
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&Vec::<HashSet<usize>>::linear(v), g);
        }
    }

    #[test]
    fn btree_map_vec() {
        for (v, g) in [
            //
            BTreeMap::new(),
            // 0
            BTreeMap::from([(0, Vec::new())]),
            // 0 → 1
            BTreeMap::from([(0, vec![1]), (1, Vec::new())]),
            // 0 → 1 → 2
            BTreeMap::from([(0, vec![1]), (1, vec![2]), (2, Vec::new())]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, Vec<usize>>::linear(v), g);
        }
    }

    #[test]
    fn btree_map_btree_set() {
        for (v, g) in [
            //
            BTreeMap::new(),
            // 0
            BTreeMap::from([(0, BTreeSet::new())]),
            // 0 → 1
            BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::new())]),
            // 0 → 1 → 2
            BTreeMap::from([
                (0, BTreeSet::from([1])),
                (1, BTreeSet::from([2])),
                (2, BTreeSet::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&BTreeMap::<usize, BTreeSet<usize>>::linear(v), g);
        }
    }

    #[test]
    fn hash_map_vec() {
        for (v, g) in [
            //
            HashMap::new(),
            // 0
            HashMap::from([(0, Vec::new())]),
            // 0 → 1
            HashMap::from([(0, vec![1]), (1, Vec::new())]),
            // 0 → 1 → 2
            HashMap::from([(0, vec![1]), (1, vec![2]), (2, Vec::new())]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, Vec<usize>>::linear(v), g);
        }
    }

    #[test]
    fn hash_map_hash_set() {
        for (v, g) in [
            //
            HashMap::new(),
            // 0
            HashMap::from([(0, HashSet::new())]),
            // 0 → 1
            HashMap::from([(0, HashSet::from([1])), (1, HashSet::new())]),
            // 0 → 1 → 2
            HashMap::from([
                (0, HashSet::from([1])),
                (1, HashSet::from([2])),
                (2, HashSet::new()),
            ]),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(&HashMap::<usize, HashSet<usize>>::linear(v), g);
        }
    }
}
