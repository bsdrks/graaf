//! A trait to add an edge to a unweighted directed graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::AddEdge;
//!
//! let mut graph = vec![Vec::new(); 3];
//!
//! graph.add_edge(0, 1);
//! graph.add_edge(0, 2);
//! graph.add_edge(2, 0);
//!
//! assert_eq!(graph, vec![vec![1, 2], Vec::new(), vec![0]]);
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

/// A trait to add an edge to a unweighted directed graph
///
/// # How can I implement `AddEdge`?
///
/// Provide an implementation of `add_edge` that adds an unweighted edge from
/// `s` to `t` to the graph.
///
/// ```
/// use graaf::op::AddEdge;
///
/// struct Graph {
///     edges: Vec<Vec<usize>>,
/// }
///
/// impl AddEdge for Graph {
///     fn add_edge(&mut self, s: usize, t: usize) {
///         self.edges[s].push(t);
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::AddEdge;
///
/// let mut graph = vec![Vec::new(); 3];
///
/// graph.add_edge(0, 1);
/// graph.add_edge(0, 2);
/// graph.add_edge(2, 0);
///
/// assert_eq!(graph, vec![vec![1, 2], Vec::new(), vec![0]]);
/// ```
///
/// # Properties
///
/// ## `AddEdge` and `RemoveEdge`
///
/// Types that also implement [`RemoveEdge`] should ensure that
/// [`add_edge_remove_edge`] holds.
///
/// ## `AddEdge` and `HasEdge`
///
/// Types that also implement [`HasEdge`] should ensure that
/// [`add_edge_has_edge`] holds.
///
/// [`HasEdge`]: crate::op::HasEdge
/// [`RemoveEdge`]: crate::op::RemoveEdge
/// [`add_edge_has_edge`]: crate::prop::add_edge_has_edge
/// [`add_edge_remove_edge`]: crate::prop::add_edge_remove_edge
pub trait AddEdge {
    /// Adds an edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    fn add_edge(&mut self, s: usize, t: usize);
}

impl AddEdge for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl AddEdge for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<H> AddEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl AddEdge for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl AddEdge for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<H> AddEdge for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<const V: usize> AddEdge for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl<const V: usize> AddEdge for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<H, const V: usize> AddEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl AddEdge for BTreeMap<usize, Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        self.get_mut(&s).expect("s is not in the graph").push(t);
    }
}

impl AddEdge for BTreeMap<usize, BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self.get_mut(&s).expect("s is not in the graph").insert(t);
    }
}

impl<H> AddEdge for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        self.get_mut(&s).expect("s is not in the graph").push(t);
    }
}

impl<H> AddEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
    HashSet<usize, H>: Default,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self.get_mut(&s).expect("s is not in the graph").insert(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        let mut graph = vec![Vec::new(); 3];

        graph.add_edge(0, 1);

        assert_eq!(graph, vec![vec![1], Vec::new(), Vec::new()]);

        graph.add_edge(0, 2);

        assert_eq!(graph, vec![vec![1, 2], Vec::new(), Vec::new()]);

        graph.add_edge(1, 2);

        assert_eq!(graph, vec![vec![1, 2], vec![2], Vec::new()]);

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(graph, vec![vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn vec_btree_set() {
        let mut graph = vec![BTreeSet::new(); 3];

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            vec![BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            vec![BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            vec![BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            vec![
                BTreeSet::from([1, 2]),
                BTreeSet::from([2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn vec_hash_set() {
        let mut graph = vec![HashSet::new(); 3];

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            vec![HashSet::from([1]), HashSet::new(), HashSet::new()]
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            vec![HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            vec![
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn slice_vec() {
        let graph: &mut [Vec<usize>] = &mut [Vec::new(), Vec::new(), Vec::new()];

        graph.add_edge(0, 1);

        assert_eq!(*graph, [vec![1], Vec::new(), Vec::new()]);

        graph.add_edge(0, 2);

        assert_eq!(*graph, [vec![1, 2], Vec::new(), Vec::new()]);

        graph.add_edge(1, 2);

        assert_eq!(*graph, [vec![1, 2], vec![2], Vec::new()]);

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(*graph, [vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<usize>] =
            &mut [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        graph.add_edge(0, 1);

        assert_eq!(
            *graph,
            [BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_edge(0, 2);

        assert_eq!(
            *graph,
            [BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            *graph,
            [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            *graph,
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<usize>] = &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_edge(0, 1);

        assert_eq!(*graph, [HashSet::from([1]), HashSet::new(), HashSet::new()]);

        graph.add_edge(0, 2);

        assert_eq!(
            *graph,
            [HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            *graph,
            [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            *graph,
            [
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn arr_vec() {
        let mut graph = [Vec::new(), Vec::new(), Vec::new()];

        graph.add_edge(0, 1);

        assert_eq!(graph, [vec![1], Vec::new(), Vec::new()]);

        graph.add_edge(0, 2);

        assert_eq!(graph, [vec![1, 2], Vec::new(), Vec::new()]);

        graph.add_edge(1, 2);

        assert_eq!(graph, [vec![1, 2], vec![2], Vec::new()]);

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(graph, [vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn arr_btree_set() {
        let mut graph = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            [BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            [BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        let mut graph = [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_edge(0, 1);

        assert_eq!(graph, [HashSet::from([1]), HashSet::new(), HashSet::new()]);

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            [HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            [
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn btree_map_vec() {
        let mut graph = BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            BTreeMap::from([(0, vec![1]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            BTreeMap::from([(0, vec![1, 2]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            BTreeMap::from([(0, vec![1, 2]), (1, vec![2]), (2, Vec::new())])
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            BTreeMap::from([(0, vec![1, 2]), (1, vec![2]), (2, vec![0, 1])])
        );
    }

    #[test]
    fn btree_map_btree_set() {
        let mut graph = BTreeMap::from([
            (0, BTreeSet::new()),
            (1, BTreeSet::new()),
            (2, BTreeSet::new()),
        ]);

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([1])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([2])),
                (2, BTreeSet::new())
            ])
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([2])),
                (2, BTreeSet::from([0, 1]))
            ])
        );
    }

    #[test]
    fn hash_map_vec() {
        let mut graph = HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1, 2]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1, 2]), (1, vec![2]), (2, Vec::new())])
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1, 2]), (1, vec![2]), (2, vec![0, 1])])
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut graph = HashMap::from([
            (0, HashSet::new()),
            (1, HashSet::new()),
            (2, HashSet::new()),
        ]);

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1])),
                (1, HashSet::new()),
                (2, HashSet::new())
            ])
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::new()),
                (2, HashSet::new())
            ])
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([2])),
                (2, HashSet::new())
            ])
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([2])),
                (2, HashSet::from([0, 1]))
            ])
        );
    }
}
