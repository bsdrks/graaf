//! A trait to check if an arc exists from one vertex to another
//!
//! To check if an arc exists from `s` to `t` and from `t` to `s`, see
//! [`HasEdge`].
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::HasArc,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([0, 1]),
//! ];
//!
//! assert!(!graph.has_arc(0, 0));
//! assert!(graph.has_arc(0, 1));
//! assert!(graph.has_arc(0, 2));
//! assert!(graph.has_arc(1, 0));
//! assert!(!graph.has_arc(1, 1));
//! assert!(!graph.has_arc(1, 2));
//! assert!(graph.has_arc(2, 0));
//! assert!(graph.has_arc(2, 1));
//! assert!(!graph.has_arc(2, 2));
//! ```
//!
//! [`HasEdge`]: crate::op::HasEdge

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

/// A trait to check if an arc exists from one vertex to another
///
/// # How can I implement `HasArc`?
///
/// Provide an implementation of `has_arc` that returns `true` if there is an
/// arc from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::HasArc,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl HasArc for Graph {
///     fn has_arc(&self, s: usize, t: usize) -> bool {
///         self.arcs.get(s).map_or(false, |set| set.contains(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::HasArc,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([0, 1]),
/// ];
///
/// assert!(!graph.has_arc(0, 0));
/// assert!(graph.has_arc(0, 1));
/// assert!(graph.has_arc(0, 2));
/// assert!(graph.has_arc(1, 0));
/// assert!(!graph.has_arc(1, 1));
/// assert!(!graph.has_arc(1, 2));
/// assert!(graph.has_arc(2, 0));
/// assert!(graph.has_arc(2, 1));
/// assert!(!graph.has_arc(2, 2));
/// ```
///
/// # Properties
///
/// ## `HasArc` and `AddArc`
///
/// Types that also implement [`AddArc`] should ensure that
/// [`add_arc_has_arc`] holds.
///
/// ## `HasArc` and `AddWeightedArc`
///
/// Types that also implement [`AddWeightedArc`] should ensure that
/// [`add_weighted_arc_has_arc`] holds.
///
/// [`AddArc`]: crate::op::AddArc
/// [`AddWeightedArc`]: crate::op::AddWeightedArc
/// [`add_arc_has_arc`]: crate::prop::add_arc_has_arc
/// [`add_weighted_arc_has_arc`]: crate::prop::add_weighted_arc_has_arc
pub trait HasArc {
    /// Returns whether an arc exists from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Panics
    ///
    /// Implementations may not panic if `s` or `t` are not in the graph.
    fn has_arc(&self, s: usize, t: usize) -> bool;
}

impl HasArc for Vec<BTreeSet<usize>> {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> HasArc for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> HasArc for Vec<BTreeMap<usize, W>> {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> HasArc for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl HasArc for [BTreeSet<usize>] {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> HasArc for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> HasArc for [BTreeMap<usize, W>] {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> HasArc for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<const V: usize> HasArc for [BTreeSet<usize>; V] {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<const V: usize, H> HasArc for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<const V: usize, W> HasArc for [BTreeMap<usize, W>; V] {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<const V: usize, W, H> HasArc for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl HasArc for BTreeSet<(usize, usize)> {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.contains(&(s, t))
    }
}

impl<H> HasArc for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.contains(&(s, t))
    }
}

impl HasArc for BTreeMap<usize, BTreeSet<usize>> {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> HasArc for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> HasArc for BTreeMap<usize, BTreeMap<usize, W>> {
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> HasArc for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |map| map.contains_key(&t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_has_arc {
        ($graph:expr) => {
            assert!(!$graph.has_arc(0, 0));
            assert!($graph.has_arc(0, 1));
            assert!($graph.has_arc(0, 2));
            assert!($graph.has_arc(1, 0));
            assert!(!$graph.has_arc(1, 1));
            assert!(!$graph.has_arc(1, 2));
            assert!($graph.has_arc(2, 0));
            assert!($graph.has_arc(2, 1));
            assert!(!$graph.has_arc(2, 2));
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_has_arc!(graph);
    }

    #[test]
    fn btree_set() {
        let graph = BTreeSet::from([(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);

        test_has_arc!(graph);
    }

    #[test]
    fn hash_set() {
        let graph = HashSet::from([(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);

        test_has_arc!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0])),
            (2, BTreeSet::from([0, 1])),
        ]);

        test_has_arc!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([0, 1])),
        ]);

        test_has_arc!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 1), (2, 1)])),
            (1, BTreeMap::from([(0, 1)])),
            (2, BTreeMap::from([(0, 1), (1, 1)])),
        ]);

        test_has_arc!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(0, 1), (1, 1)])),
        ]);

        test_has_arc!(graph);
    }
}
