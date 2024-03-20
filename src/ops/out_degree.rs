use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

/// A trait for getting the out-degree of a vertex.
pub trait OutDegree {
    fn out_degree(&self, s: usize) -> usize;
}

// Vec

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

// Arr

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

// HashMap

impl<H> OutDegree for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<H> OutDegree for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[&s].len()
    }
}

impl<H> OutDegree for HashMap<usize, HashMap<usize, usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn out_degree(&self, s: usize) -> usize {
        self[&s].len()
    }
}
