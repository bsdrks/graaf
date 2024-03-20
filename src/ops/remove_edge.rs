use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
    ops::IndexMut,
};

pub trait RemoveEdge {
    fn remove_edge(&mut self, s: usize, t: usize);
}

// Vec

impl<H> RemoveEdge for Vec<HashSet<usize, H>>
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
    fn remove_edge(&mut self, s: usize, t: usize) {
        self[s].remove(&t);
    }
}

impl<H, W> RemoveEdge for Vec<HashMap<usize, W, H>>
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
    fn remove_edge(&mut self, s: usize, t: usize) {
        self[s].remove(&t);
    }
}

// Arr

impl<const V: usize, H> RemoveEdge for [HashSet<usize, H>; V]
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
    fn remove_edge(&mut self, s: usize, t: usize) {
        self[s].remove(&t);
    }
}

impl<const V: usize, H, W> RemoveEdge for [HashMap<usize, W, H>; V]
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
    fn remove_edge(&mut self, s: usize, t: usize) {
        self[s].remove(&t);
    }
}

// HashMap

impl<H> RemoveEdge for HashMap<usize, HashSet<usize, H>>
where
    H: BuildHasher,
    Self: IndexMut<usize, Output = HashSet<usize, H>>,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn remove_edge(&mut self, s: usize, t: usize) {
        self[s].remove(&t);
    }
}

impl<H, W> RemoveEdge for HashMap<usize, HashMap<usize, W, H>>
where
    H: BuildHasher,
    Self: IndexMut<usize, Output = HashMap<usize, W, H>>,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn remove_edge(&mut self, s: usize, t: usize) {
        self[s].remove(&t);
    }
}
