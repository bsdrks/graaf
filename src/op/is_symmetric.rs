//! Determine whether a digraph is symmetric.
//!
//! A digraph is symmetric if for every arc `(s, t)` there is an arc
//! `(t, s)`.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsSymmetric,
//!     std::collections::BTreeSet,
//! };
//!
//! let digraph = vec![BTreeSet::from([1]), BTreeSet::from([0])];
//!
//! assert!(digraph.is_symmetric());
//!
//! let digraph = vec![BTreeSet::from([1]), BTreeSet::new()];
//!
//! assert!(!digraph.is_symmetric());
//!
//! let digraph = vec![
//!     BTreeSet::from([1, 2]),
//!     BTreeSet::from([2]),
//!     BTreeSet::from([0]),
//! ];
//!
//! assert!(!digraph.is_symmetric());
//! ```

use super::{
    HasArc,
    IterArcs,
};

/// Determine whether a digraph is symmetric.
///
/// # How can I implement `IsSymmetric`?
///
/// Provide an implementation of `is_symmetric` that returns `true` if the
/// digraph is symmetric and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         HasArc,
///         IsSymmetric,
///         IterArcs,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsSymmetric for Digraph {
///     fn is_symmetric(&self) -> bool {
///         self.arcs.iter_arcs().all(|(s, t)| self.arcs.has_arc(t, s))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(digraph.is_symmetric());
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new()],
/// };
///
/// assert!(!digraph.is_symmetric());
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(!digraph.is_symmetric());
/// ```
///
/// # Examples
/// ```
/// use {
///     graaf::op::IsSymmetric,
///     std::collections::BTreeSet,
/// };
///
/// let digraph = vec![BTreeSet::from([1]), BTreeSet::from([0])];
///
/// assert!(digraph.is_symmetric());
///
/// let digraph = vec![BTreeSet::from([1]), BTreeSet::new()];
///
/// assert!(!digraph.is_symmetric());
///
/// let digraph = vec![
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([2]),
///     BTreeSet::from([0]),
/// ];
///
/// assert!(!digraph.is_symmetric());
/// ```
pub trait IsSymmetric {
    /// Returns whether the digraph is symmetric.
    fn is_symmetric(&self) -> bool;
}

impl<T> IsSymmetric for T
where
    T: HasArc + IterArcs + ?Sized,
{
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::{
                Cycle,
                CycleConst,
                Empty,
                EmptyConst,
            },
            op::{
                AddArc,
                AddWeightedArc,
            },
        },
        std::collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
            HashSet,
        },
    };

    macro_rules! setup_two_cycle_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(1, 0, 1);
        };
    }

    macro_rules! setup_asymmetric_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
        };
    }

    macro_rules! setup_asymmetric_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
        };
    }

    #[test]
    fn vec_btree_set_two_cycle() {
        let digraph = Vec::<BTreeSet<usize>>::cycle(2);

        assert!(digraph.is_symmetric());
    }

    #[test]
    fn vec_hash_set_two_cycle() {
        let digraph = Vec::<HashSet<usize>>::cycle(2);

        assert!(digraph.is_symmetric());
    }

    #[test]
    fn slice_btree_set_two_cycle() {
        let digraph = Vec::<BTreeSet<usize>>::cycle(2);

        assert!(digraph.as_slice().is_symmetric());
    }

    #[test]
    fn slice_hash_set_two_cycle() {
        let digraph = Vec::<HashSet<usize>>::cycle(2);

        assert!(digraph.as_slice().is_symmetric());
    }

    #[test]
    fn arr_btree_set_two_cycle() {
        let digraph = <[BTreeSet<usize>; 2]>::cycle();

        assert!(digraph.is_symmetric());
    }

    #[test]
    fn arr_hash_set_two_cycle() {
        let digraph = <[HashSet<usize>; 2]>::cycle();

        assert!(digraph.is_symmetric());
    }

    #[test]
    fn btree_map_btree_set_two_cycle() {
        let digraph = BTreeMap::<usize, BTreeSet<usize>>::cycle(2);

        assert!(digraph.is_symmetric());
    }

    #[test]
    fn hash_map_hash_set_two_cycle() {
        let digraph = HashMap::<usize, HashSet<usize>>::cycle(2);

        assert!(digraph.is_symmetric());
    }

    #[test]
    fn vec_btree_map_two_cycle() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(2);

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.is_symmetric());
    }

    #[test]
    fn vec_hash_map_two_cycle() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(2);

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.is_symmetric());
    }

    #[test]
    fn slice_btree_map_two_cycle() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(2);

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.as_slice().is_symmetric());
    }

    #[test]
    fn slice_hash_map_two_cycle() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(2);

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.as_slice().is_symmetric());
    }

    #[test]
    fn arr_btree_map_two_cycle() {
        let mut digraph = <[BTreeMap<usize, usize>; 2]>::empty();

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.is_symmetric());
    }

    #[test]
    fn arr_hash_map_two_cycle() {
        let mut digraph = <[HashMap<usize, usize>; 2]>::empty();

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.is_symmetric());
    }

    #[test]
    fn btree_map_btree_map_two_cycle() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(2);

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.is_symmetric());
    }

    #[test]
    fn hash_map_hash_map_two_cycle() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(2);

        setup_two_cycle_weighted!(digraph);
        assert!(digraph.is_symmetric());
    }

    #[test]
    fn vec_btree_set_asymmetric() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(2);

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn vec_hash_set_asymmetric() {
        let mut digraph = Vec::<HashSet<usize>>::empty(2);

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn slice_btree_set_asymmetric() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(2);

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.as_slice().is_symmetric());
    }

    #[test]
    fn slice_hash_set_asymmetric() {
        let mut digraph = Vec::<HashSet<usize>>::empty(2);

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.as_slice().is_symmetric());
    }

    #[test]
    fn arr_btree_set_asymmetric() {
        let mut digraph = <[BTreeSet<usize>; 2]>::empty();

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn arr_hash_set_asymmetric() {
        let mut digraph = <[HashSet<usize>; 2]>::empty();

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn btree_map_btree_set_asymmetric() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(2);

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn hash_map_hash_set_asymmetric() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(2);

        setup_asymmetric_unweighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn vec_btree_map_asymmetric() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(2);

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn vec_hash_map_asymmetric() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(2);

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn slice_btree_map_asymmetric() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(2);

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.as_slice().is_symmetric());
    }

    #[test]
    fn slice_hash_map_asymmetric() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(2);

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.as_slice().is_symmetric());
    }

    #[test]
    fn arr_btree_map_asymmetric() {
        let mut digraph = <[BTreeMap<usize, usize>; 2]>::empty();

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn arr_hash_map_asymmetric() {
        let mut digraph = <[HashMap<usize, usize>; 2]>::empty();

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn btree_map_btree_map_asymmetric() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(2);

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.is_symmetric());
    }

    #[test]
    fn hash_map_hash_map_asymmetric() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(2);

        setup_asymmetric_weighted!(digraph);
        assert!(!digraph.is_symmetric());
    }
}
