//! Reverse an arc in a digraph.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         gen::Cycle,
//!         op::{
//!             IterArcs,
//!             ReverseArc,
//!         },
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let mut digraph = BTreeSet::<(usize, usize)>::cycle(3);
//!
//! digraph.reverse_arc(0, 1);
//!
//! assert!(digraph.iter_arcs().eq([(1, 0), (1, 2), (2, 0)]));
//! ```

use super::{
    AddArc,
    RemoveArc,
};

/// Reverse an arc in a digraph.
///
/// If the arc does not exist, this operation has no effect.
///
/// # Arguments
///
/// * `s`: The head vertex.
/// * `t`: The tail vertex.
///
/// # Returns
///
/// Returns `true` if the arc was reversed, and `false` otherwise.
///
/// # How can I implement `ReverseArc`?
///
/// Provide an implementation of `reverse_arc` that replaces the arc from `s` to
/// `t` with an arc from `t` to `s`.
///
/// ```
/// use {
///     graaf::{
///         gen::Cycle,
///         op::{
///             AddArc,
///             IterArcs,
///             RemoveArc,
///             ReverseArc,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     pub arcs: BTreeSet<(usize, usize)>,
/// }
///
/// impl ReverseArc for Digraph {
///     fn reverse_arc(&mut self, s: usize, t: usize) -> bool {
///         if self.arcs.remove_arc(s, t) {
///             self.arcs.add_arc(t, s);
///
///             return true;
///         }
///
///         false
///     }
/// }
///
/// let mut digraph = Digraph {
///     arcs: BTreeSet::<(usize, usize)>::cycle(3),
/// };
///
/// digraph.reverse_arc(0, 1);
/// digraph.reverse_arc(1, 2);
///
/// assert!(digraph.arcs.iter_arcs().eq([(1, 0), (2, 0), (2, 1)]));
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::{
///         gen::Cycle,
///         op::{
///             IterArcs,
///             ReverseArc,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// let mut digraph = BTreeSet::<(usize, usize)>::cycle(3);
///
/// digraph.reverse_arc(0, 1);
///
/// assert!(digraph.iter_arcs().eq([(1, 0), (1, 2), (2, 0)]));
/// ```
pub trait ReverseArc {
    /// Reverses an arc in the digraph.
    fn reverse_arc(&mut self, s: usize, t: usize) -> bool;
}

impl<T> ReverseArc for T
where
    T: AddArc + RemoveArc + ?Sized,
{
    fn reverse_arc(&mut self, s: usize, t: usize) -> bool {
        if self.remove_arc(s, t) {
            self.add_arc(t, s);

            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::{
                Cycle,
                CycleConst,
            },
            op::IterArcs,
        },
        std::collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
            HashSet,
        },
    };

    macro_rules! reverse_arcs {
        ($digraph:expr) => {
            assert!($digraph.reverse_arc(0, 1));
            assert!($digraph.reverse_arc(1, 2));
            assert!(!$digraph.reverse_arc(0, 2));
        };
    }

    macro_rules! test_stable {
        ($digraph:expr) => {
            reverse_arcs!($digraph);

            assert!($digraph.iter_arcs().eq([(1, 0), (2, 0), (2, 1)]));
        };
    }

    macro_rules! test_unstable {
        ($digraph:expr) => {
            reverse_arcs!($digraph);

            let mut iter = $digraph.iter_arcs();

            assert!(matches!(iter.next(), Some((1 | 2, 0) | (2, 1))));
            assert!(matches!(iter.next(), Some((1 | 2, 0) | (2, 1))));
            assert!(matches!(iter.next(), Some((1 | 2, 0) | (2, 1))));
            assert_eq!(iter.next(), None);
        };
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::cycle(3);

        test_stable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::cycle(3);

        test_unstable!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::cycle(3);

        test_stable!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::cycle(3);

        test_unstable!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::cycle();

        test_stable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::cycle();

        test_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::cycle(3);

        test_stable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::cycle(3);

        test_unstable!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::cycle(3);

        test_stable!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize)>::cycle(3);

        test_unstable!(digraph);
    }
}
