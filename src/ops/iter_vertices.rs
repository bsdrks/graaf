use std::{
    collections::HashMap,
    hash::BuildHasher,
};

/// A trait for iterating over the vertices in a graph.
pub trait IterVertices {
    /// Returns an iterator over the vertices.
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

// Vec

impl<T> IterVertices for Vec<T> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

// Arr

impl<T, const V: usize> IterVertices for [T; V] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..V
    }
}

// HashMap

impl<V, S> IterVertices for HashMap<usize, V, S>
where
    S: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        self.keys().copied()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::assert_matches::assert_matches,
    };

    #[test]
    fn vec() {
        let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
        let mut iter = graph.iter_vertices();

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr() {
        let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
        let mut iter = graph.iter_vertices();

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map() {
        let graph = HashMap::from([
            (0, vec![1, 2]),
            (1, vec![0, 2, 3]),
            (2, vec![0, 1, 3]),
            (3, vec![1, 2]),
        ]);
        let mut iter = graph.iter_vertices();

        assert_matches!(iter.next(), Some(0..=3));
        assert_matches!(iter.next(), Some(0..=3));
        assert_matches!(iter.next(), Some(0..=3));
        assert_matches!(iter.next(), Some(0..=3));
        assert_eq!(iter.next(), None);
    }
}
