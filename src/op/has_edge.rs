//! A trait to check if an edge exists from one vertex to another
//!
//! To check if an edge exists from `s` to `t` and from `t` to `s`, see
//! [`HasEdgeSymmetric`].
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::HasEdge,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([0, 1]),
//! ];
//!
//! assert!(!graph.has_edge(0, 0));
//! assert!(graph.has_edge(0, 1));
//! assert!(graph.has_edge(0, 2));
//! assert!(graph.has_edge(1, 0));
//! assert!(!graph.has_edge(1, 1));
//! assert!(!graph.has_edge(1, 2));
//! assert!(graph.has_edge(2, 0));
//! assert!(graph.has_edge(2, 1));
//! assert!(!graph.has_edge(2, 2));
//! ```
//!
//! [`HasEdgeSymmetric`]: crate::op::HasEdgeSymmetric

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

/// A trait to check if an edge exists from one vertex to another
///
/// # How can I implement `HasEdge`?
///
/// Provide an implementation of `has_edge` that returns `true` if there is an
/// edge from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::HasEdge,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl HasEdge for Graph {
///     fn has_edge(&self, s: usize, t: usize) -> bool {
///         self.edges.get(s).map_or(false, |set| set.contains(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::HasEdge,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([0, 1]),
/// ];
///
/// assert!(!graph.has_edge(0, 0));
/// assert!(graph.has_edge(0, 1));
/// assert!(graph.has_edge(0, 2));
/// assert!(graph.has_edge(1, 0));
/// assert!(!graph.has_edge(1, 1));
/// assert!(!graph.has_edge(1, 2));
/// assert!(graph.has_edge(2, 0));
/// assert!(graph.has_edge(2, 1));
/// assert!(!graph.has_edge(2, 2));
/// ```
///
/// # Properties
///
/// ## `HasEdge` and `AddEdge`
///
/// Types that also implement [`AddEdge`] should ensure that
/// [`add_edge_has_edge`] holds.
///
/// ## `HasEdge` and `AddWeightedEdge`
///
/// Types that also implement [`AddWeightedEdge`] should ensure that
/// [`add_weighted_edge_has_edge`] holds.
///
/// [`AddEdge`]: crate::op::AddEdge
/// [`AddWeightedEdge`]: crate::op::AddWeightedEdge
/// [`add_edge_has_edge`]: crate::prop::add_edge_has_edge
/// [`add_weighted_edge_has_edge`]: crate::prop::add_weighted_edge_has_edge
pub trait HasEdge {
    /// Returns whether an edge exists from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Panics
    ///
    /// Implementations may not panic if `s` or `t` are not in the graph.
    fn has_edge(&self, s: usize, t: usize) -> bool;
}

impl HasEdge for Vec<BTreeSet<usize>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> HasEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> HasEdge for Vec<BTreeMap<usize, W>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> HasEdge for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl HasEdge for [BTreeSet<usize>] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> HasEdge for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> HasEdge for [BTreeMap<usize, W>] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> HasEdge for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<const V: usize> HasEdge for [BTreeSet<usize>; V] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<const V: usize, H> HasEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<const V: usize, W> HasEdge for [BTreeMap<usize, W>; V] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<const V: usize, W, H> HasEdge for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl HasEdge for BTreeSet<(usize, usize)> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.contains(&(s, t))
    }
}

impl<H> HasEdge for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.contains(&(s, t))
    }
}

impl HasEdge for BTreeMap<usize, BTreeSet<usize>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> HasEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> HasEdge for BTreeMap<usize, BTreeMap<usize, W>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> HasEdge for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |map| map.contains_key(&t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_has_edge {
        ($graph:expr) => {
            assert!(!$graph.has_edge(0, 0));
            assert!($graph.has_edge(0, 1));
            assert!($graph.has_edge(0, 2));
            assert!($graph.has_edge(1, 0));
            assert!(!$graph.has_edge(1, 1));
            assert!(!$graph.has_edge(1, 2));
            assert!($graph.has_edge(2, 0));
            assert!($graph.has_edge(2, 1));
            assert!(!$graph.has_edge(2, 2));
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn btree_set() {
        let graph = BTreeSet::from([(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);

        test_has_edge!(graph);
    }

    #[test]
    fn hash_set() {
        let graph = HashSet::from([(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);

        test_has_edge!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0])),
            (2, BTreeSet::from([0, 1])),
        ]);

        test_has_edge!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([0, 1])),
        ]);

        test_has_edge!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 1), (2, 1)])),
            (1, BTreeMap::from([(0, 1)])),
            (2, BTreeMap::from([(0, 1), (1, 1)])),
        ]);

        test_has_edge!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(0, 1), (1, 1)])),
        ]);

        test_has_edge!(graph);
    }
}
