//! A trait to determine whether a vertex in a directed graph is isolated
//!
//! A vertex is isolated if it has no incoming or outgoing arcs.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsIsolated,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::new(),
//!     HashSet::new(),
//! ];
//!
//! assert!(!graph.is_isolated(0));
//! assert!(!graph.is_isolated(1));
//! assert!(!graph.is_isolated(2));
//! assert!(graph.is_isolated(3));
//! ```

extern crate alloc;

use {
    super::{
        Indegree,
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

/// A trait to determine whether a vertex is isolated
///
/// # How can I implement `IsIsolated`?
///
/// Provide an implementation of `is_isolated` that returns `true` if the vertex
/// is isolated and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IsIsolated,
///         Outdegree,
///     },
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl IsIsolated for Graph {
///     fn is_isolated(&self, s: usize) -> bool {
///         self.arcs.indegree(s) == 0 && self.arcs.outdegree(s) == 0
///     }
/// }
///
/// let graph = Graph {
///     arcs: vec![
///         HashSet::from([1, 2]),
///         HashSet::from([0]),
///         HashSet::new(),
///         HashSet::new(),
///     ],
/// };
///
/// assert!(!graph.is_isolated(0));
/// assert!(!graph.is_isolated(1));
/// assert!(!graph.is_isolated(2));
/// assert!(graph.is_isolated(3));
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::IsIsolated,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![HashSet::from([1, 2]), HashSet::from([0]), HashSet::new()];
///
/// assert!(!graph.is_isolated(0));
/// assert!(!graph.is_isolated(1));
/// assert!(!graph.is_isolated(2));
/// assert!(graph.is_isolated(3));
/// ```
pub trait IsIsolated {
    /// Returns whether the vertex is isolated.
    fn is_isolated(&self, s: usize) -> bool;
}

impl IsIsolated for Vec<BTreeSet<usize>> {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<H> IsIsolated for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl IsIsolated for [BTreeSet<usize>] {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<H> IsIsolated for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<const V: usize> IsIsolated for [BTreeSet<usize>; V] {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<const V: usize, H> IsIsolated for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl IsIsolated for BTreeMap<usize, BTreeSet<usize>> {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<H> IsIsolated for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<W> IsIsolated for Vec<BTreeMap<usize, W>> {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<W, H> IsIsolated for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<W> IsIsolated for [BTreeMap<usize, W>] {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<W, H> IsIsolated for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<const V: usize, W> IsIsolated for [BTreeMap<usize, W>; V] {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<const V: usize, W, H> IsIsolated for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<W> IsIsolated for BTreeMap<usize, BTreeMap<usize, W>> {
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<W, H> IsIsolated for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

#[cfg(test)]
mod tests {
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
    };

    macro_rules! test_is_isolated {
        ($graph:expr) => {
            assert!(!$graph.is_isolated(0));
            assert!(!$graph.is_isolated(1));
            assert!(!$graph.is_isolated(2));
            assert!($graph.is_isolated(3));
        };
    }

    macro_rules! test_is_isolated_unweighted {
        ($graph:expr) => {
            $graph.add_arc(0, 1);
            $graph.add_arc(0, 2);
            $graph.add_arc(1, 2);

            test_is_isolated!($graph);
        };
    }

    macro_rules! test_is_isolated_weighted {
        ($graph:expr) => {
            $graph.add_weighted_arc(0, 1, 1);
            $graph.add_weighted_arc(0, 2, 1);
            $graph.add_weighted_arc(1, 2, 1);

            test_is_isolated!($graph);
        };
    }

    #[test]
    fn vec_btree_set() {
        let graph = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn vec_hash_set() {
        let graph = &mut Vec::<HashSet<usize>>::empty(4);

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(4);

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(4);

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn arr_btree_set() {
        let graph: &mut [BTreeSet<usize>; 4] = &mut <[BTreeSet<usize>; 4]>::empty();

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn arr_hash_set() {
        let graph: &mut [HashSet<usize>; 4] = &mut <[HashSet<usize>; 4]>::empty();

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn btree_map_btree_set() {
        let graph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = &mut HashMap::<usize, HashSet<usize>>::empty(4);

        test_is_isolated_unweighted!(graph);
    }

    #[test]
    fn vec_btree_map() {
        let graph = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_is_isolated_weighted!(graph);
    }

    #[test]
    fn vec_hash_map() {
        let graph = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_is_isolated_weighted!(graph);
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, i32>] = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        test_is_isolated_weighted!(graph);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, i32>] = &mut Vec::<HashMap<usize, i32>>::empty(4);

        test_is_isolated_weighted!(graph);
    }

    #[test]
    fn arr_btree_map() {
        let graph = &mut <[BTreeMap<usize, i32>; 4]>::empty();

        test_is_isolated_weighted!(graph);
    }

    #[test]
    fn arr_hash_map() {
        let graph = &mut <[HashMap<usize, i32>; 4]>::empty();

        test_is_isolated_weighted!(graph);
    }

    #[test]
    fn btree_map_btree_map() {
        let graph = &mut BTreeMap::<usize, BTreeMap<usize, i32>>::empty(4);

        test_is_isolated_weighted!(graph);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = &mut HashMap::<usize, HashMap<usize, i32>>::empty(4);

        test_is_isolated_weighted!(graph);
    }
}
