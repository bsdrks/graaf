//! Iterate over the arcs in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterArcs;
//!
//! let digraph = vec![(0, 1), (1, 2), (2, 0)];
//!
//! assert!(digraph.iter_arcs().eq([(0, 1), (1, 2), (2, 0)]));
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

/// Iterate the arcs in a digraph.
///
/// # How can I implement `IterArcs`?
///
/// Provide an implementation of `iter_arcs` that returns an iterator over
/// all arcs in a digraph.
///
/// ```
/// use graaf::op::IterArcs;
///
/// struct Digraph {
///     arcs: Vec<(usize, usize)>,
/// }
///
/// impl IterArcs for Digraph {
///     fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.arcs.iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterArcs;
///
/// let digraph = vec![(0, 1), (1, 2), (2, 0)];
///
/// assert!(digraph.iter_arcs().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait IterArcs {
    /// Returns an iterator over the arcs in the digraph.
    fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)>;
}

macro_rules! impl_enumerate_unweighted {
    () => {
        fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
            self.iter()
                .enumerate()
                .flat_map(|(s, v)| v.iter().map(move |t| (s, *t)))
        }
    };
}

macro_rules! impl_map_unweighted {
    () => {
        fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
            self.iter()
                .flat_map(|(s, v)| v.iter().map(move |t| (*s, *t)))
        }
    };
}

macro_rules! impl_tuple_unweighted {
    () => {
        fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
            self.iter().copied()
        }
    };
}

macro_rules! impl_enumerate_weighted {
    () => {
        fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
            self.iter()
                .enumerate()
                .flat_map(|(s, v)| v.iter().map(move |(t, _)| (s, *t)))
        }
    };
}

macro_rules! impl_map_weighted {
    () => {
        fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
            self.iter()
                .flat_map(|(s, v)| v.iter().map(move |(t, _)| (*s, *t)))
        }
    };
}

macro_rules! impl_tuple_weighted {
    () => {
        fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
            self.iter().map(|(s, t, _)| (*s, *t))
        }
    };
}

impl IterArcs for Vec<Vec<usize>> {
    impl_enumerate_unweighted!();
}

impl IterArcs for Vec<BTreeSet<usize>> {
    impl_enumerate_unweighted!();
}

impl<H> IterArcs for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_enumerate_unweighted!();
}

impl IterArcs for [Vec<usize>] {
    impl_enumerate_unweighted!();
}

impl IterArcs for [BTreeSet<usize>] {
    impl_enumerate_unweighted!();
}

impl<H> IterArcs for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_enumerate_unweighted!();
}

impl<const V: usize> IterArcs for [Vec<usize>; V] {
    impl_enumerate_unweighted!();
}

impl<const V: usize> IterArcs for [BTreeSet<usize>; V] {
    impl_enumerate_unweighted!();
}

impl<const V: usize, H> IterArcs for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_enumerate_unweighted!();
}

impl IterArcs for BTreeMap<usize, Vec<usize>> {
    impl_map_unweighted!();
}

impl IterArcs for BTreeMap<usize, BTreeSet<usize>> {
    impl_map_unweighted!();
}

impl<H> IterArcs for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    impl_map_unweighted!();
}

impl<H> IterArcs for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_map_unweighted!();
}

impl IterArcs for Vec<(usize, usize)> {
    impl_tuple_unweighted!();
}

impl IterArcs for [(usize, usize)] {
    impl_tuple_unweighted!();
}

impl<const V: usize> IterArcs for [(usize, usize); V] {
    impl_tuple_unweighted!();
}

impl IterArcs for BTreeSet<(usize, usize)> {
    impl_tuple_unweighted!();
}

impl<H> IterArcs for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_tuple_unweighted!();
}

impl<W> IterArcs for Vec<Vec<(usize, W)>> {
    impl_enumerate_weighted!();
}

impl<W> IterArcs for Vec<BTreeSet<(usize, W)>> {
    impl_enumerate_weighted!();
}

impl<W> IterArcs for Vec<BTreeMap<usize, W>> {
    impl_enumerate_weighted!();
}

impl<W, H> IterArcs for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    impl_enumerate_weighted!();
}

impl<W, H> IterArcs for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_enumerate_weighted!();
}

impl<W> IterArcs for [Vec<(usize, W)>] {
    impl_enumerate_weighted!();
}

impl<W> IterArcs for [BTreeSet<(usize, W)>] {
    impl_enumerate_weighted!();
}

impl<W> IterArcs for [BTreeMap<usize, W>] {
    impl_enumerate_weighted!();
}

impl<W, H> IterArcs for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
{
    impl_enumerate_weighted!();
}

impl<W, H> IterArcs for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_enumerate_weighted!();
}

impl<const V: usize, W> IterArcs for [Vec<(usize, W)>; V] {
    impl_enumerate_weighted!();
}

impl<const V: usize, W> IterArcs for [BTreeSet<(usize, W)>; V] {
    impl_enumerate_weighted!();
}

impl<const V: usize, W> IterArcs for [BTreeMap<usize, W>; V] {
    impl_enumerate_weighted!();
}

impl<const V: usize, W, H> IterArcs for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    impl_enumerate_weighted!();
}

impl<const V: usize, W, H> IterArcs for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_enumerate_weighted!();
}

impl<W> IterArcs for BTreeMap<usize, Vec<(usize, W)>> {
    impl_map_weighted!();
}

impl<W> IterArcs for BTreeMap<usize, BTreeSet<(usize, W)>> {
    impl_map_weighted!();
}

impl<W> IterArcs for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_map_weighted!();
}

impl<W, H> IterArcs for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    impl_map_weighted!();
}

impl<W, H> IterArcs for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
{
    impl_map_weighted!();
}

impl<W, H> IterArcs for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_map_weighted!();
}

impl<W> IterArcs for Vec<(usize, usize, W)> {
    impl_tuple_weighted!();
}

impl<W> IterArcs for [(usize, usize, W)] {
    impl_tuple_weighted!();
}

impl<const V: usize, W> IterArcs for [(usize, usize, W); V] {
    impl_tuple_weighted!();
}

impl<W> IterArcs for BTreeSet<(usize, usize, W)> {
    impl_tuple_weighted!();
}

impl<W, H> IterArcs for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    impl_tuple_weighted!();
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
            op::{
                AddArc,
                AddWeightedArc,
            },
        },
    };

    macro_rules! setup_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(1, 2);
            $digraph.add_arc(2, 0);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 0);
            $digraph.add_weighted_arc(1, 2, 0);
            $digraph.add_weighted_arc(2, 0, 0);
        };
    }

    macro_rules! test_stable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_arcs();

            assert_eq!(iter.next(), Some((0, 1)));
            assert_eq!(iter.next(), Some((1, 2)));
            assert_eq!(iter.next(), Some((2, 0)));
            assert_eq!(iter.next(), None);
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_arcs();

            assert!(matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0))));
            assert!(matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0))));
            assert!(matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0))));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]

    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_tuple() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn slice_tuple() {
        let digraph: &[(usize, usize)] = &[(0, 1), (1, 2), (2, 0)];

        test_stable!(digraph);
    }

    #[test]
    fn arr_tuple() {
        let digraph = [(0, 1), (1, 2), (2, 0)];

        test_stable!(digraph);
    }

    #[test]
    fn btree_set() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_set() {
        let mut digraph = HashSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_btree_map_weighted() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_hash_map_weighted() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_map_weighted() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_map_weighted() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn arr_vec_weighted() {
        let mut digraph = <[Vec<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let mut digraph = <[BTreeSet<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let mut digraph = <[HashSet<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn arr_btree_map_weighted() {
        let mut digraph = <[BTreeMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn arr_hash_map_weighted() {
        let mut digraph = <[HashMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec_weighted() {
        let mut digraph = BTreeMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set_weighted() {
        let mut digraph = BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_map_weighted() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_vec_weighted() {
        let mut digraph = HashMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set_weighted() {
        let mut digraph = HashMap::<usize, HashSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_map_weighted() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn slice_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn arr_tuple_weighted() {
        let digraph = [(0, 1, 0), (1, 2, 0), (2, 0, 0)];

        test_stable!(digraph);
    }

    #[test]
    fn btree_set_weighted() {
        let mut digraph = BTreeSet::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_set_weighted() {
        let mut digraph = HashSet::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }
}
