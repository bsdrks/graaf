//! A trait to determine whether a graph is simple
//!
//! A graph is simple if it has no self-loops or parallel edges.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsSimple,
//!     std::collections::HashSet,
//! };
//!
//! let graph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
//!
//! assert!(graph.is_simple());
//!
//! let graph = [
//!     HashSet::from([0, 1, 2]),
//!     HashSet::from([0, 2]),
//!     HashSet::from([0]),
//! ];
//!
//! assert!(!graph.is_simple());
//! ```

use {
    super::{
        IterAllEdges,
        IterAllWeightedEdges,
    },
    core::hash::BuildHasher,
    std::collections::HashSet,
};

/// A trait to determine whether a graph is simple
///
/// # How can I implement `IsSimple`?
///
/// Provide an implementation of `is_simple` that returns `true` if the graph is
/// simple and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::IsSimple,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl IsSimple for Graph {
///     fn is_simple(&self) -> bool {
///         self.edges
///             .iter()
///             .enumerate()
///             .all(|(s, set)| !set.contains(&s))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::IsSimple,
///     std::collections::HashSet,
/// };
///
/// let graph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
///
/// assert!(graph.is_simple());
///
/// let graph = [
///     HashSet::from([0, 1, 2]),
///     HashSet::from([0, 2]),
///     HashSet::from([0]),
/// ];
///
/// assert!(!graph.is_simple());
/// ```
pub trait IsSimple {
    /// Determine whether the graph is simple.
    fn is_simple(&self) -> bool;
}

impl<H> IsSimple for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for [(usize, usize)] {
    fn is_simple(&self) -> bool {
        self.iter_all_edges().all(|(s, t)| s != t)
    }
}

impl<W> IsSimple for [(usize, usize, W)] {
    fn is_simple(&self) -> bool {
        self.iter_all_weighted_edges().all(|(s, t, _)| s != t)
    }
}

impl<H> IsSimple for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter_all_edges().all(|(s, t)| s != t)
    }
}

impl<W, H> IsSimple for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter_all_weighted_edges().all(|(s, t, _)| s != t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_hash_set() {
        #[allow(clippy::useless_vec)]
        let graph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(graph.is_simple());

        #[allow(clippy::useless_vec)]
        #[rustfmt::skip]
        let graph = vec![
            HashSet::from([0, 1, 2]), // Self-loop {0, 0}
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_hash_set() {
        let graph: &[HashSet<usize>] = &[HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph: &[HashSet<usize>] = &[
            HashSet::from([0, 1, 2]), // Self-loop {0, 0}
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_hash_set() {
        let graph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph = [
            HashSet::from([0, 1, 2]), // Self-loop {0, 0}
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted() {
        #[allow(clippy::useless_vec)]
        let graph = vec![(1, 2), (2, 0), (0, 1)];

        assert!(graph.is_simple());

        #[allow(clippy::useless_vec)]
        #[rustfmt::skip]
        let graph = vec![
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted() {
        let graph: &[(usize, usize)] = &[(1, 2), (2, 0), (0, 1)];

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph: &[(usize, usize)] = &[
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_tuple_unweighted() {
        let graph = [(1, 2), (2, 0), (0, 1)];

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph = [
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn hash_set_tuple_unweighted() {
        let graph: HashSet<(usize, usize)> = HashSet::from([(1, 2), (2, 0), (0, 1)]);

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph: HashSet<(usize, usize)> = HashSet::from([
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ]);

        assert!(!graph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted() {
        #[allow(clippy::useless_vec)]
        let graph = vec![(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(graph.is_simple());

        #[allow(clippy::useless_vec)]
        #[rustfmt::skip]
        let graph = vec![
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1),
            (0, 2, 1)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted() {
        let graph: &[(usize, usize, usize)] = &[(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph: &[(usize, usize, usize)] = &[
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1),
            (0, 2, 1)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_tuple_weighted() {
        let graph = [(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph = [
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1), 
            (0, 2, 1) 
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted() {
        let graph: HashSet<(usize, usize, usize)> =
            HashSet::from([(1, 2, 1), (2, 0, 1), (0, 1, 1)]);

        assert!(graph.is_simple());

        #[rustfmt::skip]
        let graph: HashSet<(usize, usize, usize)> = HashSet::from([
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1), 
            (0, 2, 1) 
        ]);

        assert!(!graph.is_simple());
    }
}
