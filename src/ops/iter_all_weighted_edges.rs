use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

pub trait IterAllWeightedEdges<W> {
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)>;
}

// Vec

impl<W> IterAllWeightedEdges<W> for Vec<(usize, usize, W)>
where
    W: Copy,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)> {
        self.iter().copied()
    }
}

// Arr

impl<const V: usize, W> IterAllWeightedEdges<W> for [(usize, usize, W); V]
where
    W: Copy,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)> {
        self.iter().copied()
    }
}

// HashMap

impl<W, H> IterAllWeightedEdges<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)> {
        self.iter()
            .flat_map(|(s, ts)| ts.iter().copied().map(move |(t, w)| (*s, t, w)))
    }
}

impl<W, H> IterAllWeightedEdges<W> for HashMap<usize, HashSet<(usize, W)>, H>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)> {
        self.iter()
            .flat_map(|(s, ts)| ts.iter().copied().map(move |(t, w)| (*s, t, w)))
    }
}

impl<W, H> IterAllWeightedEdges<W> for HashMap<usize, HashMap<usize, W, H>>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_all_weighted_edges(&self) -> impl Iterator<Item = (usize, usize, W)> {
        self.iter()
            .flat_map(|(s, ts)| ts.iter().map(move |(t, w)| (*s, *t, *w)))
    }
}
