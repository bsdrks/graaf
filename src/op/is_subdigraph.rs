//! Determine whether a digraph is a subdigraph of another digraph.
//!
//! A digraph `H` is a subdigraph of a digraph `D` if the vertex set of `H` is a
//! subset of the vertex set of `D` and the arc set of `H` is a subset of the
//! arc set of `D`. Additionally, the end-vertices of each arc in `H` must be
//! vertices in `H`.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         gen::Cycle,
//!         op::IsSubdigraph,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let h = vec![BTreeSet::from([1]), BTreeSet::new()];
//! let d = Vec::<BTreeSet<usize>>::cycle(3);
//!
//! assert!(h.is_subdigraph(&d));
//! ```
//!
//! Every digraph is a subdigraph of itself.
//!
//! ```
//! use {
//!     graaf::{
//!         gen::RandomTournament,
//!         op::IsSubdigraph,
//!     },
//!     std::collections::BTreeSet,
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
//! use {
//!     graaf::op::IsSubdigraph,
//!     std::collections::BTreeSet,
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
//! use {
//!     graaf::{
//!         gen::Empty,
//!         op::IsSubdigraph,
//!     },
//!     std::collections::BTreeSet,
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
//! use {
//!     graaf::{
//!         gen::Empty,
//!         op::IsSubdigraph,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! // The arc (0, 2) has end-vertex `2` which is not in the vertex set of `H`.
//! let h = vec![BTreeSet::from([1, 2]), BTreeSet::from([0])];
//! let d = vec![BTreeSet::from([1]), BTreeSet::from([0])];
//!
//! assert!(!h.is_subdigraph(&d));
//! ```

use {
    crate::op::{
        HasArc,
        IterArcs,
        IterVertices,
    },
    std::collections::BTreeSet,
};

/// Determine whether a digraph is a subdigraph of another digraph.
///
/// # How can I implement `IsSubdigraph`?
///
/// Provide an implementation of `is_subdigraph` that returns `true` if the
/// digraph is a subdigraph of the digraph `d` and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         HasArc,
///         IsSubdigraph,
///         IterArcs,
///         IterVertices,
///     },
///     std::collections::BTreeSet,
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

impl<T> IsSubdigraph for T
where
    T: HasArc + IterArcs + IterVertices + ?Sized,
{
    fn is_subdigraph(&self, d: &Self) -> bool {
        let hv = self.iter_vertices().collect::<BTreeSet<_>>();
        let dv = d.iter_vertices().collect::<BTreeSet<_>>();

        self.iter_arcs()
            .all(|(s, t)| d.has_arc(s, t) && hv.contains(&s) && hv.contains(&t))
            && hv.iter().all(|s| dv.contains(s))
    }
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
        std::collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
            HashSet,
        },
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
        fn random_tournament_slice_btree_set(v in 1..100_usize) {
            let tournament = Vec::<BTreeSet<usize>>::random_tournament(v);

            assert!(tournament.as_slice().is_subdigraph(tournament.as_slice()));
        }

        #[test]
        fn random_tournament_slice_hash_set(v in 1..100_usize) {
            let tournament = Vec::<HashSet<usize>>::random_tournament(v);

            assert!(tournament.as_slice().is_subdigraph(tournament.as_slice()));
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
