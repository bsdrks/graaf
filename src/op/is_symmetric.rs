//! Determine whether a digraph is symmetric
//!
//! A digraph is symmetric if for every arc `(s, t)` there is an arc
//! `(t, s)`.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::op::IsSymmetric,
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

extern crate alloc;

use {
    super::{
        HasArc,
        IterArcs,
        IterWeightedArcs,
    },
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// Determine whether a digraph is symmetric
///
/// # How can I implement `IsSymmetric`?
///
/// Provide an implementation of `is_symmetric` that returns `true` if the
/// digraph is symmetric and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::{
///         HasArc,
///         IsSymmetric,
///         IterArcs,
///     },
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
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::IsSymmetric,
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

impl IsSymmetric for Vec<BTreeSet<usize>> {
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<H> IsSymmetric for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl IsSymmetric for [BTreeSet<usize>] {
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<H> IsSymmetric for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<const V: usize> IsSymmetric for [BTreeSet<usize>; V] {
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<const V: usize, H> IsSymmetric for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl IsSymmetric for BTreeMap<usize, BTreeSet<usize>> {
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<H> IsSymmetric for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_arcs().all(|(s, t)| self.has_arc(t, s))
    }
}

impl<W> IsSymmetric for Vec<BTreeMap<usize, W>> {
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W, H> IsSymmetric for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W> IsSymmetric for [BTreeMap<usize, W>] {
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W, H> IsSymmetric for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<const V: usize, W> IsSymmetric for [BTreeMap<usize, W>; V] {
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<const V: usize, W, H> IsSymmetric for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W> IsSymmetric for BTreeMap<usize, BTreeMap<usize, W>> {
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
    }
}

impl<W, H> IsSymmetric for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn is_symmetric(&self) -> bool {
        self.iter_weighted_arcs()
            .all(|(s, t, _)| self.has_arc(t, s))
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
    };

    // #[test]
    // fn vec_btree_set() {
    // }

    // #[test]
    // fn vec_hash_set() {
    // }

    // #[test]
    // fn slice_btree_set() {
    // }

    // #[test]
    // fn slice_hash_set() {
    // }

    // #[test]
    // fn arr_btree_set() {
    // }

    // #[test]
    // fn arr_hash_set() {
    // }

    // #[test]
    // fn btree_map_btree_set() {
    // }

    // #[test]
    // fn hash_map_hash_set() {
    // }

    // #[test]
    // fn vec_btree_map() {
    // }

    // #[test]
    // fn vec_hash_map() {
    // }

    // #[test]
    // fn slice_btree_map() {
    // }

    // #[test]
    // fn slice_hash_map() {
    // }

    // #[test]
    // fn arr_btree_map() {
    // }

    // #[test]
    // fn arr_hash_map() {
    // }

    // #[test]
    // fn btree_map_btree_map() {
    // }

    // #[test]
    // fn hash_map_hash_map() {
    // }

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
