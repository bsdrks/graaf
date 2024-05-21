#![doc(alias = "circular")]
//! A trait to generate variable-sized directed cycle digraphs
//!
//! Cycle graphs are also known as circular graphs. To generate const-sized
//! cycle digraphs, see [`CycleConst`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::Cycle;
//!
//! assert_eq!(Vec::<Vec<usize>>::cycle(1), vec![vec![0]]);
//! assert_eq!(Vec::<Vec<usize>>::cycle(2), vec![vec![1], vec![0]]);
//! assert_eq!(Vec::<Vec<usize>>::cycle(3), vec![vec![1], vec![2], vec![0]]);
//! ```
//!
//! [`CycleConst`]: crate::gen::CycleConst

extern crate alloc;

use {
    crate::gen::Empty,
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

/// A trait to generate variable-size directed cycle digraphs
///
/// # How can I implement `Cycle`?
///
/// Provide an implementation of `cycle` that generates a cycle digraph with `v`
/// vertices.
///
/// ```
/// use {
///     graaf::gen::Cycle,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     arcs: HashSet<(usize, usize)>,
/// }
///
/// impl Cycle for Graph {
///     fn cycle(v: usize) -> Self {
///         let mut arcs = (0..v - 1).map(|s| (s, s + 1)).collect::<HashSet<_>>();
///
///         arcs.insert((v - 1, 0));
///
///         Graph { arcs }
///     }
/// }
///
/// let digraph = Graph::cycle(3);
///
/// assert_eq!(digraph.arcs, HashSet::from([(0, 1), (1, 2), (2, 0)]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::gen::Cycle;
///
/// assert_eq!(Vec::<Vec<usize>>::cycle(1), vec![vec![0]]);
/// assert_eq!(Vec::<Vec<usize>>::cycle(2), vec![vec![1], vec![0]]);
/// assert_eq!(Vec::<Vec<usize>>::cycle(3), vec![vec![1], vec![2], vec![0]]);
/// ```
pub trait Cycle {
    /// Generates a cycle digraph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the digraph
    #[must_use]
    fn cycle(v: usize) -> Self;
}

impl Cycle for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        let mut digraph = Self::empty(v);

        for (s, vec) in digraph.iter_mut().enumerate().take(v - 1) {
            vec.push(s + 1);
        }

        digraph[v - 1].push(0);

        digraph
    }
}

impl Cycle for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        let mut digraph = Self::empty(v);

        for (s, set) in digraph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = digraph[v - 1].insert(0);

        digraph
    }
}

impl<H> Cycle for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        let mut digraph = Self::empty(v);

        for (s, set) in digraph.iter_mut().enumerate().take(v - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = digraph[v - 1].insert(0);

        digraph
    }
}

impl Cycle for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        let mut digraph = Self::empty(v);

        for s in 0..v - 1 {
            let _ = digraph.insert(s, vec![s + 1]);
        }

        let _ = digraph.insert(v - 1, vec![0]);

        digraph
    }
}

impl Cycle for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        let mut digraph = Self::empty(v);

        for s in 0..v - 1 {
            let _ = digraph.insert(s, BTreeSet::from([s + 1]));
        }

        let _ = digraph.insert(v - 1, BTreeSet::from([0]));

        digraph
    }
}

impl<H> Cycle for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        let mut digraph = Self::empty(v);

        for s in 0..v - 1 {
            let _ = digraph.insert(s, vec![s + 1]);
        }

        let _ = digraph.insert(v - 1, vec![0]);

        digraph
    }
}

impl<H> Cycle for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        let mut digraph = Self::empty(v);

        for s in 0..v - 1 {
            let mut set = HashSet::with_hasher(H::default());
            let _ = set.insert(s + 1);
            let _ = digraph.insert(s, set);
        }

        let mut set = HashSet::with_hasher(H::default());
        let _ = set.insert(0);
        let _ = digraph.insert(v - 1, set);

        digraph
    }
}

impl Cycle for Vec<(usize, usize)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, (s + 1) % v)).collect()
    }
}

impl Cycle for BTreeSet<(usize, usize)> {
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, (s + 1) % v)).collect()
    }
}

impl<H> Cycle for HashSet<(usize, usize), H>
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn cycle(v: usize) -> Self {
        assert!(v > 0, "a graph must have at least one vertex");

        (0..v).map(|s| (s, (s + 1) % v)).collect()
    }
}

#[cfg(test)]
mod tests {
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
        proptest::prelude::*,
    };

    fn prop_size<T>(v: usize)
    where
        T: Cycle + Size,
    {
        assert_eq!(T::cycle(v).size(), v);
    }

    fn prop_order<T>(v: usize)
    where
        T: Cycle + Order,
    {
        assert_eq!(T::cycle(v).order(), v);
    }

    fn prop_indegree<T>(v: usize)
    where
        T: Cycle + Indegree,
    {
        let digraph = T::cycle(v);

        for s in 0..v {
            assert_eq!(digraph.indegree(s), 1);
        }
    }

    fn prop_is_simple<T>(v: usize)
    where
        T: Cycle + IsSimple,
    {
        assert!(T::cycle(v).is_simple());
    }

    fn prop_outdegree<T>(v: usize)
    where
        T: Cycle + Outdegree,
    {
        let digraph = T::cycle(v);

        for s in 0..v {
            assert_eq!(digraph.outdegree(s), 1);
        }
    }

    fn prop_sum_indegrees_eq_sum_outdegrees<T>(v: usize)
    where
        T: Cycle + Indegree + IterVertices + Outdegree,
    {
        assert!(sum_indegrees_eq_sum_outdegrees(&T::cycle(v)));
    }

    proptest! {
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
        fn is_simple_vec_btree_set(v in 2..100_usize) {
            prop_is_simple::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_hash_set(v in 2..100_usize) {
            prop_is_simple::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn is_simple_vec_tuple(v in 2..100_usize) {
            prop_is_simple::<Vec<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_btree_set_tuple(v in 2..100_usize) {
            prop_is_simple::<BTreeSet<(usize, usize)>>(v);
        }

        #[test]
        fn is_simple_hash_set_tuple(v in 2..100_usize) {
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
        assert_eq!(Vec::<Vec<usize>>::cycle(1), vec![vec![0]]);
        assert_eq!(Vec::<Vec<usize>>::cycle(2), vec![vec![1], vec![0]]);
        assert_eq!(Vec::<Vec<usize>>::cycle(3), vec![vec![1], vec![2], vec![0]]);
    }

    #[test]
    fn vec_btree_set() {
        assert_eq!(Vec::<BTreeSet<usize>>::cycle(1), vec![BTreeSet::from([0])]);

        assert_eq!(
            Vec::<BTreeSet<usize>>::cycle(2),
            vec![BTreeSet::from([1]), BTreeSet::from([0])]
        );

        assert_eq!(
            Vec::<BTreeSet<usize>>::cycle(3),
            vec![
                BTreeSet::from([1]),
                BTreeSet::from([2]),
                BTreeSet::from([0])
            ]
        );
    }

    #[test]
    fn vec_hash_set() {
        assert_eq!(Vec::<HashSet<usize>>::cycle(1), vec![HashSet::from([0])]);

        assert_eq!(
            Vec::<HashSet<usize>>::cycle(2),
            vec![HashSet::from([1]), HashSet::from([0])]
        );

        assert_eq!(
            Vec::<HashSet<usize>>::cycle(3),
            vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([0])]
        );
    }

    #[test]
    fn btree_map_vec() {
        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::cycle(1),
            BTreeMap::from([(0, vec![0])])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::cycle(2),
            BTreeMap::from([(0, vec![1]), (1, vec![0])])
        );

        assert_eq!(
            BTreeMap::<usize, Vec<usize>>::cycle(3),
            BTreeMap::from([(0, vec![1]), (1, vec![2]), (2, vec![0])])
        );
    }

    #[test]
    fn btree_map_btree_set() {
        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::cycle(1),
            BTreeMap::from([(0, BTreeSet::from([0]))])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::cycle(2),
            BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::from([0]))])
        );

        assert_eq!(
            BTreeMap::<usize, BTreeSet<usize>>::cycle(3),
            BTreeMap::from([
                (0, BTreeSet::from([1])),
                (1, BTreeSet::from([2])),
                (2, BTreeSet::from([0]))
            ])
        );
    }

    #[test]
    fn hash_map_vec() {
        assert_eq!(
            HashMap::<usize, Vec<usize>>::cycle(1),
            HashMap::from([(0, vec![0])])
        );

        assert_eq!(
            HashMap::<usize, Vec<usize>>::cycle(2),
            HashMap::from([(0, vec![1]), (1, vec![0])])
        );

        assert_eq!(
            HashMap::<usize, Vec<usize>>::cycle(3),
            HashMap::from([(0, vec![1]), (1, vec![2]), (2, vec![0])])
        );
    }

    #[test]
    fn hash_map_hash_set() {
        assert_eq!(
            HashMap::<usize, HashSet<usize>>::cycle(1),
            HashMap::from([(0, HashSet::from([0]))])
        );

        assert_eq!(
            HashMap::<usize, HashSet<usize>>::cycle(2),
            HashMap::from([(0, HashSet::from([1])), (1, HashSet::from([0]))])
        );

        assert_eq!(
            HashMap::<usize, HashSet<usize>>::cycle(3),
            HashMap::from([
                (0, HashSet::from([1])),
                (1, HashSet::from([2])),
                (2, HashSet::from([0]))
            ])
        );
    }

    #[test]
    fn vec_tuple() {
        assert_eq!(Vec::<(usize, usize)>::cycle(1), vec![(0, 0)]);
        assert_eq!(Vec::<(usize, usize)>::cycle(2), vec![(0, 1), (1, 0)]);

        assert_eq!(
            Vec::<(usize, usize)>::cycle(3),
            vec![(0, 1), (1, 2), (2, 0)]
        );
    }

    #[test]
    fn btree_set_tuple() {
        assert_eq!(
            BTreeSet::<(usize, usize)>::cycle(1),
            BTreeSet::from([(0, 0)])
        );

        assert_eq!(
            BTreeSet::<(usize, usize)>::cycle(2),
            BTreeSet::from([(0, 1), (1, 0)])
        );

        assert_eq!(
            BTreeSet::<(usize, usize)>::cycle(3),
            BTreeSet::from([(0, 1), (1, 2), (2, 0)])
        );
    }

    #[test]
    fn hash_set_tuple() {
        assert_eq!(HashSet::<(usize, usize)>::cycle(1), HashSet::from([(0, 0)]));

        assert_eq!(
            HashSet::<(usize, usize)>::cycle(2),
            HashSet::from([(0, 1), (1, 0)])
        );

        assert_eq!(
            HashSet::<(usize, usize)>::cycle(3),
            HashSet::from([(0, 1), (1, 2), (2, 0)])
        );
    }

    #[test]
    fn is_simple_vec_btree_set_1() {
        assert!(!Vec::<BTreeSet<usize>>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_vec_hash_set_1() {
        assert!(!Vec::<HashSet<usize>>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_vec_tuple_1() {
        assert!(!Vec::<(usize, usize)>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_btree_set_tuple_1() {
        assert!(!BTreeSet::<(usize, usize)>::cycle(1).is_simple());
    }

    #[test]
    fn is_simple_hash_set_tuple_1() {
        assert!(!HashSet::<(usize, usize)>::cycle(1).is_simple());
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_vec_unweighted_panic() {
        let _ = Vec::<Vec<usize>>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_btree_set_unweighted_panic() {
        let _ = Vec::<BTreeSet<usize>>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_hash_set_unweighted_panic() {
        let _ = Vec::<HashSet<usize>>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_vec_unweighted_panic() {
        let _ = BTreeMap::<usize, Vec<usize>>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_map_btree_set_unweighted_panic() {
        let _ = BTreeMap::<usize, BTreeSet<usize>>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_vec_unweighted_panic() {
        let _ = HashMap::<usize, Vec<usize>>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_map_hash_set_unweighted_panic() {
        let _ = HashMap::<usize, HashSet<usize>>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn vec_tuple_unweighted_panic() {
        let _ = Vec::<(usize, usize)>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn btree_set_tuple_unweighted_panic() {
        let _ = BTreeSet::<(usize, usize)>::cycle(0);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn hash_set_tuple_unweighted_panic() {
        let _ = HashSet::<(usize, usize)>::cycle(0);
    }
}
