//! A trait to get the indegree of a given vertex
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
///     edges: Vec<HashSet<usize>>,
///     vertices: usize,
/// }
///
/// impl Indegree for Graph {
///     fn indegree(&self, t: usize) -> usize {
///         self.edges.iter().filter(|set| set.contains(&t)).count()
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
    /// Return the indegree of a vertex.
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
        self.values()
            .map(|set| set.iter().filter(|&&u| u == t).count())
            .sum()
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
    use super::*;

    macro_rules! test_indegree {
        ($graph:expr) => {
            assert_eq!($graph.indegree(0), 0);
            assert_eq!($graph.indegree(1), 1);
            assert_eq!($graph.indegree(2), 2);
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        test_indegree!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 1)]),
            BTreeMap::new(),
        ];

        test_indegree!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] =
            &[BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        test_indegree!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, usize>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 1)]),
            BTreeMap::new(),
        ];

        test_indegree!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        test_indegree!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 1)]),
            BTreeMap::new(),
        ];

        test_indegree!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([2])),
            (2, BTreeSet::new()),
        ]);

        test_indegree!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3)])),
            (1, BTreeMap::from([(2, 1)])),
            (2, BTreeMap::new()),
        ]);

        test_indegree!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        test_indegree!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 1)]),
            HashMap::new(),
        ];

        test_indegree!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        test_indegree!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, usize>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 1)]),
            HashMap::new(),
        ];

        test_indegree!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        test_indegree!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 1)]),
            HashMap::new(),
        ];

        test_indegree!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([2])),
            (2, HashSet::new()),
        ]);

        test_indegree!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3)])),
            (1, HashMap::from([(2, 1)])),
            (2, HashMap::new()),
        ]);

        test_indegree!(graph);
    }
}
