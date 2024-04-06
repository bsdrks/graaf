//! A trait to iterate over all weighted edges with a given source vertex
//!
//! # Examples
//!
//! ```
//! use graaf::op::IterWeightedEdges;
//!
//! let graph = vec![
//!     vec![(1, 2), (2, 3), (3, 4)],
//!     vec![(2, 3), (3, 4), (4, 5)],
//!     vec![(3, 4), (4, 5), (5, 6)],
//! ];
//!
//! let mut iter = graph.iter_weighted_edges(0);
//!
//! assert_eq!(iter.next(), Some((1, 2)));
//! assert_eq!(iter.next(), Some((2, 3)));
//! assert_eq!(iter.next(), Some((3, 4)));
//! assert_eq!(iter.next(), None);
//!
//! let mut iter = graph.iter_weighted_edges(1);
//!
//! assert_eq!(iter.next(), Some((2, 3)));
//! assert_eq!(iter.next(), Some((3, 4)));
//! assert_eq!(iter.next(), Some((4, 5)));
//! assert_eq!(iter.next(), None);
//!
//! let mut iter = graph.iter_weighted_edges(2);
//!
//! assert_eq!(iter.next(), Some((3, 4)));
//! assert_eq!(iter.next(), Some((4, 5)));
//! assert_eq!(iter.next(), Some((5, 6)));
//! assert_eq!(iter.next(), None);
//! ```

use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to iterate over all weighted edges with a given source vertex
///
/// # How can I implement `IterWeightedEdges`?
///
/// Provide an implementation of `iter_weighted_edges` that returns an iterator
/// over all weighted edges with the source vertex `s`.
///
/// ```
/// use graaf::op::IterWeightedEdges;
///
/// struct Graph {
///     edges: Vec<Vec<(usize, usize)>>,
/// }
///
/// impl IterWeightedEdges<usize> for Graph {
///     fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, usize)> {
///         self.edges[s].iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::IterWeightedEdges;
///
/// let graph = vec![
///     vec![(1, 2), (2, 3), (3, 4)],
///     vec![(2, 3), (3, 4), (4, 5)],
///     vec![(3, 4), (4, 5), (5, 6)],
/// ];
///
/// let mut iter = graph.iter_weighted_edges(0);
///
/// assert_eq!(iter.next(), Some((1, 2)));
/// assert_eq!(iter.next(), Some((2, 3)));
/// assert_eq!(iter.next(), Some((3, 4)));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_weighted_edges(1);
///
/// assert_eq!(iter.next(), Some((2, 3)));
/// assert_eq!(iter.next(), Some((3, 4)));
/// assert_eq!(iter.next(), Some((4, 5)));
/// assert_eq!(iter.next(), None);
///
/// let mut iter = graph.iter_weighted_edges(2);
///
/// assert_eq!(iter.next(), Some((3, 4)));
/// assert_eq!(iter.next(), Some((4, 5)));
/// assert_eq!(iter.next(), Some((5, 6)));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterWeightedEdges<W> {
    /// Returns an iterator over the edges of the vertex `s`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)>;
}

// Vec

impl<W> IterWeightedEdges<W> for Vec<Vec<(usize, W)>>
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

impl<W, H> IterWeightedEdges<W> for Vec<HashSet<(usize, W), H>>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

impl<W, H> IterWeightedEdges<W> for Vec<HashMap<usize, W, H>>
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().map(|(k, v)| (*k, *v))
    }
}

// Arr

impl<const V: usize, W> IterWeightedEdges<W> for [Vec<(usize, W)>; V]
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

impl<W, H, const V: usize> IterWeightedEdges<W> for [HashSet<(usize, W), H>; V]
where
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().copied()
    }
}

impl<W, H, const V: usize> IterWeightedEdges<W> for [HashMap<usize, W, H>; V]
where
    W: Copy,
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[s].iter().map(|(k, v)| (*k, *v))
    }
}

// HashMap

impl<W, H> IterWeightedEdges<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[&s].iter().copied()
    }
}

impl<W, H> IterWeightedEdges<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[&s].iter().copied()
    }
}

impl<W, H> IterWeightedEdges<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
    W: Copy,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn iter_weighted_edges(&self, s: usize) -> impl Iterator<Item = (usize, W)> {
        self[&s].iter().map(|(&k, &v)| (k, v))
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn vec_vec() {
        let graph = vec![
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        let mut iter = graph.iter_weighted_edges(1);

        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.next(), Some((3, 4)));
        assert_eq!(iter.next(), Some((4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vec_hash_set() {
        let graph = vec![
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        let mut iter = graph.iter_weighted_edges(1);

        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vec_hash_map() {
        let graph = vec![
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        let mut iter = graph.iter_weighted_edges(1);

        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr_vec() {
        let graph = [
            vec![(1, 2), (2, 3), (3, 4)],
            vec![(2, 3), (3, 4), (4, 5)],
            vec![(3, 4), (4, 5), (5, 6)],
        ];

        let mut iter = graph.iter_weighted_edges(1);

        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.next(), Some((3, 4)));
        assert_eq!(iter.next(), Some((4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr_hash_set() {
        let graph = [
            HashSet::from([(1, 2), (2, 3), (3, 4)]),
            HashSet::from([(2, 3), (3, 4), (4, 5)]),
            HashSet::from([(3, 4), (4, 5), (5, 6)]),
        ];

        let mut iter = graph.iter_weighted_edges(1);

        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arr_hash_map() {
        let graph = [
            HashMap::from([(1, 2), (2, 3), (3, 4)]),
            HashMap::from([(2, 3), (3, 4), (4, 5)]),
            HashMap::from([(3, 4), (4, 5), (5, 6)]),
        ];

        let mut iter = graph.iter_weighted_edges(1);

        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_vec() {
        let graph = HashMap::from([
            (0, vec![(1, 2), (2, 3), (3, 4)]),
            (1, vec![(2, 3), (3, 4), (4, 5)]),
            (2, vec![(3, 4), (4, 5), (5, 6)]),
        ]);

        let mut iter = graph.iter_weighted_edges(1);

        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.next(), Some((3, 4)));
        assert_eq!(iter.next(), Some((4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_hash_set() {
        let graph = HashMap::from([
            (0, HashSet::from([(1, 2), (2, 3), (3, 4)])),
            (1, HashSet::from([(2, 3), (3, 4), (4, 5)])),
            (2, HashSet::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        let mut iter = graph.iter_weighted_edges(1);

        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn hash_map_hash_map() {
        let graph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3), (3, 4)])),
            (1, HashMap::from([(2, 3), (3, 4), (4, 5)])),
            (2, HashMap::from([(3, 4), (4, 5), (5, 6)])),
        ]);

        let mut iter = graph.iter_weighted_edges(1);

        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_matches!(iter.next(), Some((2, 3) | (3, 4) | (4, 5)));
        assert_eq!(iter.next(), None);
    }
}
