//! A trait to iterate over all vertices in a graph
//!
//! # Example
//!
//! ```
//! use graaf::op::IterVertices;
//!
//! let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert!(graph.iter_vertices().eq(0..4));
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

/// A trait to iterate over all vertices in a graph
///
/// # How can I implement `IterVertices`?
///
/// Provide an implementation of `iter_vertices` that returns an iterator over
/// all vertices in the graph.
///
/// ```
/// use graaf::op::IterVertices;
///
/// struct Graph {
///     vertices: Vec<usize>,
/// }
///
/// impl IterVertices for Graph {
///     fn iter_vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::op::IterVertices;
///
/// let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert!(graph.iter_vertices().eq(0..4));
/// ```
pub trait IterVertices {
    /// Returns an iterator over the vertices.
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

impl IterVertices for Vec<Vec<usize>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl IterVertices for Vec<BTreeSet<usize>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<H> IterVertices for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl IterVertices for [Vec<usize>] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl IterVertices for [BTreeSet<usize>] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<H> IterVertices for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize> IterVertices for [Vec<usize>; V] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize> IterVertices for [BTreeSet<usize>; V] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl IterVertices for BTreeMap<usize, BTreeSet<usize>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|set| set.iter().copied()))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

impl IterVertices for BTreeMap<usize, Vec<usize>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|set| set.iter().copied()))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

impl<H> IterVertices for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|set| set.iter().copied()))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

impl<H> IterVertices for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|set| set.iter().copied()))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

impl<W> IterVertices for Vec<Vec<(usize, W)>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W> IterVertices for Vec<BTreeSet<(usize, W)>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W> IterVertices for Vec<BTreeMap<usize, W>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W, H> IterVertices for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W, H> IterVertices for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W> IterVertices for [Vec<(usize, W)>] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W> IterVertices for [BTreeSet<(usize, W)>] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W> IterVertices for [BTreeMap<usize, W>] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W, H> IterVertices for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W, H> IterVertices for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize, W> IterVertices for [Vec<(usize, W)>; V] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize, W> IterVertices for [BTreeSet<(usize, W)>; V] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize, W> IterVertices for [BTreeMap<usize, W>; V] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize, H> IterVertices for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize, W, H> IterVertices for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize, W, H> IterVertices for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<W> IterVertices for BTreeMap<usize, Vec<(usize, W)>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|vec| vec.iter().map(|(t, _)| *t)))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

impl<W> IterVertices for BTreeMap<usize, BTreeSet<(usize, W)>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|set| set.iter().map(|(t, _)| *t)))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

impl<W> IterVertices for BTreeMap<usize, BTreeMap<usize, W>> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|map| map.keys().copied()))
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

impl<W, H> IterVertices for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|vec| vec.iter().map(|(t, _)| *t)))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

impl<W, H> IterVertices for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|set| set.iter().map(|(t, _)| *t)))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

impl<W, H> IterVertices for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys()
            .copied()
            .chain(self.values().flat_map(|map| map.keys().copied()))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec_unweighted() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_vec_unweighted() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_vec_unweighted() {
        let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn btree_map_vec_unweighted() {
        let graph = BTreeMap::from([(0, vec![2, 3]), (2, vec![0, 3]), (3, Vec::new())]);

        assert!(graph.iter_vertices().eq([0, 2, 3]));
    }

    #[test]
    fn btree_map_btree_set_unweighted() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([2, 3])),
            (2, BTreeSet::from([0, 3])),
            (3, BTreeSet::new()),
        ]);

        assert!(graph.iter_vertices().eq([0, 2, 3]));
    }

    #[test]
    fn hash_map_vec_unweighted() {
        let graph = HashMap::from([(0, vec![2, 3]), (2, vec![0, 3]), (3, Vec::new())]);
        let mut iter = graph.iter_vertices();

        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_hash_set_unweighted() {
        let graph = HashMap::from([
            (0, HashSet::from([2, 3])),
            (2, HashSet::from([0, 3])),
            (3, HashSet::new()),
        ]);

        let mut iter = graph.iter_vertices();

        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vec_vec_weighted() {
        let graph = vec![
            vec![(1, 2), (2, 3)],
            vec![(0, 4)],
            vec![(0, 7), (1, 8)],
            vec![(1, 2)],
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn vec_btree_set_weighted() {
        let graph = vec![
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
            BTreeSet::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn vec_hash_set_weighted() {
        let graph = vec![
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
            HashSet::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
            BTreeMap::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
            HashMap::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_vec_weighted() {
        let graph: &[Vec<(usize, i32)>] = &[
            vec![(1, 2), (2, 3)],
            vec![(0, 4)],
            vec![(0, 7), (1, 8)],
            vec![(1, 2)],
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_btree_set_weighted() {
        let graph: &[BTreeSet<(usize, i32)>] = &[
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
            BTreeSet::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_hash_set_weighted() {
        let graph: &[HashSet<(usize, i32)>] = &[
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
            HashSet::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
            BTreeMap::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
            HashMap::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_vec_weighted() {
        let graph = [
            vec![(1, 2), (2, 3)],
            vec![(0, 4)],
            vec![(0, 7), (1, 8)],
            vec![(1, 2)],
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_btree_set_weighted() {
        let graph = [
            BTreeSet::from([(1, 2), (2, 3)]),
            BTreeSet::from([(0, 4)]),
            BTreeSet::from([(0, 7), (1, 8)]),
            BTreeSet::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_hash_set_weighted() {
        let graph = [
            HashSet::from([(1, 2), (2, 3)]),
            HashSet::from([(0, 4)]),
            HashSet::from([(0, 7), (1, 8)]),
            HashSet::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
            BTreeMap::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
            HashMap::from([(1, 2)]),
        ];

        assert!(graph.iter_vertices().eq(0..4));
    }

    #[test]
    fn btree_map_vec_weighted() {
        let graph = BTreeMap::from([
            (0, vec![(2, 2), (3, 3)]),
            (2, vec![(0, 4), (3, 2)]),
            (3, vec![(0, 7)]),
        ]);

        assert!(graph.iter_vertices().eq([0, 2, 3]));
    }

    #[test]
    fn btree_map_btree_set_weighted() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([(2, 2), (3, 3)])),
            (2, BTreeSet::from([(0, 4), (3, 2)])),
            (3, BTreeSet::from([(0, 7)])),
        ]);

        assert!(graph.iter_vertices().eq([0, 2, 3]));
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(2, 2), (3, 3)])),
            (2, BTreeMap::from([(0, 4), (3, 2)])),
            (3, BTreeMap::from([(0, 7)])),
        ]);

        assert!(graph.iter_vertices().eq([0, 2, 3]));
    }

    #[test]
    fn hash_map_vec_weighted() {
        let graph = HashMap::from([
            (0, vec![(2, 2), (3, 3)]),
            (2, vec![(0, 4), (3, 2)]),
            (3, vec![(0, 7)]),
        ]);

        let mut iter = graph.iter_vertices();

        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_hash_set_weighted() {
        let graph = HashMap::from([
            (0, HashSet::from([(2, 2), (3, 3)])),
            (2, HashSet::from([(0, 4), (3, 2)])),
            (3, HashSet::from([(0, 7)])),
        ]);

        let mut iter = graph.iter_vertices();

        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(2, 2), (3, 3)])),
            (2, HashMap::from([(0, 4), (3, 2)])),
            (3, HashMap::from([(0, 7)])),
        ]);

        let mut iter = graph.iter_vertices();

        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert!(matches!(iter.next(), Some(0 | 2 | 3)));
        assert_eq!(iter.next(), None);
    }
}
