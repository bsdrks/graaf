//! A trait to iterate over all arcs in a weighted directed graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterAllWeightedArcs;
//!
//! let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
//!
//! assert!(graph
//!     .iter_all_weighted_arcs()
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

/// A trait to iterate over all arcs in a weighted directed graph
///
/// # How can I implement `IterAllWeightedArcs`?
///
/// Provide an implementation of `iter_all_weighted_arcs` that returns an
/// iterator over all arcs in a graph.
///
/// ```
/// use graaf::op::IterAllWeightedArcs;
///
/// struct Graph {
///     arcs: Vec<(usize, usize, usize)>,
/// }
///
/// impl IterAllWeightedArcs<usize> for Graph {
///     fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a usize)>
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
/// use graaf::op::IterAllWeightedArcs;
///
/// let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
///
/// assert!(graph
///     .iter_all_weighted_arcs()
///     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
/// ```
pub trait IterAllWeightedArcs<W> {
    /// Returns an iterator over all arcs in a graph.
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a;
}

impl<W> IterAllWeightedArcs<W> for Vec<Vec<(usize, W)>> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, vec)| vec.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for Vec<BTreeSet<(usize, W)>> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterAllWeightedArcs<W> for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for Vec<BTreeMap<usize, W>> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterAllWeightedArcs<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for [Vec<(usize, W)>] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, vec)| vec.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for [BTreeSet<(usize, W)>] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterAllWeightedArcs<W> for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for [BTreeMap<usize, W>] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W, H> IterAllWeightedArcs<W> for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W> IterAllWeightedArcs<W> for [Vec<(usize, W)>; V] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W> IterAllWeightedArcs<W> for [BTreeSet<(usize, W)>; V] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W, H> IterAllWeightedArcs<W> for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, set)| set.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W> IterAllWeightedArcs<W> for [BTreeMap<usize, W>; V] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<const V: usize, W, H> IterAllWeightedArcs<W> for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .enumerate()
            .flat_map(move |(s, map)| map.iter().map(move |(t, w)| (s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for BTreeMap<usize, Vec<(usize, W)>> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for BTreeMap<usize, BTreeSet<(usize, W)>> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for BTreeMap<usize, BTreeMap<usize, W>> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, map)| map.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W, H> IterAllWeightedArcs<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, vec)| vec.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W, H> IterAllWeightedArcs<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, set)| set.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W, H> IterAllWeightedArcs<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter()
            .flat_map(|(s, map)| map.iter().map(move |(t, w)| (*s, *t, w)))
    }
}

impl<W> IterAllWeightedArcs<W> for Vec<(usize, usize, W)> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W> IterAllWeightedArcs<W> for [(usize, usize, W)] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<const V: usize, W> IterAllWeightedArcs<W> for [(usize, usize, W); V] {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W> IterAllWeightedArcs<W> for BTreeSet<(usize, usize, W)> {
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W, H> IterAllWeightedArcs<W> for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn iter_all_weighted_arcs<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_iter_all_weighted_arcs_stable {
        ($graph:expr) => {
            assert!($graph
                .iter_all_weighted_arcs()
                .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
        };
    }

    macro_rules! test_iter_all_weighted_arcs_unstable {
        ($graph:expr) => {
            let mut iter = $graph.iter_all_weighted_arcs();

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
        let graph = vec![vec![(1, 2)], vec![(2, 3)], vec![(0, 4)]];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([(1, 2)]),
            BTreeSet::from([(2, 3)]),
            BTreeSet::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([(1, 2)]),
            HashSet::from([(2, 3)]),
            HashSet::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2)]),
            BTreeMap::from([(2, 3)]),
            BTreeMap::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn slice_vec() {
        let graph: &[Vec<(usize, usize)>] = &[vec![(1, 2)], vec![(2, 3)], vec![(0, 4)]];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<(usize, usize)>] = &[
            BTreeSet::from([(1, 2)]),
            BTreeSet::from([(2, 3)]),
            BTreeSet::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<(usize, usize)>] = &[
            HashSet::from([(1, 2)]),
            HashSet::from([(2, 3)]),
            HashSet::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph = &[
            BTreeMap::from([(1, 2)]),
            BTreeMap::from([(2, 3)]),
            BTreeMap::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph = &[
            HashMap::from([(1, 2)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn arr_vec() {
        let graph = [vec![(1, 2)], vec![(2, 3)], vec![(0, 4)]];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([(1, 2)]),
            BTreeSet::from([(2, 3)]),
            BTreeSet::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([(1, 2)]),
            HashSet::from([(2, 3)]),
            HashSet::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2)]),
            BTreeMap::from([(2, 3)]),
            BTreeMap::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(0, 4)]),
        ];

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn btree_map_vec() {
        let graph = BTreeMap::from([(0, vec![(1, 2)]), (1, vec![(2, 3)]), (2, vec![(0, 4)])]);

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([(1, 2)])),
            (1, BTreeSet::from([(2, 3)])),
            (2, BTreeSet::from([(0, 4)])),
        ]);

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2)])),
            (1, BTreeMap::from([(2, 3)])),
            (2, BTreeMap::from([(0, 4)])),
        ]);

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([(0, vec![(1, 2)]), (1, vec![(2, 3)]), (2, vec![(0, 4)])]);

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([(1, 2)])),
            (1, HashSet::from([(2, 3)])),
            (2, HashSet::from([(0, 4)])),
        ]);

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 2)])),
            (1, HashMap::from([(2, 3)])),
            (2, HashMap::from([(0, 4)])),
        ]);

        test_iter_all_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn vec() {
        let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn slice() {
        let graph: &[(usize, usize, usize)] = &[(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn arr() {
        let graph = [(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        test_iter_all_weighted_arcs_stable!(graph);
    }

    #[test]
    fn hash_set() {
        let graph: HashSet<(usize, usize, usize)> =
            HashSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        test_iter_all_weighted_arcs_unstable!(graph);
    }
}
