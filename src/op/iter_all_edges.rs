//! A trait to iterate over all unweighted edges in a graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterAllEdges;
//!
//! let graph = vec![(0, 1), (1, 2), (2, 0)];
//! let mut iter = graph.iter_all_edges();
//!
//! assert_eq!(iter.next(), Some((0, 1)));
//! assert_eq!(iter.next(), Some((1, 2)));
//! assert_eq!(iter.next(), Some((2, 0)));
//! assert_eq!(iter.next(), None);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::HashSet,
};

/// A trait to iterate over all unweighted edges in a graph
///
/// # How can I implement `IterAllEdges`?
///
/// Provide an implementation of `iter_all_edges` that returns an iterator over
/// all edges in the graph.
///
/// ```
/// use graaf::op::IterAllEdges;
///
/// struct Graph {
///     edges: Vec<(usize, usize)>,
/// }
///
/// impl IterAllEdges for Graph {
///     fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.edges.iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterAllEdges;
///
/// let graph = vec![(0, 1), (1, 2), (2, 0)];
/// let mut iter = graph.iter_all_edges();
///
/// assert_eq!(iter.next(), Some((0, 1)));
/// assert_eq!(iter.next(), Some((1, 2)));
/// assert_eq!(iter.next(), Some((2, 0)));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterAllEdges {
    /// Return an iterator that iterates over all edges in a graph.
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)>;
}

impl IterAllEdges for [(usize, usize)] {
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
    }
}

impl<H> IterAllEdges for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn iter_all_edges(&self) -> impl Iterator<Item = (usize, usize)> {
        self.iter().copied()
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
        #[allow(clippy::useless_vec)]
        let graph = vec![(0, 1), (1, 2), (2, 0)];
        let mut iter = graph.iter_all_edges();

        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn slice() {
        let graph: &[(usize, usize)] = &[(0, 1), (1, 2), (2, 0)];
        let mut iter = graph.iter_all_edges();

        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr() {
        let graph = [(0, 1), (1, 2), (2, 0)];
        let mut iter = graph.iter_all_edges();

        assert_eq!(iter.next(), Some((0, 1)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((2, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_set() {
        let graph: HashSet<(usize, usize)> = HashSet::from([(0, 1), (1, 2), (2, 0)]);
        let mut iter = graph.iter_all_edges();

        assert_matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0)));
        assert_matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0)));
        assert_matches!(iter.next(), Some((0, 1) | (1, 2) | (2, 0)));
        assert_eq!(iter.next(), None);
    }
}
