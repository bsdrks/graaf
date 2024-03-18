use std::{
    collections::HashMap,
    hash::BuildHasher,
};

pub trait EdgeWeight<W> {
    fn edge_weight(&self, s: usize, t: usize) -> W;
}

// Vec

impl<W, H> EdgeWeight<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds, or if there is no edge from `s` to
    /// `t`.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn edge_weight(&self, s: usize, t: usize) -> W {
        self[s][&t]
    }
}

// Arr

impl<const V: usize, W, H> EdgeWeight<W> for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is out of bounds, or if there is no edge from `s` to
    /// `t`.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn edge_weight(&self, s: usize, t: usize) -> W {
        self[s][&t]
    }
}

// HashMap

impl<W, H> EdgeWeight<W> for HashMap<usize, HashMap<usize, W, H>>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if there is no edge from `s` to `t`.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn edge_weight(&self, s: usize, t: usize) -> W {
        self[&s][&t]
    }
}
