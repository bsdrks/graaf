//! A trait to count the number of arcs in a digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::Size;
//!
//! let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(digraph.size(), 10);
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

/// A trait to count the number of arcs in a digraph
///
/// # How can I implement `Size`?
///
/// Provide an implementation of `size` that returns the number of
/// arcs in the digraph.
///
/// ```
/// use graaf::op::Size;
///
/// struct Graph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl Size for Graph {
///     fn size(&self) -> usize {
///         self.arcs.iter().map(Vec::len).sum()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::Size;
///
/// let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(digraph.size(), 10);
/// ```
pub trait Size {
    /// Counts all arcs.
    fn size(&self) -> usize;
}

impl<T> Size for Vec<Vec<T>> {
    fn size(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T> Size for Vec<BTreeSet<T>> {
    fn size(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<T, H> Size for Vec<HashSet<T, H>>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<K, W> Size for Vec<BTreeMap<K, W>> {
    fn size(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<K, W, H> Size for Vec<HashMap<K, W, H>>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<T> Size for [Vec<T>] {
    fn size(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T> Size for [BTreeSet<T>] {
    fn size(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<T, H> Size for [HashSet<T, H>]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<K, W> Size for [BTreeMap<K, W>] {
    fn size(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<K, W, H> Size for [HashMap<K, W, H>]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<const V: usize, T> Size for [Vec<T>; V] {
    fn size(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<const V: usize, T> Size for [BTreeSet<T>; V] {
    fn size(&self) -> usize {
        self.iter().map(BTreeSet::len).sum()
    }
}

impl<const V: usize, K, W> Size for [BTreeMap<K, W>; V] {
    fn size(&self) -> usize {
        self.iter().map(BTreeMap::len).sum()
    }
}

impl<const V: usize, T, H> Size for [HashSet<T, H>; V]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<const V: usize, K, W, H> Size for [HashMap<K, W, H>; V]
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

impl<K, T> Size for BTreeMap<K, Vec<T>> {
    fn size(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, T> Size for BTreeMap<K, BTreeSet<T>> {
    fn size(&self) -> usize {
        self.values().map(BTreeSet::len).sum()
    }
}

impl<K, W> Size for BTreeMap<K, BTreeMap<K, W>> {
    fn size(&self) -> usize {
        self.values().map(BTreeMap::len).sum()
    }
}

impl<K, T, H> Size for HashMap<K, Vec<T>, H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, T, H> Size for HashMap<K, HashSet<T, H>, H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.values().map(HashSet::len).sum()
    }
}

impl<K, W, H> Size for HashMap<K, HashMap<K, W, H>, H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.values().map(HashMap::len).sum()
    }
}

impl Size for Vec<(usize, usize)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<W> Size for Vec<(usize, usize, W)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl Size for BTreeSet<(usize, usize)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<W> Size for BTreeSet<(usize, usize, W)> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<H> Size for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.len()
    }
}

impl<W, H> Size for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn size(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn vec_btree_set() {
        let digraph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn slice_vec() {
        let digraph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &[BTreeMap<usize, usize>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &[HashMap<usize, usize>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn arr_vec() {
        let digraph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn arr_btree_set() {
        let digraph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert_eq!(digraph.size(), 10);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn btree_map_vec() {
        let digraph = BTreeMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]);

        assert_eq!(digraph.size(), 6);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0, 2])),
            (2, BTreeSet::from([0, 1])),
        ]);

        assert_eq!(digraph.size(), 6);
    }

    #[test]
    fn btree_map_btree_map() {
        let digraph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3)])),
            (1, BTreeMap::from([(0, 4)])),
            (2, BTreeMap::from([(0, 7), (1, 8)])),
        ]);

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn hash_map_vec() {
        let digraph = HashMap::from([(0, vec![1, 2]), (1, vec![0, 2]), (2, vec![0, 1])]);

        assert_eq!(digraph.size(), 6);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0, 2])),
            (2, HashSet::from([0, 1])),
        ]);

        assert_eq!(digraph.size(), 6);
    }

    #[test]
    fn hash_map_hash_map() {
        let digraph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3)])),
            (1, HashMap::from([(0, 4)])),
            (2, HashMap::from([(0, 7), (1, 8)])),
        ]);

        assert_eq!(digraph.size(), 5);
    }

    #[test]
    fn vec_tuple_unweighted() {
        let digraph = vec![(0, 1), (1, 2), (2, 0)];

        assert_eq!(digraph.size(), 3);
    }

    #[test]
    fn vec_tuple_weighted() {
        let digraph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        assert_eq!(digraph.size(), 3);
    }

    #[test]
    fn btree_set_tuple_unweighted() {
        let digraph = BTreeSet::from([(0, 1), (1, 2), (2, 0)]);

        assert_eq!(digraph.size(), 3);
    }

    #[test]
    fn btree_set_tuple_weighted() {
        let digraph = BTreeSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        assert_eq!(digraph.size(), 3);
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        let digraph = HashSet::from([(0, 1), (1, 2), (2, 0)]);

        assert_eq!(digraph.size(), 3);
    }

    #[test]
    fn hash_set_tuple_weighted() {
        let digraph = HashSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        assert_eq!(digraph.size(), 3);
    }
}
