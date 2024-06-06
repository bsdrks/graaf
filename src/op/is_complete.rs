//! Determine whether a digraph is complete.
//!
//! A digraph is complete if, for every pair `s`, `t` of distinct vertices,
//! there is an arc from `s` to `t` and an arc from `t` to `s`.
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
//!             Complete,
//!             Cycle,
//!         },
//!         op::IsComplete,
//!     },
//! };
//!
//! assert!(Vec::<BTreeSet<usize>>::complete(3).is_complete());
//! assert!(!Vec::<BTreeSet<usize>>::cycle(3).is_complete());
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
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::{
///             Complete,
///             Cycle,
///         },
///         op::{
///             HasEdge,
///             IsComplete,
///             Order,
///         },
///     },
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
    extern crate alloc;

    use {
        super::*,
        crate::gen::{
            Complete,
            CompleteConst,
            Cycle,
            CycleConst,
        },
        alloc::collections::BTreeSet,
        std::collections::HashSet,
    };

    #[test]
    fn vec_btree_set_complete() {
        assert!(Vec::<BTreeSet<usize>>::complete(3).is_complete());
    }

    #[test]
    fn vec_hash_set_complete() {
        assert!(Vec::<HashSet<usize>>::complete(3).is_complete());
    }

    #[test]
    fn arr_btree_set_complete() {
        assert!(<[BTreeSet<usize>; 3]>::complete().is_complete());
    }

    #[test]
    fn arr_hash_set_complete() {
        assert!(<[HashSet<usize>; 3]>::complete().is_complete());
    }

    #[test]
    fn vec_btree_set_cycle() {
        assert!(!Vec::<BTreeSet<usize>>::cycle(3).is_complete());
    }

    #[test]
    fn vec_hash_set_cycle() {
        assert!(!Vec::<HashSet<usize>>::cycle(3).is_complete());
    }

    #[test]
    fn arr_btree_set_cycle() {
        assert!(!<[BTreeSet<usize>; 3]>::cycle().is_complete());
    }

    #[test]
    fn arr_hash_set_cycle() {
        assert!(!<[HashSet<usize>; 3]>::cycle().is_complete());
    }
}
