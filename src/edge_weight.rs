use std::{
    collections::HashMap,
    hash::BuildHasher,
};

pub trait EdgeWeight<W> {
    fn edge_weight(&self, s: usize, t: usize) -> W;
}

impl<W, H> EdgeWeight<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// May panic if `s` is out of bounds, or if there is no edge from `s` to
    /// `t`.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn edge_weight(&self, s: usize, t: usize) -> W {
        self[s][&t]
    }
}
