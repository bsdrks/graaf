//! A trait to add a weighted edge to a graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::AddWeightedEdge;
//!
//! let mut graph = vec![Vec::new(); 3];
//!
//! graph.add_weighted_edge(0, 1, 2);
//! graph.add_weighted_edge(0, 2, 1);
//! graph.add_weighted_edge(1, 2, -3);
//!
//! assert_eq!(graph, vec![vec![(1, 2), (2, 1)], vec![(2, -3)], Vec::new()]);
//! ```
extern crate alloc;

use {
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
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
/// to `t` with weight `w` to the graph.
///
/// ```
/// use graaf::op::AddWeightedEdge;
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
/// use graaf::op::AddWeightedEdge;
///
/// let mut graph = vec![Vec::new(); 3];
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
/// ## `AddWeightedEdge` and `RemoveEdge`
///
/// Types that also implement [`crate::op::RemoveEdge`] should ensure that
/// [`crate::op::prop::add_weighted_edge_remove_edge`] holds.
///
/// ## `AddWeightedEdge` and `IsEdge`
///
/// Types that also implement [`crate::op::IsEdge`] should ensure that
/// [`crate::op::prop::add_weighted_edge_is_edge`] holds.
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

impl<W> AddWeightedEdge<W> for Vec<Vec<(usize, W)>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<W> AddWeightedEdge<W> for Vec<BTreeSet<(usize, W)>>
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<W> AddWeightedEdge<W> for Vec<BTreeMap<usize, W>>
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
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

impl<W> AddWeightedEdge<W> for [Vec<(usize, W)>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<W> AddWeightedEdge<W> for [BTreeSet<(usize, W)>]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<W> AddWeightedEdge<W> for [BTreeMap<usize, W>]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

impl<W, H> AddWeightedEdge<W> for [HashSet<(usize, W), H>]
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

impl<W, H> AddWeightedEdge<W> for [HashMap<usize, W, H>]
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

impl<const V: usize, W> AddWeightedEdge<W> for [Vec<(usize, W)>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<const V: usize, W> AddWeightedEdge<W> for [BTreeSet<(usize, W)>; V]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<const V: usize, W> AddWeightedEdge<W> for [BTreeMap<usize, W>; V]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
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

impl<W> AddWeightedEdge<W> for BTreeMap<usize, Vec<(usize, W)>> {
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self.get_mut(&s)
            .expect("s is not in the graph")
            .push((t, w));
    }
}

impl<W> AddWeightedEdge<W> for BTreeMap<usize, BTreeSet<(usize, W)>>
where
    W: Ord,
{
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self
            .get_mut(&s)
            .expect("s is not in the graph")
            .insert((t, w));
    }
}

impl<W> AddWeightedEdge<W> for BTreeMap<usize, BTreeMap<usize, W>> {
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self
            .get_mut(&s)
            .expect("s is not in the graph")
            .insert(t, w);
    }
}

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
{
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        let _ = self
            .get_mut(&s)
            .expect("s is not in the graph")
            .insert(t, w);
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
        let mut graph = vec![Vec::new(); 3];

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
    fn vec_btree_set() {
        let mut graph = vec![BTreeSet::new(); 3];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            vec![once((1, 2)).collect(), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::new(),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4)]),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn vec_hash_set() {
        let mut graph = vec![HashSet::new(); 3];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            vec![once((1, 2)).collect(), HashSet::new(), HashSet::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::new(),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4)]),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn vec_btree_map() {
        let mut graph = vec![BTreeMap::new(); 3];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            vec![BTreeMap::from([(1, 2)]), BTreeMap::new(), BTreeMap::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            vec![
                [(1, 2), (2, 1)].into_iter().collect(),
                BTreeMap::new(),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            vec![
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4)]),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            vec![
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            vec![
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::from([(0, 3)]),
            ]
        );
    }

    #[test]
    fn vec_hash_map() {
        let mut graph = vec![HashMap::new(); 3];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            vec![HashMap::from([(1, 2)]), HashMap::new(), HashMap::new()]
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
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4)]),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            vec![
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            vec![
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::from([(0, 3)]),
            ]
        );
    }

    #[test]
    fn slice_vec() {
        let graph: &mut [Vec<(usize, i32)>] = &mut [Vec::new(), Vec::new(), Vec::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(*graph, [vec![(1, 2)], Vec::new(), Vec::new()]);

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(*graph, [vec![(1, 2), (2, 1)], Vec::new(), Vec::new()]);

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(*graph, [vec![(1, 2), (2, 1)], vec![(2, 4)], Vec::new()]);

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            *graph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], Vec::new()]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            *graph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], vec![(0, 3)]]
        );
    }

    #[test]
    fn slice_btree_set() {
        let graph: &mut [BTreeSet<(usize, i32)>] =
            &mut [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            *graph,
            [BTreeSet::from([(1, 2)]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            *graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::new(),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            *graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4)]),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            *graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            *graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn slice_hash_set() {
        let graph: &mut [HashSet<(usize, i32)>] =
            &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            *graph,
            [HashSet::from([(1, 2)]), HashSet::new(), HashSet::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            *graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::new(),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            *graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4)]),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            *graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            *graph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn slice_btree_map() {
        let graph: &mut [BTreeMap<usize, i32>] =
            &mut [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            *graph,
            [BTreeMap::from([(1, 2)]), BTreeMap::new(), BTreeMap::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            *graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::new(),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            *graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4)]),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            *graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            *graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn slice_hash_map() {
        let graph: &mut [HashMap<usize, i32>] =
            &mut [HashMap::new(), HashMap::new(), HashMap::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            *graph,
            [HashMap::from([(1, 2)]), HashMap::new(), HashMap::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            *graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::new(),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            *graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4)]),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            *graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            *graph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_vec() {
        let mut graph = [Vec::new(), Vec::new(), Vec::new()];

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
    fn arr_btree_set() {
        let mut graph = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            [BTreeSet::from([(1, 2)]), BTreeSet::new(), BTreeSet::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::new(),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4)]),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        let mut graph = [HashSet::new(), HashSet::new(), HashSet::new()];

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
    fn arr_btree_map() {
        let mut graph = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            [BTreeMap::from([(1, 2)]), BTreeMap::new(), BTreeMap::new()]
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::new(),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4)]),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::new()
            ]
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_hash_map() {
        let mut graph = [HashMap::new(), HashMap::new(), HashMap::new()];

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
    fn btree_map_vec() {
        let mut graph = BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            BTreeMap::from([(0, vec![(1, 2)]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            BTreeMap::from([(0, vec![(1, 2), (2, 1)]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4)]),
                (2, Vec::new())
            ])
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4), (0, -2)]),
                (2, Vec::new())
            ])
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4), (0, -2)]),
                (2, vec![(0, 3)])
            ])
        );
    }

    #[test]
    fn btree_map_btree_set() {
        let mut graph = BTreeMap::from([
            (0, BTreeSet::new()),
            (1, BTreeSet::new()),
            (2, BTreeSet::new()),
        ]);

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2)])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::from([(2, 4)])),
                (2, BTreeSet::new())
            ])
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::from([(2, 4), (0, -2)])),
                (2, BTreeSet::new())
            ])
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::from([(2, 4), (0, -2)])),
                (2, BTreeSet::from([(0, 3)])),
            ])
        );
    }

    #[test]
    fn btree_map_btree_map() {
        let mut graph = BTreeMap::from([
            (0, BTreeMap::new()),
            (1, BTreeMap::new()),
            (2, BTreeMap::new()),
        ]);

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2)])),
                (1, BTreeMap::new()),
                (2, BTreeMap::new())
            ])
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::new()),
                (2, BTreeMap::new())
            ])
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::from([(2, 4)])),
                (2, BTreeMap::new())
            ])
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::from([(2, 4), (0, -2)])),
                (2, BTreeMap::new())
            ])
        );

        graph.add_weighted_edge(2, 0, 3);

        assert_eq!(
            graph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::from([(2, 4), (0, -2)])),
                (2, BTreeMap::from([(0, 3)])),
            ])
        );
    }

    #[test]
    fn hash_map_vec() {
        let mut graph = HashMap::new();

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
        let mut graph = HashMap::new();

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
        let mut graph = HashMap::from([
            (0, HashMap::new()),
            (1, HashMap::new()),
            (2, HashMap::new()),
        ]);

        graph.add_weighted_edge(0, 1, 2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 2)])),
                (1, HashMap::new()),
                (2, HashMap::new())
            ])
        );

        graph.add_weighted_edge(0, 2, 1);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::new()),
                (2, HashMap::new())
            ])
        );

        graph.add_weighted_edge(1, 2, 4);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4)])),
                (2, HashMap::new())
            ])
        );

        graph.add_weighted_edge(1, 0, -2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4), (0, -2)])),
                (2, HashMap::new())
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
