//! A trait to check if an edge exists between `s` and `t`
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

/// A trait to check if an edge exists between `s` and `t`
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
    use super::*;

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
        let digraph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = vec![
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &[BTreeSet<usize>] = &[
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &[HashSet<usize>] = &[
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let digraph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = [
            HashSet::from([1, 2]),
            HashSet::from([2]),
            HashSet::from([0]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = vec![
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 4)]),
            BTreeMap::from([(0, 7)]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = vec![
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 4)]),
            HashMap::from([(0, 7)]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &[BTreeMap<usize, usize>] = &[
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 4)]),
            BTreeMap::from([(0, 7)]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &[HashMap<usize, usize>] = &[
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 4)]),
            HashMap::from([(0, 7)]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = [
            BTreeMap::from([(1, 2), (2, 3)]),
            BTreeMap::from([(2, 4)]),
            BTreeMap::from([(0, 7)]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = [
            HashMap::from([(1, 2), (2, 3)]),
            HashMap::from([(2, 4)]),
            HashMap::from([(0, 7)]),
        ];

        test_has_arc!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([2])),
            (2, BTreeSet::from([0])),
        ]);

        test_has_arc!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([2])),
            (2, HashSet::from([0])),
        ]);

        test_has_arc!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let digraph = BTreeMap::from([
            (0, BTreeMap::from([(1, 2), (2, 3)])),
            (1, BTreeMap::from([(2, 4)])),
            (2, BTreeMap::from([(0, 7)])),
        ]);

        test_has_arc!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let digraph = HashMap::from([
            (0, HashMap::from([(1, 2), (2, 3)])),
            (1, HashMap::from([(2, 4)])),
            (2, HashMap::from([(0, 7)])),
        ]);

        test_has_arc!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let digraph = BTreeSet::from([(0, 1), (0, 2), (1, 2), (2, 0)]);

        test_has_arc!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let digraph = HashSet::from([(0, 1), (0, 2), (1, 2), (2, 0)]);

        test_has_arc!(digraph);
    }
}
