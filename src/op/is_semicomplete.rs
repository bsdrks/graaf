//! Determine whether a digraph is semicomplete.
//!
//! A digraph is semicomplete if, for every pair `s`, `t` of distinct vertices,
//! there is an arc between `s` and `t`.
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
//!         op::IsSemicomplete,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! assert!(!Vec::<BTreeSet<usize>>::empty(3).is_semicomplete());
//! assert!(Vec::<BTreeSet<usize>>::complete(3).is_semicomplete());
//! assert!(Vec::<BTreeSet<usize>>::cycle(3).is_semicomplete());
//! assert!(Vec::<BTreeSet<usize>>::random_tournament(3).is_semicomplete());
//! ```

use crate::op::{
    HasArc,
    Order,
};

/// Determine whether a digraph is semicomplete.
///
/// # How can I implement `IsSemicomplete`?
///
/// Provide an implementation of `is_semicomplete` that returns `true` if there
/// is an arc between every pair `s`, `t` of distinct vertices.
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
///             HasArc,
///             IsSemicomplete,
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
/// impl IsSemicomplete for Digraph {
///     fn is_semicomplete(&self) -> bool {
///         let v = self.arcs.order();
///
///         for s in 0..v {
///             for t in (s + 1)..v {
///                 if !(self.arcs.has_arc(s, t) || self.arcs.has_arc(t, s)) {
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
/// .is_semicomplete());
///
/// assert!(Digraph {
///     arcs: Vec::<BTreeSet<usize>>::cycle(3)
/// }
/// .is_semicomplete());
///
/// assert!(!Digraph {
///     arcs: Vec::<BTreeSet<usize>>::empty(3)
/// }
/// .is_semicomplete());
///
/// assert!(Digraph {
///     arcs: Vec::<BTreeSet<usize>>::random_tournament(3)
/// }
/// .is_semicomplete());
/// ```
///
/// # Examples
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
///         op::IsSemicomplete,
///     },
///     std::collections::BTreeSet,
/// };
///
/// assert!(!Vec::<BTreeSet<usize>>::empty(3).is_semicomplete());
/// assert!(Vec::<BTreeSet<usize>>::complete(3).is_semicomplete());
/// assert!(Vec::<BTreeSet<usize>>::cycle(3).is_semicomplete());
/// assert!(Vec::<BTreeSet<usize>>::random_tournament(3).is_semicomplete());
/// ```
pub trait IsSemicomplete {
    /// Determines whether the digraph is semicomplete.
    fn is_semicomplete(&self) -> bool;
}

impl<D> IsSemicomplete for D
where
    D: HasArc + Order + ?Sized,
{
    fn is_semicomplete(&self) -> bool {
        let v = self.order();

        for s in 0..v {
            for t in (s + 1)..v {
                if !(self.has_arc(s, t) || self.has_arc(t, s)) {
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

    #[test]
    fn vec_btree_set_complete() {
        assert!(Vec::<BTreeSet<usize>>::complete(3).is_semicomplete());
    }

    #[test]
    fn vec_hash_set_complete() {
        assert!(Vec::<HashSet<usize>>::complete(3).is_semicomplete());
    }

    #[test]
    fn arr_btree_set_complete() {
        assert!(<[BTreeSet<usize>; 3]>::complete().is_semicomplete());
    }

    #[test]
    fn arr_hash_set_complete() {
        assert!(<[HashSet<usize>; 3]>::complete().is_semicomplete());
    }

    #[test]
    fn vec_btree_set_cycle() {
        assert!(Vec::<BTreeSet<usize>>::cycle(3).is_semicomplete());
    }

    #[test]
    fn vec_hash_set_cycle() {
        assert!(Vec::<HashSet<usize>>::cycle(3).is_semicomplete());
    }

    #[test]
    fn arr_btree_set_cycle() {
        assert!(<[BTreeSet<usize>; 3]>::cycle().is_semicomplete());
    }

    #[test]
    fn arr_hash_set_cycle() {
        assert!(<[HashSet<usize>; 3]>::cycle().is_semicomplete());
    }

    #[test]
    fn vec_btree_set_empty() {
        assert!(!Vec::<BTreeSet<usize>>::empty(3).is_semicomplete());
    }

    #[test]
    fn vec_hash_set_empty() {
        assert!(!Vec::<HashSet<usize>>::empty(3).is_semicomplete());
    }

    #[test]
    fn arr_btree_set_empty() {
        assert!(!<[BTreeSet<usize>; 3]>::empty().is_semicomplete());
    }

    #[test]
    fn arr_hash_set_empty() {
        assert!(!<[HashSet<usize>; 3]>::empty().is_semicomplete());
    }

    #[test]
    fn vec_btree_set_random_tournament() {
        println!("{:?}", Vec::<BTreeSet<usize>>::random_tournament(3));
        assert!(Vec::<BTreeSet<usize>>::random_tournament(3).is_semicomplete());
    }

    #[test]
    fn vec_hash_set_random_tournament() {
        assert!(Vec::<HashSet<usize>>::random_tournament(3).is_semicomplete());
    }

    #[test]
    fn arr_btree_set_random_tournament() {
        assert!(<[BTreeSet<usize>; 3]>::random_tournament().is_semicomplete());
    }

    #[test]
    fn arr_hash_set_random_tournament() {
        assert!(<[HashSet<usize>; 3]>::random_tournament().is_semicomplete());
    }
}
