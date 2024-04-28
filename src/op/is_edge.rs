//! A trait to check if an edge exists between two vertices
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsEdge,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([0, 1]),
//! ];
//!
//! assert!(!graph.is_edge(0, 0));
//! assert!(graph.is_edge(0, 1));
//! assert!(graph.is_edge(0, 2));
//! assert!(graph.is_edge(1, 0));
//! assert!(!graph.is_edge(1, 1));
//! assert!(!graph.is_edge(1, 2));
//! assert!(graph.is_edge(2, 0));
//! assert!(graph.is_edge(2, 1));
//! assert!(!graph.is_edge(2, 2));
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

/// A trait to check if an edge exists between two vertices
///
/// # How can I implement `IsEdge`?
///
/// Provide an implementation of `is_edge` that returns `true` if there is an
/// edge from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::IsEdge,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl IsEdge for Graph {
///     fn is_edge(&self, s: usize, t: usize) -> bool {
///         self.edges.get(s).map_or(false, |set| set.contains(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::IsEdge,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([0, 1]),
/// ];
///
/// assert!(!graph.is_edge(0, 0));
/// assert!(graph.is_edge(0, 1));
/// assert!(graph.is_edge(0, 2));
/// assert!(graph.is_edge(1, 0));
/// assert!(!graph.is_edge(1, 1));
/// assert!(!graph.is_edge(1, 2));
/// assert!(graph.is_edge(2, 0));
/// assert!(graph.is_edge(2, 1));
/// assert!(!graph.is_edge(2, 2));
/// ```
///
/// # Properties
///
/// ## `IsEdge` and `AddEdge`
///
/// Types that also implement [`AddEdge`](crate::op::AddEdge) should ensure that
/// [`add_edge_is_edge`](crate::prop::add_edge_is_edge) holds.
///
/// ## `IsEdge` and `AddWeightedEdge`
///
/// Types that also implement [`AddWeightedEdge`](crate::op::AddWeightedEdge)
/// should ensure that
/// [`add_weighted_edge_is_edge`](crate::prop::add_weighted_edge_is_edge)
/// holds.
pub trait IsEdge {
    /// Check if there is an edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Panics
    ///
    /// Implementations may not panic if `s` or `t` are not in the graph.
    fn is_edge(&self, s: usize, t: usize) -> bool;
}

impl IsEdge for Vec<BTreeSet<usize>> {
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> IsEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> IsEdge for Vec<BTreeMap<usize, W>> {
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> IsEdge for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl IsEdge for [BTreeSet<usize>] {
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> IsEdge for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> IsEdge for [BTreeMap<usize, W>]
where
    W: Ord,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> IsEdge for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<const V: usize> IsEdge for [BTreeSet<usize>; V] {
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<const V: usize, H> IsEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |set| set.contains(&t))
    }
}

impl<const V: usize, W> IsEdge for [BTreeMap<usize, W>; V]
where
    W: Ord,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<const V: usize, W, H> IsEdge for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(s).map_or(false, |map| map.contains_key(&t))
    }
}

impl IsEdge for BTreeSet<(usize, usize)> {
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.contains(&(s, t))
    }
}

impl<H> IsEdge for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.contains(&(s, t))
    }
}

impl IsEdge for BTreeMap<usize, BTreeSet<usize>> {
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |set| set.contains(&t))
    }
}

impl<H> IsEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |set| set.contains(&t))
    }
}

impl<W> IsEdge for BTreeMap<usize, BTreeMap<usize, W>> {
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |map| map.contains_key(&t))
    }
}

impl<W, H> IsEdge for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self.get(&s).map_or(false, |map| map.contains_key(&t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_is_edge {
        ($graph:expr) => {
            assert!(!$graph.is_edge(0, 0));
            assert!($graph.is_edge(0, 1));
            assert!($graph.is_edge(0, 2));
            assert!($graph.is_edge(1, 0));
            assert!(!$graph.is_edge(1, 1));
            assert!(!$graph.is_edge(1, 2));
            assert!($graph.is_edge(2, 0));
            assert!($graph.is_edge(2, 1));
            assert!(!$graph.is_edge(2, 2));
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, i32>] = &[
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, i32>] = &[
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([0, 1]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([0, 1]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(0, 1), (1, 1)]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(0, 1), (1, 1)]),
        ];

        test_is_edge!(graph);
    }

    #[test]
    fn btree_set() {
        let graph = BTreeSet::from([(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);

        test_is_edge!(graph);
    }

    #[test]
    fn hash_set() {
        let graph = HashSet::from([(0, 1), (0, 2), (1, 0), (2, 0), (2, 1)]);

        test_is_edge!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0])),
            (2, BTreeSet::from([0, 1])),
        ]);

        test_is_edge!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([0, 1])),
        ]);

        test_is_edge!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 1), (2, 1)])),
            (1, BTreeMap::from([(0, 1)])),
            (2, BTreeMap::from([(0, 1), (1, 1)])),
        ]);

        test_is_edge!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(0, 1), (1, 1)])),
        ]);

        test_is_edge!(graph);
    }
}
