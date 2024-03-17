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

impl<E> CountAllEdges for Vec<Vec<E>> {
    /// # Panics
    ///
    /// May panic if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(Vec::len).sum()
    }
}

impl<E, H> CountAllEdges for Vec<HashSet<E, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// May panic if the number of edges exceeds `usize::MAX`.
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
    /// May panic if the number of edges exceeds `usize::MAX`.
    ///
    /// # Complexity
    ///
    /// O(V)
    fn count_all_edges(&self) -> usize {
        self.iter().map(HashMap::len).sum()
    }
}
