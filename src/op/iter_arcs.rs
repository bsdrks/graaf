//! A trait to iterate over all arcs with a given source vertex in an
//! unweighted directed graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterArcs;
//!
//! let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert!(graph.iter_arcs(0).eq([1, 2]));
//! assert!(graph.iter_arcs(1).eq([0, 2, 3]));
//! assert!(graph.iter_arcs(2).eq([0, 1, 3]));
//! assert!(graph.iter_arcs(3).eq([1, 2]));
//! ```
//!
//! The order of the arcs is not guaranteed for, e.g., `Vec<HashSet<_>>`:
//!
//! ```
//! use {
//!     graaf::op::IterArcs,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0, 2, 3]),
//!     HashSet::from([0, 1, 3]),
//!     HashSet::from([1, 2]),
//! ];
//!
//! let mut iter = graph.iter_arcs(0);
//!
//! assert!(matches!(iter.next(), Some(1 | 2)));
//! assert!(matches!(iter.next(), Some(1 | 2)));
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

/// A trait to iterate over all arcs with a given source vertex in an
/// unweighted directed graph
///
/// # How can I implement `IterArcs`?
///
/// Provide an implementation of `iter_arcs` that returns an iterator over all
/// arcs with a given source vertex.
///
/// ```
/// use graaf::op::IterArcs;
///
/// struct Graph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl IterArcs for Graph {
///     fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
///         self.arcs[s].iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterArcs;
///
/// let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert!(graph.iter_arcs(0).eq([1, 2]));
/// assert!(graph.iter_arcs(1).eq([0, 2, 3]));
/// assert!(graph.iter_arcs(2).eq([0, 1, 3]));
/// assert!(graph.iter_arcs(3).eq([1, 2]));
/// ```
///
/// The order of the arcs is not guaranteed for, e.g., `Vec<HashSet<_>>`:
///
/// ```
/// use {
///     graaf::op::IterArcs,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0, 2, 3]),
///     HashSet::from([0, 1, 3]),
///     HashSet::from([1, 2]),
/// ];
///
/// let mut iter = graph.iter_arcs(0);
///
/// assert!(matches!(iter.next(), Some(1 | 2)));
/// assert!(matches!(iter.next(), Some(1 | 2)));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterArcs {
    /// Returns an iterator over all arcs with a given source vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize>;
}

impl IterArcs for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterArcs for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterArcs for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterArcs for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterArcs for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterArcs for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize> IterArcs for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize> IterArcs for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize, H> IterArcs for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterArcs for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl<H> IterArcs for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl IterArcs for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl<H> IterArcs for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_iter_arcs_stable {
        ($graph:expr) => {
            assert!($graph.iter_arcs(0).eq([1, 2]));
            assert!($graph.iter_arcs(1).eq([0, 2, 3]));
            assert!($graph.iter_arcs(2).eq([0, 1, 3]));
            assert!($graph.iter_arcs(3).eq([1, 2]));
        };
    }

    macro_rules! test_iter_arcs_unstable {
        ($graph:expr) => {
            let mut iter = $graph.iter_arcs(0);

            assert!(matches!(iter.next(), Some(1 | 2)));
            assert!(matches!(iter.next(), Some(1 | 2)));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_arcs(1);

            assert!(matches!(iter.next(), Some(0 | 2 | 3)));
            assert!(matches!(iter.next(), Some(0 | 2 | 3)));
            assert!(matches!(iter.next(), Some(0 | 2 | 3)));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_arcs(2);

            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert_eq!(iter.next(), None);

            let mut iter = $graph.iter_arcs(3);

            assert!(matches!(iter.next(), Some(1 | 2)));
            assert!(matches!(iter.next(), Some(1 | 2)));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_iter_arcs_unstable!(graph);
    }

    #[test]
    fn slice_vec() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_iter_arcs_unstable!(graph);
    }

    #[test]
    fn arr_vec() {
        let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_iter_arcs_unstable!(graph);
    }

    #[test]
    fn btree_map_vec() {
        let graph = BTreeMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0, 2, 3])),
            (2, BTreeSet::from([0, 1, 3])),
            (3, BTreeSet::from([1, 2])),
        ]);

        test_iter_arcs_stable!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0, 2, 3])),
            (2, HashSet::from([0, 1, 3])),
            (3, HashSet::from([1, 2])),
        ]);

        test_iter_arcs_unstable!(graph);
    }
}
