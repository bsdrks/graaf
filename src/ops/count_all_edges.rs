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

impl<K, W, H> CountAllEdges for Vec<HashMap<K, W, H>>
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

impl<const V: usize, K, W, H> CountAllEdges for [HashMap<K, W, H>; V]
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

impl<K, T, H> CountAllEdges for HashMap<K, Vec<T>, H>
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

impl<K, T, H> CountAllEdges for HashMap<K, HashSet<T, H>, H>
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

impl<K, W, H> CountAllEdges for HashMap<K, HashMap<K, W, H>, H>
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
