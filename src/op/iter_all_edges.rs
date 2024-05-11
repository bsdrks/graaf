//! A trait to iterate over all edges in an unweighted directed graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterAllEdges;
//!
//! let graph = vec![(0, 1), (1, 2), (2, 0)];
//!
//! assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
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

/// A trait to iterate over all edges in an unweighted directed graph
///
/// # How can I implement `IterAllEdges`?
///
/// Provide an implementation of `iter_all_edges` that returns an iterator over
/// all edges in a graph.
///
/// ```
/// use graaf::op::IterAllEdges;
///
/// struct Graph {
///     edges: Vec<(usize, usize)>,
/// }
///
/// impl IterAllEdges for Graph {
///     fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.edges.iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterAllEdges;
///
/// let graph = vec![(0, 1), (1, 2), (2, 0)];
///
/// assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait IterAllEdges {
    /// Returns an iterator over all edges in a graph.
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)>;
}

impl IterAllEdges for Vec<Vec<usize>> {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (s, *t)))
    }
}

impl IterAllEdges for Vec<BTreeSet<usize>> {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<H> IterAllEdges for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl IterAllEdges for [Vec<usize>] {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (s, *t)))
    }
}

impl IterAllEdges for [BTreeSet<usize>] {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<H> IterAllEdges for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<const V: usize> IterAllEdges for [Vec<usize>; V] {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (s, *t)))
    }
}

impl<const V: usize> IterAllEdges for [BTreeSet<usize>; V] {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<const V: usize, H> IterAllEdges for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl IterAllEdges for BTreeMap<usize, Vec<usize>> {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (*s, *t)))
    }
}

impl IterAllEdges for BTreeMap<usize, BTreeSet<usize>> {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |t| (*s, *t)))
    }
}

impl<H> IterAllEdges for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (*s, *t)))
    }
}

impl<H> IterAllEdges for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |t| (*s, *t)))
    }
}

impl IterAllEdges for Vec<(usize, usize)> {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl IterAllEdges for [(usize, usize)] {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl<const V: usize> IterAllEdges for [(usize, usize); V] {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl IterAllEdges for BTreeSet<(usize, usize)> {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl<H> IterAllEdges for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_iter_all_edges_unstable {
        ($graph:expr) => {
            let mut iter = $graph.iter_all_edges();

            assert!(matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0))));
            assert!(matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0))));
            assert!(matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0))));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let graph = vec![vec![1], vec![2], vec![0]];

        assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_iter_all_edges_unstable!(graph);
    }

    #[test]

    fn vec_hash_set() {
        let graph = vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([0])];

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn slice_vec() {
        let graph: &[Vec<usize>] = &[vec![1], vec![2], vec![0]];

        assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] =
            &[HashSet::from([1]), HashSet::from([2]), HashSet::from([0])];

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn arr_vec() {
        let graph = [vec![1], vec![2], vec![0]];

        assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [HashSet::from([1]), HashSet::from([2]), HashSet::from([0])];

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn btree_map_vec() {
        let graph = BTreeMap::from([(0, vec![1]), (1, vec![2]), (2, vec![0])]);

        assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1])),
            (1, BTreeSet::from([2])),
            (2, BTreeSet::from([0])),
        ]);

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([(0, vec![1]), (1, vec![2]), (2, vec![0])]);

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1])),
            (1, HashSet::from([2])),
            (2, HashSet::from([0])),
        ]);

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn vec_tuple() {
        let graph = vec![(0, 1), (1, 2), (2, 0)];

        assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn slice_tuple() {
        let graph: &[(usize, usize)] = &[(0, 1), (1, 2), (2, 0)];

        assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn arr_tuple() {
        let graph = [(0, 1), (1, 2), (2, 0)];

        assert!(graph.iter_all_edges().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn btree_set() {
        let graph: BTreeSet<(usize, usize)> = BTreeSet::from([(0, 1), (1, 2), (2, 0)]);

        test_iter_all_edges_unstable!(graph);
    }

    #[test]
    fn hash_set() {
        let graph: HashSet<(usize, usize)> = HashSet::from([(0, 1), (1, 2), (2, 0)]);

        test_iter_all_edges_unstable!(graph);
    }
}
