//! A trait to iterate over all arcs with a given source vertex in a weighted
//! digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterWeightedArcs;
//!
//! let digraph = vec![
//!     vec![(1, 2), (2, 3), (3, 4)],
//!     vec![(2, 3), (3, 4), (4, 5)],
//!     vec![(3, 4), (4, 5), (5, 6)],
//! ];
//!
//! assert!(digraph
//!     .iter_weighted_arcs(0)
//!     .eq([(1, &2), (2, &3), (3, &4)]));
//! assert!(digraph
//!     .iter_weighted_arcs(1)
//!     .eq([(2, &3), (3, &4), (4, &5)]));
//! assert!(digraph
//!     .iter_weighted_arcs(2)
//!     .eq([(3, &4), (4, &5), (5, &6)]));
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
/// digraph
///
/// # How can I implement `IterWeightedArcs`?
///
/// Provide an implementation of `iter_weighted_arcs` that returns an iterator
/// over all weighted arcs with the source vertex `s`.
///
/// ```
/// use graaf::op::IterWeightedArcs;
///
/// struct Digraph {
///     arcs: Vec<Vec<(usize, usize)>>,
/// }
///
/// impl IterWeightedArcs<usize> for Digraph {
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
/// let digraph = vec![
///     vec![(1, 2), (2, 3), (3, 4)],
///     vec![(2, 3), (3, 4), (4, 5)],
///     vec![(3, 4), (4, 5), (5, 6)],
/// ];
///
/// assert!(digraph
///     .iter_weighted_arcs(0)
///     .eq([(1, &2), (2, &3), (3, &4)]));
/// assert!(digraph
///     .iter_weighted_arcs(1)
///     .eq([(2, &3), (3, &4), (4, &5)]));
/// assert!(digraph
///     .iter_weighted_arcs(2)
///     .eq([(3, &4), (4, &5), (5, &6)]));
/// ```
pub trait IterWeightedArcs<W> {
    /// Returns an iterator over the arcs with the source vertex `s`.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
    /// Panics if `s` is not in the digraph.
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
        ($digraph:expr) => {
            let mut iter = $digraph.iter_weighted_arcs(1);

            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert!(matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5))));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_vec() {
        let digraph = vec![
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        assert!(digraph
            .iter_weighted_arcs(1)
            .eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn vec_btree_set() {
        let digraph = vec![
            BTreeSet::from([(1, 2), (2, 3), (3, 4)]),
            BTreeSet::from([(2, 3), (3, 4), (4, 5)]),
            BTreeSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = vec![
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = vec![
            BTreeMap::from([(1, 2), (2, 3), (3, 4)]),
            BTreeMap::from([(2, 3), (3, 4), (4, 5)]),
            BTreeMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = vec![
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn slice_vec() {
        let digraph: &[Vec<(usize, i32)>] = &[
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        assert!(digraph
            .iter_weighted_arcs(1)
            .eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &[BTreeSet<(usize, i32)>] = &[
            BTreeSet::from([(1, 2), (2, 3), (3, 4)]),
            BTreeSet::from([(2, 3), (3, 4), (4, 5)]),
            BTreeSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &[HashSet<(usize, i32)>] = &[
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 2), (2, 3), (3, 4)]),
            BTreeMap::from([(2, 3), (3, 4), (4, 5)]),
            BTreeMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_vec() {
        let digraph = [
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        assert!(digraph
            .iter_weighted_arcs(1)
            .eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn arr_btree_set() {
        let digraph = [
            BTreeSet::from([(1, 2), (2, 3), (3, 4)]),
            BTreeSet::from([(2, 3), (3, 4), (4, 5)]),
            BTreeSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = [
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = [
            BTreeMap::from([(1, 2), (2, 3), (3, 4)]),
            BTreeMap::from([(2, 3), (3, 4), (4, 5)]),
            BTreeMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = [
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let digraph = BTreeMap::from([
            (0, vec![(1, 2), (2, 3), (3, 4)]),
            (1, vec![(2, 3), (3, 4), (4, 5)]),
            (2, vec![(3, 4), (4, 5), (5, 6)]),
        ]);

        assert!(digraph
            .iter_weighted_arcs(1)
            .eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn hash_map_vec() {
        let digraph = HashMap::from([
            (0, vec![(1, 2), (2, 3), (3, 4)]),
            (1, vec![(2, 3), (3, 4), (4, 5)]),
            (2, vec![(3, 4), (4, 5), (5, 6)]),
        ]);

        assert!(digraph
            .iter_weighted_arcs(1)
            .eq([(2, &3), (3, &4), (4, &5)]));
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = BTreeMap::from([
            (0, BTreeSet::from([(1, 2), (2, 3), (3, 4)])),
            (1, BTreeSet::from([(2, 3), (3, 4), (4, 5)])),
            (2, BTreeSet::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = HashMap::from([
            (0, HashSet::from([(1, 2), (2, 3), (3, 4)])),
            (1, HashSet::from([(2, 3), (3, 4), (4, 5)])),
            (2, HashSet::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let digraph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3), (3, 4)])),
            (1, BTreeMap::from([(2, 3), (3, 4), (4, 5)])),
            (2, BTreeMap::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let digraph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3), (3, 4)])),
            (1, HashMap::from([(2, 3), (3, 4), (4, 5)])),
            (2, HashMap::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        test_iter_weighted_arcs_unstable!(digraph);
    }
}
