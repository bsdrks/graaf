//! A trait to iterate over all vertices in a graph
//!
//! # Example
//!
//! ```
//! use graaf::op::IterVertices;
//!
//! let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//! let mut iter = graph.iter_vertices();
//!
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(3));
//! assert_eq!(iter.next(), None);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to iterate over all vertices in a graph
///
/// # How can I implement `IterVertices`?
///
/// Provide an implementation of `iter_vertices` that returns an iterator over
/// all vertices in the graph.
///
/// ```
/// use graaf::op::IterVertices;
///
/// struct Graph {
///     vertices: Vec<usize>,
/// }
///
/// impl IterVertices for Graph {
///     fn iter_vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::op::IterVertices;
///
/// let graph: [Vec<usize>; 4] = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
/// let mut iter = graph.iter_vertices();
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterVertices {
    /// Returns an iterator over the vertices.
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

// Slice

impl<T> IterVertices for [T] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

// HashSet

impl<T, H> IterVertices for HashSet<T, H>
where
    H: BuildHasher,
{
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
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
    fn slice() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
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
    fn hash_set() {
        let graph: HashSet<usize> = HashSet::from([0, 1, 2, 3]);
        let mut iter = graph.iter_vertices();

        assert_matches!(iter.next(), Some(0..=3));
        assert_matches!(iter.next(), Some(0..=3));
        assert_matches!(iter.next(), Some(0..=3));
        assert_matches!(iter.next(), Some(0..=3));
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
