//! A trait to determine whether a directed graph is symmetric
//!
//! A directed graph is symmetric if for every arc `(s, t)` there is an arc
//! `(t, s)`.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::op::IsSymmetric,
//! };
//!
//! let graph = vec![BTreeSet::from([1]), BTreeSet::from([0])];
//!
//! assert!(graph.is_symmetric());
//!
//! let graph = vec![BTreeSet::from([1]), BTreeSet::new()];
//!
//! assert!(!graph.is_symmetric());
//!
//! let graph = vec![
//!     BTreeSet::from([1, 2]),
//!     BTreeSet::from([2]),
//!     BTreeSet::from([0]),
//! ];
//!
//! assert!(!graph.is_symmetric());
//! ```

extern crate alloc;

use {
    super::{
        HasArc,
        IterAllArcs,
        IterAllWeightedArcs,
    },
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

/// A trait to determine whether a directed graph is symmetric
///
/// # How can I implement `IsSymmetric`?
///
/// Provide an implementation of `is_symmetric` that returns `true` if the graph
/// is symmetric and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::{
///         HasArc,
///         IsSymmetric,
///         IterAllArcs,
///     },
/// };
///
/// struct Graph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsSymmetric for Graph {
///     fn is_symmetric(&self) -> bool {
///         self.arcs
///             .iter_all_arcs()
///             .all(|(s, t)| self.arcs.has_arc(t, s))
///     }
/// }
///
/// let graph = Graph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(graph.is_symmetric());
///
/// let graph = Graph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new()],
/// };
///
/// assert!(!graph.is_symmetric());
///
/// let graph = Graph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(!graph.is_symmetric());
/// ```
///
/// # Examples
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::IsSymmetric,
/// };
///
/// let graph = vec![BTreeSet::from([1]), BTreeSet::from([0])];
///
/// assert!(graph.is_symmetric());
///
/// let graph = vec![BTreeSet::from([1]), BTreeSet::new()];
///
/// assert!(!graph.is_symmetric());
///
/// let graph = vec![
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([2]),
///     BTreeSet::from([0]),
/// ];
///
/// assert!(!graph.is_symmetric());
/// ```
pub trait IsSymmetric {
    /// Returns whether the graph is symmetric
    fn is_symmetric(&self) -> bool;
}

impl IsSymmetric for Vec<BTreeSet<usize>> {
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<H> IsSymmetric for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl IsSymmetric for [BTreeSet<usize>] {
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<H> IsSymmetric for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<const V: usize> IsSymmetric for [BTreeSet<usize>; V] {
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<const V: usize, H> IsSymmetric for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl IsSymmetric for BTreeMap<usize, BTreeSet<usize>> {
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<H> IsSymmetric for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<W> IsSymmetric for Vec<BTreeMap<usize, W>> {
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W, H> IsSymmetric for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W> IsSymmetric for [BTreeMap<usize, W>] {
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W, H> IsSymmetric for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<const V: usize, W> IsSymmetric for [BTreeMap<usize, W>; V] {
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<const V: usize, W, H> IsSymmetric for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W> IsSymmetric for BTreeMap<usize, BTreeMap<usize, W>> {
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W, H> IsSymmetric for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_btree_set() {
        let graph = vec![BTreeSet::from([1]), BTreeSet::from([0])];

        assert!(graph.is_symmetric());

        let graph = vec![BTreeSet::from([1]), BTreeSet::new()];

        assert!(!graph.is_symmetric());

        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![HashSet::from([1]), HashSet::from([0])];

        assert!(graph.is_symmetric());

        let graph = vec![HashSet::from([1]), HashSet::new()];

        assert!(!graph.is_symmetric());

        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn slice_btree_set() {
        let graph: &[BTreeSet<usize>] = &[BTreeSet::from([1]), BTreeSet::from([0])];

        assert!(graph.is_symmetric());

        let graph: &[BTreeSet<usize>] = &[BTreeSet::from([1]), BTreeSet::new()];

        assert!(!graph.is_symmetric());

        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[HashSet::from([1]), HashSet::from([0])];

        assert!(graph.is_symmetric());

        let graph: &[HashSet<usize>] = &[HashSet::from([1]), HashSet::new()];

        assert!(!graph.is_symmetric());

        let graph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn arr_btree_set() {
        let graph = [BTreeSet::from([1]), BTreeSet::from([0])];

        assert!(graph.is_symmetric());

        let graph = [BTreeSet::from([1]), BTreeSet::new()];

        assert!(!graph.is_symmetric());

        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn arr_hash_set() {
        let graph = [HashSet::from([1]), HashSet::from([0])];

        assert!(graph.is_symmetric());

        let graph = [HashSet::from([1]), HashSet::new()];

        assert!(!graph.is_symmetric());

        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::from([0]))]);

        assert!(graph.is_symmetric());

        let graph = BTreeMap::from([(0, BTreeSet::from([1])), (1, BTreeSet::new())]);

        assert!(!graph.is_symmetric());

        let graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([2])),
            (2, BTreeSet::from([0])),
        ]);

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([(0, HashSet::from([1])), (1, HashSet::from([0]))]);

        assert!(graph.is_symmetric());

        let graph = HashMap::from([(0, HashSet::from([1])), (1, HashSet::new())]);

        assert!(!graph.is_symmetric());

        let graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([2])),
            (2, HashSet::from([0])),
        ]);

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![BTreeMap::from([(1, 2)]), BTreeMap::from([(0, 1)])];

        assert!(graph.is_symmetric());

        let graph = vec![BTreeMap::from([(1, 2)]), BTreeMap::new()];

        assert!(!graph.is_symmetric());

        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![HashMap::from([(1, 2)]), HashMap::from([(0, 1)])];

        assert!(graph.is_symmetric());

        let graph = vec![HashMap::from([(1, 2)]), HashMap::new()];

        assert!(!graph.is_symmetric());

        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn slice_btree_map() {
        let graph: &[BTreeMap<usize, usize>] =
            &[BTreeMap::from([(1, 2)]), BTreeMap::from([(0, 1)])];

        assert!(graph.is_symmetric());

        let graph: &[BTreeMap<usize, usize>] = &[BTreeMap::from([(1, 2)]), BTreeMap::new()];

        assert!(!graph.is_symmetric());

        let graph: &[BTreeMap<usize, usize>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn slice_hash_map() {
        let graph: &[HashMap<usize, usize>] = &[HashMap::from([(1, 2)]), HashMap::from([(0, 1)])];

        assert!(graph.is_symmetric());

        let graph: &[HashMap<usize, usize>] = &[HashMap::from([(1, 2)]), HashMap::new()];

        assert!(!graph.is_symmetric());

        let graph: &[HashMap<usize, usize>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn arr_btree_map() {
        let graph = [BTreeMap::from([(1, 2)]), BTreeMap::from([(0, 1)])];

        assert!(graph.is_symmetric());

        let graph = [BTreeMap::from([(1, 2)]), BTreeMap::new()];

        assert!(!graph.is_symmetric());

        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 4)]),
            BTreeMap::from([(0, 7), (1, 8)]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn arr_hash_map() {
        let graph = [HashMap::from([(1, 2)]), HashMap::from([(0, 1)])];

        assert!(graph.is_symmetric());

        let graph = [HashMap::from([(1, 2)]), HashMap::new()];

        assert!(!graph.is_symmetric());

        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 4)]),
            HashMap::from([(0, 7), (1, 8)]),
        ];

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = BTreeMap::from([(0, BTreeMap::from([(1, 2)])), (1, BTreeMap::from([(0, 1)]))]);

        assert!(graph.is_symmetric());

        let graph = BTreeMap::from([(0, BTreeMap::from([(1, 2)])), (1, BTreeMap::new())]);

        assert!(!graph.is_symmetric());

        let graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3)])),
            (1, BTreeMap::from([(0, 4)])),
            (2, BTreeMap::from([(0, 7), (1, 8)])),
        ]);

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([(0, HashMap::from([(1, 2)])), (1, HashMap::from([(0, 1)]))]);

        assert!(graph.is_symmetric());

        let graph = HashMap::from([(0, HashMap::from([(1, 2)])), (1, HashMap::new())]);

        assert!(!graph.is_symmetric());

        let graph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3)])),
            (1, HashMap::from([(0, 4)])),
            (2, HashMap::from([(0, 7), (1, 8)])),
        ]);

        assert!(!graph.is_symmetric());
    }
}
