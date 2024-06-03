//! Determine whether a digraph is a superdigraph of another digraph.
//!
//! If digraph `H` is a subdigraph of digraph `D`, then `D` is a superdigraph of
//! `H`; the vertex set of `H` is a subset of the vertex set of `D` and the arc
//! set of `H` is a subset of the arc set of `D`. Additionally, the end-vertices
//! of each arc in `H` must be vertices in `H`.
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::{
//!             Cycle,
//!             Empty,
//!         },
//!         op::IsSuperdigraph,
//!     },
//! };
//!
//! let h = vec![BTreeSet::from([1]), BTreeSet::new()];
//! let d = Vec::<BTreeSet<usize>>::cycle(3);
//!
//! assert!(d.is_superdigraph(&h));
//! ```
//!
//! Every digraph is a superdigraph of itself.
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::RandomTournament,
//!         op::IsSuperdigraph,
//!     },
//! };
//!
//! let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
//!
//! assert!(tournament.is_superdigraph(&tournament));
//! ```

use crate::op::IsSubdigraph;

/// Determine whether a digraph is a superdigraph of another digraph.
///
/// If digraph `H` is a subdigraph of digraph `D`, then `D` is a superdigraph of
/// `H`; the vertex set of `H` is a subset of the vertex set of `D` and the arc
/// set of `H` is a subset of the arc set of `D`. Additionally, the end-vertices
/// of each arc in `H` must be vertices in `H`.
///
/// # How can I implement `IsSuperdigraph`?
///
/// Provide an implementation of `is_superdigraph` that returns `true` if the
/// digraph is a superdigraph of the digraph `d` and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::Cycle,
///         op::{
///             IsSubdigraph,
///             IsSuperdigraph,
///         },
///     },
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsSuperdigraph for Digraph {
///     fn is_superdigraph(&self, d: &Self) -> bool {
///         d.arcs.is_subdigraph(&self.arcs)
///     }
/// }
///
/// let h = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::new()],
/// };
///
/// let d = Digraph {
///     arcs: Vec::<BTreeSet<usize>>::cycle(3),
/// };
///
/// assert!(d.is_superdigraph(&h));
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
///         gen::{
///             Cycle,
///             Empty,
///         },
///         op::IsSuperdigraph,
///     },
/// };
///
/// let h = vec![BTreeSet::from([1]), BTreeSet::new()];
/// let d = Vec::<BTreeSet<usize>>::cycle(3);
///
/// assert!(d.is_superdigraph(&h));
/// ```
///
/// Every digraph is a superdigraph of itself.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::RandomTournament,
///         op::IsSuperdigraph,
///     },
/// };
///
/// let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
///
/// assert!(tournament.is_superdigraph(&tournament));
/// ```
pub trait IsSuperdigraph {
    /// Determines whether the digraph is a superdigraph of another digraph.
    fn is_superdigraph(&self, d: &Self) -> bool;
}

impl<T> IsSuperdigraph for T
where
    T: IsSubdigraph + ?Sized,
{
    fn is_superdigraph(&self, d: &Self) -> bool {
        d.is_subdigraph(self)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::gen::{
            RandomTournament,
            RandomTournamentConst,
        },
        alloc::collections::{
            BTreeMap,
            BTreeSet,
        },
        proptest::proptest,
        std::collections::{
            HashMap,
            HashSet,
        },
    };

    fn is_superdigraph_self<T>(digraph: &T)
    where
        T: IsSuperdigraph,
    {
        assert!(digraph.is_superdigraph(digraph));
    }

    proptest! {
        #[test]
        fn random_tournament_vec_btree_set(v in 1..100_usize) {
            let tournament = Vec::<BTreeSet<usize>>::random_tournament(v);

            assert!(tournament.is_superdigraph(&tournament));
        }

        #[test]
        fn random_tournament_vec_hash_set(v in 1..100_usize) {
            let tournament = Vec::<HashSet<usize>>::random_tournament(v);

            assert!(tournament.is_superdigraph(&tournament));
        }

        #[test]
        fn random_tournament_slice_btree_set(v in 1..100_usize) {
            let tournament = Vec::<BTreeSet<usize>>::random_tournament(v);

            assert!(tournament.as_slice().is_superdigraph(tournament.as_slice()));
        }

        #[test]
        fn random_tournament_slice_hash_set(v in 1..100_usize) {
            let tournament = Vec::<HashSet<usize>>::random_tournament(v);

            assert!(tournament.as_slice().is_superdigraph(tournament.as_slice()));
        }

        #[test]
        fn random_tournament_btree_map_btree_set(v in 1..100_usize) {
            let tournament = BTreeMap::<usize, BTreeSet<usize>>::random_tournament(v);

            assert!(tournament.is_superdigraph(&tournament));
        }

        #[test]
        fn random_tournament_hash_map_hash_set(v in 1..100_usize) {
            let tournament = HashMap::<usize, HashSet<usize>>::random_tournament(v);

            assert!(tournament.is_superdigraph(&tournament));
        }
    }

    #[test]
    fn random_tournament_arr_btree_set() {
        is_superdigraph_self(&<[BTreeSet<usize>; 1]>::random_tournament());
        is_superdigraph_self(&<[BTreeSet<usize>; 2]>::random_tournament());
        is_superdigraph_self(&<[BTreeSet<usize>; 3]>::random_tournament());
        is_superdigraph_self(&<[BTreeSet<usize>; 4]>::random_tournament());
        is_superdigraph_self(&<[BTreeSet<usize>; 5]>::random_tournament());
        is_superdigraph_self(&<[BTreeSet<usize>; 6]>::random_tournament());
        is_superdigraph_self(&<[BTreeSet<usize>; 7]>::random_tournament());
    }

    #[test]
    fn random_tournament_arr_hash_set() {
        is_superdigraph_self(&<[HashSet<usize>; 1]>::random_tournament());
        is_superdigraph_self(&<[HashSet<usize>; 2]>::random_tournament());
        is_superdigraph_self(&<[HashSet<usize>; 3]>::random_tournament());
        is_superdigraph_self(&<[HashSet<usize>; 4]>::random_tournament());
        is_superdigraph_self(&<[HashSet<usize>; 5]>::random_tournament());
        is_superdigraph_self(&<[HashSet<usize>; 6]>::random_tournament());
        is_superdigraph_self(&<[HashSet<usize>; 7]>::random_tournament());
    }
}
