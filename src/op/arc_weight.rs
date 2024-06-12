//! Get the weight of an arc.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::ArcWeight,
//!     std::collections::HashMap,
//! };
//!
//! let digraph = vec![
//!     HashMap::from([(1, 2), (2, 3)]),
//!     HashMap::from([(0, 4)]),
//!     HashMap::from([(0, 7), (1, 8)]),
//! ];
//!
//! assert_eq!(digraph.arc_weight(0, 0), None);
//! assert_eq!(digraph.arc_weight(0, 1), Some(&2));
//! assert_eq!(digraph.arc_weight(0, 2), Some(&3));
//! assert_eq!(digraph.arc_weight(1, 0), Some(&4));
//! assert_eq!(digraph.arc_weight(1, 1), None);
//! assert_eq!(digraph.arc_weight(2, 0), Some(&7));
//! assert_eq!(digraph.arc_weight(2, 1), Some(&8));
//! assert_eq!(digraph.arc_weight(2, 2), None);
//! ```

use {
    crate::op::HasArc,
    core::hash::BuildHasher,
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Get the weight of an arc.
///
/// # How can I implement `ArcWeight`?
///
/// Provide an implementation of `arc_weight` that returns the weight of the
/// arc from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::ArcWeight,
///     std::collections::HashMap,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashMap<usize, usize>>,
/// }
///
/// impl ArcWeight<usize> for Digraph {
///     fn arc_weight(&self, s: usize, t: usize) -> Option<&usize> {
///         self.arcs.get(s).and_then(|m| m.get(&t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::ArcWeight,
///     std::collections::HashMap,
/// };
///
/// let digraph = vec![
///     HashMap::from([(1, 2), (2, 3)]),
///     HashMap::from([(0, 4)]),
///     HashMap::from([(0, 7), (1, 8)]),
/// ];
///
/// assert_eq!(digraph.arc_weight(0, 0), None);
/// assert_eq!(digraph.arc_weight(0, 1), Some(&2));
/// assert_eq!(digraph.arc_weight(0, 2), Some(&3));
/// assert_eq!(digraph.arc_weight(1, 0), Some(&4));
/// assert_eq!(digraph.arc_weight(1, 1), None);
/// assert_eq!(digraph.arc_weight(2, 0), Some(&7));
/// assert_eq!(digraph.arc_weight(2, 1), Some(&8));
/// assert_eq!(digraph.arc_weight(2, 2), None);
/// ```
pub trait ArcWeight<W> {
    /// Returns the weight of the arc from `s` to `t` if it exists in the
    /// digraph.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W>;
}

macro_rules! impl_weighted_usize {
    () => {
        fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
            self.get(s).and_then(|m| m.get(&t))
        }
    };
}

macro_rules! impl_weighted_ref_usize {
    () => {
        fn arc_weight(&self, s: usize, t: usize) -> Option<&W> {
            self.get(&s).and_then(|m| m.get(&t))
        }
    };
}

macro_rules! impl_unweighted {
    () => {
        fn arc_weight(&self, s: usize, t: usize) -> Option<&usize> {
            self.has_arc(s, t).then_some(&1)
        }
    };
}

impl ArcWeight<usize> for Vec<BTreeSet<usize>> {
    impl_unweighted!();
}

impl<H> ArcWeight<usize> for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_unweighted!();
}

impl ArcWeight<usize> for [BTreeSet<usize>] {
    impl_unweighted!();
}

impl<H> ArcWeight<usize> for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_unweighted!();
}

impl<const V: usize> ArcWeight<usize> for [BTreeSet<usize>; V] {
    impl_unweighted!();
}

impl<const V: usize, H> ArcWeight<usize> for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_unweighted!();
}

impl ArcWeight<usize> for BTreeMap<usize, BTreeSet<usize>> {
    impl_unweighted!();
}

impl<H> ArcWeight<usize> for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_unweighted!();
}

impl ArcWeight<usize> for BTreeSet<(usize, usize)> {
    impl_unweighted!();
}

impl<H> ArcWeight<usize> for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_unweighted!();
}

impl<W> ArcWeight<W> for Vec<BTreeMap<usize, W>> {
    impl_weighted_usize!();
}

impl<W, H> ArcWeight<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_weighted_usize!();
}

impl<W> ArcWeight<W> for [BTreeMap<usize, W>] {
    impl_weighted_usize!();
}

impl<W, H> ArcWeight<W> for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_weighted_usize!();
}

impl<const V: usize, W> ArcWeight<W> for [BTreeMap<usize, W>; V] {
    impl_weighted_usize!();
}

impl<const V: usize, W, H> ArcWeight<W> for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_weighted_usize!();
}

impl<W> ArcWeight<W> for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_weighted_ref_usize!();
}

impl<W, H> ArcWeight<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_weighted_ref_usize!();
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
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(1, 0, 4);
            $digraph.add_weighted_arc(2, 0, 7);
            $digraph.add_weighted_arc(2, 1, 8);
        };
    }

    macro_rules! test_unweighted {
        ($digraph:expr) => {
            assert_eq!($digraph.arc_weight(0, 0), None);
            assert_eq!($digraph.arc_weight(0, 1), Some(&1));
            assert_eq!($digraph.arc_weight(0, 2), Some(&1));
            assert_eq!($digraph.arc_weight(0, 3), None);
            assert_eq!($digraph.arc_weight(1, 0), Some(&1));
            assert_eq!($digraph.arc_weight(1, 1), None);
            assert_eq!($digraph.arc_weight(1, 2), None);
            assert_eq!($digraph.arc_weight(1, 3), None);
            assert_eq!($digraph.arc_weight(2, 0), Some(&1));
            assert_eq!($digraph.arc_weight(2, 1), Some(&1));
            assert_eq!($digraph.arc_weight(2, 2), None);
            assert_eq!($digraph.arc_weight(2, 3), None);
        };
    }

    macro_rules! test_weighted {
        ($digraph:expr) => {
            assert_eq!($digraph.arc_weight(0, 0), None);
            assert_eq!($digraph.arc_weight(0, 1), Some(&2));
            assert_eq!($digraph.arc_weight(0, 2), Some(&3));
            assert_eq!($digraph.arc_weight(0, 3), None);
            assert_eq!($digraph.arc_weight(1, 0), Some(&4));
            assert_eq!($digraph.arc_weight(1, 1), None);
            assert_eq!($digraph.arc_weight(1, 2), None);
            assert_eq!($digraph.arc_weight(1, 3), None);
            assert_eq!($digraph.arc_weight(2, 0), Some(&7));
            assert_eq!($digraph.arc_weight(2, 1), Some(&8));
            assert_eq!($digraph.arc_weight(2, 2), None);
            assert_eq!($digraph.arc_weight(2, 3), None);
        };
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, i32>; 3]>::empty();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, i32>; 3]>::empty();

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, i32>>::empty(3);

        setup_weighted!(digraph);
        test_weighted!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_unweighted!(digraph);
    }
}
