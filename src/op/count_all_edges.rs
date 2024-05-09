//! A trait to count all edges in a directed graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::CountAllEdges;
//!
//! let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(graph.count_all_edges(), 10);
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

/// A trait to count all edges in a directed graph
///
/// # How can I implement `CountAllEdges`?
///
/// Provide an implementation of `count_all_edges` that returns the number of
/// edges in the graph.
///
/// ```
/// use graaf::op::CountAllEdges;
///
/// struct Graph {
///     edges: Vec<Vec<usize>>,
/// }
///
/// impl CountAllEdges for Graph {
///     fn count_all_edges(&self) -> usize {
///         self.edges.iter().map(Vec::len).sum()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::CountAllEdges;
///
/// let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(graph.count_all_edges(), 10);
/// ```
pub trait CountAllEdges {
    /// Counts all edges.
    fn count_all_edges(&self) -> usize;
}

impl<T> CountAllEdges for Vec<Vec<T>> {
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T> CountAllEdges for Vec<BTreeSet<T>> {
    fn count_all_edges(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<T, H> CountAllEdges for Vec<HashSet<T, H>>
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<K, W> CountAllEdges for Vec<BTreeMap<K, W>> {
    fn count_all_edges(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<K, W, H> CountAllEdges for Vec<HashMap<K, W, H>>
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<T> CountAllEdges for [Vec<T>] {
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T> CountAllEdges for [BTreeSet<T>] {
    fn count_all_edges(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<T, H> CountAllEdges for [HashSet<T, H>]
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<K, W> CountAllEdges for [BTreeMap<K, W>] {
    fn count_all_edges(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<K, W, H> CountAllEdges for [HashMap<K, W, H>]
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<const V: usize, T> CountAllEdges for [Vec<T>; V] {
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<const V: usize, T> CountAllEdges for [BTreeSet<T>; V] {
    fn count_all_edges(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<const V: usize, K, W> CountAllEdges for [BTreeMap<K, W>; V] {
    fn count_all_edges(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<const V: usize, T, H> CountAllEdges for [HashSet<T, H>; V]
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<const V: usize, K, W, H> CountAllEdges for [HashMap<K, W, H>; V]
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<K, T> CountAllEdges for BTreeMap<K, Vec<T>> {
    fn count_all_edges(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, T> CountAllEdges for BTreeMap<K, BTreeSet<T>> {
    fn count_all_edges(&self) -> usize {
        self.values().map(BTreeSet::len).sum()
    }
}

impl<K, W> CountAllEdges for BTreeMap<K, BTreeMap<K, W>> {
    fn count_all_edges(&self) -> usize {
        self.values().map(BTreeMap::len).sum()
    }
}

impl<K, T, H> CountAllEdges for HashMap<K, Vec<T>, H>
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, T, H> CountAllEdges for HashMap<K, HashSet<T, H>, H>
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.values().map(HashSet::len).sum()
    }
}

impl<K, W, H> CountAllEdges for HashMap<K, HashMap<K, W, H>, H>
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.values().map(HashMap::len).sum()
    }
}

impl CountAllEdges for Vec<(usize, usize)> {
    fn count_all_edges(&self) -> usize {
        self.len()
    }
}

impl<W> CountAllEdges for Vec<(usize, usize, W)> {
    fn count_all_edges(&self) -> usize {
        self.len()
    }
}

impl CountAllEdges for BTreeSet<(usize, usize)> {
    fn count_all_edges(&self) -> usize {
        self.len()
    }
}

impl<W> CountAllEdges for BTreeSet<(usize, usize, W)> {
    fn count_all_edges(&self) -> usize {
        self.len()
    }
}

impl<H> CountAllEdges for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.len()
    }
}

impl<W, H> CountAllEdges for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn count_all_edges(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn slice_vec() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, usize>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, usize>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn arr_vec() {
        let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(graph.count_all_edges(), 10);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn btree_map_vec() {
        let graph = BTreeMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]);

        assert_eq!(graph.count_all_edges(), 6);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0, 2])),
            (2, BTreeSet::from([0, 1])),
        ]);

        assert_eq!(graph.count_all_edges(), 6);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3)])),
            (1, BTreeMap::from([(0, 4)])),
            (2, BTreeMap::from([(0, 7), (1, 8)])),
        ]);

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]);

        assert_eq!(graph.count_all_edges(), 6);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0, 2])),
            (2, HashSet::from([0, 1])),
        ]);

        assert_eq!(graph.count_all_edges(), 6);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3)])),
            (1, HashMap::from([(0, 4)])),
            (2, HashMap::from([(0, 7), (1, 8)])),
        ]);

        assert_eq!(graph.count_all_edges(), 5);
    }

    #[test]
    fn vec_tuple_unweighted() {
        let graph = vec![(0, 1), (1, 2), (2, 0)];

        assert_eq!(graph.count_all_edges(), 3);
    }

    #[test]
    fn vec_tuple_weighted() {
        let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        assert_eq!(graph.count_all_edges(), 3);
    }

    #[test]
    fn btree_set_tuple_unweighted() {
        let graph = BTreeSet::from([(0, 1), (1, 2), (2, 0)]);

        assert_eq!(graph.count_all_edges(), 3);
    }

    #[test]
    fn btree_set_tuple_weighted() {
        let graph = BTreeSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        assert_eq!(graph.count_all_edges(), 3);
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        let graph = HashSet::from([(0, 1), (1, 2), (2, 0)]);

        assert_eq!(graph.count_all_edges(), 3);
    }

    #[test]
    fn hash_set_tuple_weighted() {
        let graph = HashSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        assert_eq!(graph.count_all_edges(), 3);
    }
}
