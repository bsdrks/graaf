use std::{
    collections::HashSet,
    hash::BuildHasher,
};

pub trait AddEdge {
    fn add_edge(&mut self, s: usize, t: usize);
}

impl AddEdge for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// May panic if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl<H> AddEdge for Vec<HashSet<usize, H>>
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
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].insert(t);
    }
}
