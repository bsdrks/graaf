//! A trait to add an edge to an unweighted graph
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

use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to add an edge to an unweighted graph
///
/// # How can I implement `AddEdge`?
///
/// Provide an implementation of `add_edge` that adds an edge from `s` to `t` to
/// the graph. Implementations should panic if `s` or `t` is known to be out of
/// bounds, for example for types with a compile-time constant size that
/// matches the number of vertices in the graph.
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
/// Types that also implement [`crate::op::RemoveEdge`] should ensure that
/// [`crate::op::prop::add_edge_remove_edge`] holds.
///
/// ## `AddEdge` and `IsEdge`
///
/// Types that also implement [`crate::op::IsEdge`] should ensure that
/// [`crate::op::prop::add_edge_is_edge`] holds.
pub trait AddEdge {
    /// Add an edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    fn add_edge(&mut self, s: usize, t: usize);
}

// Vec

impl AddEdge for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
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

// Slice

impl AddEdge for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
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

// Arr

impl<const V: usize> AddEdge for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` or `t` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        self[s].push(t);
    }
}

impl<const V: usize, H> AddEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let _ = self[s].insert(t);
    }
}

// HashMap

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
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn vec_vec_panic_s() {
        let mut graph = vec![Vec::new(); 3];

        graph.add_edge(3, 0);
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
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn vec_hash_set_panic_s() {
        let mut graph = vec![HashSet::new(); 3];

        graph.add_edge(3, 0);
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
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn slice_vec_panic_s() {
        let graph: &mut [Vec<usize>] = &mut [Vec::new(), Vec::new(), Vec::new()];

        graph.add_edge(3, 0);
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
    #[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
    fn slice_hash_set_panic_s() {
        let graph: &mut [HashSet<usize>] = &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_edge(3, 0);
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
    #[should_panic(expected = "s is not in the graph")]
    fn arr_vec_panic_s() {
        let mut graph = [Vec::new(), Vec::new(), Vec::new()];

        graph.add_edge(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn arr_vec_panic_t() {
        let mut graph = [Vec::new(), Vec::new(), Vec::new()];

        graph.add_edge(0, 3);
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
    #[should_panic(expected = "s is not in the graph")]
    fn arr_hash_set_panic_s() {
        let mut graph = [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_edge(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn arr_hash_set_panic_t() {
        let mut graph = [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_edge(0, 3);
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
    #[should_panic(expected = "s is not in the graph")]
    fn hash_map_vec_panic_s() {
        let mut graph = HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        graph.add_edge(3, 0);
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

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn hash_map_hash_set_panic_s() {
        let mut graph = HashMap::from([
            (0, HashSet::new()),
            (1, HashSet::new()),
            (2, HashSet::new()),
        ]);

        graph.add_edge(3, 0);
    }
}
