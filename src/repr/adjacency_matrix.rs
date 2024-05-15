//! An adjacency matrix representation of an unweighted directed graph
//!
//! An adjacency matrix is a symmetric binary matrix where a value of `1` at
//! row `s` and column `t` indicates an arc from vertex `s` to vertex `t`. The
//! matrix is stored as a bit array, and is suited for dense graphs with a small
//! number of vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     op::{
//!         AddArc,
//!         IsSimple,
//!     },
//!     repr::AdjacencyMatrix,
//! };
//!
//! let mut graph = AdjacencyMatrix::<3>::new();
//!
//! graph.add_arc(0, 1);
//!
//! assert!(graph.is_simple());
//!
//! graph.add_arc(1, 1);
//!
//! assert!(!graph.is_simple());
//! ```

use crate::op::{
    AddArc,
    HasArc,
    HasEdge,
    Indegree,
    IsBalanced,
    IsIsolated,
    IsSimple,
    IsSymmetric,
    IterAllArcs,
    IterArcs,
    IterVertices,
    Order,
    Outdegree,
    RemoveArc,
    Size,
};

macro_rules! blocks {
    ($v:expr) => {
        ($v * $v + 63) / 64
    };
}

/// An adjacency matrix representation of an unweighted directed graph
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdjacencyMatrix<const V: usize>
where
    [(); blocks!(V)]:,
{
    blocks: [usize; blocks!(V)],
}

impl<const V: usize> AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// Creates a new adjacency matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     op::{
    ///         Order,
    ///         Size,
    ///     },
    ///     repr::AdjacencyMatrix,
    /// };
    ///
    /// let graph = AdjacencyMatrix::<3>::new();
    ///
    /// assert_eq!(graph.size(), 0);
    /// assert_eq!(graph.order(), 3);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `V` is zero.
    #[must_use]
    pub const fn new() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        Self {
            blocks: [0; blocks!(V)],
        }
    }

    const fn mask(i: usize) -> usize {
        1 << (i & 63)
    }

    const fn index(s: usize, t: usize) -> usize {
        s * V + t
    }

    /// Toggles the arc from the source vertex to the target vertex.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Panics
    ///
    /// Panics if `s >= V` or `t >= V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     op::HasArc,
    ///     repr::AdjacencyMatrix,
    /// };
    ///
    /// let mut graph = AdjacencyMatrix::<3>::new();
    ///
    /// assert!(!graph.has_arc(0, 1));
    ///
    /// graph.toggle(0, 1);
    ///
    /// assert!(graph.has_arc(0, 1));
    /// ```
    pub fn toggle(&mut self, s: usize, t: usize) {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let i = Self::index(s, t);

        self.blocks[i >> 6] ^= Self::mask(i);
    }
}

impl<const V: usize> Default for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const V: usize> AddArc for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s` or `t` is not in the graph.
    fn add_arc(&mut self, s: usize, t: usize) {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let i = Self::index(s, t);

        self.blocks[i >> 6] |= Self::mask(i);
    }
}

impl<const V: usize> Size for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics when the number of arcs is greater than `usize::MAX`.
    fn size(&self) -> usize {
        self.blocks
            .iter()
            .map(|&block| block.count_ones() as usize)
            .sum()
    }
}

impl<const V: usize> Order for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn order(&self) -> usize {
        V
    }
}

impl<const V: usize> HasArc for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn has_arc(&self, s: usize, t: usize) -> bool {
        if s >= V || t >= V {
            return false;
        }

        let i = Self::index(s, t);

        self.blocks[i >> 6] & Self::mask(i) != 0
    }
}

impl<const V: usize> HasEdge for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<const V: usize> Indegree for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `t >= V`.
    fn indegree(&self, t: usize) -> usize {
        assert!(t < V, "t is not in the graph");

        self.iter_vertices().filter(|&s| self.has_arc(s, t)).count()
    }
}

impl<const V: usize> IsBalanced for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
    }
}

impl<const V: usize> IsIsolated for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
    }
}

impl<const V: usize> IsSimple for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn is_simple(&self) -> bool {
        self.iter_vertices().all(|s| !self.has_arc(s, s))
    }
}

impl<const V: usize> IsSymmetric for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn is_symmetric(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<const V: usize> IterAllArcs for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn iter_all_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter_vertices().flat_map(move |s| {
            self.iter_vertices()
                .filter_map(move |t| self.has_arc(s, t).then_some((s, t)))
        })
    }
}

impl<const V: usize> IterArcs for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V`.
    fn iter_arcs(&self, s: usize) -> impl Iterator<Item = usize> {
        assert!(s < V, "s is not in the graph");

        self.iter_vertices().filter(move |&t| self.has_arc(s, t))
    }
}

impl<const V: usize> IterVertices for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..V
    }
}

impl<const V: usize> Outdegree for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V`.
    fn outdegree(&self, s: usize) -> usize {
        assert!(s < V, "s is not in the graph");

        self.iter_vertices().filter(|&t| self.has_arc(s, t)).count()
    }
}

impl<const V: usize> RemoveArc for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V` or `t >= V`.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let has_arc = self.has_arc(s, t);
        let i = Self::index(s, t);

        self.blocks[i >> 6] &= !Self::mask(i);

        has_arc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.blocks, [0]);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    const fn new_panic() {
        let _ = AdjacencyMatrix::<0>::new();
    }

    #[test]
    fn toggle() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.toggle(0, 1);
        graph.toggle(0, 2);

        assert_eq!(graph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn toggle_s_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.toggle(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn toggle_t_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.toggle(0, 3);
    }

    #[test]
    fn default() {
        let graph = AdjacencyMatrix::<1>::default();

        assert_eq!(graph.blocks, [0]);
    }

    #[test]
    fn add_arc() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);

        assert_eq!(graph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn add_arc_s_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn add_arc_t_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 3);
    }

    #[test]
    fn size() {
        let graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.size(), 0);

        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 1);

        assert_eq!(graph.size(), 1);

        graph.add_arc(0, 2);

        assert_eq!(graph.size(), 2);
    }

    #[test]
    fn order() {
        let graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.order(), 3);

        let graph = AdjacencyMatrix::<1>::new();

        assert_eq!(graph.order(), 1);

        let graph = AdjacencyMatrix::<512>::new();

        assert_eq!(graph.order(), 512);
    }

    #[test]
    fn has_arc() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert!(!graph.has_arc(0, 1));
        assert!(!graph.has_arc(0, 2));
        assert!(!graph.has_arc(1, 0));
        assert!(!graph.has_arc(1, 2));
        assert!(!graph.has_arc(2, 0));
        assert!(!graph.has_arc(2, 1));
        assert!(!graph.has_arc(3, 0));
        assert!(!graph.has_arc(0, 3));

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(2, 1);

        assert!(graph.has_arc(0, 1));
        assert!(graph.has_arc(0, 2));
        assert!(!graph.has_arc(1, 0));
        assert!(!graph.has_arc(1, 2));
        assert!(!graph.has_arc(2, 0));
        assert!(graph.has_arc(2, 1));
        assert!(!graph.has_arc(3, 0));
        assert!(!graph.has_arc(0, 3));
    }

    #[test]
    fn has_edge() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(2, 0));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(0, 3));

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(1, 0);
        graph.add_arc(2, 1);

        assert!(graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
        assert!(graph.has_edge(1, 0));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(2, 0));
        assert!(!graph.has_edge(2, 1));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(0, 3));
    }

    #[test]
    fn indegree() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.indegree(0), 0);
        assert_eq!(graph.indegree(1), 0);
        assert_eq!(graph.indegree(2), 0);

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);

        assert_eq!(graph.indegree(0), 0);
        assert_eq!(graph.indegree(1), 1);
        assert_eq!(graph.indegree(2), 1);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn indegree_t_gte_v() {
        let graph = AdjacencyMatrix::<3>::new();
        let _ = graph.indegree(3);
    }

    #[test]
    fn is_balanced() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert!(graph.is_balanced());

        graph.add_arc(0, 1);

        assert!(!graph.is_balanced());

        graph.add_arc(1, 0);

        assert!(graph.is_balanced());

        graph.add_arc(0, 2);

        assert!(!graph.is_balanced());

        graph.add_arc(1, 2);

        assert!(!graph.is_balanced());

        graph.add_arc(2, 0);

        assert!(!graph.is_balanced());

        graph.add_arc(2, 1);

        assert!(graph.is_balanced());
    }

    #[test]
    fn is_isolated() {
        let mut graph = AdjacencyMatrix::<4>::new();

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(1, 2);

        assert!(!graph.is_isolated(0));
        assert!(!graph.is_isolated(1));
        assert!(!graph.is_isolated(2));
        assert!(graph.is_isolated(3));
    }

    #[test]
    fn is_simple() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(2, 1);

        assert!(graph.is_simple());
    }

    #[test]
    fn is_simple_self_loop() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 0);

        assert!(!graph.is_simple());
    }

    #[test]
    fn is_symmetric_simple() {
        let mut graph = AdjacencyMatrix::<2>::new();

        graph.add_arc(0, 1);
        graph.add_arc(1, 0);

        assert!(graph.is_symmetric());

        let mut graph = AdjacencyMatrix::<2>::new();

        graph.add_arc(0, 1);

        assert!(!graph.is_symmetric());

        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(1, 2);
        graph.add_arc(2, 0);

        assert!(!graph.is_symmetric());
    }

    #[test]
    fn iter_all_arcs() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 1);
        graph.add_arc(1, 2);
        graph.add_arc(2, 0);

        assert!(graph.iter_all_arcs().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn iter_arcs() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(2, 1);

        assert!(graph.iter_arcs(0).eq([1, 2]));
        assert!(graph.iter_arcs(1).eq([]));
        assert!(graph.iter_arcs(2).eq([1]));
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn iter_arcs_s_gte_v() {
        let graph = AdjacencyMatrix::<3>::new();
        let _ = graph.iter_arcs(3);
    }

    #[test]
    fn iter_vertices() {
        let graph = AdjacencyMatrix::<3>::new();

        assert!(graph.iter_vertices().eq([0, 1, 2]));
    }

    #[test]
    fn outdegree() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.outdegree(0), 0);
        assert_eq!(graph.outdegree(1), 0);
        assert_eq!(graph.outdegree(2), 0);

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(2, 1);

        assert_eq!(graph.outdegree(0), 2);
        assert_eq!(graph.outdegree(1), 0);
        assert_eq!(graph.outdegree(2), 1);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn outdegree_s_gte_v() {
        let graph = AdjacencyMatrix::<3>::new();
        let _ = graph.outdegree(3);
    }

    #[test]
    fn remove_arc() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_arc(0, 1);
        graph.add_arc(0, 2);
        graph.add_arc(1, 0);
        graph.add_arc(2, 1);

        assert!(!graph.has_arc(0, 0));
        assert!(graph.has_arc(0, 1));
        assert!(graph.has_arc(0, 2));
        assert!(graph.has_arc(1, 0));
        assert!(!graph.has_arc(1, 1));
        assert!(!graph.has_arc(1, 2));
        assert!(!graph.has_arc(2, 0));
        assert!(graph.has_arc(2, 1));
        assert!(!graph.has_arc(2, 2));

        assert!(graph.remove_arc(0, 1));
        assert!(graph.remove_arc(0, 2));
        assert!(graph.remove_arc(1, 0));
        assert!(graph.remove_arc(2, 1));

        assert!(!graph.has_arc(0, 1));
        assert!(!graph.has_arc(0, 2));
        assert!(!graph.has_arc(1, 0));
        assert!(!graph.has_arc(2, 1));
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn remove_arc_s_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        let _ = graph.remove_arc(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn remove_arc_t_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        let _ = graph.remove_arc(0, 3);
    }
}
