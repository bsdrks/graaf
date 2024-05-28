//! Determine whether a digraph is oriented.
//!
//! An oriented graph is a digraph with no cycle of length 2.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::Cycle,
//!         op::IsOriented,
//!     },
//! };
//!
//! assert!(!Vec::<BTreeSet<usize>>::cycle(2).is_oriented());
//! assert!(Vec::<BTreeSet<usize>>::cycle(3).is_oriented());
//! ```

extern crate alloc;

use {
    super::{
        HasArc,
        IterArcs,
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

/// Determine whether a digraph is oriented.
///
/// # How can I implement `IsOriented`?
///
/// Provide an implementation of `is_oriented` that returns `true` if the
/// digraph is oriented and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::CycleConst,
///         op::{
///             HasArc,
///             IsOriented,
///             IterArcs,
///         },
///     },
/// };
///
/// struct Digraph<const V: usize> {
///     pub arcs: [BTreeSet<usize>; V],
/// }
///
/// impl<const V: usize> IsOriented for Digraph<V> {
///     fn is_oriented(&self) -> bool {
///         self.arcs.iter_arcs().all(|(s, t)| !self.arcs.has_arc(t, s))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: <[BTreeSet<usize>; 2]>::cycle(),
/// };
///
/// assert!(!digraph.is_oriented());
///
/// let digraph = Digraph {
///     arcs: <[BTreeSet<usize>; 3]>::cycle(),
/// };
///
/// assert!(digraph.is_oriented());
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
///         gen::Cycle,
///         op::IsOriented,
///     },
/// };
///
/// assert!(!Vec::<BTreeSet<usize>>::cycle(2).is_oriented());
/// assert!(Vec::<BTreeSet<usize>>::cycle(3).is_oriented());
pub trait IsOriented {
    /// Returns whether the digraph is oriented.
    fn is_oriented(&self) -> bool;
}

macro_rules! impl_is_oriented {
    () => {
        fn is_oriented(&self) -> bool {
            self.iter_arcs().all(|(s, t)| !self.has_arc(t, s))
        }
    };
}

impl IsOriented for Vec<BTreeSet<usize>> {
    impl_is_oriented!();
}

impl<H> IsOriented for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl IsOriented for [BTreeSet<usize>] {
    impl_is_oriented!();
}

impl<H> IsOriented for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl<const V: usize> IsOriented for [BTreeSet<usize>; V] {
    impl_is_oriented!();
}

impl<const V: usize, H> IsOriented for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl IsOriented for BTreeMap<usize, BTreeSet<usize>> {
    impl_is_oriented!();
}

impl<H> IsOriented for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl<W> IsOriented for Vec<BTreeMap<usize, W>> {
    impl_is_oriented!();
}

impl<W, H> IsOriented for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl<W> IsOriented for [BTreeMap<usize, W>] {
    impl_is_oriented!();
}

impl<W, H> IsOriented for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl<const V: usize, W> IsOriented for [BTreeMap<usize, W>; V] {
    impl_is_oriented!();
}

impl<const V: usize, W, H> IsOriented for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl<W> IsOriented for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_is_oriented!();
}

impl<W, H> IsOriented for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

impl IsOriented for BTreeSet<(usize, usize)> {
    impl_is_oriented!();
}

impl<H> IsOriented for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_is_oriented!();
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::{
            gen::{
                Complete,
                CompleteConst,
                Cycle,
                CycleConst,
                Empty,
                EmptyConst,
                RandomTournament,
                RandomTournamentConst,
            },
            op::AddWeightedArc,
        },
        alloc::collections::BTreeSet,
        std::collections::HashSet,
    };

    macro_rules! test_unweighted_dynamic {
        ($ty:ty) => {
            assert!(<$ty>::empty(1).is_oriented());
            assert!(<$ty>::empty(2).is_oriented());
            assert!(<$ty>::empty(3).is_oriented());
            assert!(<$ty>::cycle(3).is_oriented());
            assert!(<$ty>::cycle(4).is_oriented());
            assert!(<$ty>::random_tournament(2).is_oriented());
            assert!(<$ty>::random_tournament(3).is_oriented());
            assert!(<$ty>::random_tournament(4).is_oriented());

            assert!(!<$ty>::cycle(2).is_oriented());
            assert!(!<$ty>::complete(2).is_oriented());
            assert!(!<$ty>::complete(3).is_oriented());
            assert!(!<$ty>::complete(4).is_oriented());
        };
    }

    macro_rules! test_unweighted_const {
        ($ty:ty) => {
            assert!(<[$ty; 1]>::empty().is_oriented());
            assert!(<[$ty; 2]>::empty().is_oriented());
            assert!(<[$ty; 3]>::empty().is_oriented());
            assert!(<[$ty; 3]>::cycle().is_oriented());
            assert!(<[$ty; 4]>::cycle().is_oriented());
            assert!(<[$ty; 2]>::random_tournament().is_oriented());
            assert!(<[$ty; 3]>::random_tournament().is_oriented());
            assert!(<[$ty; 4]>::random_tournament().is_oriented());

            assert!(!<[$ty; 2]>::cycle().is_oriented());
            assert!(!<[$ty; 2]>::complete().is_oriented());
            assert!(!<[$ty; 3]>::complete().is_oriented());
            assert!(!<[$ty; 4]>::complete().is_oriented());
        };
    }

    macro_rules! test_weighted_dynamic {
        ($ty:ty) => {
            assert!(<$ty>::empty(1).is_oriented());
            assert!(<$ty>::empty(2).is_oriented());
            assert!(<$ty>::empty(3).is_oriented());

            let mut cycle2 = <$ty>::empty(2);

            cycle2.add_weighted_arc(0, 1, 1);
            cycle2.add_weighted_arc(1, 0, 1);

            assert!(!cycle2.is_oriented());

            let mut cycle3 = <$ty>::empty(3);

            cycle3.add_weighted_arc(0, 1, 1);
            cycle3.add_weighted_arc(1, 2, 1);
            cycle3.add_weighted_arc(2, 0, 1);

            assert!(cycle3.is_oriented());

            let mut cycle4 = <$ty>::empty(4);

            cycle4.add_weighted_arc(0, 1, 1);
            cycle4.add_weighted_arc(1, 2, 1);
            cycle4.add_weighted_arc(2, 3, 1);
            cycle4.add_weighted_arc(3, 0, 1);

            assert!(cycle4.is_oriented());

            let mut complete2 = <$ty>::empty(2);

            complete2.add_weighted_arc(0, 1, 1);
            complete2.add_weighted_arc(1, 0, 1);

            assert!(!complete2.is_oriented());

            let mut complete3 = <$ty>::empty(3);

            complete3.add_weighted_arc(0, 1, 1);
            complete3.add_weighted_arc(0, 2, 1);
            complete3.add_weighted_arc(1, 0, 1);
            complete3.add_weighted_arc(1, 2, 1);
            complete3.add_weighted_arc(2, 0, 1);
            complete3.add_weighted_arc(2, 1, 1);

            assert!(!complete3.is_oriented());

            let mut complete4 = <$ty>::empty(4);

            complete4.add_weighted_arc(0, 1, 1);
            complete4.add_weighted_arc(0, 2, 1);
            complete4.add_weighted_arc(0, 3, 1);
            complete4.add_weighted_arc(1, 0, 1);
            complete4.add_weighted_arc(1, 2, 1);
            complete4.add_weighted_arc(1, 3, 1);
            complete4.add_weighted_arc(2, 0, 1);
            complete4.add_weighted_arc(2, 1, 1);
            complete4.add_weighted_arc(2, 3, 1);
            complete4.add_weighted_arc(3, 0, 1);
            complete4.add_weighted_arc(3, 1, 1);
            complete4.add_weighted_arc(3, 2, 1);

            assert!(!complete4.is_oriented());
        };
    }

    macro_rules! test_weighted_const {
        ($ty:ty) => {
            assert!(<[$ty; 1]>::empty().is_oriented());
            assert!(<[$ty; 2]>::empty().is_oriented());
            assert!(<[$ty; 3]>::empty().is_oriented());

            let mut cycle2 = <[$ty; 2]>::empty();

            cycle2.add_weighted_arc(0, 1, 1);
            cycle2.add_weighted_arc(1, 0, 1);

            assert!(!cycle2.is_oriented());

            let mut cycle3 = <[$ty; 3]>::empty();

            cycle3.add_weighted_arc(0, 1, 1);
            cycle3.add_weighted_arc(1, 2, 1);
            cycle3.add_weighted_arc(2, 0, 1);

            assert!(cycle3.is_oriented());

            let mut cycle4 = <[$ty; 4]>::empty();

            cycle4.add_weighted_arc(0, 1, 1);
            cycle4.add_weighted_arc(1, 2, 1);
            cycle4.add_weighted_arc(2, 3, 1);
            cycle4.add_weighted_arc(3, 0, 1);

            assert!(cycle4.is_oriented());

            let mut complete2 = <[$ty; 2]>::empty();

            complete2.add_weighted_arc(0, 1, 1);
            complete2.add_weighted_arc(1, 0, 1);

            assert!(!complete2.is_oriented());

            let mut complete3 = <[$ty; 3]>::empty();

            complete3.add_weighted_arc(0, 1, 1);
            complete3.add_weighted_arc(0, 2, 1);
            complete3.add_weighted_arc(1, 0, 1);
            complete3.add_weighted_arc(1, 2, 1);
            complete3.add_weighted_arc(2, 0, 1);
            complete3.add_weighted_arc(2, 1, 1);

            assert!(!complete3.is_oriented());

            let mut complete4 = <[$ty; 4]>::empty();

            complete4.add_weighted_arc(0, 1, 1);
            complete4.add_weighted_arc(0, 2, 1);
            complete4.add_weighted_arc(0, 3, 1);
            complete4.add_weighted_arc(1, 0, 1);
            complete4.add_weighted_arc(1, 2, 1);
            complete4.add_weighted_arc(1, 3, 1);
            complete4.add_weighted_arc(2, 0, 1);
            complete4.add_weighted_arc(2, 1, 1);
            complete4.add_weighted_arc(2, 3, 1);
            complete4.add_weighted_arc(3, 0, 1);
            complete4.add_weighted_arc(3, 1, 1);
            complete4.add_weighted_arc(3, 2, 1);

            assert!(!complete4.is_oriented());
        };
    }

    #[test]
    fn vec_btree_set() {
        test_unweighted_dynamic!(Vec::<BTreeSet<usize>>);
    }

    #[test]
    fn vec_hash_set() {
        test_unweighted_dynamic!(Vec::<HashSet<usize>>);
    }

    #[test]
    fn arr_btree_set() {
        test_unweighted_const!(BTreeSet<usize>);
    }

    #[test]
    fn arr_hash_set() {
        test_unweighted_const!(HashSet<usize>);
    }

    #[test]
    fn btree_map_btree_set() {
        test_unweighted_dynamic!(BTreeMap::<usize, BTreeSet<usize>>);
    }

    #[test]
    fn hash_map_hash_set() {
        test_unweighted_dynamic!(HashMap::<usize, HashSet<usize>>);
    }

    #[test]
    fn vec_btree_map() {
        test_weighted_dynamic!(Vec::<BTreeMap<usize, i32>>);
    }

    #[test]
    fn vec_hash_map() {
        test_weighted_dynamic!(Vec::<HashMap<usize, i32>>);
    }

    #[test]
    fn arr_btree_map() {
        test_weighted_const!(BTreeMap::<usize, i32>);
    }

    #[test]
    fn arr_hash_map() {
        test_weighted_const!(HashMap::<usize, i32>);
    }

    #[test]
    fn btree_map_btree_map() {
        test_weighted_dynamic!(BTreeMap::<usize, BTreeMap<usize, i32>>);
    }

    #[test]
    fn hash_map_hash_map() {
        test_weighted_dynamic!(HashMap::<usize, HashMap<usize, i32>>);
    }

    #[test]
    fn btree_set_tuple() {
        test_unweighted_dynamic!(BTreeSet<(usize, usize)>);
    }
}
