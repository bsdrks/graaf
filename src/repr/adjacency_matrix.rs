use crate::{
    ops::{
        AddEdge,
        CountAllEdges,
        CountAllVertices,
        InDegree,
        IsEdge,
        IterVertices,
        OutDegree,
        RemoveEdge,
    },
    IterEdges,
};

macro_rules! blocks {
    ($v:expr) => {
        ($v * $v + 63) / 64
    };
}

/// An adjacency matrix representation of a graph.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
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
    /// Create a new adjacency matrix.
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

    /// Toggle the edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    ///
    /// # Panics
    ///
    /// Panics if `s >= V` or `t >= V`.
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

impl<const V: usize> InDegree for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `t >= V`.
    fn in_degree(&self, t: usize) -> usize {
        assert!(t < V, "t is not in the graph");

        (0..V).filter(|&s| self.is_edge(s, t)).count()
    }
}

impl<const V: usize> IsEdge for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn is_edge(&self, s: usize, t: usize) -> bool {
        if s >= V || t >= V {
            return false;
        }

        let i = Self::index(s, t);

        self.blocks[i >> 6] & Self::mask(i) != 0
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

        (0..V).filter(move |t| self.is_edge(s, *t))
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

impl<const V: usize> OutDegree for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V`.
    fn out_degree(&self, s: usize) -> usize {
        assert!(s < V, "s is not in the graph");

        (0..V).filter(|&t| self.is_edge(s, t)).count()
    }
}

impl<const V: usize> RemoveEdge for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V` or `t >= V`.
    fn remove_edge(&mut self, s: usize, t: usize) {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let i = Self::index(s, t);

        self.blocks[i >> 6] &= !Self::mask(i);
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
    fn in_degree() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 0);
        assert_eq!(graph.in_degree(2), 0);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);

        assert_eq!(graph.in_degree(0), 0);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.in_degree(2), 1);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn in_degree_t_gte_v() {
        let graph = AdjacencyMatrix::<3>::new();

        let _ = graph.in_degree(3);
    }

    #[test]
    fn is_edge() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert!(!graph.is_edge(0, 1));
        assert!(!graph.is_edge(0, 2));
        assert!(!graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 2));
        assert!(!graph.is_edge(2, 0));
        assert!(!graph.is_edge(2, 1));
        assert!(!graph.is_edge(3, 0));

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);

        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(!graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 2));
        assert!(!graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(3, 0));
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
    fn out_degree() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert_eq!(graph.out_degree(0), 0);
        assert_eq!(graph.out_degree(1), 0);
        assert_eq!(graph.out_degree(2), 0);

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(2, 1);

        assert_eq!(graph.out_degree(0), 2);
        assert_eq!(graph.out_degree(1), 0);
        assert_eq!(graph.out_degree(2), 1);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn out_degree_s_gte_v() {
        let graph = AdjacencyMatrix::<3>::new();

        let _ = graph.out_degree(3);
    }

    #[test]
    fn remove_edge() {
        let mut graph = AdjacencyMatrix::<3>::new();

        assert!(!graph.is_edge(0, 0));
        assert!(!graph.is_edge(0, 1));
        assert!(!graph.is_edge(0, 2));
        assert!(!graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(!graph.is_edge(2, 0));
        assert!(!graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));

        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 0);
        graph.add_edge(2, 1);

        assert!(!graph.is_edge(0, 0));
        assert!(graph.is_edge(0, 1));
        assert!(graph.is_edge(0, 2));
        assert!(graph.is_edge(1, 0));
        assert!(!graph.is_edge(1, 1));
        assert!(!graph.is_edge(1, 2));
        assert!(!graph.is_edge(2, 0));
        assert!(graph.is_edge(2, 1));
        assert!(!graph.is_edge(2, 2));

        graph.remove_edge(0, 1);

        assert!(!graph.is_edge(0, 1));

        graph.remove_edge(0, 2);

        assert!(!graph.is_edge(0, 2));

        graph.remove_edge(1, 0);

        assert!(!graph.is_edge(1, 0));

        graph.remove_edge(2, 1);

        assert!(!graph.is_edge(2, 1));
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn remove_edge_s_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.remove_edge(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn remove_edge_t_gte_v() {
        let mut graph = AdjacencyMatrix::<3>::new();

        graph.remove_edge(0, 3);
    }
}
