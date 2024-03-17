use std::{
    collections::{
        HashMap,
        HashSet,
    },
    hash::BuildHasher,
};

pub trait InDegree {
    fn in_degree(&self, t: usize) -> usize;
}

impl<H> InDegree for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// TODO: O(E) or O(V)?
    fn in_degree(&self, t: usize) -> usize {
        self.iter().filter(|set| set.contains(&t)).count()
    }
}

impl<W, H> InDegree for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Complexity
    ///
    /// TODO: O(E) or O(V)?
    fn in_degree(&self, t: usize) -> usize {
        self.iter().filter(|map| map.contains_key(&t)).count()
    }
}
