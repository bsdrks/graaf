//! Remove an arc from a digraph.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::RemoveArc,
//!     std::collections::HashSet,
//! };
//!
//! let mut digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([1]),
//! ];
//!
//! assert!(digraph.remove_arc(0, 1));
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! assert!(digraph.remove_arc(0, 2));
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! assert!(digraph.remove_arc(1, 0));
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
//! );
//!
//! digraph.remove_arc(2, 1);
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::new(), HashSet::new(), HashSet::new()]
//! );
//! ```

extern crate alloc;

use {
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

/// Remove an arc from a digraph.
///
/// # How can I implement `RemoveArc`?
///
/// Provide an implementation of `remove_arc` that removes the arc from `s` to
/// `t` from a digraph. Return whether the arc was removed.
///
/// ```
/// use {
///     graaf::op::RemoveArc,
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl RemoveArc for Digraph {
///     fn remove_arc(&mut self, s: usize, t: usize) -> bool {
///         self.arcs[s].remove(&t)
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::RemoveArc,
///     std::collections::HashSet,
/// };
///
/// let mut digraph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([1]),
/// ];
///
/// assert!(digraph.remove_arc(0, 1));
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// assert!(digraph.remove_arc(0, 2));
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// assert!(digraph.remove_arc(1, 0));
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
/// );
///
/// digraph.remove_arc(2, 1);
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::new(), HashSet::new(), HashSet::new()]
/// );
/// ```
///
/// # Properties
///
/// ## `RemoveArc` and `AddArc`
///
/// Types that also implement [`AddArc`] should ensure that
/// [`add_arc_remove_arc`] holds.
///
/// ## `RemoveArc` and `AddWeightedArc`
///
/// Types that also implement [`AddWeightedArc`] should ensure that
/// [`add_weighted_arc_remove_arc`] holds.
///
/// [`AddArc`]: crate::op::AddArc
/// [`AddWeightedArc`]: crate::op::AddWeightedArc
/// [`add_arc_remove_arc`]: crate::prop::add_arc_remove_arc
/// [`add_weighted_arc_remove_arc`]: crate::prop::add_weighted_arc_remove_arc
pub trait RemoveArc {
    /// Removes the arc from `s` to `t` from a digraph. Returns whether the
    /// arc was removed.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool;
}

macro_rules! impl_set_usize {
    () => {
        fn remove_arc(&mut self, s: usize, t: usize) -> bool {
            self.get_mut(s).map_or(false, |set| set.remove(&t))
        }
    };
}

macro_rules! impl_set_ref_usize {
    () => {
        fn remove_arc(&mut self, s: usize, t: usize) -> bool {
            self.get_mut(&s).map_or(false, |set| set.remove(&t))
        }
    };
}

macro_rules! impl_map_usize {
    () => {
        fn remove_arc(&mut self, s: usize, t: usize) -> bool {
            self.get_mut(s)
                .map_or(false, |map| map.remove(&t).is_some())
        }
    };
}

macro_rules! impl_map_ref_usize {
    () => {
        fn remove_arc(&mut self, s: usize, t: usize) -> bool {
            self.get_mut(&s)
                .map_or(false, |map| map.remove(&t).is_some())
        }
    };
}

macro_rules! impl_tuple {
    () => {
        fn remove_arc(&mut self, s: usize, t: usize) -> bool {
            self.remove(&(s, t))
        }
    };
}

impl RemoveArc for Vec<BTreeSet<usize>> {
    impl_set_usize!();
}

impl<H> RemoveArc for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_set_usize!();
}

impl RemoveArc for [BTreeSet<usize>] {
    impl_set_usize!();
}

impl<H> RemoveArc for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_set_usize!();
}

impl<const V: usize> RemoveArc for [BTreeSet<usize>; V] {
    impl_set_usize!();
}

impl<const V: usize, H> RemoveArc for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_set_usize!();
}

impl RemoveArc for BTreeMap<usize, BTreeSet<usize>> {
    impl_set_ref_usize!();
}

impl<H> RemoveArc for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_set_ref_usize!();
}

impl<W> RemoveArc for Vec<BTreeMap<usize, W>> {
    impl_map_usize!();
}

impl<W, H> RemoveArc for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_map_usize!();
}

impl<W> RemoveArc for [BTreeMap<usize, W>] {
    impl_map_usize!();
}

impl<W, H> RemoveArc for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_map_usize!();
}

impl<const V: usize, W> RemoveArc for [BTreeMap<usize, W>; V] {
    impl_map_usize!();
}

impl<const V: usize, W, H> RemoveArc for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_map_usize!();
}

impl<W> RemoveArc for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_map_ref_usize!();
}

impl<W, H> RemoveArc for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_map_ref_usize!();
}

impl RemoveArc for BTreeSet<(usize, usize)> {
    impl_tuple!();
}

impl<H> RemoveArc for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_tuple!();
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
                IterArcs,
            },
        },
    };

    macro_rules! setup_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 0);
            $digraph.add_arc(2, 1);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(1, 0, 4);
            $digraph.add_weighted_arc(2, 1, 5);
        };
    }

    macro_rules! test_stable {
        ($digraph:expr) => {
            assert!($digraph.remove_arc(2, 1));
            assert!($digraph.iter_arcs().eq([(0, 1), (0, 2), (1, 0)]));
            assert!($digraph.remove_arc(1, 0));
            assert!($digraph.iter_arcs().eq([(0, 1), (0, 2)]));
            assert!($digraph.remove_arc(0, 2));
            assert!($digraph.iter_arcs().eq([(0, 1)]));
            assert!($digraph.remove_arc(0, 1));
            assert!($digraph.iter_arcs().next().is_none());
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            assert!($digraph.remove_arc(2, 1));

            let mut iter = $digraph.iter_arcs();

            assert!(matches!(iter.next(), Some((0, 1 | 2) | (1, 0))));
            assert!(matches!(iter.next(), Some((0, 1 | 2) | (1, 0))));
            assert!(matches!(iter.next(), Some((0, 1 | 2) | (1, 0))));
            assert_eq!(iter.next(), None);

            drop(iter);

            assert!($digraph.remove_arc(1, 0));

            let mut iter = $digraph.iter_arcs();

            assert!(matches!(iter.next(), Some((0, 1 | 2))));
            assert!(matches!(iter.next(), Some((0, 1 | 2))));
            assert_eq!(iter.next(), None);

            drop(iter);

            assert!($digraph.remove_arc(0, 2));

            let mut iter = $digraph.iter_arcs();

            assert_eq!(iter.next(), Some((0, 1)));
            assert_eq!(iter.next(), None);

            drop(iter);

            assert!($digraph.remove_arc(0, 1));

            assert_eq!($digraph.iter_arcs().next(), None);
        };
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_unstable!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::from([(0, 1), (0, 2), (1, 0), (2, 1)]);

        test_stable!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::from([(0, 1), (0, 2), (1, 0), (2, 1)]);

        test_unstable!(digraph);
    }
}
