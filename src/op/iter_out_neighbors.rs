//! Iterate over the out-neighbors of a vertex in a digraph.
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
//! The function does not guarantee the order of the out-neighbors, for, e.g.,
//! `Vec<HashSet<_>>`.
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
#![doc(alias = "iter_out_neighbours")]

use {
    crate::op::IterArcs,
    core::hash::BuildHasher,
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Iterate over the out-neighbors of a vertex in a digraph.
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
///
/// let digraph = Digraph {
///     arcs: vec![vec![1, 2], vec![0], vec![0, 1, 3], vec![0]],
/// };
///
/// assert!(digraph.iter_out_neighbors(0).eq([1, 2]));
/// assert!(digraph.iter_out_neighbors(1).eq([0]));
/// assert!(digraph.iter_out_neighbors(2).eq([0, 1, 3]));
/// assert!(digraph.iter_out_neighbors(3).eq([0]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterOutNeighbors;
///
/// let digraph = [vec![1, 2], vec![0], vec![0, 1, 3], vec![0]];
///
/// assert!(digraph.iter_out_neighbors(0).eq([1, 2]));
/// assert!(digraph.iter_out_neighbors(1).eq([0]));
/// assert!(digraph.iter_out_neighbors(2).eq([0, 1, 3]));
/// assert!(digraph.iter_out_neighbors(3).eq([0]));
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
///     HashSet::from([0]),
///     HashSet::from([0, 1, 3]),
///     HashSet::from([0]),
/// ];
///
/// let mut iter = digraph.iter_out_neighbors(0);
///
/// assert!(matches!(iter.next(), Some(1 | 2)));
/// assert!(matches!(iter.next(), Some(1 | 2)));
/// assert_eq!(iter.next(), None);
/// ```
#[doc(alias = "IterOutNeighbours")]
pub trait IterOutNeighbors {
    /// Returns an iterator over the out-neighbors of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `s`: The tail vertex.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize>;
}

macro_rules! impl_usize {
    () => {
        /// # Panics
        ///
        /// Panics if `s` is not in the digraph.
        fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
            self[s].iter().copied()
        }
    };
}

macro_rules! impl_ref_usize {
    () => {
        /// # Panics
        ///
        /// Panics if `s` is not in the digraph.
        fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
            self[&s].iter().copied()
        }
    };
}

macro_rules! impl_iter_arc {
    () => {
        fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
            self.iter_arcs()
                .filter_map(move |(s_, t)| (s == s_).then_some(t))
        }
    };
}

impl IterOutNeighbors for Vec<Vec<usize>> {
    impl_usize!();
}

impl IterOutNeighbors for Vec<BTreeSet<usize>> {
    impl_usize!();
}

impl<H> IterOutNeighbors for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_usize!();
}

impl IterOutNeighbors for [Vec<usize>] {
    impl_usize!();
}

impl IterOutNeighbors for [BTreeSet<usize>] {
    impl_usize!();
}

impl<H> IterOutNeighbors for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_usize!();
}

impl<const V: usize> IterOutNeighbors for [Vec<usize>; V] {
    impl_usize!();
}

impl<const V: usize> IterOutNeighbors for [BTreeSet<usize>; V] {
    impl_usize!();
}

impl<const V: usize, H> IterOutNeighbors for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_usize!();
}

impl IterOutNeighbors for BTreeMap<usize, Vec<usize>> {
    impl_ref_usize!();
}

impl<H> IterOutNeighbors for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    impl_ref_usize!();
}

impl IterOutNeighbors for BTreeMap<usize, BTreeSet<usize>> {
    impl_ref_usize!();
}

impl<H> IterOutNeighbors for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_ref_usize!();
}

impl IterOutNeighbors for Vec<(usize, usize)> {
    impl_iter_arc!();
}

impl IterOutNeighbors for [(usize, usize)] {
    impl_iter_arc!();
}

impl<const V: usize> IterOutNeighbors for [(usize, usize); V] {
    impl_iter_arc!();
}

impl IterOutNeighbors for BTreeSet<(usize, usize)> {
    impl_iter_arc!();
}

impl<H> IterOutNeighbors for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_iter_arc!();
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
            assert!($digraph.iter_out_neighbors(0).eq([1, 2]));
            assert!($digraph.iter_out_neighbors(1).eq([0]));
            assert!($digraph.iter_out_neighbors(2).eq([0, 1, 3]));
            assert!($digraph.iter_out_neighbors(3).eq([0]));
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_out_neighbors(0);

            assert!(matches!(iter.next(), Some(1 | 2)));
            assert!(matches!(iter.next(), Some(1 | 2)));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_neighbors(1);

            assert_eq!(iter.next(), Some(0));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_neighbors(2);

            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert!(matches!(iter.next(), Some(0 | 1 | 3)));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_neighbors(3);

            assert_eq!(iter.next(), Some(0));
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
        test_stable!(digraph);
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
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(4);

        setup!(digraph);

        let slice = digraph.as_slice();
        test_unstable!(slice);
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
        test_stable!(digraph);
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
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(4);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        setup!(digraph);
        test_stable!(digraph);
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
    fn arr_tuple() {
        let digraph = [(0, 1), (0, 2), (1, 0), (2, 0), (2, 1), (2, 3), (3, 0)];

        test_stable!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }
}
