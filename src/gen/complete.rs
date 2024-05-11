//! A trait to generate variable-sized complete symmetric directed graphs
//!
//! The generated graphs are simple; they contain no self-loops. To generate
//! const-sized complete graphs, see [`CompleteConst`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Complete;
//!
//! assert_eq!(Vec::<Vec<usize>>::complete(1), vec![Vec::new()]);
//! assert_eq!(Vec::<Vec<usize>>::complete(2), vec![vec![1], vec![0]]);
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::complete(3),
//!     vec![vec![1, 2], vec![0, 2], vec![0, 1]]
//! );
//! ```
//!
//! [`CompleteConst`]: crate::gen::CompleteConst

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

/// A trait to generate variable-size symmetric complete directed graphs
///
/// # How can I implement `Complete`?
///
/// Provide an implementation of `complete` that generates a symmetric complete
/// graph with `v` vertices.
///
/// ```
/// use {
///     graaf::gen::Complete,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Complete for Graph {
///     /// # Panics
///     ///
///     /// Panics if `v` is 0.
///     fn complete(v: usize) -> Self {
///         assert!(v > 0, "a graph must have at least one vertex");
///
///         let mut edges = HashSet::new();
///
///         for s in 0..v {
///             for t in 0..v {
///                 if s != t {
///                     let _ = edges.insert((s, t));
///                     let _ = edges.insert((t, s));
///                 }
///             }
///         }
///
///         Graph { edges }
///     }
/// }
///
/// let graph = Graph::complete(3);
///
/// assert_eq!(
///     graph.edges,
///     HashSet::from([(0, 1), (1, 0), (0, 2), (2, 0), (1, 2), (2, 1)])
/// );
/// ```
pub trait Complete {
    /// Generates a complete graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn complete(v: usize) -> Self;
}

impl Complete for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (0..v).filter(|&t| s != t).collect())
            .collect()
    }
}

impl Complete for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (0..v).filter(|&t| s != t).collect())
            .collect()
    }
}

impl<H> Complete for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (0..v).filter(|&t| s != t).collect())
            .collect()
    }
}

impl Complete for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl Complete for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl<H> Complete for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl<H> Complete for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (s, (0..v).filter(|&t| s != t).collect()))
            .collect()
    }
}

impl Complete for Vec<(usize, usize)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .flat_map(|s| (0..v).filter(move |&t| s != t).map(move |t| (s, t)))
            .collect()
    }
}

impl Complete for BTreeSet<(usize, usize)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .flat_map(|s| (0..v).filter(move |&t| s != t).map(move |t| (s, t)))
            .collect()
    }
}

impl<H> Complete for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .flat_map(|s| (0..v).filter(move |&t| s != t).map(move |t| (s, t)))
            .collect()
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
            IsSimple,
            Outdegree,
        },
        proptest::prelude::*,
    };

    fn prop_count_all_edges<T: Complete + CountAllEdges>(v: usize) {
        assert_eq!(T::complete(v).count_all_edges(), v * (v - 1));
    }

    fn prop_count_all_vertices<T: Complete + CountAllVertices>(v: usize) {
        assert_eq!(T::complete(v).count_all_vertices(), v);
    }

    fn prop_indegree<T: Complete + Indegree>(v: usize) {
        let graph = T::complete(v);

        for s in 0..v {
            assert_eq!(graph.indegree(s), v - 1);
        }
    }

    fn prop_is_simple<T: Complete + IsSimple>(v: usize) {
        assert!(T::complete(v).is_simple());
    }

    fn prop_outdegree<T: Complete + Outdegree>(v: usize) {
        let graph = T::complete(v);

        for s in 0..v {
            assert_eq!(graph.outdegree(s), v - 1);
        }
    }

    proptest! {
        #[test]
        fn count_all_edges_vec_vec(v in 1..100_usize) {
            prop_count_all_edges::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_set(v in 1..100_usize) {
            prop_count_all_edges::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_set(v in 1..100_usize) {
            prop_count_all_edges::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_vec(v in 1..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_set(v in 1..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_vec(v in 1..100_usize) {
            prop_count_all_edges::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_set(v in 1..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_tuple(v in 1..100_usize) {
            prop_count_all_edges::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_btree_set_tuple(v in 1..100_usize) {
            prop_count_all_edges::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_hash_set_tuple(v in 1..100_usize) {
            prop_count_all_edges::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_vertices_vec_vec(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_btree_set(v in 1..100_usize) {
            prop_indegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_hash_set(v in 1..100_usize) {
            prop_indegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_btree_map_btree_set(v in 1..100_usize) {
            prop_indegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_hash_map_hash_set(v in 1..100_usize) {
            prop_indegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_btree_set(v in 1..100_usize) {
            prop_is_simple::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_hash_set(v in 1..100_usize) {
            prop_is_simple::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_tuple(v in 1..100_usize) {
            prop_is_simple::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_btree_set_tuple(v in 1..100_usize) {
            prop_is_simple::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_hash_set_tuple(v in 1..100_usize) {
            prop_is_simple::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn outdegree_vec_vec(v in 1..100_usize) {
            prop_outdegree::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_set(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_set(v in 1..100_usize) {
            prop_outdegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_vec(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_btree_set(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_vec(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_hash_set(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, HashSet<usize>>>(v);
        }
    }

    #[test]
    fn vec_vec() {
        assert_eq!(Vec::<Vec<usize>>::complete(1), vec![Vec::new()]);
        assert_eq!(Vec::<Vec<usize>>::complete(2), vec![vec![1], vec![0]]);

        assert_eq!(
            Vec::<Vec<usize>>::complete(3),
            vec![vec![1, 2], vec![0, 2], vec![0, 1]]
        );
    }

    #[test]
    fn vec_btree_set() {
        assert_eq!(Vec::<BTreeSet<usize>>::complete(1), vec![BTreeSet::new()]);

        assert_eq!(
            Vec::<BTreeSet<usize>>::complete(2),
            vec![BTreeSet::from([1]), BTreeSet::from([0])]
        );

        assert_eq!(
            Vec::<BTreeSet<usize>>::complete(3),
            vec![
                BTreeSet::from([1, 2]),
                BTreeSet::from([0, 2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn vec_hash_set() {
        assert_eq!(Vec::<HashSet<usize>>::complete(1), vec![HashSet::new()]);

        assert_eq!(
            Vec::<HashSet<usize>>::complete(2),
            vec![HashSet::from([1]), HashSet::from([0])]
        );

        assert_eq!(
            Vec::<HashSet<usize>>::complete(3),
            vec![
                HashSet::from([1, 2]),
                HashSet::from([0, 2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn btree_map_vec() {
        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::complete(1),
            BTreeMap::from([(0, vec![])])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::complete(2),
            BTreeMap::from([(0, vec![1]), (1, vec![0])])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::complete(3),
            BTreeMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])])
        );
    }

    #[test]
    fn btree_map_btree_set() {
        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::complete(1),
            BTreeMap::from([(0, BTreeSet::new())])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::complete(2),
            BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::from([0]))])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::complete(3),
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([0, 2])),
                (2, BTreeSet::from([0, 1]))
            ])
        );
    }

    #[test]
    fn hash_map_vec() {
        assert_eq!(
            HashMap::<usize, Vec<usize>>::complete(1),
            HashMap::from([(0, vec![])])
        );

        assert_eq!(
            HashMap::<usize, Vec<usize>>::complete(2),
            HashMap::from([(0, vec![1]), (1, vec![0])])
        );

        assert_eq!(
            HashMap::<usize, Vec<usize>>::complete(3),
            HashMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])])
        );
    }

    #[test]
    fn hash_map_hash_set() {
        assert_eq!(
            HashMap::<usize, HashSet<usize>>::complete(1),
            HashMap::from([(0, HashSet::new())])
        );

        assert_eq!(
            HashMap::<usize, HashSet<usize>>::complete(2),
            HashMap::from([(0, HashSet::from([1])), (1, HashSet::from([0]))])
        );

        assert_eq!(
            HashMap::<usize, HashSet<usize>>::complete(3),
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([0, 2])),
                (2, HashSet::from([0, 1]))
            ])
        );
    }

    #[test]
    fn vec_tuple() {
        assert_eq!(Vec::<(usize, usize)>::complete(1), Vec::new());
        assert_eq!(Vec::<(usize, usize)>::complete(2), vec![(0, 1), (1, 0)]);

        assert_eq!(
            Vec::<(usize, usize)>::complete(3),
            vec![(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]
        );
    }

    #[test]
    fn btree_set_tuple() {
        assert_eq!(BTreeSet::<(usize, usize)>::complete(1), BTreeSet::new());

        assert_eq!(
            BTreeSet::<(usize, usize)>::complete(2),
            BTreeSet::from([(0, 1), (1, 0)])
        );

        assert_eq!(
            BTreeSet::<(usize, usize)>::complete(3),
            BTreeSet::from([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)])
        );
    }

    #[test]
    fn hash_set_tuple() {
        assert_eq!(HashSet::<(usize, usize)>::complete(1), HashSet::new());

        assert_eq!(
            HashSet::<(usize, usize)>::complete(2),
            HashSet::from([(0, 1), (1, 0)])
        );

        assert_eq!(
            HashSet::<(usize, usize)>::complete(3),
            HashSet::from([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)])
        );
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_vec_panic() {
        let _ = Vec::<Vec<usize>>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_btree_set_panic() {
        let _ = Vec::<BTreeSet<usize>>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_hash_set_panic() {
        let _ = Vec::<HashSet<usize>>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_vec_panic() {
        let _ = BTreeMap::<usize, Vec<usize>>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_btree_set_panic() {
        let _ = BTreeMap::<usize, BTreeSet<usize>>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_vec_panic() {
        let _ = HashMap::<usize, Vec<usize>>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_hash_set_panic() {
        let _ = HashMap::<usize, HashSet<usize>>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_tuple_panic() {
        let _ = Vec::<(usize, usize)>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_set_tuple_panic() {
        let _ = BTreeSet::<(usize, usize)>::complete(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_set_tuple_panic() {
        let _ = HashSet::<(usize, usize)>::complete(0);
    }
}
