use std::{
    collections::HashMap,
    hash::{
        BuildHasher,
        Hash,
    },
};

pub trait CountAllVertices {
    fn count_all_vertices(&self) -> usize;
}

// Vec

impl<T> CountAllVertices for Vec<T> {
    /// # Complexity
    ///
    /// O(1)
    fn count_all_vertices(&self) -> usize {
        self.len()
    }
}

// Array

impl<const V: usize, T> CountAllVertices for [T; V] {
    /// # Complexity
    ///
    /// O(1)
    fn count_all_vertices(&self) -> usize {
        V
    }
}

// HashMap

impl<K, V, S> CountAllVertices for HashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    /// # Complexity
    ///
    /// O(1)
    fn count_all_vertices(&self) -> usize {
        self.len()
    }
}
