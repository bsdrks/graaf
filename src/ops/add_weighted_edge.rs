use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::{
        BuildHasher,
        Hash,
    },
};

pub trait AddWeightedEdge<W> {
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W);
}

// Vec

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

impl<W, H> AddWeightedEdge<W> for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
    W: Eq + Hash,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].insert((t, w));
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

// Arr

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

impl<const V: usize, W, H> AddWeightedEdge<W> for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
    W: Eq + Hash,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self[s].insert((t, w));
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

// HashMap

impl<W, H> AddWeightedEdge<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// O(1)
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self.entry(s).or_default().push((t, w));
    }
}

impl<W, H> AddWeightedEdge<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
    W: Eq + Hash,
    HashSet<(usize, W), H>: Default,
{
    /// # Complexity
    ///
    /// O(1)
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self.entry(s).or_default().insert((t, w));
    }
}

impl<W, H> AddWeightedEdge<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
    HashMap<usize, W, H>: Default,
{
    /// # Complexity
    ///
    /// O(1)
    fn add_weighted_edge(&mut self, s: usize, t: usize, w: W) {
        self.entry(s).or_default().insert(t, w);
    }
}
