//! Iterate over the weighted arcs going out from a vertex in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterOutWeightedNeighbors;
//!
//! let digraph = vec![
//!     vec![(1, 2), (2, 3), (3, 4)],
//!     vec![(2, 3), (3, 4), (4, 5)],
//!     vec![(3, 4), (4, 5), (5, 6)],
//! ];
//!
//! assert!(digraph
//!     .iter_out_weighted_neighbors(0)
//!     .eq([(1, &2), (2, &3), (3, &4)]));
//!
//! assert!(digraph
//!     .iter_out_weighted_neighbors(1)
//!     .eq([(2, &3), (3, &4), (4, &5)]));
//!
//! assert!(digraph
//!     .iter_out_weighted_neighbors(2)
//!     .eq([(3, &4), (4, &5), (5, &6)]));
//! ```
#![doc(alias = "iter_out_weighted_neighbours")]

use {
    core::hash::BuildHasher,
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Iterate over the weighted arcs going out from a vertex in a digraph.
///
/// # How can I implement `IterOutWeightedNeighbors`?
///
/// Provide an implementation of `iter_out_weighted_neighbors` that returns an
/// iterator over all weighted arcs with the source vertex `s`.
///
/// ```
/// use graaf::op::IterOutWeightedNeighbors;
///
/// struct Digraph {
///     arcs: Vec<Vec<(usize, usize)>>,
/// }
///
/// impl IterOutWeightedNeighbors<usize> for Digraph {
///     fn iter_out_weighted_neighbors<'a>(
///         &'a self,
///         s: usize,
///     ) -> impl Iterator<Item = (usize, &'a usize)>
///     where
///         usize: 'a,
///     {
///         self.arcs[s].iter().map(|(t, w)| (*t, w))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterOutWeightedNeighbors;
///
/// let digraph = vec![
///     vec![(1, 2), (2, 3), (3, 4)],
///     vec![(2, 3), (3, 4), (4, 5)],
///     vec![(3, 4), (4, 5), (5, 6)],
/// ];
///
/// assert!(digraph
///     .iter_out_weighted_neighbors(0)
///     .eq([(1, &2), (2, &3), (3, &4)]));
///
/// assert!(digraph
///     .iter_out_weighted_neighbors(1)
///     .eq([(2, &3), (3, &4), (4, &5)]));
///
/// assert!(digraph
///     .iter_out_weighted_neighbors(2)
///     .eq([(3, &4), (4, &5), (5, &6)]));
/// ```
#[doc(alias = "IterOutWeightedNeighbours")]
pub trait IterOutWeightedNeighbors<W> {
    /// Returns an iterator over the out-neighbors of the head vertex and their
    /// weights.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    fn iter_out_weighted_neighbors<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a;
}

macro_rules! impl_usize {
    () => {
        /// # Panics
        ///
        /// Panics if `s` is not in the digraph.
        fn iter_out_weighted_neighbors<'a>(
            &'a self,
            s: usize,
        ) -> impl Iterator<Item = (usize, &'a W)>
        where
            W: 'a,
        {
            self[s].iter().map(|(t, w)| (*t, w))
        }
    };
}

macro_rules! impl_ref_usize {
    () => {
        /// # Panics
        ///
        /// Panics if `s` is not in the digraph.
        fn iter_out_weighted_neighbors<'a>(
            &'a self,
            s: usize,
        ) -> impl Iterator<Item = (usize, &'a W)>
        where
            W: 'a,
        {
            self[&s].iter().map(|(t, w)| (*t, w))
        }
    };
}

impl<W> IterOutWeightedNeighbors<W> for Vec<Vec<(usize, W)>>
where
    W: Copy,
{
    impl_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for Vec<BTreeSet<(usize, W)>>
where
    W: Copy,
{
    impl_usize!();
}

impl<W, H> IterOutWeightedNeighbors<W> for Vec<HashSet<(usize, W), H>>
where
    W: Copy,
    H: BuildHasher,
{
    impl_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for Vec<BTreeMap<usize, W>>
where
    W: Copy,
{
    impl_usize!();
}

impl<W, H> IterOutWeightedNeighbors<W> for Vec<HashMap<usize, W, H>>
where
    W: Copy,
    H: BuildHasher,
{
    impl_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for [Vec<(usize, W)>]
where
    W: Copy,
{
    impl_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for [BTreeSet<(usize, W)>]
where
    W: Copy,
{
    impl_usize!();
}

impl<W, H> IterOutWeightedNeighbors<W> for [HashSet<(usize, W), H>]
where
    W: Copy,
    H: BuildHasher,
{
    impl_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for [BTreeMap<usize, W>]
where
    W: Copy,
{
    impl_usize!();
}

impl<W, H> IterOutWeightedNeighbors<W> for [HashMap<usize, W, H>]
where
    W: Copy,
    H: BuildHasher,
{
    impl_usize!();
}

impl<const V: usize, W> IterOutWeightedNeighbors<W> for [Vec<(usize, W)>; V]
where
    W: Copy,
{
    impl_usize!();
}

impl<const V: usize, W> IterOutWeightedNeighbors<W> for [BTreeSet<(usize, W)>; V]
where
    W: Copy,
{
    impl_usize!();
}

impl<const V: usize, W, H> IterOutWeightedNeighbors<W> for [HashSet<(usize, W), H>; V]
where
    W: Copy,
    H: BuildHasher,
{
    impl_usize!();
}

impl<const V: usize, W> IterOutWeightedNeighbors<W> for [BTreeMap<usize, W>; V]
where
    W: Copy,
{
    impl_usize!();
}

impl<const V: usize, W, H> IterOutWeightedNeighbors<W> for [HashMap<usize, W, H>; V]
where
    W: Copy,
    H: BuildHasher,
{
    impl_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for BTreeMap<usize, Vec<(usize, W)>>
where
    W: Copy,
{
    impl_ref_usize!();
}

impl<W, H> IterOutWeightedNeighbors<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
    W: Copy,
{
    impl_ref_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for BTreeMap<usize, BTreeSet<(usize, W)>>
where
    W: Copy,
{
    impl_ref_usize!();
}

impl<W, H> IterOutWeightedNeighbors<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    impl_ref_usize!();
}

impl<W> IterOutWeightedNeighbors<W> for BTreeMap<usize, BTreeMap<usize, W>>
where
    W: Copy,
{
    impl_ref_usize!();
}

impl<W, H> IterOutWeightedNeighbors<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    impl_ref_usize!();
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
            op::AddWeightedArc,
        },
    };

    macro_rules! test_stable {
        ($digraph:expr) => {
            assert!($digraph
                .iter_out_weighted_neighbors(0)
                .eq([(1, &2), (2, &3), (3, &4)]));

            assert!($digraph
                .iter_out_weighted_neighbors(1)
                .eq([(2, &3), (3, &4), (4, &5)]));

            assert!($digraph
                .iter_out_weighted_neighbors(2)
                .eq([(3, &4), (4, &5), (5, &6)]));
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_out_weighted_neighbors(0);

            assert!(matches!(iter.next(), Some((1, 2) | (2, 3) | (3, 4))));
            assert!(matches!(iter.next(), Some((1, 2) | (2, 3) | (3, 4))));
            assert!(matches!(iter.next(), Some((1, 2) | (2, 3) | (3, 4))));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_weighted_neighbors(1);

            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert_eq!(iter.next(), None);

            let mut iter = $digraph.iter_out_weighted_neighbors(2);

            assert!(matches!(iter.next(), Some((3, 4) | (4, 5) | (5, 6))));
            assert!(matches!(iter.next(), Some((3, 4) | (4, 5) | (5, 6))));
            assert!(matches!(iter.next(), Some((3, 4) | (4, 5) | (5, 6))));
            assert_eq!(iter.next(), None);
        };
    }

    macro_rules! setup {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(0, 3, 4);
            $digraph.add_weighted_arc(1, 2, 3);
            $digraph.add_weighted_arc(1, 3, 4);
            $digraph.add_weighted_arc(1, 4, 5);
            $digraph.add_weighted_arc(2, 3, 4);
            $digraph.add_weighted_arc(2, 4, 5);
            $digraph.add_weighted_arc(2, 5, 6);
        };
    }

    #[test]
    fn vec_vec() {
        let mut digraph = Vec::<Vec<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let mut digraph = Vec::<Vec<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(6);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<(usize, i32)>; 6]>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<(usize, i32)>; 6]>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<(usize, i32)>; 6]>::empty();

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, i32>; 6]>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, i32>; 6]>::empty();

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<(usize, i32)>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, i32>>::empty(6);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, i32>>::empty(6);

        setup!(digraph);
        test_unstable!(digraph);
    }
}
