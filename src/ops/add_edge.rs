use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

pub trait AddEdge {
    fn add_edge(&mut self, s: usize, t: usize);
}

impl AddEdge for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl<const V: usize> AddEdge for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds or if the new capacity of the vector
    /// exceeds `isize::MAX`.
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
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].insert(t);
    }
}

impl<const V: usize, H> AddEdge for [HashSet<usize, H>; V]
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
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].insert(t);
    }
}

impl<H> AddEdge for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// O(1)
    fn add_edge(&mut self, s: usize, t: usize) {
        self.entry(s).or_default().push(t);
    }
}

impl<H> AddEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
    HashSet<usize, H>: Default,
{
    /// # Complexity
    ///
    /// O(1)
    fn add_edge(&mut self, s: usize, t: usize) {
        self.entry(s).or_default().insert(t);
    }
}
