//! A trait to determine whether a digraph is regular
//!
//! A digraph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         gen::Cycle,
//!         op::{
//!             IsRegular,
//!             RemoveArc,
//!         },
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let mut digraph = Vec::<BTreeSet<usize>>::cycle(7);
//!
//! assert!(digraph.is_regular());
//!
//! digraph.remove_arc(6, 0);
//!
//! assert!(!digraph.is_regular());
//! ```

extern crate alloc;

use {
    crate::op::{
        Indegree,
        IterVertices,
        Outdegree,
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

/// A trait to determine whether a digraph is regular
///
/// # How can I implement `IsRegular`?
///
/// Provide an implementation of `is_regular` that returns `true` if the digraph
/// is regular and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::{
///         Indegree,
///         IsRegular,
///         IterVertices,
///         Outdegree,
///     },
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsRegular for Digraph {
///     /// # Panics
///     ///
///     /// Panics if the digraph has no vertices.
///     fn is_regular(&self) -> bool {
///         let mut vertices = self.arcs.iter_vertices();
///
///         let v = vertices
///             .next()
///             .expect("a graph must have at least one vertex");
///
///         let indegree = self.arcs.indegree(v);
///         let outdegree = self.arcs.outdegree(v);
///
///         vertices
///             .all(|v| self.arcs.indegree(v) == indegree && self.arcs.outdegree(v) == outdegree)
///     }
/// }
/// ```
pub trait IsRegular {
    /// Returns whether the digraph is regular.
    fn is_regular(&self) -> bool;
}

macro_rules! impl_is_regular {
    () => {
        /// # Panics
        ///
        /// Panics if the digraph has no vertices.
        fn is_regular(&self) -> bool {
            let mut vertices = self.iter_vertices();

            let v = vertices
                .next()
                .expect("a graph must have at least one vertex");

            let indegree = self.indegree(v);
            let outdegree = self.outdegree(v);

            vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
        }
    };
}

impl IsRegular for Vec<BTreeSet<usize>> {
    impl_is_regular!();
}

impl<H> IsRegular for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_is_regular!();
}

impl IsRegular for [BTreeSet<usize>] {
    impl_is_regular!();
}

impl<H> IsRegular for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_is_regular!();
}

impl<const V: usize> IsRegular for [BTreeSet<usize>; V] {
    impl_is_regular!();
}

impl<const V: usize, H> IsRegular for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_is_regular!();
}

impl IsRegular for BTreeMap<usize, BTreeSet<usize>> {
    impl_is_regular!();
}

impl<H> IsRegular for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_is_regular!();
}

impl<W> IsRegular for Vec<BTreeMap<usize, W>> {
    impl_is_regular!();
}

impl<W, H> IsRegular for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_is_regular!();
}

impl<W> IsRegular for [BTreeMap<usize, W>] {
    impl_is_regular!();
}

impl<W, H> IsRegular for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    impl_is_regular!();
}

impl<const V: usize, W> IsRegular for [BTreeMap<usize, W>; V] {
    impl_is_regular!();
}

impl<const V: usize, W, H> IsRegular for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_is_regular!();
}

impl<W> IsRegular for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_is_regular!();
}

impl<W, H> IsRegular for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_is_regular!();
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
                AddWeightedArc,
                RemoveArc,
            },
        },
        proptest::proptest,
    };

    macro_rules! test_is_regular {
        ($digraph:expr) => {
            assert!($digraph.is_regular());

            let _ = $digraph.remove_arc(2, 0);

            assert!(!$digraph.is_regular());
        };
    }

    proptest! {
        #[test]
        fn empty_vec_btree_set_unweighted(v in 1..100_usize) {
            let digraph = Vec::<BTreeSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_vec_hash_set_unweighted(v in 1..100_usize) {
            let digraph = Vec::<HashSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_btree_map_btree_set_unweighted(v in 1..100_usize) {
            let digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_hash_map_hash_set_unweighted(v in 1..100_usize) {
            let digraph = HashMap::<usize, HashSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_vec_btree_map(v in 1..100_usize) {
            let digraph = Vec::<BTreeMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_vec_hash_map(v in 1..100_usize) {
            let digraph = Vec::<HashMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_btree_map_btree_map(v in 1..100_usize) {
            let digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_hash_map_hash_map(v in 1..100_usize) {
            let digraph = HashMap::<usize, HashMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let digraph = &mut <Vec<BTreeSet<usize>>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let digraph = &mut <Vec<HashSet<usize>>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let digraph = &mut <[BTreeSet<usize>; 3]>::cycle();

        test_is_regular!(digraph);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let digraph = &mut <[HashSet<usize>; 3]>::cycle();

        test_is_regular!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    macro_rules! setup_weighted_cycle {
        ($ty:ty) => {{
            let mut digraph = <$ty>::empty(3);

            digraph.add_weighted_arc(0, 1, 4);
            digraph.add_weighted_arc(1, 2, 3);
            digraph.add_weighted_arc(2, 0, 3);

            digraph
        }};
    }

    macro_rules! setup_weighted_cycle_const {
        ($ty:ty) => {{
            let mut digraph = <$ty>::empty();

            digraph.add_weighted_arc(0, 1, 4);
            digraph.add_weighted_arc(1, 2, 3);
            digraph.add_weighted_arc(2, 0, 3);

            digraph
        }};
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = setup_weighted_cycle!(Vec<BTreeMap<usize, usize>>);

        test_is_regular!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = setup_weighted_cycle!(Vec<HashMap<usize, usize>>);

        test_is_regular!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = setup_weighted_cycle!(Vec<BTreeMap<usize, usize>>);

        test_is_regular!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = setup_weighted_cycle!(Vec<HashMap<usize, usize>>);

        test_is_regular!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = setup_weighted_cycle_const!([BTreeMap<usize, usize>; 3]);

        test_is_regular!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = setup_weighted_cycle_const!([HashMap<usize, usize>; 3]);

        test_is_regular!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = setup_weighted_cycle!(BTreeMap<usize, BTreeMap<usize, usize>>);

        test_is_regular!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = setup_weighted_cycle!(HashMap<usize, HashMap<usize, usize>>);

        test_is_regular!(digraph);
    }
}
