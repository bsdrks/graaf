#![doc(alias = "in_degree")]
//! Get the indegree of a vertex.
//!
//! The indegree is the number of arcs incident into a vertex.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::Indegree,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
//!
//! assert_eq!(digraph.indegree(0), 0);
//! assert_eq!(digraph.indegree(1), 1);
//! assert_eq!(digraph.indegree(2), 2);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Get the indegree of a vertex.
///
/// # How can I implement `Indegree`?
///
/// Provide an implementation of `indegree` that returns the indegree of the
/// target vertex.
///
/// ```
/// use {
///     graaf::op::Indegree,
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl Indegree for Digraph {
///     fn indegree(&self, t: usize) -> usize {
///         self.arcs.iter().filter(|set| set.contains(&t)).count()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::Indegree,
///     std::collections::HashSet,
/// };
///
/// let digraph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
///
/// assert_eq!(digraph.indegree(0), 0);
/// assert_eq!(digraph.indegree(1), 1);
/// assert_eq!(digraph.indegree(2), 2);
/// ```
pub trait Indegree {
    /// Returns the indegree of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `t`: The tail vertex.
    fn indegree(&self, t: usize) -> usize;

    /// Returns whether a vertex is a source of the digraph.
    ///
    /// A source is a vertex with an indegree of 0.
    ///
    /// # Arguments
    ///
    /// * `t`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use {
    ///     graaf::op::Indegree,
    ///     std::collections::HashSet,
    /// };
    ///
    /// let digraph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
    ///
    /// assert!(digraph.is_source(0));
    /// assert!(!digraph.is_source(1));
    /// assert!(!digraph.is_source(2));
    /// ```
    fn is_source(&self, t: usize) -> bool {
        self.indegree(t) == 0
    }
}

macro_rules! impl_iter_contains {
    () => {
        fn indegree(&self, t: usize) -> usize {
            self.iter().filter(|set| set.contains(&t)).count()
        }
    };
}

macro_rules! impl_values_contains {
    () => {
        fn indegree(&self, t: usize) -> usize {
            self.values().filter(|set| set.contains(&t)).count()
        }
    };
}

macro_rules! impl_iter_contains_key {
    () => {
        fn indegree(&self, t: usize) -> usize {
            self.iter().filter(|map| map.contains_key(&t)).count()
        }
    };
}

macro_rules! impl_values_contains_key {
    () => {
        fn indegree(&self, t: usize) -> usize {
            self.values().filter(|map| map.contains_key(&t)).count()
        }
    };
}

impl Indegree for Vec<BTreeSet<usize>> {
    impl_iter_contains!();
}

impl<H> Indegree for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_iter_contains!();
}

impl Indegree for [BTreeSet<usize>] {
    impl_iter_contains!();
}

impl<H> Indegree for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_iter_contains!();
}

impl<const V: usize> Indegree for [BTreeSet<usize>; V] {
    impl_iter_contains!();
}

impl<const V: usize, H> Indegree for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_iter_contains!();
}

impl Indegree for BTreeMap<usize, BTreeSet<usize>> {
    impl_values_contains!();
}

impl<H> Indegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_values_contains!();
}

impl<W> Indegree for Vec<BTreeMap<usize, W>> {
    impl_iter_contains_key!();
}

impl<W, H> Indegree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_iter_contains_key!();
}

impl<W> Indegree for [BTreeMap<usize, W>] {
    impl_iter_contains_key!();
}

impl<W, H> Indegree for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_iter_contains_key!();
}

impl<const V: usize, W> Indegree for [BTreeMap<usize, W>; V] {
    impl_iter_contains_key!();
}

impl<const V: usize, W, H> Indegree for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_iter_contains_key!();
}

impl<W> Indegree for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_values_contains_key!();
}

impl<W, H> Indegree for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_values_contains_key!();
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::{
                Empty,
                EmptyConst,
            },
            op::{
                AddArc,
                AddWeightedArc,
            },
        },
    };

    macro_rules! test_indegree {
        ($digraph:expr) => {
            assert_eq!($digraph.indegree(0), 0);
            assert_eq!($digraph.indegree(1), 1);
            assert_eq!($digraph.indegree(2), 2);
        };
    }

    macro_rules! test_indegree_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 2);

            test_indegree!($digraph);
        };
    }

    macro_rules! test_indegree_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 2);
            $digraph.add_weighted_arc(1, 2, 3);

            test_indegree!($digraph);
        };
    }

    macro_rules! test_is_source {
        ($digraph:expr) => {
            assert!($digraph.is_source(0));
            assert!(!$digraph.is_source(1));
            assert!($digraph.is_source(2));
            assert!(!$digraph.is_source(3));
        };
    }

    macro_rules! test_is_source_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 3);
            $digraph.add_arc(2, 1);

            test_is_source!($digraph);
        };
    }

    macro_rules! test_is_source_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 3, 2);
            $digraph.add_weighted_arc(2, 1, 4);

            test_is_source!($digraph);
        };
    }

    #[test]
    fn vec_btree_set() {
        let digraph = &mut <Vec<BTreeSet<usize>>>::empty(3);

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = &mut <Vec<HashSet<usize>>>::empty(3);

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(3);

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(3);

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let digraph = &mut <[BTreeSet<usize>; 3]>::empty();

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = &mut <[HashSet<usize>; 3]>::empty();

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = &mut <Vec<BTreeMap<usize, usize>>>::empty(3);

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = &mut <Vec<HashMap<usize, usize>>>::empty(3);

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, usize>] = &mut Vec::<BTreeMap<usize, usize>>::empty(3);

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &mut [HashMap<usize, usize>] = &mut Vec::<HashMap<usize, usize>>::empty(3);

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = &mut <[BTreeMap<usize, usize>; 3]>::empty();

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = &mut <[HashMap<usize, usize>; 3]>::empty();

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let digraph = &mut BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = &mut HashMap::<usize, HashSet<usize>>::empty(3);

        test_indegree_unweighted!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let digraph = &mut HashMap::<usize, HashMap<usize, usize>>::empty(3);

        test_indegree_weighted!(digraph);
    }

    #[test]
    fn is_source_vec_btree_set() {
        let digraph = &mut <Vec<BTreeSet<usize>>>::empty(3);

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_vec_hash_set() {
        let digraph = &mut <Vec<HashSet<usize>>>::empty(3);

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_slice_btree_set() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(3);

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_slice_hash_set() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(3);

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_arr_btree_set() {
        let digraph = &mut <[BTreeSet<usize>; 3]>::empty();

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_arr_hash_set() {
        let digraph = &mut <[HashSet<usize>; 3]>::empty();

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_vec_btree_map() {
        let digraph = &mut <Vec<BTreeMap<usize, usize>>>::empty(3);

        test_is_source_weighted!(digraph);
    }

    #[test]
    fn is_source_vec_hash_map() {
        let digraph = &mut <Vec<HashMap<usize, usize>>>::empty(3);

        test_is_source_weighted!(digraph);
    }

    #[test]
    fn is_source_slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, usize>] = &mut Vec::<BTreeMap<usize, usize>>::empty(3);

        test_is_source_weighted!(digraph);
    }

    #[test]
    fn is_source_slice_hash_map() {
        let digraph: &mut [HashMap<usize, usize>] = &mut Vec::<HashMap<usize, usize>>::empty(3);

        test_is_source_weighted!(digraph);
    }

    #[test]
    fn is_source_arr_btree_map() {
        let digraph = &mut <[BTreeMap<usize, usize>; 3]>::empty();

        test_is_source_weighted!(digraph);
    }

    #[test]
    fn is_source_arr_hash_map() {
        let digraph = &mut <[HashMap<usize, usize>; 3]>::empty();

        test_is_source_weighted!(digraph);
    }

    #[test]
    fn is_source_btree_map_btree_set() {
        let digraph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_btree_map_btree_map() {
        let digraph = &mut BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        test_is_source_weighted!(digraph);
    }

    #[test]
    fn is_source_hash_map_hash_set() {
        let digraph = &mut HashMap::<usize, HashSet<usize>>::empty(3);

        test_is_source_unweighted!(digraph);
    }

    #[test]
    fn is_source_hash_map_hash_map() {
        let digraph = &mut HashMap::<usize, HashMap<usize, usize>>::empty(3);

        test_is_source_weighted!(digraph);
    }
}
