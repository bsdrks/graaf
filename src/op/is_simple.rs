//! Determine whether a digraph is simple
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
        IterArcs,
        IterWeightedArcs,
    },
    alloc::collections::BTreeSet,
    core::hash::BuildHasher,
    std::collections::HashSet,
};

/// Determine whether a digraph is simple
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
        self.iter_arcs().all(|(s, t)| s != t)
    }
}

impl<H> IsSimple for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        self.iter_arcs().all(|(s, t)| s != t)
    }
}

impl IsSimple for Vec<(usize, usize)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_arcs().all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl IsSimple for [(usize, usize)] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_arcs().all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl<const V: usize> IsSimple for [(usize, usize); V] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_arcs().all(|(s, t)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for Vec<(usize, usize, W)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for [(usize, usize, W)] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<const V: usize, W> IsSimple for [(usize, usize, W); V] {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<W> IsSimple for BTreeSet<(usize, usize, W)> {
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

impl<W, H> IsSimple for HashSet<(usize, usize, W), H>
where
    H: BuildHasher,
{
    fn is_simple(&self) -> bool {
        let mut set = HashSet::new();

        self.iter_weighted_arcs()
            .all(|(s, t, _)| s != t && set.insert((s, t)))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::{
                Empty,
                EmptyConst,
            },
            op::{
                AddArc,
                AddWeightedArc,
            },
        },
    };

    macro_rules! setup_simple_unweighted {
        ($digraph:ident) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 2);
        };
    }

    macro_rules! setup_self_loop_unweighted {
        ($digraph:ident) => {
            $digraph.add_arc(0, 0);
            $digraph.add_arc(1, 0);
            $digraph.add_arc(1, 2);
        };
    }

    macro_rules! setup_parallel_arcs_unweighted {
        ($digraph:ident) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 1);
            $digraph.add_arc(1, 2);
        };
    }

    macro_rules! setup_simple_weighted {
        ($digraph:ident) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 1);
            $digraph.add_weighted_arc(1, 2, 1);
        };
    }

    macro_rules! setup_self_loop_weighted {
        ($digraph:ident) => {
            $digraph.add_weighted_arc(0, 0, 1);
            $digraph.add_weighted_arc(1, 0, 1);
            $digraph.add_weighted_arc(1, 2, 1);
        };
    }

    macro_rules! setup_parallel_arcs_weighted {
        ($digraph:ident) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(1, 2, 1);
        };
    }

    #[test]
    fn vec_vec_simple() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_vec_self_loop() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_vec_parallel_arcs() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_parallel_arcs_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_btree_set_simple() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_btree_set_self_loop() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_hash_set_simple() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_hash_set_self_loop() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_vec_simple() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_vec_self_loop() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_vec_parallel_arcs() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        setup_parallel_arcs_unweighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_btree_set_simple() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_btree_set_self_loop() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_hash_set_simple() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_hash_set_self_loop() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
    }

    #[test]
    fn arr_vec_simple() {
        let mut digraph = <[Vec<usize>; 3]>::empty();

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_vec_self_loop() {
        let mut digraph = <[Vec<usize>; 3]>::empty();

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_vec_parallel_arcs() {
        let mut digraph = <[Vec<usize>; 3]>::empty();

        setup_parallel_arcs_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_btree_set_simple() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_btree_set_self_loop() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn arr_hash_set_simple() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn arr_hash_set_self_loop() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_simple() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_self_loop() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_unweighted_parallel_arcs() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_parallel_arcs_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_simple() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_self_loop() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_tuple_unweighted_parallel_arcs() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        setup_parallel_arcs_unweighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
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
        let mut digraph = BTreeSet::<(usize, usize)>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn btree_set_tuple_unweighted_self_loop() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_unweighted_simple() {
        let mut digraph = HashSet::<(usize, usize)>::empty(3);

        setup_simple_unweighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_unweighted_self_loop() {
        let mut digraph = HashSet::<(usize, usize)>::empty(3);

        setup_self_loop_unweighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_simple() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_simple_weighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_self_loop() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_self_loop_weighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn vec_tuple_weighted_parallel_arcs() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_parallel_arcs_weighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn slice_tuple_weighted_simple() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_simple_weighted!(digraph);
        assert!(digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_tuple_weighted_self_loop() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_self_loop_weighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
    }

    #[test]
    fn slice_tuple_weighted_parallel_arcs() {
        let mut digraph = Vec::<(usize, usize, usize)>::empty(3);

        setup_parallel_arcs_weighted!(digraph);
        assert!(!digraph.as_slice().is_simple());
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
        let mut digraph = BTreeSet::<(usize, usize, usize)>::empty(3);

        setup_simple_weighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn btree_set_tuple_weighted_self_loop() {
        let mut digraph = BTreeSet::<(usize, usize, usize)>::empty(3);

        setup_self_loop_weighted!(digraph);
        assert!(!digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_simple() {
        let mut digraph = HashSet::<(usize, usize, usize)>::empty(3);

        setup_simple_weighted!(digraph);
        assert!(digraph.is_simple());
    }

    #[test]
    fn hash_set_tuple_weighted_self_loop() {
        let mut digraph = HashSet::<(usize, usize, usize)>::empty(3);

        setup_self_loop_weighted!(digraph);
        assert!(!digraph.is_simple());
    }
}
