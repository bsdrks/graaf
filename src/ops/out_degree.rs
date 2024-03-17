use std::{
    collections::HashSet,
    hash::BuildHasher,
};

pub trait OutDegree {
    fn out_degree(&self, s: usize) -> usize;
}

impl<T> OutDegree for Vec<Vec<T>> {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, T> OutDegree for [Vec<T>; V] {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
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
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[s].len()
    }
}

impl<const V: usize, H> OutDegree for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[s].len()
    }
}
