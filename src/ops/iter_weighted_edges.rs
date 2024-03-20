use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

pub trait IterWeightedEdges<W> {
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)>;
}

// Vec

impl<W> IterWeightedEdges<W> for Vec<Vec<(usize, W)>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

impl<W, H> IterWeightedEdges<W> for Vec<HashSet<(usize, W), H>>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

// Arr

impl<const V: usize, W> IterWeightedEdges<W> for [Vec<(usize, W)>; V]
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

impl<const V: usize, W> IterWeightedEdges<W> for [HashSet<(usize, W)>; V]
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

// HashMap

impl<W, H> IterWeightedEdges<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[&s].iter().copied()
    }
}

impl<W, H> IterWeightedEdges<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[&s].iter().copied()
    }
}

impl<W, H> IterWeightedEdges<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[&s].iter().map(|(&k, &v)| (k, v))
    }
}
