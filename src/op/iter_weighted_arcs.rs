//! A trait to iterate over all arcs with a given source vertex in a weighted
//! directed graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterWeightedArcs;
//!
//! let graph = vec![
//!     vec![(1, 2), (2, 3), (3, 4)],
//!     vec![(2, 3), (3, 4), (4, 5)],
//!     vec![(3, 4), (4, 5), (5, 6)],
//! ];
//!
//! assert!(graph.iter_weighted_arcs(0).eq([(1, &2), (2, &3), (3, &4)]));
//! assert!(graph.iter_weighted_arcs(1).eq([(2, &3), (3, &4), (4, &5)]));
//! assert!(graph.iter_weighted_arcs(2).eq([(3, &4), (4, &5), (5, &6)]));
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

/// A trait to iterate over all arcs with a given source vertex in a weighted
/// directed graph
///
/// # How can I implement `IterWeightedArcs`?
///
/// Provide an implementation of `iter_weighted_arcs` that returns an iterator
/// over all weighted arcs with the source vertex `s`.
///
/// ```
/// use graaf::op::IterWeightedArcs;
///
/// struct Graph {
///     arcs: Vec<Vec<(usize, usize)>>,
/// }
///
/// impl IterWeightedArcs<usize> for Graph {
///     fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a usize)>
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
/// use graaf::op::IterWeightedArcs;
///
/// let graph = vec![
///     vec![(1, 2), (2, 3), (3, 4)],
///     vec![(2, 3), (3, 4), (4, 5)],
///     vec![(3, 4), (4, 5), (5, 6)],
/// ];
///
/// assert!(graph.iter_weighted_arcs(0).eq([(1, &2), (2, &3), (3, &4)]));
/// assert!(graph.iter_weighted_arcs(1).eq([(2, &3), (3, &4), (4, &5)]));
/// assert!(graph.iter_weighted_arcs(2).eq([(3, &4), (4, &5), (5, &6)]));
/// ```
pub trait IterWeightedArcs<W> {
    /// Returns an iterator over the arcs with the source vertex `s`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a;
}

impl<W> IterWeightedArcs<W> for Vec<Vec<(usize, W)>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for Vec<BTreeSet<(usize, W)>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for Vec<HashSet<(usize, W), H>>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for Vec<BTreeMap<usize, W>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for Vec<HashMap<usize, W, H>>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for [Vec<(usize, W)>]
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for [BTreeSet<(usize, W)>]
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for [HashSet<(usize, W), H>]
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for [BTreeMap<usize, W>]
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for [HashMap<usize, W, H>]
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<const V: usize, W> IterWeightedArcs<W> for [Vec<(usize, W)>; V]
where
    W: Copy,
{
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<const V: usize, W> IterWeightedArcs<W> for [BTreeSet<(usize, W)>; V]
where
    W: Copy,
{
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<const V: usize, W, H> IterWeightedArcs<W> for [HashSet<(usize, W), H>; V]
where
    W: Copy,
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<const V: usize, W> IterWeightedArcs<W> for [BTreeMap<usize, W>; V]
where
    W: Copy,
{
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<const V: usize, W, H> IterWeightedArcs<W> for [HashMap<usize, W, H>; V]
where
    W: Copy,
    H: BuildHasher,
{
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for BTreeMap<usize, Vec<(usize, W)>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[&s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[&s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for BTreeMap<usize, BTreeSet<(usize, W)>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[&s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[&s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W> IterWeightedArcs<W> for BTreeMap<usize, BTreeMap<usize, W>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[&s].iter().map(|(t, w)| (*t, w))
    }
}

impl<W, H> IterWeightedArcs<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_arcs<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a,
    {
        self[&s].iter().map(|(t, w)| (*t, w))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_iter_weighted_arcs_unstable {
        ($graph:expr) => {
            let mut iter = $graph.iter_weighted_arcs(1);

            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let graph = vec![
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        assert!(graph.iter_weighted_arcs(1).eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([(1, 2), (2, 3), (3, 4)]),
            BTreeSet::from([(2, 3), (3, 4), (4, 5)]),
            BTreeSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3), (3, 4)]),
            BTreeMap::from([(2, 3), (3, 4), (4, 5)]),
            BTreeMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn slice_vec() {
        let graph: &[Vec<(usize, i32)>] = &[
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        assert!(graph.iter_weighted_arcs(1).eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<(usize, i32)>] = &[
            BTreeSet::from([(1, 2), (2, 3), (3, 4)]),
            BTreeSet::from([(2, 3), (3, 4), (4, 5)]),
            BTreeSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<(usize, i32)>] = &[
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 2), (2, 3), (3, 4)]),
            BTreeMap::from([(2, 3), (3, 4), (4, 5)]),
            BTreeMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn arr_vec() {
        let graph = [
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        assert!(graph.iter_weighted_arcs(1).eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([(1, 2), (2, 3), (3, 4)]),
            BTreeSet::from([(2, 3), (3, 4), (4, 5)]),
            BTreeSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3), (3, 4)]),
            BTreeMap::from([(2, 3), (3, 4), (4, 5)]),
            BTreeMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn btree_map_vec() {
        let graph = BTreeMap::from([
            (0, vec![(1, 2), (2, 3), (3, 4)]),
            (1, vec![(2, 3), (3, 4), (4, 5)]),
            (2, vec![(3, 4), (4, 5), (5, 6)]),
        ]);

        assert!(graph.iter_weighted_arcs(1).eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([
            (0, vec![(1, 2), (2, 3), (3, 4)]),
            (1, vec![(2, 3), (3, 4), (4, 5)]),
            (2, vec![(3, 4), (4, 5), (5, 6)]),
        ]);

        assert!(graph.iter_weighted_arcs(1).eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([(1, 2), (2, 3), (3, 4)])),
            (1, BTreeSet::from([(2, 3), (3, 4), (4, 5)])),
            (2, BTreeSet::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([(1, 2), (2, 3), (3, 4)])),
            (1, HashSet::from([(2, 3), (3, 4), (4, 5)])),
            (2, HashSet::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3), (3, 4)])),
            (1, BTreeMap::from([(2, 3), (3, 4), (4, 5)])),
            (2, BTreeMap::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3), (3, 4)])),
            (1, HashMap::from([(2, 3), (3, 4), (4, 5)])),
            (2, HashMap::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(graph);
    }
}
