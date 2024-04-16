//! A trait to iterate over all unweighted edges with a given source vertex
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterEdges;
//!
//! let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//! let mut iter = graph.iter_edges(0);
//!
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), None);
//!
//! let mut iter = graph.iter_edges(1);
//!
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(3));
//! assert_eq!(iter.next(), None);
//!
//! let mut iter = graph.iter_edges(2);
//!
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(3));
//! assert_eq!(iter.next(), None);
//!
//! let mut iter = graph.iter_edges(3);
//!
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), None);
//! ```
//!
//! The order of the edges is not guaranteed for, e.g., `Vec<HashSet<_>>`:
//!
//! ```
//! #![feature(assert_matches)]
//!
//! use {
//!     graaf::op::IterEdges,
//!     std::{
//!         assert_matches::assert_matches,
//!         collections::HashSet,
//!     },
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0, 2, 3]),
//!     HashSet::from([0, 1, 3]),
//!     HashSet::from([1, 2]),
//! ];
//!
//! let mut iter = graph.iter_edges(0);
//!
//! assert_matches!(iter.next(), Some(1 | 2));
//! assert_matches!(iter.next(), Some(1 | 2));
//! assert_eq!(iter.next(), None);
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

/// A trait to iterate over all unweighted edges with a given source vertex
///
/// # How can I implement `IterEdges`?
///
/// Provide an implementation of `iter_edges` that returns an iterator over all
/// edges of the source vertex.
///
/// ```
/// use graaf::op::IterEdges;
///
/// struct Graph {
///     edges: Vec<Vec<usize>>,
/// }
///
/// impl IterEdges for Graph {
///     fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
///         self.edges[s].iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterEdges;
///
/// let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
/// let mut iter = graph.iter_edges(0);
///
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_edges(1);
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_edges(2);
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_edges(3);
///
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), None);
/// ```
///
/// The order of the edges is not guaranteed for, e.g., `Vec<HashSet<_>>`:
///
/// ```
/// #![feature(assert_matches)]
///
/// use {
///     graaf::op::IterEdges,
///     std::{
///         assert_matches::assert_matches,
///         collections::HashSet,
///     },
/// };
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0, 2, 3]),
///     HashSet::from([0, 1, 3]),
///     HashSet::from([1, 2]),
/// ];
///
/// let mut iter = graph.iter_edges(0);
///
/// assert_matches!(iter.next(), Some(1 | 2));
/// assert_matches!(iter.next(), Some(1 | 2));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterEdges {
    /// Return an iterator over the edges of the vertex `s`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize>;
}

impl IterEdges for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterEdges for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterEdges for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterEdges for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterEdges for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterEdges for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize> IterEdges for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize> IterEdges for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize, H> IterEdges for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterEdges for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl<H> IterEdges for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl IterEdges for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl<H> IterEdges for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::assert_matches::assert_matches,
    };

    macro_rules! test_stable {
        ($graph:expr) => {
            let mut iter = $graph.iter_edges(0);

            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_edges(1);

            assert_eq!(iter.next(), Some(0));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_edges(2);

            assert_eq!(iter.next(), Some(0));
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_edges(3);

            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), None);
        };
    }

    macro_rules! test_unstable {
        ($graph:expr) => {
            let mut iter = $graph.iter_edges(0);

            assert_matches!(iter.next(), Some(1 | 2));
            assert_matches!(iter.next(), Some(1 | 2));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_edges(1);

            assert_matches!(iter.next(), Some(0 | 2 | 3));
            assert_matches!(iter.next(), Some(0 | 2 | 3));
            assert_matches!(iter.next(), Some(0 | 2 | 3));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_edges(2);

            assert_matches!(iter.next(), Some(0 | 1 | 3));
            assert_matches!(iter.next(), Some(0 | 1 | 3));
            assert_matches!(iter.next(), Some(0 | 1 | 3));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_edges(3);

            assert_matches!(iter.next(), Some(1 | 2));
            assert_matches!(iter.next(), Some(1 | 2));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_stable!(graph);
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_stable!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_unstable!(graph);
    }

    #[test]
    fn slice_vec() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_stable!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_stable!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_unstable!(graph);
    }

    #[test]
    fn arr_vec() {
        let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_stable!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_stable!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_unstable!(graph);
    }

    #[test]
    fn btree_map_vec() {
        let graph = BTreeMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_stable!(graph);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_stable!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0, 2, 3])),
            (2, BTreeSet::from([0, 1, 3])),
            (3, BTreeSet::from([1, 2])),
        ]);

        test_stable!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0, 2, 3])),
            (2, HashSet::from([0, 1, 3])),
            (3, HashSet::from([1, 2])),
        ]);

        test_unstable!(graph);
    }
}
