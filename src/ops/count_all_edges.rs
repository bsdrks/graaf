use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

pub trait CountAllEdges {
    fn count_all_edges(&self) -> usize;
}

// Vec

impl<T> CountAllEdges for Vec<Vec<T>> {
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<T, H> CountAllEdges for Vec<HashSet<T, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<W, H> CountAllEdges for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

// Array

impl<const V: usize, T> CountAllEdges for [Vec<T>; V] {
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<const V: usize, T, H> CountAllEdges for [HashSet<T, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashSet::len).sum()
    }
}

impl<const V: usize, W, H> CountAllEdges for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}

// HashMap

impl<K, V, H> CountAllEdges for HashMap<K, Vec<V>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.values().map(Vec::len).sum()
    }
}

impl<K, H> CountAllEdges for HashMap<K, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.values().map(HashSet::len).sum()
    }
}

impl<K, W, H> CountAllEdges for HashMap<K, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.values().map(HashMap::len).sum()
    }
}
