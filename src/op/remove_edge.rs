//! A trait to remove an edge from a graph
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::RemoveEdge,
//!     std::collections::HashSet,
//! };
//!
//! let mut graph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([1]),
//! ];
//!
//! assert!(graph.remove_edge(0, 1));
//!
//! assert_eq!(
//!     graph,
//!     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! assert!(graph.remove_edge(0, 2));
//!
//! assert_eq!(
//!     graph,
//!     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! assert!(graph.remove_edge(1, 0));
//!
//! assert_eq!(
//!     graph,
//!     vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
//! );
//!
//! graph.remove_edge(2, 1);
//!
//! assert_eq!(graph, vec![HashSet::new(), HashSet::new(), HashSet::new()]);
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

/// A trait to remove an edge from a graph
///
/// # How can I implement `RemoveEdge`?
///
/// Provide an implementation of `remove_edge` that removes the edge from `s` to
/// `t`.
///
/// ```
/// use {
///     graaf::op::RemoveEdge,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl RemoveEdge for Graph {
///     fn remove_edge(&mut self, s: usize, t: usize) -> bool {
///         self.edges[s].remove(&t)
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::RemoveEdge,
///     std::collections::HashSet,
/// };
///
/// let mut graph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([1]),
/// ];
///
/// assert!(graph.remove_edge(0, 1));
///
/// assert_eq!(
///     graph,
///     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// assert!(graph.remove_edge(0, 2));
///
/// assert_eq!(
///     graph,
///     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// assert!(graph.remove_edge(1, 0));
///
/// assert_eq!(
///     graph,
///     vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
/// );
///
/// graph.remove_edge(2, 1);
///
/// assert_eq!(graph, vec![HashSet::new(), HashSet::new(), HashSet::new()]);
/// ```
///
/// # Properties
///
/// ## `RemoveEdge` and `AddEdge`
///
/// Types that also implement [`AddEdge`] should ensure that
/// [`add_edge_remove_edge`] holds.
///
/// ## `RemoveEdge` and `AddWeightedEdge`
///
/// Types that also implement [`AddWeightedEdge`] should ensure that
/// [`add_weighted_edge_remove_edge`] holds.
///
/// [`AddEdge`]: crate::op::AddEdge
/// [`AddWeightedEdge`]: crate::op::AddWeightedEdge
/// [`add_edge_remove_edge`]: crate::prop::add_edge_remove_edge
/// [`add_weighted_edge_remove_edge`]: crate::prop::add_weighted_edge_remove_edge
pub trait RemoveEdge {
    /// Removes the edge from `s` to `t`. Returns whether the edge was removed.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool;
}

impl RemoveEdge for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<H> RemoveEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<W> RemoveEdge for Vec<BTreeMap<usize, W>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<W, H> RemoveEdge for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl RemoveEdge for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<H> RemoveEdge for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<W> RemoveEdge for [BTreeMap<usize, W>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<W, H> RemoveEdge for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<const V: usize> RemoveEdge for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<const V: usize, H> RemoveEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<const V: usize, W> RemoveEdge for [BTreeMap<usize, W>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<const V: usize, W, H> RemoveEdge for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl RemoveEdge for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s).unwrap().remove(&t)
    }
}

impl<H> RemoveEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s).unwrap().remove(&t)
    }
}

impl<W> RemoveEdge for BTreeMap<usize, BTreeMap<usize, W>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s).unwrap().remove(&t).is_some()
    }
}

impl<W, H> RemoveEdge for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s).unwrap().remove(&t).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_btree_set() {
        let mut graph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
        ];

        assert_eq!(
            graph,
            vec![
                BTreeSet::from([1, 2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![
                BTreeSet::from([2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![BTreeSet::new(), BTreeSet::from([0]), BTreeSet::from([1])]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::from([1])]
        );
    }

    #[test]
    fn vec_btree_map() {
        let mut graph = vec![
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        assert_eq!(
            graph,
            vec![
                BTreeMap::from([(1, 1), (2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![
                BTreeMap::from([(2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![
                BTreeMap::new(),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            vec![BTreeMap::new(), BTreeMap::new(), BTreeMap::from([(1, 1)])]
        );
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<usize>] =
            &mut [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert_eq!(
            graph,
            &[BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![BTreeSet::from([2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![BTreeSet::new(), BTreeSet::from([2]), BTreeSet::new()]
        );

        assert!(graph.remove_edge(1, 2));

        assert_eq!(
            graph,
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]
        );
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, i32>] = &mut [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        assert_eq!(
            graph,
            &[
                BTreeMap::from([(1, 1), (2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![
                BTreeMap::from([(2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![
                BTreeMap::new(),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            vec![BTreeMap::new(), BTreeMap::new(), BTreeMap::from([(1, 1)])]
        );
    }

    #[test]
    fn arr_btree_set() {
        let mut graph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
        ];

        assert_eq!(
            graph,
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            [
                BTreeSet::from([2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            [BTreeSet::new(), BTreeSet::from([0]), BTreeSet::from([1])]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            [BTreeSet::new(), BTreeSet::new(), BTreeSet::from([1])]
        );
    }

    #[test]
    fn arr_btree_map() {
        let mut graph = [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        assert_eq!(
            graph,
            [
                BTreeMap::from([(1, 1), (2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            [
                BTreeMap::from([(2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            [
                BTreeMap::new(),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            [BTreeMap::new(), BTreeMap::new(), BTreeMap::from([(1, 1)])]
        );
    }

    #[test]
    fn btree_map_btree_set() {
        let mut graph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0])),
            (2, BTreeSet::from([1])),
        ]);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([0])),
                (2, BTreeSet::from([1]))
            ])
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([2])),
                (1, BTreeSet::from([0])),
                (2, BTreeSet::from([1]))
            ])
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::from([0])),
                (2, BTreeSet::from([1]))
            ])
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::new()),
                (2, BTreeSet::from([1]))
            ])
        );
    }

    #[test]
    fn btree_map_btree_map() {
        let mut graph = BTreeMap::from([
            (0, BTreeMap::from([(1, 1), (2, 1)])),
            (1, BTreeMap::from([(0, 1)])),
            (2, BTreeMap::from([(1, 1)])),
        ]);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 1), (2, 1)])),
                (1, BTreeMap::from([(0, 1)])),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::from([(2, 1)])),
                (1, BTreeMap::from([(0, 1)])),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::new()),
                (1, BTreeMap::from([(0, 1)])),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::new()),
                (1, BTreeMap::new()),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );
    }

    #[test]
    fn vec_hash_set() {
        let mut graph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        assert_eq!(
            graph,
            vec![
                HashSet::from([1, 2]),
                HashSet::from([0]),
                HashSet::from([1])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
        );
    }

    #[test]
    fn vec_hash_map() {
        let mut graph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(
            graph,
            vec![
                HashMap::from([(1, 1), (2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            vec![HashMap::new(), HashMap::new(), HashMap::from([(1, 1)])]
        );
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize>] =
            &mut [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert_eq!(
            graph,
            &[HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![HashSet::from([2]), HashSet::from([2]), HashSet::new()]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![HashSet::new(), HashSet::from([2]), HashSet::new()]
        );

        assert!(graph.remove_edge(1, 2));

        assert_eq!(graph, vec![HashSet::new(), HashSet::new(), HashSet::new()]);
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, i32>] = &mut [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(
            graph,
            &[
                HashMap::from([(1, 1), (2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            vec![
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            vec![
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            vec![HashMap::new(), HashMap::new(), HashMap::from([(1, 1)])]
        );
    }

    #[test]
    fn arr_hash_set() {
        let mut graph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        assert_eq!(
            graph,
            [
                HashSet::from([1, 2]),
                HashSet::from([0]),
                HashSet::from([1])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            [HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            [HashSet::new(), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(graph, [HashSet::new(), HashSet::new(), HashSet::from([1])]);
    }

    #[test]
    fn arr_hash_map() {
        let mut graph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(
            graph,
            [
                HashMap::from([(1, 1), (2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            [
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            [
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            [HashMap::new(), HashMap::new(), HashMap::from([(1, 1)])]
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut graph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([1])),
        ]);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([2])),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::new()),
                (2, HashSet::from([1]))
            ])
        );
    }

    #[test]
    fn hash_map_hash_map() {
        let mut graph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(1, 1)])),
        ]);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 1), (2, 1)])),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        assert!(graph.remove_edge(0, 1));

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(2, 1)])),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        assert!(graph.remove_edge(0, 2));

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::new()),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        assert!(graph.remove_edge(1, 0));

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::new()),
                (1, HashMap::new()),
                (2, HashMap::from([(1, 1)]))
            ])
        );
    }
}
