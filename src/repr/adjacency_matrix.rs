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

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct AdjacencyMatrix<const V: usize>
where
    [(); blocks!(V)]:,
{
    // TODO: generalize over the container, so we can support weighted matrices.
    blocks: [usize; blocks!(V)],
}

impl<const V: usize> AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
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

    pub fn toggle(&mut self, s: usize, t: usize) {
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
    /// Panics if `s` or `t` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_edge(&mut self, s: usize, t: usize) {
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
    ///
    /// # Complexity
    ///
    /// O(V^2)
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
    /// # Complexity
    ///
    /// O(1)
    fn count_all_vertices(&self) -> usize {
        V
    }
}

impl<const V: usize> InDegree for &AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `t` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn in_degree(&self, t: usize) -> usize {
        (0..V).filter(|&s| self.is_edge(s, t)).count()
    }
}

impl<const V: usize> IsEdge for &AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s` or `t` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn is_edge(&self, s: usize, t: usize) -> bool {
        let i = AdjacencyMatrix::<V>::index(s, t);

        self.blocks[i >> 6] & AdjacencyMatrix::<V>::mask(i) != 0
    }
}

impl<const V: usize> IterEdges for &AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        (0..V).filter(move |&t| self.is_edge(s, t))
    }
}

impl<const V: usize> IterVertices for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..V
    }
}

impl<const V: usize> OutDegree for &AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn out_degree(&self, s: usize) -> usize {
        (0..V).filter(|&t| self.is_edge(s, t)).count()
    }
}

impl<const V: usize> RemoveEdge for AdjacencyMatrix<V>
where
    [(); blocks!(V)]:,
{
    /// # Panics
    ///
    /// Panics if `s` or `t` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn remove_edge(&mut self, s: usize, t: usize) {
        let i = Self::index(s, t);

        self.blocks[i >> 6] &= !Self::mask(i);
    }
}
