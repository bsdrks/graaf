//! A trait to determine whether a digraph is a subdigraph of another digraph.
//!
//! A digraph `H` is a subdigraph of a digraph `D` if the vertex set of `H` is a
//! subset of the vertex set of `D` and the arc set of `H` is a subset of the
//! arc set of `D`. Additionally, the end-vertices of each arc in `H` must be
//! vertices in `H`.
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
//!         op::IsSubdigraph,
//!     },
//! };
//!
//! let h = vec![BTreeSet::from([1]), BTreeSet::new()];
//!
//! let d = vec![
//!     BTreeSet::from([1]),
//!     BTreeSet::from([2]),
//!     BTreeSet::from([0]),
//! ];
//!
//! assert!(h.is_subdigraph(&d));
//! ```
//!
//! Every digraph is a subdigraph of itself.
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::RandomTournament,
//!         op::IsSubdigraph,
//!     },
//! };
//!
//! let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
//!
//! assert!(tournament.is_subdigraph(&tournament));
//! ```
//!
//! A digraph `H` with arcs not in the arc set of a digraph `D` is not a
//! subdigraph of `D`.
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::op::IsSubdigraph,
//! };
//!
//! let h = vec![BTreeSet::from([1]), BTreeSet::from([0])];
//! let d = vec![BTreeSet::from([1])];
//!
//! assert!(!h.is_subdigraph(&d));
//! ```
//!
//! A digraph `H` with vertices not in the vertex set of a digraph `D` is not a
//! subdigraph of `D`.
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::Empty,
//!         op::IsSubdigraph,
//!     },
//! };
//!
//! let h = Vec::<BTreeSet<usize>>::empty(2);
//! let d = vec![BTreeSet::from([1])];
//!
//! assert!(!h.is_subdigraph(&d));
//! ```
//!
//! A digraph `H` with arcs whose end-vertices are not in the vertex set of `H`
//! is not a subdigraph of a digraph `D`.
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::Empty,
//!         op::IsSubdigraph,
//!     },
//! };
//!
//! // The arc (0, 2) has end-vertex `2` which is not in the vertex set of `H`.
//! let h = vec![BTreeSet::from([1, 2]), BTreeSet::from([0])];
//! let d = vec![BTreeSet::from([1]), BTreeSet::from([0])];
//!
//! assert!(!h.is_subdigraph(&d));
//! ```

extern crate alloc;

use {
    crate::op::{
        HasArc,
        IterArcs,
        IterVertices,
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

/// A trait to determine whether a digraph is a subdigraph of another digraph.
///
/// # How can I implement `IsSubdigraph`?
///
/// Provide an implementation of `is_subdigraph` that returns `true` if the
/// digraph is a subdigraph of the digraph `D` and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::{
///         HasArc,
///         IsSubdigraph,
///         IterArcs,
///         IterVertices,
///     },
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsSubdigraph for Digraph {
///     fn is_subdigraph(&self, d: &Self) -> bool {
///         let hv = self.arcs.iter_vertices().collect::<BTreeSet<_>>();
///         let dv = d.arcs.iter_vertices().collect::<BTreeSet<_>>();
///
///         self.arcs
///             .iter_arcs()
///             .all(|(s, t)| d.arcs.has_arc(s, t) && hv.contains(&s) && hv.contains(&t))
///             && hv.iter().all(|s| dv.contains(&s))
///     }
/// }
///
/// let h = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new()],
/// };
///
/// let d = Digraph {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(h.is_subdigraph(&d));
/// ```
pub trait IsSubdigraph {
    /// Determines whether the digraph is a subdigraph of another digraph.
    fn is_subdigraph(&self, d: &Self) -> bool;
}

macro_rules! impl_is_subdigraph {
    () => {
        fn is_subdigraph(&self, d: &Self) -> bool {
            let hv = self.iter_vertices().collect::<BTreeSet<_>>();
            let dv = d.iter_vertices().collect::<BTreeSet<_>>();

            self.iter_arcs()
                .all(|(s, t)| d.has_arc(s, t) && hv.contains(&s) && hv.contains(&t))
                && hv.iter().all(|s| dv.contains(s))
        }
    };
}

impl IsSubdigraph for Vec<BTreeSet<usize>> {
    impl_is_subdigraph!();
}

impl<H> IsSubdigraph for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_is_subdigraph!();
}

impl<const V: usize> IsSubdigraph for [BTreeSet<usize>; V] {
    impl_is_subdigraph!();
}

impl<const V: usize, H> IsSubdigraph for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_is_subdigraph!();
}

impl IsSubdigraph for BTreeMap<usize, BTreeSet<usize>> {
    impl_is_subdigraph!();
}

impl<H> IsSubdigraph for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    impl_is_subdigraph!();
}

impl<W> IsSubdigraph for Vec<BTreeMap<usize, W>> {
    impl_is_subdigraph!();
}

impl<W, H> IsSubdigraph for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    impl_is_subdigraph!();
}

impl<const V: usize, W> IsSubdigraph for [BTreeMap<usize, W>; V] {
    impl_is_subdigraph!();
}

impl<const V: usize, W, H> IsSubdigraph for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    impl_is_subdigraph!();
}

impl<W> IsSubdigraph for BTreeMap<usize, BTreeMap<usize, W>> {
    impl_is_subdigraph!();
}

impl<W, H> IsSubdigraph for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    impl_is_subdigraph!();
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::gen::{
            RandomTournament,
            RandomTournamentConst,
        },
        proptest::proptest,
    };

    fn is_subdigraph_self<T>(digraph: &T)
    where
        T: IsSubdigraph,
    {
        assert!(digraph.is_subdigraph(digraph));
    }

    proptest! {
        #[test]
        fn random_tournament_vec_btree_set(v in 1..100_usize) {
            let tournament = Vec::<BTreeSet<usize>>::random_tournament(v);

            assert!(tournament.is_subdigraph(&tournament));
        }

        #[test]
        fn random_tournament_vec_hash_set(v in 1..100_usize) {
            let tournament = Vec::<HashSet<usize>>::random_tournament(v);

            assert!(tournament.is_subdigraph(&tournament));
        }

        #[test]
        fn random_tournament_btree_map_btree_set(v in 1..100_usize) {
            let tournament = BTreeMap::<usize, BTreeSet<usize>>::random_tournament(v);

            assert!(tournament.is_subdigraph(&tournament));
        }

        #[test]
        fn random_tournament_hash_map_hash_set(v in 1..100_usize) {
            let tournament = HashMap::<usize, HashSet<usize>>::random_tournament(v);

            assert!(tournament.is_subdigraph(&tournament));
        }
    }

    #[test]
    fn random_tournament_arr_btree_set() {
        is_subdigraph_self(&<[BTreeSet<usize>; 1]>::random_tournament());
        is_subdigraph_self(&<[BTreeSet<usize>; 2]>::random_tournament());
        is_subdigraph_self(&<[BTreeSet<usize>; 3]>::random_tournament());
        is_subdigraph_self(&<[BTreeSet<usize>; 4]>::random_tournament());
        is_subdigraph_self(&<[BTreeSet<usize>; 5]>::random_tournament());
        is_subdigraph_self(&<[BTreeSet<usize>; 6]>::random_tournament());
        is_subdigraph_self(&<[BTreeSet<usize>; 7]>::random_tournament());
    }

    #[test]
    fn random_tournament_arr_hash_set() {
        is_subdigraph_self(&<[HashSet<usize>; 1]>::random_tournament());
        is_subdigraph_self(&<[HashSet<usize>; 2]>::random_tournament());
        is_subdigraph_self(&<[HashSet<usize>; 3]>::random_tournament());
        is_subdigraph_self(&<[HashSet<usize>; 4]>::random_tournament());
        is_subdigraph_self(&<[HashSet<usize>; 5]>::random_tournament());
        is_subdigraph_self(&<[HashSet<usize>; 6]>::random_tournament());
        is_subdigraph_self(&<[HashSet<usize>; 7]>::random_tournament());
    }
}
