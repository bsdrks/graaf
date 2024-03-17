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
    /// May panic if `s` is out of bounds.
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
    /// May panic if `s` is out of bounds.
    ///
    /// # Complexity
    ///
    /// TODO: O(1)?
    fn is_edge(&self, s: usize, t: usize) -> bool {
        self[s].contains_key(&t)
    }
}
