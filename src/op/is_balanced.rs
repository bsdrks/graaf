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
        IterVertices,
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
///         IterVertices,
///         Outdegree,
///     },
/// };
///
/// struct Graph<const V: usize> {
///     pub arcs: [BTreeSet<usize>; V],
/// }
///
/// impl<const V: usize> IsBalanced for Graph<V> {
///     fn is_balanced(&self) -> bool {
///         self.arcs
///             .iter_vertices()
///             .all(|s| self.arcs.indegree(s) == self.arcs.outdegree(s))
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
pub trait IsBalanced {
    /// Returns whether the graph is balanced.
    fn is_balanced(&self) -> bool;
}

impl IsBalanced for Vec<BTreeSet<usize>> {
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<H> IsBalanced for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl IsBalanced for [BTreeSet<usize>] {
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<H> IsBalanced for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<const V: usize> IsBalanced for [BTreeSet<usize>; V] {
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<const V: usize, H> IsBalanced for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<W> IsBalanced for Vec<BTreeMap<usize, W>> {
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<W, H> IsBalanced for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<W> IsBalanced for [BTreeMap<usize, W>] {
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<W, H> IsBalanced for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<const V: usize, W> IsBalanced for [BTreeMap<usize, W>; V] {
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<const V: usize, W, H> IsBalanced for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::{
            gen::{
                Empty,
                EmptyConst,
            },
            op::{
                AddArc,
                AddWeightedArc,
            },
        },
        alloc::collections::BTreeSet,
        std::collections::HashSet,
    };

    macro_rules! test_is_balanced_unweighted {
        ($graph:expr) => {
            assert!($graph.is_balanced());

            $graph.add_arc(0, 1);

            assert!(!$graph.is_balanced());

            $graph.add_arc(1, 0);

            assert!($graph.is_balanced());

            $graph.add_arc(0, 2);

            assert!(!$graph.is_balanced());

            $graph.add_arc(1, 2);

            assert!(!$graph.is_balanced());

            $graph.add_arc(2, 0);

            assert!(!$graph.is_balanced());

            $graph.add_arc(2, 1);

            assert!($graph.is_balanced());
        };
    }

    macro_rules! test_is_balanced_weighted {
        ($graph:expr) => {
            assert!($graph.is_balanced());

            $graph.add_weighted_arc(0, 1, 1);

            assert!(!$graph.is_balanced());

            $graph.add_weighted_arc(1, 0, -3);

            assert!($graph.is_balanced());

            $graph.add_weighted_arc(0, 2, 2);

            assert!(!$graph.is_balanced());

            $graph.add_weighted_arc(1, 2, 0);

            assert!(!$graph.is_balanced());

            $graph.add_weighted_arc(2, 0, 1);

            assert!(!$graph.is_balanced());

            $graph.add_weighted_arc(2, 1, 1);

            assert!($graph.is_balanced());
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = &mut <Vec<BTreeSet<usize>>>::empty(3);

        test_is_balanced_unweighted!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = &mut <Vec<HashSet<usize>>>::empty(3);

        test_is_balanced_unweighted!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(3);

        test_is_balanced_unweighted!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(3);

        test_is_balanced_unweighted!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph = &mut <[BTreeSet<usize>; 3]>::empty();

        test_is_balanced_unweighted!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph = &mut <[HashSet<usize>; 3]>::empty();

        test_is_balanced_unweighted!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = &mut <Vec<BTreeMap<usize, i32>>>::empty(3);

        test_is_balanced_weighted!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = &mut <Vec<HashMap<usize, i32>>>::empty(3);

        test_is_balanced_weighted!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, i32>] = &mut Vec::<BTreeMap<usize, i32>>::empty(3);

        test_is_balanced_weighted!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, i32>] = &mut Vec::<HashMap<usize, i32>>::empty(3);

        test_is_balanced_weighted!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = &mut <[BTreeMap<usize, i32>; 3]>::empty();

        test_is_balanced_weighted!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = &mut <[HashMap<usize, i32>; 3]>::empty();

        test_is_balanced_weighted!(graph);
    }
}
