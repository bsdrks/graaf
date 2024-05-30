//! Reverse an arc.
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
//!         op::{
//!             IterArcs,
//!             ReverseArc,
//!         },
//!     },
//! };
//!
//! let mut digraph = BTreeSet::<(usize, usize)>::cycle(3);
//!
//! digraph.reverse_arc(0, 1);
//!
//! assert!(digraph.iter_arcs().eq([(1, 0), (1, 2), (2, 0)]));
//! ```

extern crate alloc;

use {
    super::{
        AddArc,
        RemoveArc,
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

/// Reverse an arc.
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
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::Cycle,
///         op::{
///             AddArc,
///             IterArcs,
///             RemoveArc,
///             ReverseArc,
///         },
///     },
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
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::Cycle,
///         op::{
///             IterArcs,
///             ReverseArc,
///         },
///     },
/// };
///
/// let mut digraph = BTreeSet::<(usize, usize)>::cycle(3);
///
/// digraph.reverse_arc(0, 1);
///
/// assert!(digraph.iter_arcs().eq([(1, 0), (1, 2), (2, 0)]));
/// ```
pub trait ReverseArc {
    /// Reverse an arc.
    fn reverse_arc(&mut self, s: usize, t: usize) -> bool;
}

macro_rules! impl_reverse_arc {
    ($digraph:expr) => {
        fn reverse_arc(&mut self, s: usize, t: usize) -> bool {
            if self.remove_arc(s, t) {
                self.add_arc(t, s);

                return true;
            }

            false
        }
    };
}

macro_rules! impl_reverse_arc_panic {
    ($digraph:expr) => {
        /// # Panics
        ///
        /// Panics if `s` is not in the digraph.
        fn reverse_arc(&mut self, s: usize, t: usize) -> bool {
            if self.remove_arc(s, t) {
                self.add_arc(t, s);

                return true;
            }

            false
        }
    };
}

impl ReverseArc for Vec<BTreeSet<usize>> {
    impl_reverse_arc_panic!(self);
}

impl<H> ReverseArc for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_reverse_arc_panic!(self);
}

impl ReverseArc for [BTreeSet<usize>] {
    impl_reverse_arc_panic!(self);
}

impl<H> ReverseArc for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_reverse_arc_panic!(self);
}

impl<const V: usize> ReverseArc for [BTreeSet<usize>; V] {
    impl_reverse_arc_panic!(self);
}

impl<const V: usize, H> ReverseArc for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_reverse_arc_panic!(self);
}

impl ReverseArc for BTreeMap<usize, BTreeSet<usize>> {
    impl_reverse_arc!(self);
}

impl<H> ReverseArc for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
    HashSet<usize, H>: Default,
{
    impl_reverse_arc!(self);
}

impl ReverseArc for BTreeSet<(usize, usize)> {
    impl_reverse_arc!(self);
}

impl<H> ReverseArc for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    impl_reverse_arc!(self);
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
    };

    macro_rules! reverse_arcs {
        ($digraph:expr) => {
            assert!($digraph.reverse_arc(0, 1));
            assert!($digraph.reverse_arc(1, 2));
            assert!(!$digraph.reverse_arc(0, 2));
        };
    }

    macro_rules! test_reverse_arc_stable {
        ($digraph:expr) => {
            reverse_arcs!($digraph);

            assert!($digraph.iter_arcs().eq([(1, 0), (2, 0), (2, 1)]));
        };
    }

    macro_rules! test_reverse_arc_unstable {
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

        test_reverse_arc_stable!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::cycle(3);

        test_reverse_arc_unstable!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::cycle(3);

        test_reverse_arc_stable!(digraph.as_mut_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::cycle(3);

        test_reverse_arc_unstable!(digraph.as_mut_slice());
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::cycle();

        test_reverse_arc_stable!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::cycle();

        test_reverse_arc_unstable!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::cycle(3);

        test_reverse_arc_stable!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::cycle(3);

        test_reverse_arc_unstable!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::cycle(3);

        test_reverse_arc_stable!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize)>::cycle(3);

        test_reverse_arc_unstable!(digraph);
    }
}
