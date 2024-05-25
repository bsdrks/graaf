//! A trait to determine whether a digraph is simple
//!
//! A digraph is simple if it has no self-loops or parallel arcs.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsSimple,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
//!
//! assert!(digraph.is_simple());
//!
//! let digraph = [
//!     HashSet::from([0, 1, 2]),
//!     HashSet::from([0, 2]),
//!     HashSet::from([0]),
//! ];
//!
//! assert!(!digraph.is_simple());
//! ```

extern crate alloc;

use {
    super::{
        IterAllArcs,
        IterAllWeightedArcs,
    },
    alloc::collections::BTreeSet,
    core::hash::BuildHasher,
    std::collections::HashSet,
};

/// A trait to determine whether a digraph is simple
///
/// # How can I implement `IsSimple`?
///
/// Provide an implementation of `is_simple` that returns `true` if the digraph
/// is simple and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::IsSimple,
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl IsSimple for Digraph {
///     fn is_simple(&self) -> bool {
///         self.arcs
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
/// let digraph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];
///
/// assert!(digraph.is_simple());
///
/// let digraph = [
///     HashSet::from([0, 1, 2]),
///     HashSet::from([0, 2]),
///     HashSet::from([0]),
/// ];
///
/// assert!(!digraph.is_simple());
/// ```
pub trait IsSimple {
    /// Returns whether the digraph is simple.
    fn is_simple(&self) -> bool;
}

impl IsSimple for Vec<Vec<usize>> {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, vec)| {
            let mut set = HashSet::new();

            vec.iter().all(|&t| s != t && set.insert(t))
        })
    }
}

impl IsSimple for Vec<BTreeSet<usize>> {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl<H> IsSimple for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for [Vec<usize>] {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, vec)| {
            let mut set = HashSet::new();

            vec.iter().all(|&t| s != t && set.insert(t))
        })
    }
}

impl IsSimple for [BTreeSet<usize>] {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
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

impl<const V: usize> IsSimple for [Vec<usize>; V] {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, vec)| {
            let mut set = HashSet::new();

            vec.iter().all(|&t| s != t && set.insert(t))
        })
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

impl<const V: usize> IsSimple for [BTreeSet<usize>; V] {
    fn is_simple(&self) -> bool {
        self.iter().enumerate().all(|(s, set)| !set.contains(&s))
    }
}

impl IsSimple for BTreeSet<(usize, usize)> {
    fn is_simple(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| s != t)
    }
}

impl<H> IsSimple for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter_all_arcs().all(|(s, t)| s != t)
    }
}

impl IsSimple for Vec<(usize, usize)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_arcs()
            .all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl IsSimple for [(usize, usize)] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_arcs()
            .all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl<const V: usize> IsSimple for [(usize, usize); V] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_arcs()
            .all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for Vec<(usize, usize, W)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for [(usize, usize, W)] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<const V: usize, W> IsSimple for [(usize, usize, W); V] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for BTreeSet<(usize, usize, W)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<W, H> IsSimple for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_all_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec_simple() {
        let digraph = vec![vec![1, 2], vec![2], vec![]];

        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_vec_self_loop() {
        let digraph = vec![vec![0, 1, 2], vec![0, 2], vec![0]];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_vec_parallel_arcs() {
        let digraph = vec![vec![0, 1], vec![0, 1], vec![0]];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_btree_set_simple() {
        let digraph = vec![BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_btree_set_self_loop() {
        let digraph = vec![
            BTreeSet::from([0, 1, 2]),
            BTreeSet::from([0, 2]),
            BTreeSet::from([0]),
        ];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_hash_set_simple() {
        let digraph = vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_hash_set_self_loop() {
        let digraph = vec![
            HashSet::from([0, 1, 2]),
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_vec_simple() {
        let digraph: &[Vec<usize>] = &[vec![1, 2], vec![2], vec![]];

        assert!(digraph.is_simple());
    }

    #[test]
    fn slice_vec_self_loop() {
        let digraph: &[Vec<usize>] = &[vec![0, 1, 2], vec![0, 2], vec![0]];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_vec_parallel_arcs() {
        let digraph: &[Vec<usize>] = &[vec![0, 1], vec![0, 1], vec![0]];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_btree_set_simple() {
        let digraph: &[BTreeSet<usize>] =
            &[BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert!(digraph.is_simple());
    }

    #[test]
    fn slice_btree_set_self_loop() {
        let digraph: &[BTreeSet<usize>] = &[
            BTreeSet::from([0, 1, 2]),
            BTreeSet::from([0, 2]),
            BTreeSet::from([0]),
        ];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_hash_set_simple() {
        let digraph: &[HashSet<usize>] =
            &[HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(digraph.is_simple());
    }

    #[test]
    fn slice_hash_set_self_loop() {
        let digraph: &[HashSet<usize>] = &[
            HashSet::from([0, 1, 2]),
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_vec_simple() {
        let digraph = [vec![1, 2], vec![2], vec![]];

        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_vec_self_loop() {
        let digraph = [vec![0, 1, 2], vec![0, 2], vec![0]];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_vec_parallel_arcs() {
        let digraph = [vec![0, 1], vec![0, 1], vec![0]];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_btree_set_simple() {
        let digraph = [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_btree_set_self_loop() {
        let digraph = [
            BTreeSet::from([0, 1, 2]),
            BTreeSet::from([0, 2]),
            BTreeSet::from([0]),
        ];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_hash_set_simple() {
        let digraph = [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_hash_set_self_loop() {
        let digraph = [
            HashSet::from([0, 1, 2]),
            HashSet::from([0, 2]),
            HashSet::from([0]),
        ];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_simple() {
        let digraph = vec![(1, 2), (2, 0), (0, 1)];

        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_self_loop() {
        let digraph = vec![(0, 0), (0, 1), (0, 2)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_parallel_arcs() {
        let digraph = vec![(0, 1), (0, 1), (0, 2)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_simple() {
        let digraph: &[(usize, usize)] = &[(1, 2), (2, 0), (0, 1)];

        assert!(digraph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_self_loop() {
        let digraph: &[(usize, usize)] = &[(0, 0), (0, 1), (0, 2)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_parallel_arcs() {
        let digraph: &[(usize, usize)] = &[(0, 1), (0, 1), (0, 2)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_tuple_unweighted_simple() {
        let digraph = [(1, 2), (2, 0), (0, 1)];

        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_tuple_unweighted_self_loop() {
        let digraph = [(0, 0), (0, 1), (0, 2)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_tuple_unweighted_parallel_arcs() {
        let digraph = [(0, 1), (0, 1), (0, 2)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn btree_set_tuple_unweighted_simple() {
        let digraph: BTreeSet<(usize, usize)> = BTreeSet::from([(1, 2), (2, 0), (0, 1)]);

        assert!(digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_unweighted_simple() {
        let digraph: HashSet<(usize, usize)> = HashSet::from([(1, 2), (2, 0), (0, 1)]);

        assert!(digraph.is_simple());
    }

    #[test]
    fn btree_set_tuple_unweighted_self_loop() {
        let digraph: BTreeSet<(usize, usize)> = BTreeSet::from([(0, 0), (0, 1), (0, 2)]);

        assert!(!digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_unweighted_self_loop() {
        let digraph: HashSet<(usize, usize)> = HashSet::from([(0, 0), (0, 1), (0, 2)]);

        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_simple() {
        let digraph = vec![(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_self_loop() {
        let digraph = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_parallel_arcs() {
        let digraph = vec![(0, 1, 1), (0, 1, 1), (0, 2, 1)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted_simple() {
        let digraph: &[(usize, usize, usize)] = &[(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(digraph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted_self_loop() {
        let digraph: &[(usize, usize, usize)] = &[(0, 0, 1), (0, 1, 1), (0, 2, 1)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted_parallel_arcs() {
        let digraph: &[(usize, usize, usize)] = &[(0, 1, 1), (0, 1, 1), (0, 2, 1)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_tuple_weighted_simple() {
        let digraph = [(1, 2, 1), (2, 0, 1), (0, 1, 1)];

        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_tuple_weighted_self_loop() {
        let digraph = [(0, 0, 1), (0, 1, 1), (0, 2, 1)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_tuple_weighted_parallel_arcs() {
        let digraph = [(0, 1, 1), (0, 1, 1), (0, 2, 1)];

        assert!(!digraph.is_simple());
    }

    #[test]
    fn btree_set_tuple_weighted_simple() {
        let digraph: BTreeSet<(usize, usize, usize)> =
            BTreeSet::from([(1, 2, 1), (2, 0, 1), (0, 1, 1)]);

        assert!(digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_simple() {
        let digraph: HashSet<(usize, usize, usize)> =
            HashSet::from([(1, 2, 1), (2, 0, 1), (0, 1, 1)]);

        assert!(digraph.is_simple());
    }

    #[test]
    fn btree_set_tuple_weighted_self_loop() {
        let digraph: BTreeSet<(usize, usize, usize)> =
            BTreeSet::from([(0, 0, 1), (0, 1, 1), (0, 2, 1)]);

        assert!(!digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_self_loop() {
        let digraph: HashSet<(usize, usize, usize)> =
            HashSet::from([(0, 0, 1), (0, 1, 1), (0, 2, 1)]);

        assert!(!digraph.is_simple());
    }

    #[test]
    fn btree_set_tuple_weighted_parallel_arcs() {
        let digraph: BTreeSet<(usize, usize, usize)> =
            BTreeSet::from([(0, 1, 1), (0, 1, 2), (0, 2, 1)]);

        assert!(!digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_parallel_arcs() {
        let digraph: HashSet<(usize, usize, usize)> =
            HashSet::from([(0, 1, 1), (0, 1, 2), (0, 2, 1)]);

        assert!(!digraph.is_simple());
    }
}
