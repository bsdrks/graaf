#![doc(alias = "isograph")]
#![doc(alias = "pseudosymmetric")]
//! A trait to determine whether a directed graph is balanced
//!
//! A directed graph is balanced if the indegree of each vertex is equal to its
//! outdegree.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::op::IsBalanced,
//!     std::collections::HashSet,
//! };
//!
//! let graph: [BTreeSet<usize>; 3] = [
//!     BTreeSet::from([1, 2]),
//!     BTreeSet::from([0, 2]),
//!     BTreeSet::from([0, 1]),
//! ];
//!
//! assert!(graph.is_balanced());
//!
//! let graph: [BTreeSet<usize>; 3] = [
//!     BTreeSet::from([1, 2]),
//!     BTreeSet::from([0, 2]),
//!     BTreeSet::from([0]),
//! ];
//!
//! assert!(!graph.is_balanced());
//! ```

extern crate alloc;

use {
    super::{
        Indegree,
        IterAllEdges,
        IterAllWeightedEdges,
        Outdegree,
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

/// A trait to determine whether a graph is balanced
///
/// # How can I implement `IsBalanced`?
///
/// Provide an implementation of `is_balanced` that returns `true` if the graph
/// is balanced and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::{
///         Indegree,
///         IsBalanced,
///         IterAllEdges,
///         Outdegree,
///     },
/// };
///
/// struct Graph<const V: usize> {
///     pub edges: [BTreeSet<usize>; V],
/// }
///
/// impl<const V: usize> IsBalanced for Graph<V> {
///     fn is_balanced(&self) -> bool {
///         self.edges
///             .iter_all_edges()
///             .all(|(s, t)| self.edges.indegree(t) == self.edges.outdegree(s))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::IsBalanced,
/// };
///
/// let graph: [BTreeSet<usize>; 3] = [
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0, 1]),
/// ];
///
/// assert!(graph.is_balanced());
///
/// let graph: [BTreeSet<usize>; 3] = [
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0]),
/// ];
///
/// assert!(!graph.is_balanced());
/// ```
#[doc(alias = "isograph")]
#[doc(alias = "pseudosymmetric")]
pub trait IsBalanced {
    /// Returns whether the graph is balanced.
    fn is_balanced(&self) -> bool;
}

impl IsBalanced for Vec<BTreeSet<usize>> {
    fn is_balanced(&self) -> bool {
        self.iter_all_edges()
            .all(|(s, t)| self.indegree(t) == self.outdegree(s))
    }
}

impl<H> IsBalanced for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_all_edges()
            .all(|(s, t)| self.indegree(t) == self.outdegree(s))
    }
}

impl<const V: usize> IsBalanced for [BTreeSet<usize>; V] {
    fn is_balanced(&self) -> bool {
        self.iter_all_edges()
            .all(|(s, t)| self.indegree(t) == self.outdegree(s))
    }
}

impl<const V: usize, H> IsBalanced for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_all_edges()
            .all(|(s, t)| self.indegree(t) == self.outdegree(s))
    }
}

impl<W> IsBalanced for Vec<BTreeMap<usize, W>> {
    fn is_balanced(&self) -> bool {
        self.iter_all_weighted_edges()
            .all(|(s, t, _)| self.indegree(t) == self.outdegree(s))
    }
}

impl<W, H> IsBalanced for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_all_weighted_edges()
            .all(|(s, t, _)| self.indegree(t) == self.outdegree(s))
    }
}

impl<const V: usize, W> IsBalanced for [BTreeMap<usize, W>; V] {
    fn is_balanced(&self) -> bool {
        self.iter_all_weighted_edges()
            .all(|(s, t, _)| self.indegree(t) == self.outdegree(s))
    }
}

impl<const V: usize, W, H> IsBalanced for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_all_weighted_edges()
            .all(|(s, t, _)| self.indegree(t) == self.outdegree(s))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        alloc::collections::BTreeSet,
        std::collections::HashSet,
    };

    #[test]
    fn vec_btree_set() {
        let graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2]),
            BTreeSet::from([0, 1]),
        ];

        assert!(graph.is_balanced());
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0, 2]),
            HashSet::from([0, 1]),
        ];

        assert!(graph.is_balanced());
    }

    #[test]
    fn arr_btree_set() {
        let graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0, 2]),
            BTreeSet::from([0, 1]),
        ];

        assert!(graph.is_balanced());
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([1, 2]),
            HashSet::from([0, 2]),
            HashSet::from([0, 1]),
        ];

        assert!(graph.is_balanced());
    }

    #[test]
    fn vec_btree_map() {
        let graph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 2), (2, 3)]),
            BTreeMap::from([(0, 1), (1, 2)]),
        ];

        assert!(graph.is_balanced());
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 2), (2, 3)]),
            HashMap::from([(0, 1), (1, 2)]),
        ];

        assert!(graph.is_balanced());
    }

    #[test]
    fn arr_btree_map() {
        let graph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(0, 2), (2, 3)]),
            BTreeMap::from([(0, 1), (1, 2)]),
        ];

        assert!(graph.is_balanced());
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(0, 2), (2, 3)]),
            HashMap::from([(0, 1), (1, 2)]),
        ];

        assert!(graph.is_balanced());
    }
}
