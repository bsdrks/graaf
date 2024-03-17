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
    /// Panics if `s >= self.vertex_count()` or if there is no edge `s -> t`.
    ///
    /// # Complexity
    ///
    /// O(1)
    fn edge_weight(&self, s: usize, t: usize) -> W {
        self[s][&t]
    }
}
