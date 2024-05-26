//! A trait to iterate over all arcs in a weighted digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterWeightedArcs;
//!
//! let digraph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
//!
//! assert!(digraph
//!     .iter_weighted_arcs()
//!     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
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

/// A trait to iterate over all arcs in a weighted digraph
///
/// # How can I implement `IterWeightedArcs`?
///
/// Provide an implementation of `iter_weighted_arcs` that returns an
/// iterator over all arcs in a digraph.
///
/// ```
/// use graaf::op::IterWeightedArcs;
///
/// struct Digraph {
///     arcs: Vec<(usize, usize, usize)>,
/// }
///
/// impl IterWeightedArcs<usize> for Digraph {
///     fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a usize)>
///     where
///         usize: 'a,
///     {
///         self.arcs.iter().map(|(s, t, w)| (*s, *t, w))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterWeightedArcs;
///
/// let digraph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
///
/// assert!(digraph
///     .iter_weighted_arcs()
///     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
/// ```
pub trait IterWeightedArcs<W> {
    /// Returns an iterator over all arcs in a digraph.
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a;
}

impl<W> IterWeightedArcs<W> for Vec<Vec<(usize, W)>> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, vec)| vec.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for Vec<BTreeSet<(usize, W)>> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterWeightedArcs<W> for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for Vec<BTreeMap<usize, W>> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterWeightedArcs<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for [Vec<(usize, W)>] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, vec)| vec.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for [BTreeSet<(usize, W)>] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterWeightedArcs<W> for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for [BTreeMap<usize, W>] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterWeightedArcs<W> for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W> IterWeightedArcs<W> for [Vec<(usize, W)>; V] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W> IterWeightedArcs<W> for [BTreeSet<(usize, W)>; V] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W, H> IterWeightedArcs<W> for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W> IterWeightedArcs<W> for [BTreeMap<usize, W>; V] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W, H> IterWeightedArcs<W> for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for BTreeMap<usize, Vec<(usize, W)>> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for BTreeMap<usize, BTreeSet<(usize, W)>> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for BTreeMap<usize, BTreeMap<usize, W>> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, map)| map.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W, H> IterWeightedArcs<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W, H> IterWeightedArcs<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W, H> IterWeightedArcs<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, map)| map.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W> IterWeightedArcs<W> for Vec<(usize, usize, W)> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W> IterWeightedArcs<W> for [(usize, usize, W)] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<const V: usize, W> IterWeightedArcs<W> for [(usize, usize, W); V] {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W> IterWeightedArcs<W> for BTreeSet<(usize, usize, W)> {
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
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
            op::AddWeightedArc,
        },
    };

    macro_rules! setup {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(1, 2, 3);
            $digraph.add_weighted_arc(2, 0, 4);
        };
    }

    macro_rules! test_stable {
        ($digraph:expr) => {
            assert!($digraph
                .iter_weighted_arcs()
                .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            let mut iter = $digraph.iter_weighted_arcs();

            assert!(matches!(
                iter.next(),
                Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4))
            ));

            assert!(matches!(
                iter.next(),
                Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4))
            ));

            assert!(matches!(
                iter.next(),
                Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4))
            ));

            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let digraph = vec![vec![(1, 2)], vec![(2, 3)], vec![(0, 4)]];

        test_stable!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let mut digraph = Vec::<Vec<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup!(digraph);
        test_unstable!(digraph.as_slice());
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<(usize, usize)>; 3]>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<(usize, usize)>; 3]>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<(usize, usize)>; 3]>::empty();

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, usize>; 3]>::empty();

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, usize>; 3]>::empty();

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<(usize, usize)>>::empty(3);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_tuple() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn slice_tuple() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup!(digraph);
        test_stable!(digraph.as_slice());
    }

    #[test]
    fn arr_tuple() {
        let digraph = [(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        test_stable!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize, usize)>::empty(3);

        setup!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize, usize)>::empty(3);

        setup!(digraph);
        test_unstable!(digraph);
    }
}
