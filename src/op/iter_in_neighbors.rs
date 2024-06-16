//! Iterate over the in-neighbors of a vertex in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterInNeighbors;
//!
//! let digraph = [vec![1, 2], vec![0], vec![0, 1, 3], vec![0]];
//!
//! assert!(digraph.iter_in_neighbors(0).eq([1, 2, 3]));
//! assert!(digraph.iter_in_neighbors(1).eq([0, 2]));
//! assert!(digraph.iter_in_neighbors(2).eq([0]));
//! assert!(digraph.iter_in_neighbors(3).eq([2]));
//! ```
//!
//! The function does not guarantee the order of the in-neighbors, for, e.g.,
//! `Vec<HashSet<_>>`.
//!
//! ```
//! use {
//!     graaf::op::IterInNeighbors,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([0, 1, 3]),
//!     HashSet::from([0]),
//! ];
//!
//! let mut iter = digraph.iter_in_neighbors(0);
//!
//! assert!(matches!(iter.next(), Some(1 | 2 | 3)));
//! assert!(matches!(iter.next(), Some(1 | 2 | 3)));
//! assert!(matches!(iter.next(), Some(1 | 2 | 3)));
//! assert_eq!(iter.next(), None);
//! ```
#![doc(alias = "iter_in_neighbours")]

use super::IterArcs;

/// Iterate over the in-neighbors of a vertex in a digraph.
///
/// # How can I implement `IterInNeighbors`?
///
/// Provide an implementation of `iter_in_neighbors` that returns an iterator
/// over the in-neighbors of a vertex.
///
/// ```
/// use graaf::op::{
///     IterArcs,
///     IterInNeighbors,
/// };
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl IterInNeighbors for Digraph {
///     fn iter_in_neighbors(&self, t: usize) -> impl Iterator<Item = usize> {
///         self.arcs
///             .iter_arcs()
///             .filter_map(move |(s, t_)| (t == t_).then_some(s))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![vec![1, 2], vec![0], vec![0, 1, 3], vec![0]],
/// };
///
/// assert!(digraph.iter_in_neighbors(0).eq([1, 2, 3]));
/// assert!(digraph.iter_in_neighbors(1).eq([0, 2]));
/// assert!(digraph.iter_in_neighbors(2).eq([0]));
/// assert!(digraph.iter_in_neighbors(3).eq([2]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterInNeighbors;
///
/// let digraph = [vec![1, 2], vec![0], vec![0, 1, 3], vec![0]];
///
/// assert!(digraph.iter_in_neighbors(0).eq([1, 2, 3]));
/// assert!(digraph.iter_in_neighbors(1).eq([0, 2]));
/// assert!(digraph.iter_in_neighbors(2).eq([0]));
/// assert!(digraph.iter_in_neighbors(3).eq([2]));
/// ```
///
/// The order of the in-neighbors is not guaranteed for, e.g.,
///
/// ```
/// use {
///     graaf::op::IterInNeighbors,
///     std::collections::HashSet,
/// };
///
/// let digraph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([0, 1, 3]),
///     HashSet::from([0]),
/// ];
///
/// let mut iter = digraph.iter_in_neighbors(0);
///
/// assert!(matches!(iter.next(), Some(1 | 2 | 3)));
/// assert!(matches!(iter.next(), Some(1 | 2 | 3)));
/// assert!(matches!(iter.next(), Some(1 | 2 | 3)));
/// assert_eq!(iter.next(), None);
/// ```
#[doc(alias = "IterInNeighbours")]
pub trait IterInNeighbors {
    /// Returns an iterator over the in-neighbors of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `t`: The head vertex.
    fn iter_in_neighbors(&self, t: usize) -> impl Iterator<Item = usize>;
}

impl<D> IterInNeighbors for D
where
    D: IterArcs + ?Sized,
{
    fn iter_in_neighbors(&self, t: usize) -> impl Iterator<Item = usize> {
        self.iter_arcs()
            .filter_map(move |(s, t_)| (t == t_).then_some(s))
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
            op::AddArc,
        },
        std::collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
            HashSet,
        },
    };

    macro_rules! setup {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 0);
            $digraph.add_arc(2, 0);
            $digraph.add_arc(2, 1);
            $digraph.add_arc(2, 3);
            $digraph.add_arc(3, 0);
        };
    }

    macro_rules! test_stable {
        ($digraph:expr) => {
            assert!($digraph.iter_in_neighbors(0).eq([1, 2, 3]));
            assert!($digraph.iter_in_neighbors(1).eq([0, 2]));
            assert!($digraph.iter_in_neighbors(2).eq([0]));
            assert!($digraph.iter_in_neighbors(3).eq([2]));
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_in_neighbors(0);

            assert!(matches!(iter.next(), Some(1..=3)));
            assert!(matches!(iter.next(), Some(1..=3)));
            assert!(matches!(iter.next(), Some(1..=3)));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_in_neighbors(1);

            assert!(matches!(iter.next(), Some(0 | 2)));
            assert!(matches!(iter.next(), Some(0 | 2)));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_in_neighbors(2);

            assert_eq!(iter.next(), Some(0));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_in_neighbors(3);

            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(4);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(4);

        setup!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<usize>; 4]>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 4]>::empty();

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 4]>::empty();

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(4);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(4);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(4);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_tuple() {
        let mut digraph = Vec::<(usize, usize)>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn slice_tuple() {
        let mut digraph = Vec::<(usize, usize)>::empty();

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize)>::empty();

        setup!(digraph);
        test_unstable!(digraph);
    }
}
