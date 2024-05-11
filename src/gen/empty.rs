#![doc(alias = "edgeless")]
//! A trait to generate empty variable-sized directed graphs
//!
//! Empty graphs are also known as edgeless graphs. To generate empty
//! const-sized directed graphs, see [`EmptyConst`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Empty;
//!
//! assert_eq!(Vec::<Vec<usize>>::empty(1), vec![Vec::new()]);
//! assert_eq!(Vec::<Vec<usize>>::empty(2), vec![Vec::new(), Vec::new()]);
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::empty(3),
//!     vec![Vec::new(), Vec::new(), Vec::new()]
//! );
//! ```
//!
//! [`EmptyConst`]: crate::gen::EmptyConst

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

/// A trait to generate empty variable-sized directed graphs
///
/// # How can I implement `Empty`?
///
/// Provide an implementation of `empty` that generates an empty graph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Empty,
///     std::collections::HashSet,
/// };
///
/// #[derive(Debug, PartialEq)]
/// struct Graph {
///     edges: HashSet<(usize, usize)>,
/// }
///
/// impl Empty for Graph {
///     /// # Panics
///     ///
///     /// Panics if `v` is 0.
///     fn empty(v: usize) -> Self {
///         assert!(v > 0, "a graph must have at least one vertex");
///
///         Graph {
///             edges: HashSet::new(),
///         }
///     }
/// }
///
/// assert_eq!(
///     Graph::empty(3),
///     Graph {
///         edges: HashSet::new()
///     }
/// );
/// ```
///
/// # Examples
///
/// ```
/// use graaf::gen::Empty;
///
/// assert_eq!(Vec::<Vec<usize>>::empty(1), vec![Vec::new()]);
/// assert_eq!(Vec::<Vec<usize>>::empty(2), vec![Vec::new(), Vec::new()]);
///
/// assert_eq!(
///     Vec::<Vec<usize>>::empty(3),
///     vec![Vec::new(), Vec::new(), Vec::new()]
/// );
/// ```
pub trait Empty {
    /// Generates an empty graph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the graph
    fn empty(v: usize) -> Self;
}

impl Empty for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![Vec::new(); v]
    }
}

impl Empty for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![BTreeSet::new(); v]
    }
}

impl<H> Empty for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![HashSet::with_hasher(H::default()); v]
    }
}

impl Empty for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl Empty for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, BTreeSet::new())).collect()
    }
}

impl<H> Empty for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl<H> Empty for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (s, HashSet::with_hasher(H::default())))
            .collect()
    }
}

impl Empty for Vec<(usize, usize)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        Self::new()
    }
}

impl Empty for BTreeSet<(usize, usize)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        Self::new()
    }
}

impl<H> Empty for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        Self::with_hasher(H::default())
    }
}

impl<W> Empty for Vec<Vec<(usize, W)>>
where
    W: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![Vec::new(); v]
    }
}

impl<W> Empty for Vec<BTreeSet<(usize, W)>>
where
    W: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![BTreeSet::new(); v]
    }
}

impl<W, H> Empty for Vec<HashSet<(usize, W), H>>
where
    W: Clone,
    H: BuildHasher + Default,
    HashSet<(usize, W), H>: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![HashSet::with_hasher(H::default()); v]
    }
}

impl<W> Empty for Vec<BTreeMap<usize, W>>
where
    W: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![BTreeMap::new(); v]
    }
}

impl<W, H> Empty for Vec<HashMap<usize, W, H>>
where
    W: Clone,
    H: BuildHasher + Default,
    HashMap<usize, W, H>: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        vec![HashMap::with_hasher(H::default()); v]
    }
}

impl<W> Empty for BTreeMap<usize, Vec<(usize, W)>>
where
    W: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl<W> Empty for BTreeMap<usize, BTreeSet<(usize, W)>>
where
    W: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, BTreeSet::new())).collect()
    }
}

impl<W> Empty for BTreeMap<usize, BTreeMap<usize, W>>
where
    W: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, BTreeMap::new())).collect()
    }
}

impl<W, H> Empty for HashMap<usize, Vec<(usize, W)>, H>
where
    W: Clone,
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, Vec::new())).collect()
    }
}

impl<W, H> Empty for HashMap<usize, HashSet<(usize, W), H>, H>
where
    W: Clone,
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (s, HashSet::with_hasher(H::default())))
            .collect()
    }
}

impl<W, H> Empty for HashMap<usize, HashMap<usize, W, H>, H>
where
    W: Clone,
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v)
            .map(|s| (s, HashMap::with_hasher(H::default())))
            .collect()
    }
}

impl<W> Empty for Vec<(usize, usize, W)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        Self::new()
    }
}

impl<W> Empty for BTreeSet<(usize, usize, W)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        Self::new()
    }
}

impl<W, H> Empty for HashSet<(usize, usize, W), H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn empty(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        Self::with_hasher(H::default())
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

    fn prop_count_all_edges<T: CountAllEdges + Empty>(v: usize) {
        assert_eq!(T::empty(v).count_all_edges(), 0);
    }

    fn prop_count_all_vertices<T: CountAllVertices + Empty>(v: usize) {
        assert_eq!(T::empty(v).count_all_vertices(), v);
    }

    fn prop_indegree<T: Indegree + Empty>(v: usize) {
        let graph = T::empty(v);

        for s in 0..v {
            assert_eq!(graph.indegree(s), 0);
        }
    }

    fn prop_is_simple<T: IsSimple + Empty>(v: usize) {
        assert!(T::empty(v).is_simple());
    }

    fn prop_outdegree<T: Outdegree + Empty>(v: usize) {
        let graph = T::empty(v);

        for s in 0..v {
            assert_eq!(graph.outdegree(s), 0);
        }
    }

    proptest! {
        #[test]
        fn count_all_edges_vec_vec_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_set_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_set_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_vec_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_set_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_vec_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_set_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_tuple_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_btree_set_tuple_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_hash_set_tuple_unweighted(v in 1..100_usize) {
            prop_count_all_edges::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_vec_vec_weighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_set_weighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_set_weighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_vec_btree_map_weighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_hash_map_weighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<HashMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_vec_weighted(v in 1..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_set_weighted(v in 1..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_btree_map_btree_map(v in 1..100_usize) {
            prop_count_all_edges::<BTreeMap<usize, BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_vec_weighted(v in 1..100_usize) {
            prop_count_all_edges::<HashMap<usize, Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_set_weighted(v in 1..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_edges_hash_map_hash_map(v in 1..100_usize) {
            prop_count_all_edges::<HashMap<usize, HashMap<usize, usize>>>(v);
        }

        #[test]
        fn count_all_edges_vec_tuple_weighted(v in 1..100_usize) {
            prop_count_all_edges::<Vec<(usize, usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_btree_set_tuple_weighted(v in 1..100_usize) {
            prop_count_all_edges::<BTreeSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn count_all_edges_hash_set_tuple_weighted(v in 1..100_usize) {
            prop_count_all_edges::<HashSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn count_all_vertices_vec_vec_unweighted(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set_unweighted(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set_unweighted(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_vec_weighted(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_btree_set_weighted(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn count_all_vertices_vec_hash_set_weighted(v in 1..100_usize) {
            prop_count_all_vertices::<Vec<HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn indegree_vec_btree_set_unweighted(v in 1..100_usize) {
            prop_indegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_hash_set_unweighted(v in 1..100_usize) {
            prop_indegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_btree_map_btree_set_unweighted(v in 1..100_usize) {
            prop_indegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_hash_map_hash_set_unweighted(v in 1..100_usize) {
            prop_indegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_btree_map_weighted(v in 1..100_usize) {
            prop_indegree::<Vec<BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn indegree_vec_hash_map_weighted(v in 1..100_usize) {
            prop_indegree::<Vec<HashMap<usize, usize>>>(v);
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
        fn indegree_btree_map_btree_map(v in 1..100_usize) {
            prop_indegree::<BTreeMap<usize, BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn indegree_hash_map_hash_map(v in 1..100_usize) {
            prop_indegree::<HashMap<usize, HashMap<usize, usize>>>(v);
        }

        #[test]
        fn is_simple_vec_btree_set_unweighted(v in 1..100_usize) {
            prop_is_simple::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_hash_set_unweighted(v in 1..100_usize) {
            prop_is_simple::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_tuple_unweighted(v in 1..100_usize) {
            prop_is_simple::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_btree_set_tuple_unweighted(v in 1..100_usize) {
            prop_is_simple::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_hash_set_tuple_unweighted(v in 1..100_usize) {
            prop_is_simple::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_vec_tuple_weighted(v in 1..100_usize) {
            prop_is_simple::<Vec<(usize, usize, usize)>>(v);
        }

        #[test]
        fn is_simple_btree_set_tuple_weighted(v in 1..100_usize) {
            prop_is_simple::<BTreeSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn is_simple_hash_set_tuple_weighted(v in 1..100_usize) {
            prop_is_simple::<HashSet<(usize, usize, usize)>>(v);
        }

        #[test]
        fn outdegree_vec_vec_unweighted(v in 1..100_usize) {
            prop_outdegree::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_vec_unweighted(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_btree_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_vec_unweighted(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_hash_set_unweighted(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_vec_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<Vec<(usize, usize)>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_set_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeSet<(usize, usize)>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_set_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<HashSet<(usize, usize)>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_map_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_map_weighted(v in 1..100_usize) {
            prop_outdegree::<Vec<HashMap<usize, usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_btree_map(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, BTreeMap<usize, usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_hash_map(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, HashMap<usize, usize>>>(v);
        }
    }

    #[test]
    fn vec_vec_unweighted() {
        assert_eq!(Vec::<Vec<usize>>::empty(1), vec![Vec::new()]);
        assert_eq!(Vec::<Vec<usize>>::empty(2), vec![Vec::new(), Vec::new()]);

        assert_eq!(
            Vec::<Vec<usize>>::empty(3),
            vec![Vec::new(), Vec::new(), Vec::new()]
        );
    }

    #[test]
    fn vec_btree_set_unweighted() {
        assert_eq!(Vec::<BTreeSet<usize>>::empty(1), vec![BTreeSet::new()]);

        assert_eq!(
            Vec::<BTreeSet<usize>>::empty(2),
            vec![BTreeSet::new(), BTreeSet::new()]
        );

        assert_eq!(
            Vec::<BTreeSet<usize>>::empty(3),
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]
        );
    }

    #[test]
    fn vec_hash_set_unweighted() {
        assert_eq!(Vec::<HashSet<usize>>::empty(1), vec![HashSet::new()]);

        assert_eq!(
            Vec::<HashSet<usize>>::empty(2),
            vec![HashSet::new(), HashSet::new()]
        );

        assert_eq!(
            Vec::<HashSet<usize>>::empty(3),
            vec![HashSet::new(), HashSet::new(), HashSet::new()]
        );
    }

    #[test]
    fn btree_map_vec_unweighted() {
        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::empty(1),
            BTreeMap::from([(0, Vec::new())])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::empty(2),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new())])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::empty(3),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())])
        );
    }

    #[test]
    fn btree_map_btree_set_unweighted() {
        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::empty(1),
            BTreeMap::from([(0, BTreeSet::new())])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::empty(2),
            BTreeMap::from([(0, BTreeSet::new()), (1, BTreeSet::new())])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::empty(3),
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::new()),
                (2, BTreeSet::new()),
            ])
        );
    }

    #[test]
    fn hash_map_vec_unweighted() {
        assert_eq!(
            HashMap::<usize, Vec<usize>>::empty(1),
            HashMap::from([(0, Vec::new())])
        );

        assert_eq!(
            HashMap::<usize, Vec<usize>>::empty(2),
            HashMap::from([(0, Vec::new()), (1, Vec::new())])
        );

        assert_eq!(
            HashMap::<usize, Vec<usize>>::empty(3),
            HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())])
        );
    }

    #[test]
    fn hash_map_hash_set_unweighted() {
        assert_eq!(
            HashMap::<usize, HashSet<usize>>::empty(1),
            HashMap::from([(0, HashSet::new())])
        );

        assert_eq!(
            HashMap::<usize, HashSet<usize>>::empty(2),
            HashMap::from([(0, HashSet::new()), (1, HashSet::new())])
        );

        assert_eq!(
            HashMap::<usize, HashSet<usize>>::empty(3),
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::new()),
                (2, HashSet::new()),
            ])
        );
    }

    #[test]
    fn vec_tuple_unweighted() {
        assert_eq!(Vec::<(usize, usize)>::empty(1), Vec::new());
        assert_eq!(Vec::<(usize, usize)>::empty(2), Vec::new());
        assert_eq!(Vec::<(usize, usize)>::empty(3), Vec::new());
    }

    #[test]
    fn btree_set_tuple_unweighted() {
        assert_eq!(BTreeSet::<(usize, usize)>::empty(1), BTreeSet::new());
        assert_eq!(BTreeSet::<(usize, usize)>::empty(2), BTreeSet::new());
        assert_eq!(BTreeSet::<(usize, usize)>::empty(3), BTreeSet::new());
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        assert_eq!(HashSet::<(usize, usize)>::empty(1), HashSet::new());
        assert_eq!(HashSet::<(usize, usize)>::empty(2), HashSet::new());
        assert_eq!(HashSet::<(usize, usize)>::empty(3), HashSet::new());
    }

    #[test]
    fn vec_vec_weighted() {
        assert_eq!(Vec::<Vec<(usize, usize)>>::empty(1), vec![Vec::new()]);

        assert_eq!(
            Vec::<Vec<(usize, usize)>>::empty(2),
            vec![Vec::new(), Vec::new()]
        );

        assert_eq!(
            Vec::<Vec<(usize, usize)>>::empty(3),
            vec![Vec::new(), Vec::new(), Vec::new()]
        );
    }

    #[test]
    fn vec_btree_set_weighted() {
        assert_eq!(
            Vec::<BTreeSet<(usize, usize)>>::empty(1),
            vec![BTreeSet::new()]
        );

        assert_eq!(
            Vec::<BTreeSet<(usize, usize)>>::empty(2),
            vec![BTreeSet::new(), BTreeSet::new()]
        );

        assert_eq!(
            Vec::<BTreeSet<(usize, usize)>>::empty(3),
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]
        );
    }

    #[test]
    fn vec_hash_set_weighted() {
        assert_eq!(
            Vec::<HashSet<(usize, usize)>>::empty(1),
            vec![HashSet::new()]
        );

        assert_eq!(
            Vec::<HashSet<(usize, usize)>>::empty(2),
            vec![HashSet::new(), HashSet::new()]
        );

        assert_eq!(
            Vec::<HashSet<(usize, usize)>>::empty(3),
            vec![HashSet::new(), HashSet::new(), HashSet::new()]
        );
    }

    #[test]
    fn vec_btree_map_weighted() {
        assert_eq!(
            Vec::<BTreeMap<usize, usize>>::empty(1),
            vec![BTreeMap::new()]
        );

        assert_eq!(
            Vec::<BTreeMap<usize, usize>>::empty(2),
            vec![BTreeMap::new(), BTreeMap::new()]
        );

        assert_eq!(
            Vec::<BTreeMap<usize, usize>>::empty(3),
            vec![BTreeMap::new(), BTreeMap::new(), BTreeMap::new()]
        );
    }

    #[test]
    fn vec_hash_map_weighted() {
        assert_eq!(Vec::<HashMap<usize, usize>>::empty(1), vec![HashMap::new()]);

        assert_eq!(
            Vec::<HashMap<usize, usize>>::empty(2),
            vec![HashMap::new(), HashMap::new()]
        );

        assert_eq!(
            Vec::<HashMap<usize, usize>>::empty(3),
            vec![HashMap::new(), HashMap::new(), HashMap::new()]
        );
    }

    #[test]
    fn btree_map_vec_weighted() {
        assert_eq!(
            BTreeMap::<usize, Vec<(usize, usize)>>::empty(1),
            BTreeMap::from([(0, Vec::new())])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<(usize, usize)>>::empty(2),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new())])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<(usize, usize)>>::empty(3),
            BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())])
        );
    }

    #[test]
    fn btree_map_btree_set_weighted() {
        assert_eq!(
            BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(1),
            BTreeMap::from([(0, BTreeSet::new())])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(2),
            BTreeMap::from([(0, BTreeSet::new()), (1, BTreeSet::new())])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(3),
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::new()),
                (2, BTreeSet::new()),
            ])
        );
    }

    #[test]
    fn btree_map_btree_map() {
        assert_eq!(
            BTreeMap::<usize, BTreeMap<usize, usize>>::empty(1),
            BTreeMap::from([(0, BTreeMap::new())])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeMap<usize, usize>>::empty(2),
            BTreeMap::from([(0, BTreeMap::new()), (1, BTreeMap::new())])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3),
            BTreeMap::from([
                (0, BTreeMap::new()),
                (1, BTreeMap::new()),
                (2, BTreeMap::new()),
            ])
        );
    }

    #[test]
    fn hash_map_vec_weighted() {
        assert_eq!(
            HashMap::<usize, Vec<(usize, usize)>>::empty(1),
            HashMap::from([(0, Vec::new())])
        );

        assert_eq!(
            HashMap::<usize, Vec<(usize, usize)>>::empty(2),
            HashMap::from([(0, Vec::new()), (1, Vec::new())])
        );

        assert_eq!(
            HashMap::<usize, Vec<(usize, usize)>>::empty(3),
            HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())])
        );
    }

    #[test]
    fn hash_map_hash_set_weighted() {
        assert_eq!(
            HashMap::<usize, HashSet<(usize, usize)>>::empty(1),
            HashMap::from([(0, HashSet::new())])
        );

        assert_eq!(
            HashMap::<usize, HashSet<(usize, usize)>>::empty(2),
            HashMap::from([(0, HashSet::new()), (1, HashSet::new())])
        );

        assert_eq!(
            HashMap::<usize, HashSet<(usize, usize)>>::empty(3),
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::new()),
                (2, HashSet::new()),
            ])
        );
    }

    #[test]
    fn hash_map_hash_map() {
        assert_eq!(
            HashMap::<usize, HashMap<usize, usize>>::empty(1),
            HashMap::from([(0, HashMap::new())])
        );

        assert_eq!(
            HashMap::<usize, HashMap<usize, usize>>::empty(2),
            HashMap::from([(0, HashMap::new()), (1, HashMap::new())])
        );

        assert_eq!(
            HashMap::<usize, HashMap<usize, usize>>::empty(3),
            HashMap::from([
                (0, HashMap::new()),
                (1, HashMap::new()),
                (2, HashMap::new()),
            ])
        );
    }

    #[test]
    fn vec_tuple_weighted() {
        assert_eq!(Vec::<(usize, usize, usize)>::empty(1), Vec::new());
        assert_eq!(Vec::<(usize, usize, usize)>::empty(2), Vec::new());
        assert_eq!(Vec::<(usize, usize, usize)>::empty(3), Vec::new());
    }

    #[test]
    fn btree_set_tuple_weighted() {
        assert_eq!(BTreeSet::<(usize, usize, usize)>::empty(1), BTreeSet::new());
        assert_eq!(BTreeSet::<(usize, usize, usize)>::empty(2), BTreeSet::new());
        assert_eq!(BTreeSet::<(usize, usize, usize)>::empty(3), BTreeSet::new());
    }

    #[test]
    fn hash_set_tuple_weighted() {
        assert_eq!(HashSet::<(usize, usize, usize)>::empty(1), HashSet::new());
        assert_eq!(HashSet::<(usize, usize, usize)>::empty(2), HashSet::new());
        assert_eq!(HashSet::<(usize, usize, usize)>::empty(3), HashSet::new());
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_vec_unweighted_panic() {
        let _ = Vec::<Vec<usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_btree_set_unweighted_panic() {
        let _ = Vec::<BTreeSet<usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_hash_set_unweighted_panic() {
        let _ = Vec::<HashSet<usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_vec_unweighted_panic() {
        let _ = BTreeMap::<usize, Vec<usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_btree_set_unweighted_panic() {
        let _ = BTreeMap::<usize, BTreeSet<usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_vec_unweighted_panic() {
        let _ = HashMap::<usize, Vec<usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_hash_set_unweighted_panic() {
        let _ = HashMap::<usize, HashSet<usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_tuple_unweighted_panic() {
        let _ = Vec::<(usize, usize)>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_set_tuple_unweighted_panic() {
        let _ = BTreeSet::<(usize, usize)>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_set_tuple_unweighted_panic() {
        let _ = HashSet::<(usize, usize)>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_vec_weighted_panic() {
        let _ = Vec::<Vec<(usize, usize)>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_btree_set_weighted_panic() {
        let _ = Vec::<BTreeSet<(usize, usize)>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_hash_set_weighted_panic() {
        let _ = Vec::<HashSet<(usize, usize)>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_btree_map_weighted_panic() {
        let _ = Vec::<BTreeMap<usize, usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_hash_map_weighted_panic() {
        let _ = Vec::<HashMap<usize, usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_vec_weighted_panic() {
        let _ = BTreeMap::<usize, Vec<(usize, usize)>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_btree_set_weighted_panic() {
        let _ = BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_btree_map_panic() {
        let _ = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_vec_weighted_panic() {
        let _ = HashMap::<usize, Vec<(usize, usize)>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_hash_set_weighted_panic() {
        let _ = HashMap::<usize, HashSet<(usize, usize)>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_hash_map_panic() {
        let _ = HashMap::<usize, HashMap<usize, usize>>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_tuple_weighted_panic() {
        let _ = Vec::<(usize, usize, usize)>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_set_tuple_weighted_panic() {
        let _ = BTreeSet::<(usize, usize, usize)>::empty(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_set_tuple_weighted_panic() {
        let _ = HashSet::<(usize, usize, usize)>::empty(0);
    }
}
