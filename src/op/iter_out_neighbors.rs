//! A trait to iterate over the out-neighbors of a vertex in a digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterOutNeighbors;
//!
//! let digraph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert!(digraph.iter_out_neighbors(0).eq([1, 2]));
//! assert!(digraph.iter_out_neighbors(1).eq([0, 2, 3]));
//! assert!(digraph.iter_out_neighbors(2).eq([0, 1, 3]));
//! assert!(digraph.iter_out_neighbors(3).eq([1, 2]));
//! ```
//!
//! The order of the out-neighbors is not guaranteed for, e.g.,
//! `Vec<HashSet<_>>`:
//!
//! ```
//! use {
//!     graaf::op::IterOutNeighbors,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0, 2, 3]),
//!     HashSet::from([0, 1, 3]),
//!     HashSet::from([1, 2]),
//! ];
//!
//! let mut iter = digraph.iter_out_neighbors(0);
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

/// A trait to iterate over the out-neighbors of a vertex in a digraph
///
/// # How can I implement `IterOutNeighbors`?
///
/// Provide an implementation of `iter_out_neighbors` that returns an iterator
/// over the out-neighbors of a vertex.
///
/// ```
/// use graaf::op::IterOutNeighbors;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl IterOutNeighbors for Digraph {
///     fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
///         self.arcs[s].iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterOutNeighbors;
///
/// let digraph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert!(digraph.iter_out_neighbors(0).eq([1, 2]));
/// assert!(digraph.iter_out_neighbors(1).eq([0, 2, 3]));
/// assert!(digraph.iter_out_neighbors(2).eq([0, 1, 3]));
/// assert!(digraph.iter_out_neighbors(3).eq([1, 2]));
/// ```
///
/// The order of the out-neighbors is not guaranteed for, e.g.,
///
/// ```
/// use {
///     graaf::op::IterOutNeighbors,
///     std::collections::HashSet,
/// };
///
/// let digraph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0, 2, 3]),
///     HashSet::from([0, 1, 3]),
///     HashSet::from([1, 2]),
/// ];
///
/// let mut iter = digraph.iter_out_neighbors(0);
///
/// assert!(matches!(iter.next(), Some(1 | 2)));
/// assert!(matches!(iter.next(), Some(1 | 2)));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterOutNeighbors {
    /// Returns an iterator over the out-neighbors of a vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The tail vertex.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize>;
}

impl IterOutNeighbors for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterOutNeighbors for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterOutNeighbors for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterOutNeighbors for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterOutNeighbors for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterOutNeighbors for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize> IterOutNeighbors for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize> IterOutNeighbors for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<const V: usize, H> IterOutNeighbors for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl IterOutNeighbors for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl<H> IterOutNeighbors for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl IterOutNeighbors for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

impl<H> IterOutNeighbors for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
        self[&s].iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_iter_out_neighbors_stable {
        ($digraph:expr) => {
            assert!($digraph.iter_out_neighbors(0).eq([1, 2]));
            assert!($digraph.iter_out_neighbors(1).eq([0, 2, 3]));
            assert!($digraph.iter_out_neighbors(2).eq([0, 1, 3]));
            assert!($digraph.iter_out_neighbors(3).eq([1, 2]));
        };
    }

    macro_rules! test_iter_out_neighbors_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_out_neighbors(0);

            assert!(matches!(iter.next(), Some(1 | 2)));
            assert!(matches!(iter.next(), Some(1 | 2)));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_neighbors(1);

            assert!(matches!(iter.next(), Some(0 | 2 | 3)));
            assert!(matches!(iter.next(), Some(0 | 2 | 3)));
            assert!(matches!(iter.next(), Some(0 | 2 | 3)));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_neighbors(2);

            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_neighbors(3);

            assert!(matches!(iter.next(), Some(1 | 2)));
            assert!(matches!(iter.next(), Some(1 | 2)));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let digraph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let digraph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_iter_out_neighbors_unstable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let digraph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_iter_out_neighbors_unstable!(digraph);
    }

    #[test]
    fn arr_vec() {
        let digraph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let digraph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2, 3]),
            BTreeSet::from([0, 1, 3]),
            BTreeSet::from([1, 2]),
        ];

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2, 3]),
            HashSet::from([0, 1, 3]),
            HashSet::from([1, 2]),
        ];

        test_iter_out_neighbors_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let digraph = BTreeMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let digraph = HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0, 2, 3])),
            (2, BTreeSet::from([0, 1, 3])),
            (3, BTreeSet::from([1, 2])),
        ]);

        test_iter_out_neighbors_stable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0, 2, 3])),
            (2, HashSet::from([0, 1, 3])),
            (3, HashSet::from([1, 2])),
        ]);

        test_iter_out_neighbors_unstable!(digraph);
    }
}
