//! Check if an edge exists between two vertices.
//!
//! To check if an arc exists from `s` to `t`, see [`HasArc`].
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::HasEdge,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = vec![HashSet::from([1]), HashSet::from([0])];
//!
//! assert!(digraph.has_edge(0, 1));
//! assert!(digraph.has_edge(1, 0));
//!
//! let digraph = vec![HashSet::from([1]), HashSet::new()];
//!
//! assert!(!digraph.has_edge(0, 1));
//! assert!(!digraph.has_edge(1, 0));
//!
//! let digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([2]),
//!     HashSet::from([0]),
//! ];
//!
//! assert!(!digraph.has_edge(0, 1));
//! assert!(digraph.has_edge(0, 2));
//! assert!(!digraph.has_edge(1, 0));
//! assert!(!digraph.has_edge(1, 2));
//! assert!(digraph.has_edge(2, 0));
//! assert!(!digraph.has_edge(2, 1));
//! ```
//!
//! [`HasArc`]: crate::op::HasArc

extern crate alloc;

use {
    super::HasArc,
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

/// Check if an edge exists between two vertices.
///
/// # How can I implement `HasEdge`?
///
/// Provide an implementation of `has_edge` that returns `true` if the
/// digraph has an arc between `s` and `t` and `false` otherwise.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::op::HasEdge,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasEdge for Digraph {
///     fn has_edge(&self, s: usize, t: usize) -> bool {
///         self.arcs[s].contains(&t) && self.arcs[t].contains(&s)
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1]), BTreeSet::from([0])],
/// };
///
/// assert!(digraph.has_edge(0, 1));
/// assert!(digraph.has_edge(1, 0));
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::HasEdge,
///     std::collections::HashSet,
/// };
///
/// let digraph = vec![HashSet::from([1]), HashSet::from([0])];
///
/// assert!(digraph.has_edge(0, 1));
/// assert!(digraph.has_edge(1, 0));
///
/// let digraph = vec![HashSet::from([1]), HashSet::new()];
///
/// assert!(!digraph.has_edge(0, 1));
/// assert!(!digraph.has_edge(1, 0));
///
/// let digraph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([2]),
///     HashSet::from([0]),
/// ];
///
/// assert!(!digraph.has_edge(0, 1));
/// assert!(digraph.has_edge(0, 2));
/// assert!(!digraph.has_edge(1, 0));
/// assert!(!digraph.has_edge(1, 2));
/// assert!(digraph.has_edge(2, 0));
/// assert!(!digraph.has_edge(2, 1));
/// ```
pub trait HasEdge {
    /// Returns whether the digraph has an arc from `s` to `t` and from `t` to
    /// `s`
    fn has_edge(&self, s: usize, t: usize) -> bool;
}

impl HasEdge for Vec<BTreeSet<usize>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<H> HasEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl HasEdge for [BTreeSet<usize>] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<H> HasEdge for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<const V: usize> HasEdge for [BTreeSet<usize>; V] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<const V: usize, H> HasEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl HasEdge for BTreeMap<usize, BTreeSet<usize>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<H> HasEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<W> HasEdge for Vec<BTreeMap<usize, W>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<W, H> HasEdge for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<W> HasEdge for [BTreeMap<usize, W>] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<W, H> HasEdge for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<const V: usize, W> HasEdge for [BTreeMap<usize, W>; V] {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<const V: usize, W, H> HasEdge for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<W> HasEdge for BTreeMap<usize, BTreeMap<usize, W>> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<W, H> HasEdge for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl HasEdge for BTreeSet<(usize, usize)> {
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

impl<H> HasEdge for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn has_edge(&self, s: usize, t: usize) -> bool {
        self.has_arc(s, t) && self.has_arc(t, s)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            gen::{
                Empty,
                EmptyConst,
            },
            op::{
                AddArc,
                AddWeightedArc,
            },
        },
    };

    macro_rules! setup_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 2);
            $digraph.add_arc(2, 0);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 3);
            $digraph.add_weighted_arc(1, 2, 4);
            $digraph.add_weighted_arc(2, 0, 7);
        };
    }

    macro_rules! test_has_arc {
        ($digraph:expr) => {
            assert!(!$digraph.has_edge(0, 1));
            assert!($digraph.has_edge(0, 2));
            assert!(!$digraph.has_edge(1, 0));
            assert!(!$digraph.has_edge(1, 2));
            assert!($digraph.has_edge(2, 0));
            assert!(!$digraph.has_edge(2, 1));
        };
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph.as_slice());
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph.as_slice());
    }

    #[test]
    fn slice_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph.as_slice());
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize)>::empty(3);

        setup_unweighted!(digraph);
        test_has_arc!(digraph);
    }
}
