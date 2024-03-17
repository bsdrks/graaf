use std::{
    collections::HashSet,
    hash::BuildHasher,
};

pub trait OutDegree {
    fn out_degree(&self, s: usize) -> usize;
}

impl<E> OutDegree for Vec<Vec<E>> {
    /// # Panics
    ///
    /// May panic if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<H> OutDegree for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// May panic if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[s].len()
    }
}
