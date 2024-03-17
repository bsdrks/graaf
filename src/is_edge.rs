use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

pub trait IsEdge {
    fn is_edge(&self, s: usize, t: usize) -> bool;
}

impl<H> IsEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s >= self.vertex_count()`.
    ///
    /// # Complexity
    ///
    /// TODO
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self[s].contains(&t)
    }
}

impl<W, H> IsEdge for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s >= self.vertex_count()`.
    ///
    /// # Complexity
    ///
    /// TODO: O(1)?
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self[s].contains_key(&t)
    }
}
