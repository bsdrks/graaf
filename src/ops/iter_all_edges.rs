use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

pub trait IterAllEdges {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)>;
}

// Vec

impl IterAllEdges for Vec<(usize, usize)> {
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

// Arr

impl<const V: usize> IterAllEdges for [(usize, usize); V] {
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

// HashMap

impl<H> IterAllEdges for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, t)| t.iter().copied().map(move |t| (*s, t)))
    }
}

impl<H> IterAllEdges for HashMap<usize, HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter()
            .flat_map(|(s, ts)| ts.iter().map(move |t| (*s, *t)))
    }
}
