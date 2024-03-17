use std::{
    collections::HashMap,
    hash::BuildHasher,
};

pub trait AddWeightedEdge<W> {
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W);
}

impl<W> AddWeightedEdge<W> for Vec<Vec<(usize, W)>> {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<const V: usize, W> AddWeightedEdge<W> for [Vec<(usize, W)>; V] {
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<W, H> AddWeightedEdge<W> for Vec<HashMap<usize, W, H>>
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
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].insert(t, w);
    }
}

impl<const V: usize, W, H> AddWeightedEdge<W> for [HashMap<usize, W, H>; V]
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
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].insert(t, w);
    }
}
