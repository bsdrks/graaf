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
//! graph.remove_edge(0, 1);
//!
//! assert_eq!(
//!     graph,
//!     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! graph.remove_edge(0, 2);
//!
//! assert_eq!(
//!     graph,
//!     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! graph.remove_edge(1, 0);
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

use {
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
///     fn remove_edge(&mut self, s: usize, t: usize) {
///         let _ = self.edges[s].remove(&t);
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
/// graph.remove_edge(0, 1);
///
/// assert_eq!(
///     graph,
///     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// graph.remove_edge(0, 2);
///
/// assert_eq!(
///     graph,
///     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// graph.remove_edge(1, 0);
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
/// Types that also implement [`crate::op::AddEdge`] should ensure that
/// [`crate::op::prop::add_edge_remove_edge`] holds.
///
/// ## `RemoveEdge` and `AddWeightedEdge`
///
/// Types that also implement [`crate::op::AddWeightedEdge`] should ensure that
/// [`crate::op::prop::add_weighted_edge_remove_edge`] holds.
///
/// ## `RemoveEdge` and `IsEdge`
///
/// Types that also implement [`crate::op::IsEdge`] should ensure that
/// [`crate::op::prop::remove_edge_is_edge`] holds.
pub trait RemoveEdge {
    /// Remove the edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    fn remove_edge(&mut self, s: usize, t: usize);
}

// Vec

impl<H> RemoveEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].remove(&t);
    }
}

impl<H, W> RemoveEdge for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].remove(&t);
    }
}

// Arr

impl<const V: usize, H> RemoveEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].remove(&t);
    }
}

impl<const V: usize, H, W> RemoveEdge for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].remove(&t);
    }
}

// HashMap

impl<H> RemoveEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) {
        let _ = self.get_mut(&s).unwrap().remove(&t);
    }
}

impl<H, W> RemoveEdge for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn remove_edge(&mut self, s: usize, t: usize) {
        let _ = self.get_mut(&s).unwrap().remove(&t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        graph.remove_edge(0, 1);

        assert_eq!(
            graph,
            vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
        );

        graph.remove_edge(0, 2);

        assert_eq!(
            graph,
            vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
        );

        graph.remove_edge(1, 0);

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

        graph.remove_edge(0, 1);

        assert_eq!(
            graph,
            vec![
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        graph.remove_edge(0, 2);

        assert_eq!(
            graph,
            vec![
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        graph.remove_edge(1, 0);

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

        graph.remove_edge(0, 1);

        assert_eq!(
            graph,
            [HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
        );

        graph.remove_edge(0, 2);

        assert_eq!(
            graph,
            [HashSet::new(), HashSet::from([0]), HashSet::from([1])]
        );

        graph.remove_edge(1, 0);

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

        graph.remove_edge(0, 1);

        assert_eq!(
            graph,
            [
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        graph.remove_edge(0, 2);

        assert_eq!(
            graph,
            [
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        graph.remove_edge(1, 0);

        assert_eq!(
            graph,
            [HashMap::new(), HashMap::new(), HashMap::from([(1, 1)])]
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::from([
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

        graph.remove_edge(0, 1);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([2])),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        graph.remove_edge(0, 2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        graph.remove_edge(1, 0);

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
        let mut graph: HashMap<usize, HashMap<usize, usize>> = HashMap::from([
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

        graph.remove_edge(0, 1);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(2, 1)])),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        graph.remove_edge(0, 2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::new()),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        graph.remove_edge(1, 0);

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
