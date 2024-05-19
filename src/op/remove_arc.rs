//! A trait to remove an arc from a digraph
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::RemoveArc,
//!     std::collections::HashSet,
//! };
//!
//! let mut digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::from([1]),
//! ];
//!
//! assert!(digraph.remove_arc(0, 1));
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! assert!(digraph.remove_arc(0, 2));
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
//! );
//!
//! assert!(digraph.remove_arc(1, 0));
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
//! );
//!
//! digraph.remove_arc(2, 1);
//!
//! assert_eq!(
//!     digraph,
//!     vec![HashSet::new(), HashSet::new(), HashSet::new()]
//! );
//! ```

extern crate alloc;

use {
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

/// A trait to remove an arc from a digraph
///
/// # How can I implement `RemoveArc`?
///
/// Provide an implementation of `remove_arc` that removes the arc from `s` to
/// `t` from a digraph. Return whether the arc was removed.
///
/// ```
/// use {
///     graaf::op::RemoveArc,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl RemoveArc for Graph {
///     fn remove_arc(&mut self, s: usize, t: usize) -> bool {
///         self.arcs[s].remove(&t)
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::RemoveArc,
///     std::collections::HashSet,
/// };
///
/// let mut digraph = vec![
///     HashSet::from([1, 2]),
///     HashSet::from([0]),
///     HashSet::from([1]),
/// ];
///
/// assert!(digraph.remove_arc(0, 1));
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// assert!(digraph.remove_arc(0, 2));
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
/// );
///
/// assert!(digraph.remove_arc(1, 0));
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
/// );
///
/// digraph.remove_arc(2, 1);
///
/// assert_eq!(
///     digraph,
///     vec![HashSet::new(), HashSet::new(), HashSet::new()]
/// );
/// ```
///
/// # Properties
///
/// ## `RemoveArc` and `AddArc`
///
/// Types that also implement [`AddArc`] should ensure that
/// [`add_arc_remove_arc`] holds.
///
/// ## `RemoveArc` and `AddWeightedArc`
///
/// Types that also implement [`AddWeightedArc`] should ensure that
/// [`add_weighted_arc_remove_arc`] holds.
///
/// [`AddArc`]: crate::op::AddArc
/// [`AddWeightedArc`]: crate::op::AddWeightedArc
/// [`add_arc_remove_arc`]: crate::prop::add_arc_remove_arc
/// [`add_weighted_arc_remove_arc`]: crate::prop::add_weighted_arc_remove_arc
pub trait RemoveArc {
    /// Removes the arc from `s` to `t` from a digraph. Returns whether the
    /// arc was removed.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool;
}

impl RemoveArc for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<H> RemoveArc for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<W> RemoveArc for Vec<BTreeMap<usize, W>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<W, H> RemoveArc for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl RemoveArc for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<H> RemoveArc for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<W> RemoveArc for [BTreeMap<usize, W>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<W, H> RemoveArc for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<const V: usize> RemoveArc for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<const V: usize, H> RemoveArc for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t)
    }
}

impl<const V: usize, W> RemoveArc for [BTreeMap<usize, W>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl<const V: usize, W, H> RemoveArc for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self[s].remove(&t).is_some()
    }
}

impl RemoveArc for BTreeMap<usize, BTreeSet<usize>> {
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s).map(|set| set.remove(&t)).unwrap()
    }
}

impl<W> RemoveArc for BTreeMap<usize, BTreeMap<usize, W>> {
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s)
            .map_or(false, |map| map.remove(&t).is_some())
    }
}

impl<H> RemoveArc for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
{
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s).map(|set| set.remove(&t)).unwrap()
    }
}

impl<W, H> RemoveArc for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
{
    fn remove_arc(&mut self, s: usize, t: usize) -> bool {
        self.get_mut(&s)
            .map_or(false, |map| map.remove(&t).is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_btree_set() {
        let mut digraph = vec![
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
        ];

        assert_eq!(
            digraph,
            vec![
                BTreeSet::from([1, 2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![
                BTreeSet::from([2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![BTreeSet::new(), BTreeSet::from([0]), BTreeSet::from([1])]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::from([1])]
        );
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = vec![
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        assert_eq!(
            digraph,
            vec![
                BTreeMap::from([(1, 1), (2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![
                BTreeMap::from([(2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![
                BTreeMap::new(),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            vec![BTreeMap::new(), BTreeMap::new(), BTreeMap::from([(1, 1)])]
        );
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &mut [BTreeSet<usize>] =
            &mut [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()];

        assert_eq!(
            digraph,
            &[BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![BTreeSet::from([2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![BTreeSet::new(), BTreeSet::from([2]), BTreeSet::new()]
        );

        assert!(digraph.remove_arc(1, 2));

        assert_eq!(
            digraph,
            vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()]
        );
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, i32>] = &mut [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        assert_eq!(
            digraph,
            &[
                BTreeMap::from([(1, 1), (2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![
                BTreeMap::from([(2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![
                BTreeMap::new(),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            vec![BTreeMap::new(), BTreeMap::new(), BTreeMap::from([(1, 1)])]
        );
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = [
            BTreeSet::from([1, 2]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
        ];

        assert_eq!(
            digraph,
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            [
                BTreeSet::from([2]),
                BTreeSet::from([0]),
                BTreeSet::from([1])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            [BTreeSet::new(), BTreeSet::from([0]), BTreeSet::from([1])]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            [BTreeSet::new(), BTreeSet::new(), BTreeSet::from([1])]
        );
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = [
            BTreeMap::from([(1, 1), (2, 1)]),
            BTreeMap::from([(0, 1)]),
            BTreeMap::from([(1, 1)]),
        ];

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 1), (2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(2, 1)]),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            [
                BTreeMap::new(),
                BTreeMap::from([(0, 1)]),
                BTreeMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            [BTreeMap::new(), BTreeMap::new(), BTreeMap::from([(1, 1)])]
        );
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::from([
            (0, BTreeSet::from([1, 2])),
            (1, BTreeSet::from([0])),
            (2, BTreeSet::from([1])),
        ]);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([0])),
                (2, BTreeSet::from([1]))
            ])
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([2])),
                (1, BTreeSet::from([0])),
                (2, BTreeSet::from([1]))
            ])
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::from([0])),
                (2, BTreeSet::from([1]))
            ])
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::new()),
                (1, BTreeSet::new()),
                (2, BTreeSet::from([1]))
            ])
        );
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::from([
            (0, BTreeMap::from([(1, 1), (2, 1)])),
            (1, BTreeMap::from([(0, 1)])),
            (2, BTreeMap::from([(1, 1)])),
        ]);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 1), (2, 1)])),
                (1, BTreeMap::from([(0, 1)])),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::from([(2, 1)])),
                (1, BTreeMap::from([(0, 1)])),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::new()),
                (1, BTreeMap::from([(0, 1)])),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::new()),
                (1, BTreeMap::new()),
                (2, BTreeMap::from([(1, 1)]))
            ])
        );
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = vec![
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        assert_eq!(
            digraph,
            vec![
                HashSet::from([1, 2]),
                HashSet::from([0]),
                HashSet::from([1])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![HashSet::new(), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            vec![HashSet::new(), HashSet::new(), HashSet::from([1])]
        );
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = vec![
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(
            digraph,
            vec![
                HashMap::from([(1, 1), (2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            vec![HashMap::new(), HashMap::new(), HashMap::from([(1, 1)])]
        );
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &mut [HashSet<usize>] =
            &mut [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()];

        assert_eq!(
            digraph,
            &[HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![HashSet::from([2]), HashSet::from([2]), HashSet::new()]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![HashSet::new(), HashSet::from([2]), HashSet::new()]
        );

        assert!(digraph.remove_arc(1, 2));

        assert_eq!(
            digraph,
            vec![HashSet::new(), HashSet::new(), HashSet::new()]
        );
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &mut [HashMap<usize, i32>] = &mut [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(
            digraph,
            &[
                HashMap::from([(1, 1), (2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            vec![
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            vec![
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            vec![HashMap::new(), HashMap::new(), HashMap::from([(1, 1)])]
        );
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = [
            HashSet::from([1, 2]),
            HashSet::from([0]),
            HashSet::from([1]),
        ];

        assert_eq!(
            digraph,
            [
                HashSet::from([1, 2]),
                HashSet::from([0]),
                HashSet::from([1])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            [HashSet::from([2]), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            [HashSet::new(), HashSet::from([0]), HashSet::from([1])]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            [HashSet::new(), HashSet::new(), HashSet::from([1])]
        );
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = [
            HashMap::from([(1, 1), (2, 1)]),
            HashMap::from([(0, 1)]),
            HashMap::from([(1, 1)]),
        ];

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 1), (2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            [
                HashMap::from([(2, 1)]),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            [
                HashMap::new(),
                HashMap::from([(0, 1)]),
                HashMap::from([(1, 1)])
            ]
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            [HashMap::new(), HashMap::new(), HashMap::from([(1, 1)])]
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::from([
            (0, HashSet::from([1, 2])),
            (1, HashSet::from([0])),
            (2, HashSet::from([1])),
        ]);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([2])),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::from([0])),
                (2, HashSet::from([1]))
            ])
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::new()),
                (1, HashSet::new()),
                (2, HashSet::from([1]))
            ])
        );
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::from([
            (0, HashMap::from([(1, 1), (2, 1)])),
            (1, HashMap::from([(0, 1)])),
            (2, HashMap::from([(1, 1)])),
        ]);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::from([(1, 1), (2, 1)])),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        assert!(digraph.remove_arc(0, 1));

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::from([(2, 1)])),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        assert!(digraph.remove_arc(0, 2));

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::new()),
                (1, HashMap::from([(0, 1)])),
                (2, HashMap::from([(1, 1)]))
            ])
        );

        assert!(digraph.remove_arc(1, 0));

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::new()),
                (1, HashMap::new()),
                (2, HashMap::from([(1, 1)]))
            ])
        );
    }
}
