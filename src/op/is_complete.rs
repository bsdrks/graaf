//! Determine whether a digraph is complete.
//!
//! A digraph is complete if, for every pair `s`, `t` of distinct vertices,
//! there is an arc from `s` to `t` and an arc from `t` to `s`.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         gen::{
//!             Complete,
//!             Cycle,
//!             Empty,
//!             RandomTournament,
//!         },
//!         op::IsComplete,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! assert!(Vec::<BTreeSet<usize>>::complete(3).is_complete());
//! assert!(!Vec::<BTreeSet<usize>>::cycle(3).is_complete());
//! assert!(!Vec::<BTreeSet<usize>>::empty(3).is_complete());
//! assert!(!Vec::<BTreeSet<usize>>::random_tournament(3).is_complete());
//! ```

use crate::op::{
    HasEdge,
    Order,
};

/// Determine whether a digraph is complete.
///
/// # How can I implement `IsComplete`?
///
/// Provide an implementation of `is_complete` that returns `true` if, for every
/// pair `s`, `t` of distinct vertices, there is an arc from `s` to `t` and an
/// arc from `t` to `s`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Complete,
///             Cycle,
///             Empty,
///             RandomTournament,
///         },
///         op::{
///             HasEdge,
///             IsComplete,
///             Order,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsComplete for Digraph {
///     fn is_complete(&self) -> bool {
///         let v = self.arcs.order();
///
///         for s in 0..v {
///             for t in (s + 1)..v {
///                 if !self.arcs.has_edge(s, t) {
///                     return false;
///                 }
///             }
///         }
///
///         true
///     }
/// }
///
/// assert!(Digraph {
///     arcs: Vec::<BTreeSet<usize>>::complete(3)
/// }
/// .is_complete());
///
/// assert!(!Digraph {
///     arcs: Vec::<BTreeSet<usize>>::cycle(3)
/// }
/// .is_complete());
///
/// assert!(!Digraph {
///     arcs: Vec::<BTreeSet<usize>>::empty(3)
/// }
/// .is_complete());
///
/// assert!(!Digraph {
///     arcs: Vec::<BTreeSet<usize>>::random_tournament(3)
/// }
/// .is_complete());
/// ```
pub trait IsComplete {
    /// Determines whether the digraph is complete.
    fn is_complete(&self) -> bool;
}

impl<D> IsComplete for D
where
    D: HasEdge + Order + ?Sized,
{
    fn is_complete(&self) -> bool {
        let v = self.order();

        for s in 0..v {
            for t in (s + 1)..v {
                if !self.has_edge(s, t) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::gen::{
            Complete,
            CompleteConst,
            Cycle,
            CycleConst,
            Empty,
            EmptyConst,
            RandomTournament,
            RandomTournamentConst,
        },
        std::collections::{
            BTreeSet,
            HashSet,
        },
    };

    macro_rules! test_const {
        ($ty:ty) => {
            assert!(<[$ty; 1]>::empty().is_complete());
            assert!(<[$ty; 1]>::cycle().is_complete());
            assert!(<[$ty; 2]>::cycle().is_complete());
            assert!(<[$ty; 2]>::complete().is_complete());
            assert!(<[$ty; 3]>::complete().is_complete());
            assert!(<[$ty; 4]>::complete().is_complete());

            assert!(!<[$ty; 2]>::empty().is_complete());
            assert!(!<[$ty; 3]>::empty().is_complete());
            assert!(!<[$ty; 3]>::cycle().is_complete());
            assert!(!<[$ty; 4]>::cycle().is_complete());
            assert!(!<[$ty; 2]>::random_tournament().is_complete());
            assert!(!<[$ty; 3]>::random_tournament().is_complete());
            assert!(!<[$ty; 4]>::random_tournament().is_complete());
        };
    }

    macro_rules! test_dynamic {
        ($ty:ty) => {
            assert!(<$ty>::empty(1).is_complete());
            assert!(<$ty>::cycle(1).is_complete());
            assert!(<$ty>::cycle(2).is_complete());
            assert!(<$ty>::complete(2).is_complete());
            assert!(<$ty>::complete(3).is_complete());
            assert!(<$ty>::complete(4).is_complete());

            assert!(!<$ty>::empty(2).is_complete());
            assert!(!<$ty>::empty(3).is_complete());
            assert!(!<$ty>::cycle(3).is_complete());
            assert!(!<$ty>::cycle(4).is_complete());
            assert!(!<$ty>::random_tournament(2).is_complete());
            assert!(!<$ty>::random_tournament(3).is_complete());
            assert!(!<$ty>::random_tournament(4).is_complete());
        };
    }

    #[test]
    fn vec_btree_set() {
        test_dynamic!(Vec::<BTreeSet<usize>>);
    }

    #[test]
    fn vec_hash_set() {
        test_dynamic!(Vec::<HashSet<usize>>);
    }

    #[test]
    fn arr_btree_set() {
        test_const!(BTreeSet<usize>);
    }

    #[test]
    fn arr_hash_set() {
        test_const!(HashSet<usize>);
    }
}
