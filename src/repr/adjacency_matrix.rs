//! An adjacency matrix representation of an unweighted directed graph
//!
//! The matrix is stored as a bit array, and is suited for dense graphs with a
//! small number of vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     op::{
//!         AddEdge,
//!         IsSimple,
//!     },
//!     repr::AdjacencyMatrix,
//! };
//!
//! let mut graph = AdjacencyMatrix::<3>::new();
//!
//! graph.add_edge(0, 1);
//!
//! assert!(graph.is_simple());
//!
//! graph.add_edge(1, 1);
//!
//! assert!(!graph.is_simple());
//! ```

use crate::op::{
    AddEdge,
    CountAllEdges,
    CountAllVertices,
    HasEdge,
    Indegree,
    IsSimple,
    IterEdges,
    IterVertices,
    Outdegree,
    RemoveEdge,
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
    ///         CountAllEdges,
    ///         CountAllVertices,
    ///     },
    ///     repr::AdjacencyMatrix,
    /// };
    ///
    /// let graph = AdjacencyMatrix::<3>::new();
    ///
    /// assert_eq!(graph.count_all_edges(), 0);
    /// assert_eq!(graph.count_all_vertices(), 3);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
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

    /// Toggles the edge from `s` to `t`.
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
    ///     op::HasEdge,
    ///     repr::AdjacencyMatrix,
    /// };
    ///
    /// let mut graph = AdjacencyMatrix::<3>::new();
    ///
    /// assert!(!graph.has_edge(0, 1));
    ///
    /// graph.toggle(0, 1);
    ///
    /// assert!(graph.has_edge(0, 1));
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

impl<const V: usize> AddEdge for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s` or `t` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let i = Self::index(s, t);

        self.blocks[i >> 6] |= Self::mask(i);
    }
}

impl<const V: usize> CountAllEdges for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics when the number of edges is greater than `usize::MAX`.
    fn count_all_edges(&self) -> usize {
        self.blocks
            .iter()
            .map(|&block| block.count_ones() as usize)
            .sum()
    }
}

impl<const V: usize> CountAllVertices for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn count_all_vertices(&self) -> usize {
        V
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

        (0..V).filter(|&s| self.has_edge(s, t)).count()
    }
}

impl<const V: usize> HasEdge for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        if s >= V || t >= V {
            return false;
        }

        let i = Self::index(s, t);

        self.blocks[i >> 6] & Self::mask(i) != 0
    }
}

impl<const V: usize> IsSimple for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn is_simple(&self) -> bool {
        (0..V).all(|s| !self.has_edge(s, s))
    }
}

impl<const V: usize> IterEdges for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V`.
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        assert!(s < V, "s is not in the graph");

        (0..V).filter(move |t| self.has_edge(s, *t))
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

        (0..V).filter(|&t| self.has_edge(s, t)).count()
    }
}

impl<const V: usize> RemoveEdge for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V` or `t >= V`.
    fn remove_edge(&mut self, s: usize, t: usize) -> bool {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let has_edge = self.has_edge(s, t);
        let i = Self::index(s, t);

        self.blocks[i >> 6] &= !Self::mask(i);

        has_edge
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
    fn add_edge() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

        assert_eq!(graph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn add_edge_s_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn add_edge_t_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 3);
    }

    #[test]
    fn count_all_edges() {
        let graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.count_all_edges(), 0);

        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 1);

        assert_eq!(graph.count_all_edges(), 1);

        graph.add_edge(0, 2);

        assert_eq!(graph.count_all_edges(), 2);
    }

    #[test]
    fn count_all_vertices() {
        let graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.count_all_vertices(), 3);

        let graph = AdjacencyMatrix::<1>::new();

        assert_eq!(graph.count_all_vertices(), 1);

        let graph = AdjacencyMatrix::<512>::new();

        assert_eq!(graph.count_all_vertices(), 512);
    }

    #[test]
    fn indegree() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.indegree(0), 0);
        assert_eq!(graph.indegree(1), 0);
        assert_eq!(graph.indegree(2), 0);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

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

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);

        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(0, 2));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(2, 0));
        assert!(graph.has_edge(2, 1));
        assert!(!graph.has_edge(3, 0));
        assert!(!graph.has_edge(0, 3));
    }

    #[test]
    fn is_simple() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);

        assert!(graph.is_simple());
    }

    #[test]
    fn is_simple_self_loop() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 0); // Self-loop {0, 0}

        assert!(!graph.is_simple());
    }

    #[test]
    fn iter_edges() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);

        assert_eq!(graph.iter_edges(0).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(graph.iter_edges(1).collect::<Vec<_>>(), Vec::new());
        assert_eq!(graph.iter_edges(2).collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn iter_edges_s_gte_v() {
        let graph = AdjacencyMatrix::<3>::new();
        let _ = graph.iter_edges(3);
    }

    #[test]
    fn iter_vertices() {
        let graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.iter_vertices().collect::<Vec<_>>(), vec![0, 1, 2]);
    }

    #[test]
    fn outdegree() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.outdegree(0), 0);
        assert_eq!(graph.outdegree(1), 0);
        assert_eq!(graph.outdegree(2), 0);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);

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
    fn remove_edge() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 0);
        graph.add_edge(2, 1);

        assert!(!graph.has_edge(0, 0));
        assert!(graph.has_edge(0, 1));
        assert!(graph.has_edge(0, 2));
        assert!(graph.has_edge(1, 0));
        assert!(!graph.has_edge(1, 1));
        assert!(!graph.has_edge(1, 2));
        assert!(!graph.has_edge(2, 0));
        assert!(graph.has_edge(2, 1));
        assert!(!graph.has_edge(2, 2));

        assert!(graph.remove_edge(0, 1));
        assert!(graph.remove_edge(0, 2));
        assert!(graph.remove_edge(1, 0));
        assert!(graph.remove_edge(2, 1));

        assert!(!graph.has_edge(0, 1));
        assert!(!graph.has_edge(0, 2));
        assert!(!graph.has_edge(1, 0));
        assert!(!graph.has_edge(2, 1));
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn remove_edge_s_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        let _ = graph.remove_edge(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn remove_edge_t_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        let _ = graph.remove_edge(0, 3);
    }
}
