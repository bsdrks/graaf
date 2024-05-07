//! A trait to iterate over all edges in a weighted directed graph
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterAllWeightedEdges;
//!
//! let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];
//!
//! assert!(graph
//!     .iter_all_weighted_edges()
//!     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)].into_iter()));
//! ```

extern crate alloc;

use {
    alloc::collections::BTreeSet,
    core::hash::BuildHasher,
    std::collections::HashSet,
};

/// A trait to iterate over all edges in a weighted directed graph
///
/// # How can I implement `IterAllWeightedEdges`?
///
/// Provide an implementation of `iter_all_weighted_edges` that returns an
/// iterator over all edges in a graph.
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
///
/// assert!(graph
///     .iter_all_weighted_edges()
///     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)].into_iter()));
/// ```
pub trait IterAllWeightedEdges<W> {
    /// Returns an iterator over all edges in a graph.
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a;
}

impl<W> IterAllWeightedEdges<W> for Vec<(usize, usize, W)> {
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W> IterAllWeightedEdges<W> for [(usize, usize, W)] {
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<const V: usize, W> IterAllWeightedEdges<W> for [(usize, usize, W); V] {
    fn iter_all_weighted_edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a,
    {
        self.iter().map(|(s, t, w)| (*s, *t, w))
    }
}

impl<W> IterAllWeightedEdges<W> for BTreeSet<(usize, usize, W)> {
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
    use super::*;

    macro_rules! test_iter_all_weighted_edges_stable {
        ($graph:expr) => {
            assert!($graph
                .iter_all_weighted_edges()
                .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)].into_iter()));
        };
    }

    macro_rules! test_iter_all_weighted_edges_unstable {
        ($graph:expr) => {
            let mut iter = $graph.iter_all_weighted_edges();

            assert!(matches!(
                iter.next(),
                Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4))
            ));

            assert!(matches!(
                iter.next(),
                Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4))
            ));

            assert!(matches!(
                iter.next(),
                Some((0, 1, &2) | (1, 2, &3) | (2, 0, &4))
            ));

            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec() {
        let graph = vec![(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        test_iter_all_weighted_edges_stable!(graph);
    }

    #[test]
    fn slice() {
        let graph: &[(usize, usize, usize)] = &[(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        test_iter_all_weighted_edges_stable!(graph);
    }

    #[test]
    fn arr() {
        let graph = [(0, 1, 2), (1, 2, 3), (2, 0, 4)];

        test_iter_all_weighted_edges_stable!(graph);
    }

    #[test]
    fn hash_set() {
        let graph: HashSet<(usize, usize, usize)> =
            HashSet::from([(0, 1, 2), (1, 2, 3), (2, 0, 4)]);

        test_iter_all_weighted_edges_unstable!(graph);
    }
}
