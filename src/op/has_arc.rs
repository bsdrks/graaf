//! Check if an arc exists from one vertex to another.
//!
//! To check if an arc exists from `s` to `t` and from `t` to `s`, see
//! [`HasEdge`].
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::HasArc,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([0, 1]),
//! ];
//!
//! assert!(digraph.has_arc(0, 1));
//! assert!(digraph.has_arc(0, 2));
//! assert!(digraph.has_arc(1, 0));
//! assert!(digraph.has_arc(2, 0));
//! assert!(digraph.has_arc(2, 1));
//!
//! assert!(!digraph.has_arc(0, 0));
//! assert!(!digraph.has_arc(1, 1));
//! assert!(!digraph.has_arc(1, 2));
//! assert!(!digraph.has_arc(2, 2));
//! ```
//!
//! [`HasEdge`]: crate::op::HasEdge

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

/// Check if an arc exists from one vertex to another.
///
/// # How can I implement `HasArc`?
///
/// Provide an implementation of `has_arc` that returns `true` if there is an
/// arc from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::HasArc,
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, s: usize, t: usize) -> bool {
///         self.arcs.get(s).map_or(false, |set| set.contains(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::HasArc,
///     std::collections::HashSet,
/// };
///
/// let digraph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([0, 1]),
/// ];
///
/// assert!(digraph.has_arc(0, 1));
/// assert!(digraph.has_arc(0, 2));
/// assert!(digraph.has_arc(1, 0));
/// assert!(digraph.has_arc(2, 0));
/// assert!(digraph.has_arc(2, 1));
///
/// assert!(!digraph.has_arc(0, 0));
/// assert!(!digraph.has_arc(1, 1));
/// assert!(!digraph.has_arc(1, 2));
/// assert!(!digraph.has_arc(2, 2));
/// ```
///
/// # Properties
///
/// ## `HasArc` and `AddArc`
///
/// Types that also implement [`AddArc`] should ensure that
/// [`add_arc_has_arc`] holds.
///
/// ## `HasArc` and `AddWeightedArc`
///
/// Types that also implement [`AddWeightedArc`] should ensure that
/// [`add_weighted_arc_has_arc`] holds.
///
/// [`AddArc`]: crate::op::AddArc
/// [`AddWeightedArc`]: crate::op::AddWeightedArc
/// [`add_arc_has_arc`]: crate::prop::add_arc_has_arc
/// [`add_weighted_arc_has_arc`]: crate::prop::add_weighted_arc_has_arc
pub trait HasArc {
    /// Returns whether an arc exists from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    ///
    /// # Panics
    ///
    /// Implementations may not panic if `s` or `t` are not in the digraph.
    fn has_arc(&self, s: usize, t: usize) -> bool;
}

macro_rules! impl_set_usize {
    () => {
        fn has_arc(&self, s: usize, t: usize) -> bool {
            self.get(s).map_or(false, |set| set.contains(&t))
        }
    };
}

macro_rules! impl_set_ref_usize {
    () => {
        fn has_arc(&self, s: usize, t: usize) -> bool {
            self.get(&s).map_or(false, |set| set.contains(&t))
        }
    };
}

macro_rules! impl_map_usize {
    () => {
        fn has_arc(&self, s: usize, t: usize) -> bool {
            self.get(s).map_or(false, |map| map.contains_key(&t))
        }
    };
}

macro_rules! impl_map_ref_usize {
    () => {
        fn has_arc(&self, s: usize, t: usize) -> bool {
            self.get(&s).map_or(false, |map| map.contains_key(&t))
        }
    };
}

macro_rules! impl_tuple {
    () => {
        fn has_arc(&self, s: usize, t: usize) -> bool {
            self.contains(&(s, t))
        }
    };
}

impl HasArc for Vec<BTreeSet<usize>> {
    impl_set_usize!();
}

impl<H> HasArc for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_set_usize!();
}

impl HasArc for [BTreeSet<usize>] {
    impl_set_usize!();
}

impl HasArc for &[BTreeSet<usize>] {
    impl_set_usize!();
}

impl<H> HasArc for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_set_usize!();
}

impl<H> HasArc for &[HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_set_usize!();
}

impl<const V: usize> HasArc for [BTreeSet<usize>; V] {
    impl_set_usize!();
}

impl<const V: usize, H> HasArc for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_set_usize!();
}

impl HasArc for BTreeMap<usize, BTreeSet<usize>> {
    impl_set_ref_usize!();
}

impl<H> HasArc for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_set_ref_usize!();
}

impl<W> HasArc for Vec<BTreeMap<usize, W>> {
    impl_map_usize!();
}

impl<W, H> HasArc for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_map_usize!();
}

impl<W> HasArc for [BTreeMap<usize, W>] {
    impl_map_usize!();
}

impl<W> HasArc for &[BTreeMap<usize, W>] {
    impl_map_usize!();
}

impl<W, H> HasArc for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_map_usize!();
}

impl<W, H> HasArc for &[HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_map_usize!();
}

impl<const V: usize, W> HasArc for [BTreeMap<usize, W>; V] {
    impl_map_usize!();
}

impl<const V: usize, W, H> HasArc for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_map_usize!();
}

impl<W> HasArc for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_map_ref_usize!();
}

impl<W, H> HasArc for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_map_ref_usize!();
}

impl HasArc for BTreeSet<(usize, usize)> {
    impl_tuple!();
}

impl<H> HasArc for HashSet<(usize, usize), H>
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
            },
        },
    };

    macro_rules! setup_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 0);
            $digraph.add_arc(2, 0);
            $digraph.add_arc(2, 1);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 1);
            $digraph.add_weighted_arc(1, 0, 1);
            $digraph.add_weighted_arc(2, 0, 1);
            $digraph.add_weighted_arc(2, 1, 1);
        };
    }

    macro_rules! test_has_arc {
        ($digraph:expr) => {
            assert!(!$digraph.has_arc(0, 0));
            assert!($digraph.has_arc(0, 1));
            assert!($digraph.has_arc(0, 2));
            assert!($digraph.has_arc(1, 0));
            assert!(!$digraph.has_arc(1, 1));
            assert!(!$digraph.has_arc(1, 2));
            assert!($digraph.has_arc(2, 0));
            assert!($digraph.has_arc(2, 1));
            assert!(!$digraph.has_arc(2, 2));
        };
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, i32>; 3]>::empty();

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, i32>; 3]>::empty();

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn btree_set() {
        let mut digraph = BTreeSet::<(usize, usize)>::new();

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn hash_set() {
        let mut digraph = HashSet::<(usize, usize)>::new();

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }
}
