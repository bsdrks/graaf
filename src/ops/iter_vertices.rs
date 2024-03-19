use std::{
    collections::HashMap,
    hash::{
        BuildHasher,
        Hash,
    },
};

pub trait IterVertices {
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

// Vec

impl<T> IterVertices for Vec<T> {
    /// # Complexity
    ///
    /// O(V)
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

// Arr

impl<T, const V: usize> IterVertices for [T; V] {
    /// # Complexity
    ///
    /// O(V)
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..V
    }
}

// HashMap

impl<V, S> IterVertices for HashMap<usize, V, S>
where
    S: BuildHasher,
{
    /// # Complexity
    ///
    /// O(V)
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys().copied()
    }
}
