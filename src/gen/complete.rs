//! Generate variable-sized complete symmetric digraphs.
//!
//! The generated digraphs are simple; they contain no self-loops. To generate
//! constant-sized complete digraphs, see [`CompleteConst`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Complete;
//!
//! // 0 -> {}
//!
//! assert_eq!(Vec::<Vec<usize>>::complete(1), vec![Vec::new()]);
//!
//! // 0 -> {1}
//! // 1 -> {0}
//!
//! assert_eq!(Vec::<Vec<usize>>::complete(2), vec![vec![1], vec![0]]);
//!
//! // 0 -> {1, 2}
//! // 1 -> {0, 2}
//! // 2 -> {0, 1}
//!
//! assert_eq!(
//!     Vec::<Vec<usize>>::complete(3),
//!     vec![vec![1, 2], vec![0, 2], vec![0, 1]]
//! );
//! ```
//!
//! [`CompleteConst`]: crate::gen::CompleteConst

use {
    super::Empty,
    crate::op::AddArc,
};

/// Generate variable-size symmetric complete digraphs.
///
/// # How can I implement `Complete`?
///
/// Provide an implementation of `complete` that generates a symmetric complete
/// digraph with `v` vertices.
///
/// ```
/// use {
///     graaf::gen::Complete,
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: HashSet<(usize, usize)>,
/// }
///
/// impl Complete for Digraph {
///     /// # Panics
///     ///
///     /// Panics if `v` is 0.
///     fn complete(v: usize) -> Self {
///         assert!(v > 0, "a graph must have at least one vertex");
///
///         let mut arcs = HashSet::new();
///
///         for s in 0..v {
///             for t in (s + 1)..v {
///                 let _ = arcs.insert((s, t));
///                 let _ = arcs.insert((t, s));
///             }
///         }
///
///         Digraph { arcs }
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0, 1}
///
/// let digraph = Digraph::complete(3);
///
/// assert_eq!(
///     digraph.arcs,
///     HashSet::from([(0, 1), (1, 0), (0, 2), (2, 0), (1, 2), (2, 1)])
/// );
/// ```
pub trait Complete {
    /// Generates a complete digraph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the digraph
    fn complete(v: usize) -> Self;
}

impl<D> Complete for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete(v: usize) -> Self {
        let mut digraph = D::empty(v);

        for s in 0..v {
            for t in (s + 1)..v {
                digraph.add_arc(s, t);
                digraph.add_arc(t, s);
            }
        }

        digraph
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::{
            op::{
                Indegree,
                IsSimple,
                IterVertices,
                Order,
                Outdegree,
                Size,
            },
            prop::sum_indegrees_eq_sum_outdegrees,
        },
        alloc::collections::{
            BTreeMap,
            BTreeSet,
        },
        proptest::prelude::*,
        std::collections::{
            HashMap,
            HashSet,
        },
    };

    fn prop_indegree<T>(v: usize)
    where
        T: Complete + Indegree,
    {
        let digraph = T::complete(v);

        for s in 0..v {
            assert_eq!(digraph.indegree(s), v - 1);
        }
    }

    fn prop_is_simple<T>(v: usize)
    where
        T: Complete + IsSimple,
    {
        assert!(T::complete(v).is_simple());
    }

    fn prop_order<T>(v: usize)
    where
        T: Complete + Order,
    {
        assert_eq!(T::complete(v).order(), v);
    }

    fn prop_outdegree<T>(v: usize)
    where
        T: Complete + Outdegree,
    {
        let digraph = T::complete(v);

        for s in 0..v {
            assert_eq!(digraph.outdegree(s), v - 1);
        }
    }

    fn prop_size<T>(v: usize)
    where
        T: Complete + Size,
    {
        assert_eq!(T::complete(v).size(), v * (v - 1));
    }

    fn prop_sum_indegrees_eq_sum_outdegrees<T>(v: usize)
    where
        T: Complete + Indegree + IterVertices + Outdegree,
    {
        assert!(sum_indegrees_eq_sum_outdegrees(&T::complete(v)));
    }

    proptest! {
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
        fn order_vec_vec(v in 1..100_usize) {
            prop_order::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn order_vec_btree_set(v in 1..100_usize) {
            prop_order::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn order_vec_hash_set(v in 1..100_usize) {
            prop_order::<Vec<HashSet<usize>>>(v);
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

        #[test]
        fn size_vec_vec(v in 1..100_usize) {
            prop_size::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn size_vec_btree_set(v in 1..100_usize) {
            prop_size::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn size_vec_hash_set(v in 1..100_usize) {
            prop_size::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn size_btree_map_vec(v in 1..100_usize) {
            prop_size::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn size_btree_map_btree_set(v in 1..100_usize) {
            prop_size::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn size_hash_map_vec(v in 1..100_usize) {
            prop_size::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn size_hash_map_hash_set(v in 1..100_usize) {
            prop_size::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn size_vec_tuple(v in 1..100_usize) {
            prop_size::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn size_btree_set_tuple(v in 1..100_usize) {
            prop_size::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn size_hash_set_tuple(v in 1..100_usize) {
            prop_size::<HashSet<(usize, usize)>>(v);
        }

        #[test]
        fn sum_indegrees_eq_sum_outdegrees_vec_btree_set(v in 1..100_usize) {
            prop_sum_indegrees_eq_sum_outdegrees::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn sum_indegrees_eq_sum_outdegrees_vec_hash_set(v in 1..100_usize) {
            prop_sum_indegrees_eq_sum_outdegrees::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn sum_indegrees_eq_sum_outdegrees_btree_map_btree_set(v in 1..100_usize) {
            prop_sum_indegrees_eq_sum_outdegrees::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn sum_indegrees_eq_sum_outdegrees_hash_map_hash_set(v in 1..100_usize) {
            prop_sum_indegrees_eq_sum_outdegrees::<HashMap<usize, HashSet<usize>>>(v);
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
            vec![(0, 1), (1, 0), (0, 2), (2, 0), (1, 2), (2, 1)]
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
