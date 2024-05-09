//! A trait to check if a an edge exists from `s` to `t` and from `t` to `s`
//!
//! To check only if an edge exists from `s` to `t`, see [`HasEdge`].
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::HasEdgeSymmetric,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![HashSet::from([1]), HashSet::from([0])];
//!
//! assert!(graph.has_edge_symmetric(0, 1));
//! assert!(graph.has_edge_symmetric(1, 0));
//!
//! let graph = vec![HashSet::from([1]), HashSet::new()];
//!
//! assert!(!graph.has_edge_symmetric(0, 1));
//! assert!(!graph.has_edge_symmetric(1, 0));
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([2]),
//!     HashSet::from([0]),
//! ];
//!
//! assert!(!graph.has_edge_symmetric(0, 1));
//! assert!(graph.has_edge_symmetric(0, 2));
//! assert!(!graph.has_edge_symmetric(1, 0));
//! assert!(!graph.has_edge_symmetric(1, 2));
//! assert!(graph.has_edge_symmetric(2, 0));
//! assert!(!graph.has_edge_symmetric(2, 1));
//! ```
//!
//! [`HasEdge`]: crate::op::HasEdge

extern crate alloc;

use {
    super::HasEdge,
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

/// A trait to check if a an edge exists from `s` to `t` and from `t` to `s`
///
/// # How can I implement `HasEdgeSymmetric`?
///
/// Provide an implementation of `has_edge_symmetric` that returns `true` if the
/// graph has an edge from `s` to `t` and from `t` to `s` and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::HasEdgeSymmetric,
/// };
///
/// struct Graph {
///     edges: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasEdgeSymmetric for Graph {
///     fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
///         self.edges[s].contains(&t) && self.edges[t].contains(&s)
///     }
/// }
///
/// let graph = Graph {
///     edges: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(graph.has_edge_symmetric(0, 1));
/// assert!(graph.has_edge_symmetric(1, 0));
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::HasEdgeSymmetric,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![HashSet::from([1]), HashSet::from([0])];
///
/// assert!(graph.has_edge_symmetric(0, 1));
/// assert!(graph.has_edge_symmetric(1, 0));
///
/// let graph = vec![HashSet::from([1]), HashSet::new()];
///
/// assert!(!graph.has_edge_symmetric(0, 1));
/// assert!(!graph.has_edge_symmetric(1, 0));
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([2]),
///     HashSet::from([0]),
/// ];
///
/// assert!(!graph.has_edge_symmetric(0, 1));
/// assert!(graph.has_edge_symmetric(0, 2));
/// assert!(!graph.has_edge_symmetric(1, 0));
/// assert!(!graph.has_edge_symmetric(1, 2));
/// assert!(graph.has_edge_symmetric(2, 0));
/// assert!(!graph.has_edge_symmetric(2, 1));
/// ```
pub trait HasEdgeSymmetric {
    /// Returns whether the graph has an edge from `s` to `t` and from `t` to
    /// `s`
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool;
}

impl HasEdgeSymmetric for Vec<BTreeSet<usize>> {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<H> HasEdgeSymmetric for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl HasEdgeSymmetric for [BTreeSet<usize>] {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<H> HasEdgeSymmetric for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<const V: usize> HasEdgeSymmetric for [BTreeSet<usize>; V] {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<const V: usize, H> HasEdgeSymmetric for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl HasEdgeSymmetric for BTreeMap<usize, BTreeSet<usize>> {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<H> HasEdgeSymmetric for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<W> HasEdgeSymmetric for Vec<BTreeMap<usize, W>> {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<W, H> HasEdgeSymmetric for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<W> HasEdgeSymmetric for [BTreeMap<usize, W>] {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<W, H> HasEdgeSymmetric for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<const V: usize, W> HasEdgeSymmetric for [BTreeMap<usize, W>; V] {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<const V: usize, W, H> HasEdgeSymmetric for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<W> HasEdgeSymmetric for BTreeMap<usize, BTreeMap<usize, W>> {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<W, H> HasEdgeSymmetric for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl HasEdgeSymmetric for BTreeSet<(usize, usize)> {
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

impl<H> HasEdgeSymmetric for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn has_edge_symmetric(&self, s: usize, t: usize) -> bool {
        self.has_edge(s, t) && self.has_edge(t, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_has_edge {
        ($graph:expr) => {
            assert!(!$graph.has_edge_symmetric(0, 1));
            assert!($graph.has_edge_symmetric(0, 2));
            assert!(!$graph.has_edge_symmetric(1, 0));
            assert!(!$graph.has_edge_symmetric(1, 2));
            assert!($graph.has_edge_symmetric(2, 0));
            assert!(!$graph.has_edge_symmetric(2, 1));
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 4)]),
            BTreeMap::from([(0, 7)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 4)]),
            HashMap::from([(0, 7)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, usize>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 4)]),
            BTreeMap::from([(0, 7)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, usize>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 4)]),
            HashMap::from([(0, 7)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 4)]),
            BTreeMap::from([(0, 7)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 4)]),
            HashMap::from([(0, 7)]),
        ];

        test_has_edge!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([2])),
            (2, BTreeSet::from([0])),
        ]);

        test_has_edge!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([2])),
            (2, HashSet::from([0])),
        ]);

        test_has_edge!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3)])),
            (1, BTreeMap::from([(2, 4)])),
            (2, BTreeMap::from([(0, 7)])),
        ]);

        test_has_edge!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3)])),
            (1, HashMap::from([(2, 4)])),
            (2, HashMap::from([(0, 7)])),
        ]);

        test_has_edge!(graph);
    }

    #[test]
    fn btree_set_tuple() {
        let graph = BTreeSet::from([(0, 1), (0, 2), (1, 2), (2, 0)]);

        test_has_edge!(graph);
    }

    #[test]
    fn hash_set_tuple() {
        let graph = HashSet::from([(0, 1), (0, 2), (1, 2), (2, 0)]);

        test_has_edge!(graph);
    }
}
