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
    /// May panic if `s` is out of bounds.
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
    /// May panic if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn iter_edges(&self, s: usize) -> impl Iterator<Item = usize> {
        self[s].iter().copied()
    }
}
