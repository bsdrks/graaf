//! A trait to get the outdegree of a given vertex
//!
//! # Examples
//!
//! ```
//! use graaf::op::Outdegree;
//!
//! let graph = vec![vec![1, 2], vec![0], vec![1]];
//!
//! assert_eq!(graph.outdegree(0), 2);
//! assert_eq!(graph.outdegree(1), 1);
//! assert_eq!(graph.outdegree(2), 1);
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

/// A trait to get the outdegree of a given vertex
///
/// # How can I implement `Outdegree`?
///
/// Provide an implementation of `outdegree` that returns the outdegree of the
/// target vertex.
///
/// ```
/// use graaf::op::Outdegree;
///
/// struct Graph {
///     edges: Vec<Vec<usize>>,
/// }
///
/// impl Outdegree for Graph {
///     fn outdegree(&self, s: usize) -> usize {
///         self.edges[s].len()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::Outdegree;
///
/// let graph = vec![vec![1, 2], vec![0], vec![1]];
///
/// assert_eq!(graph.outdegree(0), 2);
/// assert_eq!(graph.outdegree(1), 1);
/// assert_eq!(graph.outdegree(2), 1);
/// ```
pub trait Outdegree {
    /// Returns the outdegree of a vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn outdegree(&self, s: usize) -> usize;
}

impl Outdegree for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W> Outdegree for Vec<Vec<(usize, W)>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl Outdegree for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W> Outdegree for Vec<BTreeSet<(usize, W)>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<H> Outdegree for Vec<HashSet<usize, H>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W, H> Outdegree for Vec<HashSet<(usize, W), H>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W> Outdegree for Vec<BTreeMap<usize, W>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W, H> Outdegree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl Outdegree for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W> Outdegree for [Vec<(usize, W)>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl Outdegree for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W> Outdegree for [BTreeSet<(usize, W)>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<H> Outdegree for [HashSet<usize, H>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W, H> Outdegree for [HashSet<(usize, W), H>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W> Outdegree for [BTreeMap<usize, W>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<W, H> Outdegree for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize> Outdegree for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, W> Outdegree for [Vec<(usize, W)>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize> Outdegree for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, W> Outdegree for [BTreeSet<(usize, W)>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, H> Outdegree for [HashSet<usize, H>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, W, H> Outdegree for [HashSet<(usize, W), H>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, W> Outdegree for [BTreeMap<usize, W>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, W, H> Outdegree for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl Outdegree for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<H> Outdegree for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl Outdegree for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<H> Outdegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<W> Outdegree for BTreeMap<usize, BTreeMap<usize, W>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<W, H> Outdegree for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn outdegree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_outdegree {
        ($graph:expr) => {
            assert_eq!($graph.outdegree(0), 2);
            assert_eq!($graph.outdegree(1), 1);
            assert_eq!($graph.outdegree(2), 1);
        };
    }

    #[test]
    fn vec_vec_unweighted() {
        let graph = vec![vec![1, 2], vec![0], vec![1]];

        test_outdegree!(graph);
    }

    #[test]
    fn vec_vec_weighted() {
        let graph = vec![vec![(1, 1), (2, 1)], vec![(0, 1)], vec![(1, 1)]];

        test_outdegree!(graph);
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let graph = vec![
            BTreeSet::from([(1, 1), (2, 1)]),
            BTreeSet::from([(0, 1)]),
            BTreeSet::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let graph = vec![
            HashSet::from([(1, 1), (2, 1)]),
            HashSet::from([(0, 1)]),
            HashSet::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_vec_unweighted() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0], vec![1]];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_vec_weighted() {
        let graph: &[Vec<(usize, i32)>] = &[vec![(1, 1), (2, 1)], vec![(0, 1)], vec![(1, 1)]];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_btree_set_weighted() {
        let graph: &[BTreeSet<(usize, i32)>] = &[
            BTreeSet::from([(1, 1), (2, 1)]),
            BTreeSet::from([(0, 1)]),
            BTreeSet::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_hash_set_weighted() {
        let graph: &[HashSet<(usize, i32)>] = &[
            HashSet::from([(1, 1), (2, 1)]),
            HashSet::from([(0, 1)]),
            HashSet::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_vec_unweighted() {
        let graph = [vec![1, 2], vec![0], vec![1]];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_vec_weighted() {
        let graph = [vec![(1, 1), (2, 1)], vec![(0, 1)], vec![(1, 1)]];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let graph = [
            BTreeSet::from([(1, 1), (2, 1)]),
            BTreeSet::from([(0, 1)]),
            BTreeSet::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let graph = [
            HashSet::from([(1, 1), (2, 1)]),
            HashSet::from([(0, 1)]),
            HashSet::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        test_outdegree!(graph);
    }

    #[test]
    fn btree_map_vec() {
        let graph = BTreeMap::from([(0, vec![1, 2]), (1, vec![0]), (2, vec![1])]);

        test_outdegree!(graph);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([(0, vec![1, 2]), (1, vec![0]), (2, vec![1])]);

        test_outdegree!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0])),
            (2, BTreeSet::from([1])),
        ]);

        test_outdegree!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([1])),
        ]);

        test_outdegree!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 1), (2, 1)])),
            (1, BTreeMap::from([(0, 1)])),
            (2, BTreeMap::from([(1, 1)])),
        ]);

        test_outdegree!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(1, 1)])),
        ]);

        test_outdegree!(graph);
    }
}
