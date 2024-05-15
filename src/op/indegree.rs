#![doc(alias = "in_valence")]
#![doc(alias = "inward_demidegree")]
//! A trait to get the indegree of a given vertex
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
//! let graph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
//!
//! assert_eq!(graph.indegree(0), 0);
//! assert_eq!(graph.indegree(1), 1);
//! assert_eq!(graph.indegree(2), 2);
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

/// A trait to get the indegree of a given vertex
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
/// struct Graph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl Indegree for Graph {
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
/// let graph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
///
/// assert_eq!(graph.indegree(0), 0);
/// assert_eq!(graph.indegree(1), 1);
/// assert_eq!(graph.indegree(2), 2);
/// ```
pub trait Indegree {
    /// Returns the indegree of a vertex.
    ///
    /// # Arguments
    ///
    /// * `t`: The target vertex.
    fn indegree(&self, t: usize) -> usize;
}

impl Indegree for Vec<BTreeSet<usize>> {
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl<H> Indegree for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl Indegree for [BTreeSet<usize>] {
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl<H> Indegree for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl<const V: usize> Indegree for [BTreeSet<usize>; V] {
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl<const V: usize, H> Indegree for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl Indegree for BTreeMap<usize, BTreeSet<usize>> {
    fn indegree(&self, t: usize) -> usize {
        self.values().filter(|set| set.contains(&t)).count()
    }
}

impl<H> Indegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.values().filter(|set| set.contains(&t)).count()
    }
}

impl<W> Indegree for Vec<BTreeMap<usize, W>> {
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

impl<W, H> Indegree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

impl<W> Indegree for [BTreeMap<usize, W>] {
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

impl<W, H> Indegree for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

impl<const V: usize, W> Indegree for [BTreeMap<usize, W>; V] {
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

impl<const V: usize, W, H> Indegree for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}

impl<W> Indegree for BTreeMap<usize, BTreeMap<usize, W>> {
    fn indegree(&self, t: usize) -> usize {
        self.values().filter(|map| map.contains_key(&t)).count()
    }
}

impl<W, H> Indegree for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn indegree(&self, t: usize) -> usize {
        self.values()
            .map(|map| map.keys().filter(|&&u| u == t).count())
            .sum()
    }
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
        ($graph:expr) => {
            assert_eq!($graph.indegree(0), 0);
            assert_eq!($graph.indegree(1), 1);
            assert_eq!($graph.indegree(2), 2);
        };
    }

    macro_rules! test_indegree_unweighted {
        ($graph:expr) => {
            $graph.add_arc(0, 1);
            $graph.add_arc(0, 2);
            $graph.add_arc(1, 2);

            test_indegree!($graph);
        };
    }

    macro_rules! test_indegree_weighted {
        ($graph:expr) => {
            $graph.add_weighted_arc(0, 1, 1);
            $graph.add_weighted_arc(0, 2, 2);
            $graph.add_weighted_arc(1, 2, 3);

            test_indegree!($graph);
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = &mut <Vec<BTreeSet<usize>>>::empty(3);

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = &mut <Vec<HashSet<usize>>>::empty(3);

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(3);

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(3);

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = &mut <[BTreeSet<usize>; 3]>::empty();

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = &mut <[HashSet<usize>; 3]>::empty();

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = &mut <Vec<BTreeMap<usize, usize>>>::empty(3);

        test_indegree_weighted!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = &mut <Vec<HashMap<usize, usize>>>::empty(3);

        test_indegree_weighted!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, usize>] = &mut Vec::<BTreeMap<usize, usize>>::empty(3);

        test_indegree_weighted!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, usize>] = &mut Vec::<HashMap<usize, usize>>::empty(3);

        test_indegree_weighted!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = &mut <[BTreeMap<usize, usize>; 3]>::empty();

        test_indegree_weighted!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = &mut <[HashMap<usize, usize>; 3]>::empty();

        test_indegree_weighted!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = &mut BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        test_indegree_weighted!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = &mut HashMap::<usize, HashSet<usize>>::empty(3);

        test_indegree_unweighted!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = &mut HashMap::<usize, HashMap<usize, usize>>::empty(3);

        test_indegree_weighted!(graph);
    }
}
