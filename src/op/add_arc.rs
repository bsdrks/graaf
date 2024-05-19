//! A trait to add an arc to a unweighted digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::AddArc;
//!
//! let mut digraph = vec![Vec::new(); 3];
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(2, 0);
//!
//! assert_eq!(digraph, vec![vec![1, 2], Vec::new(), vec![0]]);
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

/// A trait to add an arc to a unweighted digraph
///
/// # How can I implement `AddArc`?
///
/// Provide an implementation of `add_arc` that adds an unweighted arc from
/// `s` to `t` to the digraph.
///
/// ```
/// use graaf::op::AddArc;
///
/// struct Graph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl AddArc for Graph {
///     fn add_arc(&mut self, s: usize, t: usize) {
///         self.arcs[s].push(t);
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::AddArc;
///
/// let mut digraph = vec![Vec::new(); 3];
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(2, 0);
///
/// assert_eq!(digraph, vec![vec![1, 2], Vec::new(), vec![0]]);
/// ```
///
/// # Properties
///
/// ## `AddArc` and `RemoveArc`
///
/// Types that also implement [`RemoveArc`] should ensure that
/// [`add_arc_remove_arc`] holds.
///
/// ## `AddArc` and `HasArc`
///
/// Types that also implement [`HasArc`] should ensure that
/// [`add_arc_has_arc`] holds.
///
/// [`HasArc`]: crate::op::HasArc
/// [`RemoveArc`]: crate::op::RemoveArc
/// [`add_arc_has_arc`]: crate::prop::add_arc_has_arc
/// [`add_arc_remove_arc`]: crate::prop::add_arc_remove_arc
pub trait AddArc {
    /// Adds an arc from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    fn add_arc(&mut self, s: usize, t: usize);
}

impl AddArc for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl AddArc for Vec<BTreeSet<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<H> AddArc for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl AddArc for [Vec<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl AddArc for [BTreeSet<usize>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<H> AddArc for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<const V: usize> AddArc for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl<const V: usize> AddArc for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl<H, const V: usize> AddArc for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

impl AddArc for BTreeMap<usize, Vec<usize>> {
    fn add_arc(&mut self, s: usize, t: usize) {
        self.entry(s).or_default().push(t);
    }
}

impl AddArc for BTreeMap<usize, BTreeSet<usize>> {
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self.entry(s).or_default().insert(t);
    }
}

impl<H> AddArc for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    fn add_arc(&mut self, s: usize, t: usize) {
        self.entry(s).or_default().push(t);
    }
}

impl<H> AddArc for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
    HashSet<usize, H>: Default,
{
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self.entry(s).or_default().insert(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        let mut digraph = vec![Vec::new(); 3];

        digraph.add_arc(0, 1);

        assert_eq!(digraph, vec![vec![1], Vec::new(), Vec::new()]);

        digraph.add_arc(0, 2);

        assert_eq!(digraph, vec![vec![1, 2], Vec::new(), Vec::new()]);

        digraph.add_arc(1, 2);

        assert_eq!(digraph, vec![vec![1, 2], vec![2], Vec::new()]);

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(digraph, vec![vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = vec![BTreeSet::new(); 3];

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            vec![BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            vec![BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            vec![BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            vec![
                BTreeSet::from([1, 2]),
                BTreeSet::from([2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = vec![HashSet::new(); 3];

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            vec![HashSet::from([1]), HashSet::new(), HashSet::new()]
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            vec![HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            vec![
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn slice_vec() {
        let digraph: &mut [Vec<usize>] = &mut [Vec::new(), Vec::new(), Vec::new()];

        digraph.add_arc(0, 1);

        assert_eq!(*digraph, [vec![1], Vec::new(), Vec::new()]);

        digraph.add_arc(0, 2);

        assert_eq!(*digraph, [vec![1, 2], Vec::new(), Vec::new()]);

        digraph.add_arc(1, 2);

        assert_eq!(*digraph, [vec![1, 2], vec![2], Vec::new()]);

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(*digraph, [vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &mut [BTreeSet<usize>] =
            &mut [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        digraph.add_arc(0, 1);

        assert_eq!(
            *digraph,
            [BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            *digraph,
            [BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            *digraph,
            [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            *digraph,
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &mut [HashSet<usize>] = &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        digraph.add_arc(0, 1);

        assert_eq!(
            *digraph,
            [HashSet::from([1]), HashSet::new(), HashSet::new()]
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            *digraph,
            [HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            *digraph,
            [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            *digraph,
            [
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn arr_vec() {
        let mut digraph = [Vec::new(), Vec::new(), Vec::new()];

        digraph.add_arc(0, 1);

        assert_eq!(digraph, [vec![1], Vec::new(), Vec::new()]);

        digraph.add_arc(0, 2);

        assert_eq!(digraph, [vec![1, 2], Vec::new(), Vec::new()]);

        digraph.add_arc(1, 2);

        assert_eq!(digraph, [vec![1, 2], vec![2], Vec::new()]);

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(digraph, [vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            [BTreeSet::from([1]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            [BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            [BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = [HashSet::new(), HashSet::new(), HashSet::new()];

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            [HashSet::from([1]), HashSet::new(), HashSet::new()]
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            [HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            [
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            BTreeMap::from([(0, vec![1]), (1, Vec::new()), (2, Vec::new())])
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            BTreeMap::from([(0, vec![1, 2]), (1, Vec::new()), (2, Vec::new())])
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            BTreeMap::from([(0, vec![1, 2]), (1, vec![2]), (2, Vec::new())])
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            BTreeMap::from([(0, vec![1, 2]), (1, vec![2]), (2, vec![0, 1])])
        );
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::from([
            (0, BTreeSet::new()),
            (1, BTreeSet::new()),
            (2, BTreeSet::new()),
        ]);

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([1])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([2])),
                (2, BTreeSet::new())
            ])
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([1, 2])),
                (1, BTreeSet::from([2])),
                (2, BTreeSet::from([0, 1]))
            ])
        );
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            HashMap::from([(0, vec![1]), (1, Vec::new()), (2, Vec::new())])
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            HashMap::from([(0, vec![1, 2]), (1, Vec::new()), (2, Vec::new())])
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            HashMap::from([(0, vec![1, 2]), (1, vec![2]), (2, Vec::new())])
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            HashMap::from([(0, vec![1, 2]), (1, vec![2]), (2, vec![0, 1])])
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::from([
            (0, HashSet::new()),
            (1, HashSet::new()),
            (2, HashSet::new()),
        ]);

        digraph.add_arc(0, 1);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([1])),
                (1, HashSet::new()),
                (2, HashSet::new())
            ])
        );

        digraph.add_arc(0, 2);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::new()),
                (2, HashSet::new())
            ])
        );

        digraph.add_arc(1, 2);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([2])),
                (2, HashSet::new())
            ])
        );

        digraph.add_arc(2, 0);
        digraph.add_arc(2, 1);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([2])),
                (2, HashSet::from([0, 1]))
            ])
        );
    }
}
