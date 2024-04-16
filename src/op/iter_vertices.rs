//! A trait to iterate over all vertices in a graph
//!
//! # Example
//!
//! ```
//! use graaf::op::IterVertices;
//!
//! let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//! let mut iter = graph.iter_vertices();
//!
//! assert_eq!(iter.next(), Some(0));
//! assert_eq!(iter.next(), Some(1));
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(3));
//! assert_eq!(iter.next(), None);
//! ```

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
/// let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
/// let mut iter = graph.iter_vertices();
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterVertices {
    /// Return an iterator over the vertices.
    fn iter_vertices(&self) -> impl Iterator<Item = usize>;
}

impl<T> IterVertices for Vec<T> {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<T> IterVertices for [T] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

impl<const V: usize, T> IterVertices for [T; V] {
    fn iter_vertices(&self) -> impl Iterator<Item = usize> {
        0..self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec() {
        let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
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
        let graph = [vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
        let mut iter = graph.iter_vertices();

        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}
