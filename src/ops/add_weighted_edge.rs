//! A trait to add a weighted edge to a graph
//!
//! # Examples
//!
//! ```
//! use graaf::ops::AddWeightedEdge;
//!
//! let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); 3];
//!
//! graph.add_weighted_edge(0, 1, 2);
//! graph.add_weighted_edge(0, 2, 1);
//! graph.add_weighted_edge(1, 2, -3);
//!
//! assert_eq!(graph, vec![vec![(1, 2), (2, 1)], vec![(2, -3)], Vec::new()]);
//! ```

use {
    core::hash::{
        BuildHasher,
        Hash,
    },
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to add an edge to a weighted graph
///
/// # How can I implement `AddWeightedEdge`?
///
/// Provide an implementation of `add_weighted_edge` that adds an edge from `s`
/// to `t` with weight `w` to the type.
///
/// ```
/// use graaf::ops::AddWeightedEdge;
///
/// struct Graph {
///     edges: Vec<Vec<(usize, i32)>>,
/// }
///
/// impl AddWeightedEdge<i32> for Graph {
///     fn add_weighted_edge(&mut self, s: usize, t: usize, w: i32) {
///         self.edges[s].push((t, w));
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::ops::AddWeightedEdge;
///
/// let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); 3];
///
/// graph.add_weighted_edge(0, 1, 2);
/// graph.add_weighted_edge(0, 2, 1);
/// graph.add_weighted_edge(1, 2, -3);
///
/// assert_eq!(graph, vec![vec![(1, 2), (2, 1)], vec![(2, -3)], Vec::new()]);
/// ```
///
/// # Properties
///
/// ## `AddWeightedEdge` and [`crate::ops::RemoveEdge`]
///
/// Types that also implement [`crate::ops::RemoveEdge`] should ensure that the
/// following property holds for every `graph`, `s`, `t`, and `w` of the given
/// types:
///
/// ```
/// use graaf::ops::{
///     AddWeightedEdge,
///     RemoveEdge,
/// };
///
/// fn prop_add_weighted_edge_remove_edge<G, W>(graph: G, s: usize, t: usize, w: W) -> bool
/// where
///     G: AddWeightedEdge<W> + Clone + PartialEq + RemoveEdge,
/// {
///     let mut clone = graph.clone();
///
///     clone.add_weighted_edge(s, t, w);
///     clone.remove_edge(s, t);
///
///     graph == clone
/// }
/// ```
///
/// ## `AddWeightedEdge` and [`crate::ops::IsEdge`]
///
/// Types that also implement [`crate::ops::IsEdge`] should ensure that the
/// following property holds for every `graph`, `s`, `t`, and `w` of the given
/// types:
///
/// ```
/// use graaf::ops::{
///     AddWeightedEdge,
///     IsEdge,
/// };
///
/// fn prop_add_edge_weighted_is_edge<G, W>(graph: &mut G, s: usize, t: usize, w: W) -> bool
/// where
///     G: AddWeightedEdge<W> + IsEdge,
/// {
///     graph.add_weighted_edge(s, t, w);
///
///     graph.is_edge(s, t)
/// }
/// ```
pub trait AddWeightedEdge<W> {
    /// Add an edge from `s` to `t` with weight `w`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    /// * `w`: The weight of the edge.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W);
}

// Vec

impl<W> AddWeightedEdge<W> for Vec<Vec<(usize, W)>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<W, H> AddWeightedEdge<W> for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
    W: Eq + Hash,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<W, H> AddWeightedEdge<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

// Arr

impl<const V: usize, W> AddWeightedEdge<W> for [Vec<(usize, W)>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<const V: usize, W, H> AddWeightedEdge<W> for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
    W: Eq + Hash,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<const V: usize, W, H> AddWeightedEdge<W> for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

// HashMap

impl<W, H> AddWeightedEdge<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self.entry(s).or_default().push((t, w));
    }
}

impl<W, H> AddWeightedEdge<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
    W: Eq + Hash,
    HashSet<(usize, W), H>: Default,
{
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self.entry(s).or_default().insert((t, w));
    }
}

impl<W, H> AddWeightedEdge<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
    HashMap<usize, W, H>: Default,
{
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self.entry(s).or_default().insert(t, w);
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        core::iter::once,
    };

    #[test]
    fn vec_vec() {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); 3];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(graph, vec![vec![(1, 2)], Vec::new(), Vec::new()]);

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(graph, vec![vec![(1, 2), (2, 1)], Vec::new(), Vec::new()]);

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(graph, vec![vec![(1, 2), (2, 1)], vec![(2, 4)], Vec::new()]);

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            vec![vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], Vec::new()]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            vec![vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], vec![(0, 3)]]
        );
    }

    #[test]
    fn vec_hash_set() {
        let mut graph: Vec<HashSet<(usize, i32)>> = vec![HashSet::new(); 3];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            vec![once((1, 2)).collect(), HashSet::new(), HashSet::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].iter().copied().collect(),
                HashSet::new(),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].iter().copied().collect(),
                once((2, 4)).collect(),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].iter().copied().collect(),
                [(2, 4), (0, -2)].iter().copied().collect(),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].iter().copied().collect(),
                [(2, 4), (0, -2)].iter().copied().collect(),
                once((0, 3)).collect()
            ]
        );
    }

    #[test]
    fn vec_hash_map() {
        let mut graph: Vec<HashMap<usize, i32>> = vec![HashMap::new(); 3];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            vec![once((1, 2)).collect(), HashMap::new(), HashMap::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].into_iter().collect(),
                HashMap::new(),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].iter().copied().collect(),
                once((2, 4)).collect(),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].iter().copied().collect(),
                [(2, 4), (0, -2)].iter().copied().collect(),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].iter().copied().collect(),
                [(2, 4), (0, -2)].iter().copied().collect(),
                once((0, 3)).collect()
            ]
        );
    }

    #[test]
    fn arr_vec() {
        let mut graph: [Vec<(usize, i32)>; 3] = [Vec::new(), Vec::new(), Vec::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(graph, [vec![(1, 2)], Vec::new(), Vec::new()]);

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(graph, [vec![(1, 2), (2, 1)], Vec::new(), Vec::new()]);

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(graph, [vec![(1, 2), (2, 1)], vec![(2, 4)], Vec::new()]);

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], Vec::new()]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], vec![(0, 3)]]
        );
    }

    #[test]
    fn arr_hash_set() {
        let mut graph: [HashSet<(usize, i32)>; 3] =
            [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            [HashSet::from([(1, 2)]), HashSet::new(), HashSet::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::new(),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4)]),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_hash_map() {
        let mut graph: [HashMap<usize, i32>; 3] = [HashMap::new(), HashMap::new(), HashMap::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            [HashMap::from([(1, 2)]), HashMap::new(), HashMap::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::new(),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4)]),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn hash_map_vec() {
        let mut graph: HashMap<usize, Vec<(usize, i32)>> = HashMap::new();

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(graph, HashMap::from([(0, vec![(1, 2)])]));

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(graph, HashMap::from([(0, vec![(1, 2), (2, 1)])]));

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![(1, 2), (2, 1)]), (1, vec![(2, 4)])])
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![(1, 2), (2, 1)]), (1, vec![(2, 4), (0, -2)])])
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            HashMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4), (0, -2)]),
                (2, vec![(0, 3)])
            ])
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut graph: HashMap<usize, HashSet<(usize, i32)>> = HashMap::new();

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(graph, HashMap::from([(0, HashSet::from([(1, 2)]))]));

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(graph, HashMap::from([(0, HashSet::from([(1, 2), (2, 1)]))]));

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([(1, 2), (2, 1)])),
                (1, HashSet::from([(2, 4)])),
            ])
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([(1, 2), (2, 1)])),
                (1, HashSet::from([(2, 4), (0, -2)])),
            ])
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([(1, 2), (2, 1)])),
                (1, HashSet::from([(2, 4), (0, -2)])),
                (2, HashSet::from([(0, 3)])),
            ])
        );
    }

    #[test]
    fn hash_map_hash_map() {
        let mut graph: HashMap<usize, HashMap<usize, i32>> = HashMap::new();

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(graph, HashMap::from([(0, HashMap::from([(1, 2)]))]));

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(graph, HashMap::from([(0, HashMap::from([(1, 2), (2, 1)]))]));

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4)])),
            ])
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4), (0, -2)])),
            ])
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4), (0, -2)])),
                (2, HashMap::from([(0, 3)])),
            ])
        );
    }
}
