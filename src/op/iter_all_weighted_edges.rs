//! A trait to iterate over all weighted edges in a graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterAllWeightedEdges;
//!
//! let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
//! let mut iter = graph.iter_all_weighted_edges();
//!
//! assert_eq!(iter.next(), Some((0, 1, &2)));
//! assert_eq!(iter.next(), Some((1, 2, &3)));
//! assert_eq!(iter.next(), Some((2, 0, &4)));
//! assert_eq!(iter.next(), None);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::HashSet,
};

/// A trait to iterate over all weighted edges in a graph
///
/// # How can I implement `IterAllWeightedEdges`?
///
/// Provide an implementation of `iter_all_weighted_edges` that returns an
/// iterator over all weighted edges in the graph.
///
/// ```
/// use graaf::op::IterAllWeightedEdges;
///
/// struct Graph {
///     edges: Vec<(usize, usize, usize)>,
/// }
///
/// impl IterAllWeightedEdges<usize> for Graph {
///     fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a usize)>
///     where
///         usize: 'a,
///     {
///         self.edges.iter().map(|(s, t, w)| (*s, *t, w))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterAllWeightedEdges;
///
/// let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
/// let mut iter = graph.iter_all_weighted_edges();
///
/// assert_eq!(iter.next(), Some((0, 1, &2)));
/// assert_eq!(iter.next(), Some((1, 2, &3)));
/// assert_eq!(iter.next(), Some((2, 0, &4)));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterAllWeightedEdges<W> {
    /// Return an iterator that iterates over all weighted edges in a graph.
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a;
}

impl<W> IterAllWeightedEdges<W> for [(usize, usize, W)] {
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W, H> IterAllWeightedEdges<W> for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
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
        let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
        let mut iter = graph.iter_all_weighted_edges();

        assert_eq!(iter.next(), Some((0, 1, &2)));
        assert_eq!(iter.next(), Some((1, 2, &3)));
        assert_eq!(iter.next(), Some((2, 0, &4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn slice() {
        let graph: &[(usize, usize, usize)] = &[(0, 1, 2), (1, 2, 3), (2, 0, 4)];
        let mut iter = graph.iter_all_weighted_edges();

        assert_eq!(iter.next(), Some((0, 1, &2)));
        assert_eq!(iter.next(), Some((1, 2, &3)));
        assert_eq!(iter.next(), Some((2, 0, &4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr() {
        let graph = [(0, 1, 2), (1, 2, 3), (2, 0, 4)];
        let mut iter = graph.iter_all_weighted_edges();

        assert_eq!(iter.next(), Some((0, 1, &2)));
        assert_eq!(iter.next(), Some((1, 2, &3)));
        assert_eq!(iter.next(), Some((2, 0, &4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_set() {
        let graph: HashSet<(usize, usize, usize)> =
            HashSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        let mut iter = graph.iter_all_weighted_edges();

        assert_matches!(iter.next(), Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4)));
        assert_matches!(iter.next(), Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4)));
        assert_matches!(iter.next(), Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4)));
        assert_matches!(iter.next(), None);
    }
}
