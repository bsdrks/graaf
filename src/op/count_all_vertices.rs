//! A trait to count all vertices in a graph
//!
//! # Example
//!
//! ```
//! use graaf::op::CountAllVertices;
//!
//! let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert_eq!(graph.count_all_vertices(), 4);
//! ```
use {
    core::hash::{
        BuildHasher,
        Hash,
    },
    std::collections::HashMap,
};

/// A trait to count all vertices in a graph
///
/// # How can I implement `CountAllVertices`?
///
/// Provide an implementation of `count_all_vertices` that returns the number of
/// vertices in the graph.
///
/// ```
/// use graaf::op::CountAllVertices;
///
/// struct Graph {
///     vertices: Vec<usize>,
/// }
///
/// impl CountAllVertices for Graph {
///     fn count_all_vertices(&self) -> usize {
///         self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::op::CountAllVertices;
///
/// let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert_eq!(graph.count_all_vertices(), 4);
/// ```
pub trait CountAllVertices {
    /// Counts all vertices.
    fn count_all_vertices(&self) -> usize;
}

// Vec

impl<T> CountAllVertices for Vec<T> {
    fn count_all_vertices(&self) -> usize {
        self.len()
    }
}

// Slice

impl<T> CountAllVertices for [T] {
    fn count_all_vertices(&self) -> usize {
        self.len()
    }
}

// Arr

impl<const V: usize, T> CountAllVertices for [T; V] {
    fn count_all_vertices(&self) -> usize {
        V
    }
}

// HashMap

impl<K, T, H> CountAllVertices for HashMap<K, T, H>
where
    K: Hash + Eq,
    H: BuildHasher,
{
    fn count_all_vertices(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_vertices(), 4);
    }

    #[test]
    fn slice() {
        let graph: &[Vec<usize>] = &[vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];

        assert_eq!(graph.count_all_vertices(), 4);
    }

    #[test]
    fn arr() {
        let graph = [(0, 1), (1, 2), (2, 0)];

        assert_eq!(graph.count_all_vertices(), 3);
    }

    #[test]
    fn hash_map() {
        let mut graph = HashMap::new();
        let _ = graph.insert(0, vec![1, 2]);
        let _ = graph.insert(1, vec![0, 2]);
        let _ = graph.insert(2, vec![0, 1]);

        assert_eq!(graph.count_all_vertices(), 3);
    }
}
