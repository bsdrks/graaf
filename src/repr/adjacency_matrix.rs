//! An adjacency matrix representation for unweighted digraphs
//!
//! An adjacency matrix is a symmetric binary matrix where a value of `1` at
//! row `s` and column `t` indicates an arc from vertex `s` to vertex `t`. The
//! matrix is stored as a bit array, and is suited for dense digraphs with a
//! small number of vertices.
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
//! let mut digraph = AdjacencyMatrix::<3>::new();
//!
//! digraph.add_arc(0, 1);
//!
//! assert!(digraph.is_simple());
//!
//! digraph.add_arc(1, 1);
//!
//! assert!(!digraph.is_simple());
//! ```

use crate::{
    gen::{
        CycleConst,
        EmptyConst,
    },
    op::{
        AddArc,
        ArcWeight,
        HasArc,
        Indegree,
        IsSimple,
        IterArcs,
        IterOutNeighbors,
        IterVertices,
        Order,
        Outdegree,
        RemoveArc,
        Size,
    },
};

macro_rules! blocks {
    ($v:expr) => {
        ($v * $v + 63) / 64
    };
}

/// An adjacency matrix representation of an unweighted digraph
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
    /// let digraph = AdjacencyMatrix::<3>::new();
    ///
    /// assert_eq!(digraph.size(), 0);
    /// assert_eq!(digraph.order(), 3);
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
    /// let mut digraph = AdjacencyMatrix::<3>::new();
    ///
    /// assert!(!digraph.has_arc(0, 1));
    ///
    /// digraph.toggle(0, 1);
    ///
    /// assert!(digraph.has_arc(0, 1));
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
    /// Panics if `s` or `t` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        assert!(s < V, "s is not in the graph");
        assert!(t < V, "t is not in the graph");

        let i = Self::index(s, t);

        self.blocks[i >> 6] |= Self::mask(i);
    }
}

impl<const V: usize> ArcWeight<usize> for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn arc_weight(&self, s: usize, t: usize) -> Option<&usize> {
        self.has_arc(s, t).then_some(&1)
    }
}

impl<const V: usize> CycleConst for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn cycle() -> Self {
        let mut digraph = Self::empty();

        for i in 0..V - 1 {
            digraph.add_arc(i, i + 1);
        }

        digraph.add_arc(V - 1, 0);

        digraph
    }
}

impl<const V: usize> EmptyConst for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn empty() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        Self::new()
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

impl<const V: usize> IsSimple for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn is_simple(&self) -> bool {
        self.iter_vertices().all(|s| !self.has_arc(s, s))
    }
}

impl<const V: usize> IterArcs for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn iter_arcs(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter_vertices().flat_map(move |s| {
            self.iter_vertices()
                .filter_map(move |t| self.has_arc(s, t).then_some((s, t)))
        })
    }
}

impl<const V: usize> IterOutNeighbors for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s >= V`.
    fn iter_out_neighbors(&self, s: usize) -> impl Iterator<Item = usize> {
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

impl<const V: usize> Order for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    fn order(&self) -> usize {
        V
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::CycleConst,
            op::{
                HasEdge,
                IsBalanced,
                IsIsolated,
                IsSymmetric,
            },
        },
    };

    #[test]
    fn new() {
        let digraph = AdjacencyMatrix::<3>::new();

        assert_eq!(digraph.blocks, [0]);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    const fn new_panic() {
        let _ = AdjacencyMatrix::<0>::new();
    }

    #[test]
    fn toggle() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.toggle(0, 1);
        digraph.toggle(0, 2);

        assert_eq!(digraph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn toggle_panic_s_gte_v() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.toggle(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn toggle_panic_t_gte_v() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.toggle(0, 3);
    }

    #[test]
    fn default() {
        let digraph = AdjacencyMatrix::<1>::default();

        assert_eq!(digraph.blocks, [0]);
    }

    #[test]
    fn add_arc() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);

        assert_eq!(digraph.blocks, [0b110]);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn add_arc_panic_s_gte_v() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn add_arc_panic_t_gte_v() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 3);
    }

    #[test]
    fn arc_weight() {
        let digraph = AdjacencyMatrix::<3>::cycle();

        assert_eq!(digraph.arc_weight(0, 0), None);
        assert_eq!(digraph.arc_weight(0, 1), Some(&1));
        assert_eq!(digraph.arc_weight(0, 2), None);
        assert_eq!(digraph.arc_weight(0, 3), None);
        assert_eq!(digraph.arc_weight(1, 0), None);
        assert_eq!(digraph.arc_weight(1, 1), None);
        assert_eq!(digraph.arc_weight(1, 2), Some(&1));
        assert_eq!(digraph.arc_weight(1, 3), None);
        assert_eq!(digraph.arc_weight(2, 0), Some(&1));
        assert_eq!(digraph.arc_weight(2, 1), None);
        assert_eq!(digraph.arc_weight(2, 2), None);
        assert_eq!(digraph.arc_weight(2, 3), None);
    }

    #[test]
    fn cycle() {
        assert!(AdjacencyMatrix::<1>::cycle().iter_arcs().eq([(0, 0)]));

        assert!(AdjacencyMatrix::<2>::cycle()
            .iter_arcs()
            .eq([(0, 1), (1, 0)]));

        assert!(AdjacencyMatrix::<3>::cycle()
            .iter_arcs()
            .eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn empty() {
        assert_eq!(AdjacencyMatrix::<1>::empty().blocks, [0]);
        assert_eq!(AdjacencyMatrix::<2>::empty().blocks, [0]);
        assert_eq!(AdjacencyMatrix::<3>::empty().blocks, [0]);
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn empty_panic() {
        let _ = AdjacencyMatrix::<0>::empty();
    }

    #[test]
    fn has_arc() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        assert!(!digraph.has_arc(0, 1));
        assert!(!digraph.has_arc(0, 2));
        assert!(!digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(1, 2));
        assert!(!digraph.has_arc(2, 0));
        assert!(!digraph.has_arc(2, 1));
        assert!(!digraph.has_arc(3, 0));
        assert!(!digraph.has_arc(0, 3));

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert!(digraph.has_arc(0, 1));
        assert!(digraph.has_arc(0, 2));
        assert!(!digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(1, 2));
        assert!(!digraph.has_arc(2, 0));
        assert!(digraph.has_arc(2, 1));
        assert!(!digraph.has_arc(3, 0));
        assert!(!digraph.has_arc(0, 3));
    }

    #[test]
    fn has_edge() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        assert!(!digraph.has_edge(0, 1));
        assert!(!digraph.has_edge(0, 2));
        assert!(!digraph.has_edge(1, 0));
        assert!(!digraph.has_edge(1, 2));
        assert!(!digraph.has_edge(2, 0));
        assert!(!digraph.has_edge(2, 1));
        assert!(!digraph.has_edge(3, 0));
        assert!(!digraph.has_edge(0, 3));

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 0);
        digraph.add_arc(2, 1);

        assert!(digraph.has_edge(0, 1));
        assert!(!digraph.has_edge(0, 2));
        assert!(digraph.has_edge(1, 0));
        assert!(!digraph.has_edge(1, 2));
        assert!(!digraph.has_edge(2, 0));
        assert!(!digraph.has_edge(2, 1));
        assert!(!digraph.has_edge(3, 0));
        assert!(!digraph.has_edge(0, 3));
    }

    #[test]
    fn indegree() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        assert_eq!(digraph.indegree(0), 0);
        assert_eq!(digraph.indegree(1), 0);
        assert_eq!(digraph.indegree(2), 0);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);

        assert_eq!(digraph.indegree(0), 0);
        assert_eq!(digraph.indegree(1), 1);
        assert_eq!(digraph.indegree(2), 1);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn indegree_panic_t_gte_v() {
        let digraph = AdjacencyMatrix::<3>::new();
        let _ = digraph.indegree(3);
    }

    #[test]
    fn is_balanced() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        assert!(digraph.is_balanced());

        digraph.add_arc(0, 1);

        assert!(!digraph.is_balanced());

        digraph.add_arc(1, 0);

        assert!(digraph.is_balanced());

        digraph.add_arc(0, 2);

        assert!(!digraph.is_balanced());

        digraph.add_arc(1, 2);

        assert!(!digraph.is_balanced());

        digraph.add_arc(2, 0);

        assert!(!digraph.is_balanced());

        digraph.add_arc(2, 1);

        assert!(digraph.is_balanced());
    }

    #[test]
    fn is_isolated() {
        let mut digraph = AdjacencyMatrix::<4>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 2);

        assert!(!digraph.is_isolated(0));
        assert!(!digraph.is_isolated(1));
        assert!(!digraph.is_isolated(2));
        assert!(digraph.is_isolated(3));
    }

    #[test]
    fn is_simple() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert!(digraph.is_simple());
    }

    #[test]
    fn is_simple_self_loop() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 0);

        assert!(!digraph.is_simple());
    }

    #[test]
    fn is_symmetric_simple() {
        let mut digraph = AdjacencyMatrix::<2>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(1, 0);

        assert!(digraph.is_symmetric());

        let mut digraph = AdjacencyMatrix::<2>::new();

        digraph.add_arc(0, 1);

        assert!(!digraph.is_symmetric());

        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 2);
        digraph.add_arc(2, 0);

        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn iter_arcs() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(1, 2);
        digraph.add_arc(2, 0);

        assert!(digraph.iter_arcs().eq([(0, 1), (1, 2), (2, 0)]));
    }

    #[test]
    fn iter_out_neighbors() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert!(digraph.iter_out_neighbors(0).eq([1, 2]));
        assert!(digraph.iter_out_neighbors(1).eq([]));
        assert!(digraph.iter_out_neighbors(2).eq([1]));
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn iter_out_neighbors_panic_s_gte_v() {
        let digraph = AdjacencyMatrix::<3>::new();
        let _ = digraph.iter_out_neighbors(3);
    }

    #[test]
    fn iter_vertices() {
        let digraph = AdjacencyMatrix::<3>::new();

        assert!(digraph.iter_vertices().eq([0, 1, 2]));
    }

    #[test]
    fn outdegree() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        assert_eq!(digraph.outdegree(0), 0);
        assert_eq!(digraph.outdegree(1), 0);
        assert_eq!(digraph.outdegree(2), 0);

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(2, 1);

        assert_eq!(digraph.outdegree(0), 2);
        assert_eq!(digraph.outdegree(1), 0);
        assert_eq!(digraph.outdegree(2), 1);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn outdegree_panic_s_gte_v() {
        let digraph = AdjacencyMatrix::<3>::new();
        let _ = digraph.outdegree(3);
    }

    #[test]
    fn remove_arc() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 1);
        digraph.add_arc(0, 2);
        digraph.add_arc(1, 0);
        digraph.add_arc(2, 1);

        assert!(!digraph.has_arc(0, 0));
        assert!(digraph.has_arc(0, 1));
        assert!(digraph.has_arc(0, 2));
        assert!(digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(1, 1));
        assert!(!digraph.has_arc(1, 2));
        assert!(!digraph.has_arc(2, 0));
        assert!(digraph.has_arc(2, 1));
        assert!(!digraph.has_arc(2, 2));

        assert!(digraph.remove_arc(0, 1));
        assert!(digraph.remove_arc(0, 2));
        assert!(digraph.remove_arc(1, 0));
        assert!(digraph.remove_arc(2, 1));

        assert!(!digraph.has_arc(0, 1));
        assert!(!digraph.has_arc(0, 2));
        assert!(!digraph.has_arc(1, 0));
        assert!(!digraph.has_arc(2, 1));
    }

    #[test]
    fn order() {
        let digraph = AdjacencyMatrix::<3>::new();

        assert_eq!(digraph.order(), 3);

        let digraph = AdjacencyMatrix::<1>::new();

        assert_eq!(digraph.order(), 1);

        let digraph = AdjacencyMatrix::<512>::new();

        assert_eq!(digraph.order(), 512);
    }

    #[test]
    #[should_panic(expected = "s is not in the graph")]
    fn remove_arc_panic_s_gte_v() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        let _ = digraph.remove_arc(3, 0);
    }

    #[test]
    #[should_panic(expected = "t is not in the graph")]
    fn remove_arc_panic_t_gte_v() {
        let mut digraph = AdjacencyMatrix::<3>::new();

        let _ = digraph.remove_arc(0, 3);
    }

    #[test]
    fn size() {
        let digraph = AdjacencyMatrix::<3>::new();

        assert_eq!(digraph.size(), 0);

        let mut digraph = AdjacencyMatrix::<3>::new();

        digraph.add_arc(0, 1);

        assert_eq!(digraph.size(), 1);

        digraph.add_arc(0, 2);

        assert_eq!(digraph.size(), 2);
    }
}
