//! Check whether a sequence of vertices is a walk in a digraph.
//!
//! A sequence of vertices is a walk in a digraph if each pair of consecutive
//! vertices in the sequence is an arc in the digraph.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::Empty,
//!         op::{
//!             AddArc,
//!             IsWalk,
//!         },
//!     },
//! };
//! ```

extern crate alloc;

use {
    super::HasArc,
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

/// Check whether a sequence of vertices is a walk in a digraph.
///
/// # How do I implement `IsWalk`?
///
/// Provide an implementation of `is_walk` that returns `true` if each pair of
/// consecutive vertices in the sequence is an arc in the digraph and `false`
/// otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::{
///         HasArc,
///         IsWalk,
///     },
/// };
///
/// struct Digraph {
///     pub arcs: BTreeSet<(usize, usize)>,
/// }
///
/// impl IsWalk for Digraph {
///     fn is_walk(&self, walk: &[usize]) -> bool {
///         let mut arcs = walk.iter().zip(walk.iter().skip(1));
///
///         arcs.clone().count() > 0 && arcs.all(|(s, t)| self.arcs.has_arc(*s, *t))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::Empty,
///         op::{
///             AddArc,
///             IsWalk,
///         },
///     },
/// };
/// ```
pub trait IsWalk {
    /// Returns whether the sequence of vertices is a walk in the digraph.
    fn is_walk(&self, walk: &[usize]) -> bool;
}

macro_rules! impl_is_walk {
    () => {
        fn is_walk(&self, walk: &[usize]) -> bool {
            let mut arcs = walk.iter().zip(walk.iter().skip(1));

            arcs.clone().count() > 0 && arcs.all(|(s, t)| self.has_arc(*s, *t))
        }
    };
}

impl IsWalk for Vec<BTreeSet<usize>> {
    impl_is_walk!();
}

impl<H> IsWalk for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl IsWalk for [BTreeSet<usize>] {
    impl_is_walk!();
}

impl<H> IsWalk for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl<const V: usize> IsWalk for [BTreeSet<usize>; V] {
    impl_is_walk!();
}

impl<const V: usize, H> IsWalk for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl IsWalk for BTreeMap<usize, BTreeSet<usize>> {
    impl_is_walk!();
}

impl<H> IsWalk for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl<W> IsWalk for Vec<BTreeMap<usize, W>> {
    impl_is_walk!();
}

impl<W, H> IsWalk for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl<W> IsWalk for [BTreeMap<usize, W>] {
    impl_is_walk!();
}

impl<W, H> IsWalk for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl<const V: usize, W> IsWalk for [BTreeMap<usize, W>; V] {
    impl_is_walk!();
}

impl<const V: usize, W, H> IsWalk for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl<W> IsWalk for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_is_walk!();
}

impl<W, H> IsWalk for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_is_walk!();
}

impl IsWalk for BTreeSet<(usize, usize)> {
    impl_is_walk!();
}

impl<H> IsWalk for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_is_walk!();
}

#[allow(clippy::cognitive_complexity)]
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
            op::AddWeightedArc,
        },
        std::collections::HashSet,
    };

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(1, 2, 2);
            $digraph.add_weighted_arc(2, 0, 3);
        };
    }

    macro_rules! test {
        ($digraph:expr) => {
            assert!($digraph.is_walk(&[0, 1]));
            assert!($digraph.is_walk(&[1, 2]));
            assert!($digraph.is_walk(&[2, 0]));
            assert!($digraph.is_walk(&[0, 1, 2]));
            assert!($digraph.is_walk(&[1, 2, 0]));
            assert!($digraph.is_walk(&[2, 0, 1]));

            assert!(!$digraph.is_walk(&[0]));
            assert!(!$digraph.is_walk(&[1]));
            assert!(!$digraph.is_walk(&[2]));
            assert!(!$digraph.is_walk(&[3]));
            assert!(!$digraph.is_walk(&[0, 0]));
            assert!(!$digraph.is_walk(&[0, 2]));
            assert!(!$digraph.is_walk(&[0, 3]));
            assert!(!$digraph.is_walk(&[1, 0]));
            assert!(!$digraph.is_walk(&[1, 1]));
            assert!(!$digraph.is_walk(&[1, 3]));
            assert!(!$digraph.is_walk(&[2, 1]));
            assert!(!$digraph.is_walk(&[2, 2]));
            assert!(!$digraph.is_walk(&[2, 3]));
            assert!(!$digraph.is_walk(&[3, 0]));
            assert!(!$digraph.is_walk(&[3, 1]));
            assert!(!$digraph.is_walk(&[3, 2]));
            assert!(!$digraph.is_walk(&[3, 3]));
            assert!(!$digraph.is_walk(&[0, 0, 0]));
            assert!(!$digraph.is_walk(&[0, 0, 1]));
            assert!(!$digraph.is_walk(&[0, 0, 2]));
            assert!(!$digraph.is_walk(&[0, 0, 3]));
            assert!(!$digraph.is_walk(&[0, 1, 0]));
            assert!(!$digraph.is_walk(&[0, 1, 1]));
            assert!(!$digraph.is_walk(&[0, 1, 3]));
            assert!(!$digraph.is_walk(&[0, 2, 0]));
            assert!(!$digraph.is_walk(&[0, 2, 1]));
            assert!(!$digraph.is_walk(&[0, 2, 2]));
            assert!(!$digraph.is_walk(&[0, 2, 3]));
            assert!(!$digraph.is_walk(&[0, 3, 0]));
            assert!(!$digraph.is_walk(&[0, 3, 1]));
            assert!(!$digraph.is_walk(&[0, 3, 2]));
            assert!(!$digraph.is_walk(&[0, 3, 3]));
            assert!(!$digraph.is_walk(&[1, 0, 0]));
            assert!(!$digraph.is_walk(&[1, 0, 1]));
            assert!(!$digraph.is_walk(&[1, 0, 2]));
            assert!(!$digraph.is_walk(&[1, 0, 3]));
            assert!(!$digraph.is_walk(&[1, 1, 0]));
            assert!(!$digraph.is_walk(&[1, 1, 1]));
            assert!(!$digraph.is_walk(&[1, 1, 2]));
            assert!(!$digraph.is_walk(&[1, 1, 3]));
            assert!(!$digraph.is_walk(&[1, 2, 1]));
            assert!(!$digraph.is_walk(&[1, 2, 2]));
            assert!(!$digraph.is_walk(&[1, 2, 3]));
            assert!(!$digraph.is_walk(&[1, 3, 0]));
            assert!(!$digraph.is_walk(&[1, 3, 1]));
            assert!(!$digraph.is_walk(&[1, 3, 2]));
            assert!(!$digraph.is_walk(&[1, 3, 3]));
            assert!(!$digraph.is_walk(&[2, 0, 0]));
            assert!(!$digraph.is_walk(&[2, 0, 2]));
            assert!(!$digraph.is_walk(&[2, 0, 3]));
            assert!(!$digraph.is_walk(&[2, 1, 0]));
            assert!(!$digraph.is_walk(&[2, 1, 1]));
            assert!(!$digraph.is_walk(&[2, 1, 2]));
            assert!(!$digraph.is_walk(&[2, 1, 3]));
            assert!(!$digraph.is_walk(&[2, 2, 0]));
            assert!(!$digraph.is_walk(&[2, 2, 1]));
            assert!(!$digraph.is_walk(&[2, 2, 2]));
            assert!(!$digraph.is_walk(&[2, 2, 3]));
            assert!(!$digraph.is_walk(&[2, 3, 0]));
            assert!(!$digraph.is_walk(&[2, 3, 1]));
            assert!(!$digraph.is_walk(&[2, 3, 2]));
            assert!(!$digraph.is_walk(&[2, 3, 3]));
            assert!(!$digraph.is_walk(&[3, 0, 0]));
            assert!(!$digraph.is_walk(&[3, 0, 1]));
            assert!(!$digraph.is_walk(&[3, 0, 2]));
            assert!(!$digraph.is_walk(&[3, 0, 3]));
            assert!(!$digraph.is_walk(&[3, 1, 0]));
            assert!(!$digraph.is_walk(&[3, 1, 1]));
            assert!(!$digraph.is_walk(&[3, 1, 2]));
            assert!(!$digraph.is_walk(&[3, 1, 3]));
            assert!(!$digraph.is_walk(&[3, 2, 0]));
            assert!(!$digraph.is_walk(&[3, 2, 1]));
            assert!(!$digraph.is_walk(&[3, 2, 2]));
            assert!(!$digraph.is_walk(&[3, 2, 3]));
            assert!(!$digraph.is_walk(&[3, 3, 0]));
            assert!(!$digraph.is_walk(&[3, 3, 1]));
            assert!(!$digraph.is_walk(&[3, 3, 2]));
            assert!(!$digraph.is_walk(&[3, 3, 3]));
        };
    }

    #[test]
    fn vec_btree_set() {
        let digraph = Vec::<BTreeSet<usize>>::cycle(3);

        test!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = Vec::<HashSet<usize>>::cycle(3);

        test!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph = Vec::<BTreeSet<usize>>::cycle(3);

        test!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set() {
        let digraph = Vec::<HashSet<usize>>::cycle(3);

        test!(digraph.as_slice());
    }

    #[test]
    fn arr_btree_set() {
        let digraph = <[BTreeSet<usize>; 3]>::cycle();

        test!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = <[HashSet<usize>; 3]>::cycle();

        test!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = BTreeMap::<usize, BTreeSet<usize>>::cycle(3);

        test!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = HashMap::<usize, HashSet<usize>>::cycle(3);

        test!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test!(digraph.as_slice());
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let digraph = BTreeSet::<(usize, usize)>::cycle(3);

        test!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let digraph = HashSet::<(usize, usize)>::cycle(3);

        test!(digraph);
    }
}
