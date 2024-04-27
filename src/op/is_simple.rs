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

extern crate alloc;

use {
    super::{
        IterAllEdges,
        IterAllWeightedEdges,
    },
    alloc::collections::BTreeSet,
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

impl<H> IsSimple for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for Vec<BTreeSet<usize>> {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for Vec<(usize, usize)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_edges()
            .all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for Vec<(usize, usize, W)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_edges()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<H> IsSimple for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for [BTreeSet<usize>] {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for [(usize, usize)] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_edges()
            .all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for [(usize, usize, W)] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_edges()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<const V: usize, H> IsSimple for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for [BTreeSet<usize>; 3] {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl<const V: usize> IsSimple for [(usize, usize); V] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_edges()
            .all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl<const V: usize, W> IsSimple for [(usize, usize, W); V] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_edges()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl IsSimple for BTreeSet<(usize, usize)> {
    fn is_simple(&self) -> bool {
        self.iter_all_edges().all(|(s, t)| s != t)
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

impl<W> IsSimple for BTreeSet<(usize, usize, W)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_edges()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<W, H> IsSimple for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_edges()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_btree_set_simple() {
        let graph = vec![BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert!(graph.is_simple());
    }

    #[test]
    fn vec_hash_set_simple() {
        let graph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(graph.is_simple());
    }

    #[test]
    fn vec_btree_set_self_loop() {
        #[rustfmt::skip]
        let graph = vec![
            BTreeSet::from([0, 1, 2]), // Self-loop {0, 0}
            BTreeSet::from([0, 2]),
            BTreeSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn vec_hash_set_self_loop() {
        #[rustfmt::skip]
        let graph = vec![
            HashSet::from([0, 1, 2]), // Self-loop {0, 0}
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_btree_set_simple() {
        let graph: &[BTreeSet<usize>] =
            &[BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert!(graph.is_simple());
    }

    #[test]
    fn slice_hash_set_simple() {
        let graph: &[HashSet<usize>] = &[HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(graph.is_simple());
    }

    #[test]
    fn slice_btree_set_self_loop() {
        #[rustfmt::skip]
        let graph: &[BTreeSet<usize>] = &[
            BTreeSet::from([0, 1, 2]), // Self-loop {0, 0}
            BTreeSet::from([0, 2]),
            BTreeSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_hash_set_self_loop() {
        #[rustfmt::skip]
        let graph: &[HashSet<usize>] = &[
            HashSet::from([0, 1, 2]), // Self-loop {0, 0}
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_btree_set_simple() {
        let graph = [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert!(graph.is_simple());
    }

    #[test]
    fn arr_hash_set_simple() {
        let graph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(graph.is_simple());
    }

    #[test]
    fn arr_btree_set_self_loop() {
        #[rustfmt::skip]
        let graph = [
            BTreeSet::from([0, 1, 2]), // Self-loop {0, 0}
            BTreeSet::from([0, 2]),
            BTreeSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_hash_set_self_loop() {
        #[rustfmt::skip]
        let graph = [
            HashSet::from([0, 1, 2]), // Self-loop {0, 0}
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_simple() {
        let graph = vec![(1, 2), (2, 0), (0, 1)];

        assert!(graph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_self_loop() {
        #[rustfmt::skip]
        let graph = vec![
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_parallel_edges() {
        #[rustfmt::skip]
        let graph = vec![
            (0, 1), // Parallel edge {0, 1}
            (0, 1), // Parallel edge {0, 1}
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_simple() {
        let graph: &[(usize, usize)] = &[(1, 2), (2, 0), (0, 1)];

        assert!(graph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_self_loop() {
        #[rustfmt::skip]
        let graph: &[(usize, usize)] = &[
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_parallel_edges() {
        #[rustfmt::skip]
        let graph: &[(usize, usize)] = &[
            (0, 1), // Parallel edge {0, 1}
            (0, 1), // Parallel edge {0, 1}
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_tuple_unweighted_simple() {
        let graph = [(1, 2), (2, 0), (0, 1)];

        assert!(graph.is_simple());
    }

    #[test]
    fn arr_tuple_unweighted_self_loop() {
        #[rustfmt::skip]
        let graph = [
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_tuple_unweighted_parallel_edges() {
        #[rustfmt::skip]
        let graph = [
            (0, 1), // Parallel edge {0, 1}
            (0, 1), // Parallel edge {0, 1}
            (0, 2)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn btree_set_tuple_unweighted_simple() {
        let graph: BTreeSet<(usize, usize)> = BTreeSet::from([(1, 2), (2, 0), (0, 1)]);

        assert!(graph.is_simple());
    }

    #[test]
    fn hash_set_tuple_unweighted_simple() {
        let graph: HashSet<(usize, usize)> = HashSet::from([(1, 2), (2, 0), (0, 1)]);

        assert!(graph.is_simple());
    }

    #[test]
    fn btree_set_tuple_unweighted_self_loop() {
        #[rustfmt::skip]
        let graph: BTreeSet<(usize, usize)> = BTreeSet::from([
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ]);

        assert!(!graph.is_simple());
    }

    #[test]
    fn hash_set_tuple_unweighted_self_loop() {
        #[rustfmt::skip]
        let graph: HashSet<(usize, usize)> = HashSet::from([
            (0, 0), // Self-loop {0, 0}
            (0, 1),
            (0, 2)
        ]);

        assert!(!graph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_simple() {
        let graph = vec![(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(graph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_self_loop() {
        #[rustfmt::skip]
        let graph = vec![
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1),
            (0, 2, 1)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_parallel_edges() {
        #[rustfmt::skip]
        let graph = vec![
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 2, 1)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted_simple() {
        let graph: &[(usize, usize, usize)] = &[(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(graph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted_self_loop() {
        #[rustfmt::skip]
        let graph: &[(usize, usize, usize)] = &[
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1),
            (0, 2, 1)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted_parallel_edges() {
        #[rustfmt::skip]
        let graph: &[(usize, usize, usize)] = &[
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 2, 1)
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_tuple_weighted_simple() {
        let graph = [(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(graph.is_simple());
    }

    #[test]
    fn arr_tuple_weighted_self_loop() {
        #[rustfmt::skip]
        let graph = [
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1), 
            (0, 2, 1) 
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn arr_tuple_weighted_parallel_edges() {
        #[rustfmt::skip]
        let graph = [
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 2, 1) 
        ];

        assert!(!graph.is_simple());
    }

    #[test]
    fn btree_set_tuple_weighted_simple() {
        let graph: BTreeSet<(usize, usize, usize)> =
            BTreeSet::from([(1, 2, 1), (2, 0, 1), (0, 1, 1)]);

        assert!(graph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_simple() {
        let graph: HashSet<(usize, usize, usize)> =
            HashSet::from([(1, 2, 1), (2, 0, 1), (0, 1, 1)]);

        assert!(graph.is_simple());
    }

    #[test]
    fn btree_set_tuple_weighted_self_loop() {
        #[rustfmt::skip]
        let graph: BTreeSet<(usize, usize, usize)> = BTreeSet::from([
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1), 
            (0, 2, 1) 
        ]);

        assert!(!graph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_self_loop() {
        #[rustfmt::skip]
        let graph: HashSet<(usize, usize, usize)> = HashSet::from([
            (0, 0, 1), // Self-loop {0, 0}
            (0, 1, 1), 
            (0, 2, 1) 
        ]);

        assert!(!graph.is_simple());
    }

    #[test]
    fn btree_set_tuple_weighted_parallel_edges() {
        #[rustfmt::skip]
        let graph: BTreeSet<(usize, usize, usize)> = BTreeSet::from([
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 1, 2), // Parallel edge {0, 1}
            (0, 2, 1) 
        ]);

        assert!(!graph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_parallel_edges() {
        #[rustfmt::skip]
        let graph: HashSet<(usize, usize, usize)> = HashSet::from([
            (0, 1, 1), // Parallel edge {0, 1}
            (0, 1, 2), // Parallel edge {0, 1}
            (0, 2, 1) 
        ]);

        assert!(!graph.is_simple());
    }
}
