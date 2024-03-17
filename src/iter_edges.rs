use std::{
    collections::HashSet,
    hash::BuildHasher,
};

pub trait IterEdges {
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize>;
}

impl IterEdges for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s >= self.vertex_count()`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}

impl<H> IterEdges for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s >= self.vertex_count()`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}
