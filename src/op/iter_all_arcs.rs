//! A trait to iterate over all arcs in an unweighted digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterAllArcs;
//!
//! let digraph = vec![(0, 1), (1, 2), (2, 0)];
//!
//! assert!(digraph.iter_all_arcs().eq([(0, 1), (1, 2), (2, 0)]));
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

/// A trait to iterate over all arcs in an unweighted digraph
///
/// # How can I implement `IterAllArcs`?
///
/// Provide an implementation of `iter_all_arcs` that returns an iterator over
/// all arcs in a digraph.
///
/// ```
/// use graaf::op::IterAllArcs;
///
/// struct Digraph {
///     arcs: Vec<(usize, usize)>,
/// }
///
/// impl IterAllArcs for Digraph {
///     fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.arcs.iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterAllArcs;
///
/// let digraph = vec![(0, 1), (1, 2), (2, 0)];
///
/// assert!(digraph.iter_all_arcs().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait IterAllArcs {
    /// Returns an iterator over all arcs in a digraph.
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)>;
}

impl IterAllArcs for Vec<Vec<usize>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (s, *t)))
    }
}

impl IterAllArcs for Vec<BTreeSet<usize>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<H> IterAllArcs for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl IterAllArcs for [Vec<usize>] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (s, *t)))
    }
}

impl IterAllArcs for [BTreeSet<usize>] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<H> IterAllArcs for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<const V: usize> IterAllArcs for [Vec<usize>; V] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (s, *t)))
    }
}

impl<const V: usize> IterAllArcs for [BTreeSet<usize>; V] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl<const V: usize, H> IterAllArcs for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |t| (s, *t)))
    }
}

impl IterAllArcs for BTreeMap<usize, Vec<usize>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (*s, *t)))
    }
}

impl IterAllArcs for BTreeMap<usize, BTreeSet<usize>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |t| (*s, *t)))
    }
}

impl<H> IterAllArcs for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |t| (*s, *t)))
    }
}

impl<H> IterAllArcs for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |t| (*s, *t)))
    }
}

impl IterAllArcs for Vec<(usize, usize)> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl IterAllArcs for [(usize, usize)] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl<const V: usize> IterAllArcs for [(usize, usize); V] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl IterAllArcs for BTreeSet<(usize, usize)> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl<H> IterAllArcs for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl<W> IterAllArcs for Vec<Vec<(usize, W)>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<W> IterAllArcs for Vec<BTreeSet<(usize, W)>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<W> IterAllArcs for Vec<BTreeMap<usize, W>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, map)| map.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<W, H> IterAllArcs for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<W, H> IterAllArcs for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, map)| map.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<const V: usize, W> IterAllArcs for [Vec<(usize, W)>; V] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<const V: usize, W> IterAllArcs for [BTreeSet<(usize, W)>; V] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<const V: usize, W> IterAllArcs for [BTreeMap<usize, W>; V] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, map)| map.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<const V: usize, W, H> IterAllArcs for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, set)| set.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<const V: usize, W, H> IterAllArcs for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .enumerate()
            .flat_map(|(s, map)| map.iter().map(move |(t, _)| (s, *t)))
    }
}

impl<W> IterAllArcs for BTreeMap<usize, Vec<(usize, W)>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, _)| (*s, *t)))
    }
}

impl<W> IterAllArcs for BTreeMap<usize, BTreeSet<(usize, W)>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |(t, _)| (*s, *t)))
    }
}

impl<W> IterAllArcs for BTreeMap<usize, BTreeMap<usize, W>> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, map)| map.iter().map(move |(t, _)| (*s, *t)))
    }
}

impl<W, H> IterAllArcs for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, _)| (*s, *t)))
    }
}

impl<W, H> IterAllArcs for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |(t, _)| (*s, *t)))
    }
}

impl<W, H> IterAllArcs for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, map)| map.iter().map(move |(t, _)| (*s, *t)))
    }
}

impl<W> IterAllArcs for Vec<(usize, usize, W)> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().map(|(s, t, _)| (*s, *t))
    }
}

impl<W> IterAllArcs for [(usize, usize, W)] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().map(|(s, t, _)| (*s, *t))
    }
}

impl<const V: usize, W> IterAllArcs for [(usize, usize, W); V] {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().map(|(s, t, _)| (*s, *t))
    }
}

impl<W> IterAllArcs for BTreeSet<(usize, usize, W)> {
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().map(|(s, t, _)| (*s, *t))
    }
}

impl<W, H> IterAllArcs for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().map(|(s, t, _)| (*s, *t))
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

    macro_rules! test_iter_all_arcs_stable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_all_arcs();

            assert_eq!(iter.next(), Some((0, 1)));
            assert_eq!(iter.next(), Some((1, 2)));
            assert_eq!(iter.next(), Some((2, 0)));
            assert_eq!(iter.next(), None);
        };
    }

    macro_rules! test_iter_all_arcs_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_all_arcs();

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
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]

    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let digraph: &[Vec<usize>] = &[vec![1], vec![2], vec![0]];

        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &[HashSet<usize>] =
            &[HashSet::from([1]), HashSet::from([2]), HashSet::from([0])];

        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_tuple() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn slice_tuple() {
        let digraph: &[(usize, usize)] = &[(0, 1), (1, 2), (2, 0)];

        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn arr_tuple() {
        let digraph = [(0, 1), (1, 2), (2, 0)];

        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn btree_set() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn hash_set() {
        let mut digraph = HashSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_vec_weighted() {
        let mut digraph = Vec::<Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn vec_btree_set_weighted() {
        let mut digraph = Vec::<BTreeSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_btree_map_weighted() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_hash_set_weighted() {
        let mut digraph = Vec::<HashSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_hash_map_weighted() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_arr_weighted() {
        let mut digraph = <[Vec<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn arr_btree_set_weighted() {
        let mut digraph = <[BTreeSet<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_btree_map_weighted() {
        let mut digraph = <[BTreeMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_hash_set_weighted() {
        let mut digraph = <[HashSet<(usize, usize)>; 3]>::empty();

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_hash_map_weighted() {
        let mut digraph = <[HashMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec_weighted() {
        let mut digraph = BTreeMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set_weighted() {
        let mut digraph = BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_map_weighted() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn hash_map_vec_weighted() {
        let mut digraph = HashMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set_weighted() {
        let mut digraph = HashMap::<usize, HashSet<(usize, usize)>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_map_weighted() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_tuple_weighted() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn slice_tuple_weighted() {
        let digraph: &[(usize, usize, usize)] = &[(0, 1, 0), (1, 2, 0), (2, 0, 0)];

        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn arr_tuple_weighted() {
        let digraph = [(0, 1, 0), (1, 2, 0), (2, 0, 0)];

        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn btree_set_weighted() {
        let mut digraph = BTreeSet::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_stable!(digraph);
    }

    #[test]
    fn hash_set_weighted() {
        let mut digraph = HashSet::<(usize, usize, usize)>::empty(3);

        setup_weighted!(digraph);
        test_iter_all_arcs_unstable!(digraph);
    }
}
