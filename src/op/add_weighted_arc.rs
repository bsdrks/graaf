//! A trait to add an arc to a directed weighted digraph
//!
//! # Examples
//!
//! ```
//! use graaf::op::AddWeightedArc;
//!
//! let mut digraph = vec![Vec::new(); 3];
//!
//! digraph.add_weighted_arc(0, 1, 2);
//! digraph.add_weighted_arc(0, 2, 1);
//! digraph.add_weighted_arc(1, 2, -3);
//!
//! assert_eq!(
//!     digraph,
//!     vec![vec![(1, 2), (2, 1)], vec![(2, -3)], Vec::new()]
//! );
//! ```

extern crate alloc;

use {
    alloc::collections::{
        BTreeMap,
        BTreeSet,
    },
    core::hash::{
        BuildHasher,
        Hash,
    },
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to add an arc to a weighted digraph
///
/// # How can I implement `AddWeightedArc`?
///
/// Provide an implementation of `add_weighted_arc` that adds an arc from `s`
/// to `t` to the digraph with weight `w`.
///
/// ```
/// use graaf::op::AddWeightedArc;
///
/// struct Graph {
///     arcs: Vec<Vec<(usize, i32)>>,
/// }
///
/// impl AddWeightedArc<i32> for Graph {
///     fn add_weighted_arc(&mut self, s: usize, t: usize, w: i32) {
///         self.arcs[s].push((t, w));
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::op::AddWeightedArc;
///
/// let mut digraph = vec![Vec::new(); 3];
///
/// digraph.add_weighted_arc(0, 1, 2);
/// digraph.add_weighted_arc(0, 2, 1);
/// digraph.add_weighted_arc(1, 2, -3);
///
/// assert_eq!(
///     digraph,
///     vec![vec![(1, 2), (2, 1)], vec![(2, -3)], Vec::new()]
/// );
/// ```
///
/// # Properties
///
/// ## `AddWeightedArc` and `RemoveArc`
///
/// Types that also implement [`RemoveArc`] should
/// ensure that [`add_weighted_arc_remove_arc`] holds.
///
/// ## `AddWeightedArc` and `HasArc`
///
/// Types that also implement [`HasArc`] should ensure that
/// [`add_weighted_arc_has_arc`] holds.
///
/// [`HasArc`]: crate::op::HasArc
/// [`RemoveArc`]: crate::op::RemoveArc
/// [`add_weighted_arc_has_arc`]: crate::prop::add_weighted_arc_has_arc
/// [`add_weighted_arc_remove_arc`]: crate::prop::add_weighted_arc_remove_arc
pub trait AddWeightedArc<W> {
    /// Adds an arc from `s` to `t` with weight `w`.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    /// * `w`: The weight of the arc.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W);
}

impl<W> AddWeightedArc<W> for Vec<Vec<(usize, W)>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<W> AddWeightedArc<W> for Vec<BTreeSet<(usize, W)>>
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<W> AddWeightedArc<W> for Vec<BTreeMap<usize, W>>
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

impl<W, H> AddWeightedArc<W> for Vec<HashSet<(usize, W), H>>
where
    H: BuildHasher,
    W: Eq + Hash,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<W, H> AddWeightedArc<W> for Vec<HashMap<usize, W, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

impl<W> AddWeightedArc<W> for [Vec<(usize, W)>] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<W> AddWeightedArc<W> for [BTreeSet<(usize, W)>]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<W> AddWeightedArc<W> for [BTreeMap<usize, W>]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

impl<W, H> AddWeightedArc<W> for [HashSet<(usize, W), H>]
where
    H: BuildHasher,
    W: Eq + Hash,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<W, H> AddWeightedArc<W> for [HashMap<usize, W, H>]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

impl<const V: usize, W> AddWeightedArc<W> for [Vec<(usize, W)>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        self[s].push((t, w));
    }
}

impl<const V: usize, W> AddWeightedArc<W> for [BTreeSet<(usize, W)>; V]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<const V: usize, W> AddWeightedArc<W> for [BTreeMap<usize, W>; V]
where
    W: Ord,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

impl<const V: usize, W, H> AddWeightedArc<W> for [HashSet<(usize, W), H>; V]
where
    H: BuildHasher,
    W: Eq + Hash,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert((t, w));
    }
}

impl<const V: usize, W, H> AddWeightedArc<W> for [HashMap<usize, W, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the digraph.
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self[s].insert(t, w);
    }
}

impl<W> AddWeightedArc<W> for BTreeMap<usize, Vec<(usize, W)>> {
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        self.entry(s).or_default().push((t, w));
    }
}

impl<W> AddWeightedArc<W> for BTreeMap<usize, BTreeSet<(usize, W)>>
where
    W: Ord,
{
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self.entry(s).or_default().insert((t, w));
    }
}

impl<W> AddWeightedArc<W> for BTreeMap<usize, BTreeMap<usize, W>> {
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self.entry(s).or_default().insert(t, w);
    }
}

impl<W, H> AddWeightedArc<W> for HashMap<usize, Vec<(usize, W)>, H>
where
    H: BuildHasher,
{
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        self.entry(s).or_default().push((t, w));
    }
}

impl<W, H> AddWeightedArc<W> for HashMap<usize, HashSet<(usize, W), H>, H>
where
    H: BuildHasher,
    W: Eq + Hash,
    HashSet<(usize, W), H>: Default,
{
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self.entry(s).or_default().insert((t, w));
    }
}

impl<W, H> AddWeightedArc<W> for HashMap<usize, HashMap<usize, W, H>, H>
where
    H: BuildHasher,
    HashMap<usize, W, H>: Default,
{
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self.entry(s).or_default().insert(t, w);
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        core::iter::once,
    };

    #[test]
    fn vec_vec() {
        let mut digraph = vec![Vec::new(); 3];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(digraph, vec![vec![(1, 2)], Vec::new(), Vec::new()]);

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(digraph, vec![vec![(1, 2), (2, 1)], Vec::new(), Vec::new()]);

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            vec![vec![(1, 2), (2, 1)], vec![(2, 4)], Vec::new()]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            vec![vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], Vec::new()]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            vec![vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], vec![(0, 3)]]
        );
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = vec![BTreeSet::new(); 3];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            vec![once((1, 2)).collect(), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::new(),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4)]),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            vec![
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = vec![HashSet::new(); 3];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            vec![once((1, 2)).collect(), HashSet::new(), HashSet::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::new(),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4)]),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            vec![
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = vec![BTreeMap::new(); 3];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            vec![BTreeMap::from([(1, 2)]), BTreeMap::new(), BTreeMap::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            vec![
                [(1, 2), (2, 1)].into_iter().collect(),
                BTreeMap::new(),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            vec![
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4)]),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            vec![
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            vec![
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::from([(0, 3)]),
            ]
        );
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = vec![HashMap::new(); 3];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            vec![HashMap::from([(1, 2)]), HashMap::new(), HashMap::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            vec![
                [(1, 2), (2, 1)].into_iter().collect(),
                HashMap::new(),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            vec![
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4)]),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            vec![
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            vec![
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::from([(0, 3)]),
            ]
        );
    }

    #[test]
    fn slice_vec() {
        let digraph: &mut [Vec<(usize, i32)>] = &mut [Vec::new(), Vec::new(), Vec::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(digraph, [vec![(1, 2)], Vec::new(), Vec::new()]);

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(digraph, [vec![(1, 2), (2, 1)], Vec::new(), Vec::new()]);

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(digraph, [vec![(1, 2), (2, 1)], vec![(2, 4)], Vec::new()]);

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], Vec::new()]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], vec![(0, 3)]]
        );
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &mut [BTreeSet<(usize, i32)>] =
            &mut [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [BTreeSet::from([(1, 2)]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::new(),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4)]),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &mut [HashSet<(usize, i32)>] =
            &mut [HashSet::new(), HashSet::new(), HashSet::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [HashSet::from([(1, 2)]), HashSet::new(), HashSet::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::new(),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4)]),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, i32>] =
            &mut [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [BTreeMap::from([(1, 2)]), BTreeMap::new(), BTreeMap::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::new(),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4)]),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &mut [HashMap<usize, i32>] =
            &mut [HashMap::new(), HashMap::new(), HashMap::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [HashMap::from([(1, 2)]), HashMap::new(), HashMap::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::new(),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4)]),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_vec() {
        let mut digraph = [Vec::new(), Vec::new(), Vec::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(digraph, [vec![(1, 2)], Vec::new(), Vec::new()]);

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(digraph, [vec![(1, 2), (2, 1)], Vec::new(), Vec::new()]);

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(digraph, [vec![(1, 2), (2, 1)], vec![(2, 4)], Vec::new()]);

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], Vec::new()]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [vec![(1, 2), (2, 1)], vec![(2, 4), (0, -2)], vec![(0, 3)]]
        );
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [BTreeSet::from([(1, 2)]), BTreeSet::new(), BTreeSet::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::new(),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4)]),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                BTreeSet::from([(1, 2), (2, 1)]),
                BTreeSet::from([(2, 4), (0, -2)]),
                BTreeSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = [HashSet::new(), HashSet::new(), HashSet::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [HashSet::from([(1, 2)]), HashSet::new(), HashSet::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::new(),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4)]),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                HashSet::from([(1, 2), (2, 1)]),
                HashSet::from([(2, 4), (0, -2)]),
                HashSet::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = [BTreeMap::new(), BTreeMap::new(), BTreeMap::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [BTreeMap::from([(1, 2)]), BTreeMap::new(), BTreeMap::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::new(),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4)]),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                BTreeMap::from([(1, 2), (2, 1)]),
                BTreeMap::from([(2, 4), (0, -2)]),
                BTreeMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = [HashMap::new(), HashMap::new(), HashMap::new()];

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            [HashMap::from([(1, 2)]), HashMap::new(), HashMap::new()]
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::new(),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4)]),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::new()
            ]
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            [
                HashMap::from([(1, 2), (2, 1)]),
                HashMap::from([(2, 4), (0, -2)]),
                HashMap::from([(0, 3)])
            ]
        );
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            BTreeMap::from([(0, vec![(1, 2)]), (1, Vec::new()), (2, Vec::new())])
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            BTreeMap::from([(0, vec![(1, 2), (2, 1)]), (1, Vec::new()), (2, Vec::new())])
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4)]),
                (2, Vec::new())
            ])
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4), (0, -2)]),
                (2, Vec::new())
            ])
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4), (0, -2)]),
                (2, vec![(0, 3)])
            ])
        );
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::from([
            (0, BTreeSet::new()),
            (1, BTreeSet::new()),
            (2, BTreeSet::new()),
        ]);

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2)])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::new()),
                (2, BTreeSet::new())
            ])
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::from([(2, 4)])),
                (2, BTreeSet::new())
            ])
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::from([(2, 4), (0, -2)])),
                (2, BTreeSet::new())
            ])
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeSet::from([(1, 2), (2, 1)])),
                (1, BTreeSet::from([(2, 4), (0, -2)])),
                (2, BTreeSet::from([(0, 3)])),
            ])
        );
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::from([
            (0, BTreeMap::new()),
            (1, BTreeMap::new()),
            (2, BTreeMap::new()),
        ]);

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2)])),
                (1, BTreeMap::new()),
                (2, BTreeMap::new())
            ])
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::new()),
                (2, BTreeMap::new())
            ])
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::from([(2, 4)])),
                (2, BTreeMap::new())
            ])
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::from([(2, 4), (0, -2)])),
                (2, BTreeMap::new())
            ])
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            BTreeMap::from([
                (0, BTreeMap::from([(1, 2), (2, 1)])),
                (1, BTreeMap::from([(2, 4), (0, -2)])),
                (2, BTreeMap::from([(0, 3)])),
            ])
        );
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::new();

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(digraph, HashMap::from([(0, vec![(1, 2)])]));

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(digraph, HashMap::from([(0, vec![(1, 2), (2, 1)])]));

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            HashMap::from([(0, vec![(1, 2), (2, 1)]), (1, vec![(2, 4)])])
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            HashMap::from([(0, vec![(1, 2), (2, 1)]), (1, vec![(2, 4), (0, -2)])])
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, vec![(1, 2), (2, 1)]),
                (1, vec![(2, 4), (0, -2)]),
                (2, vec![(0, 3)])
            ])
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::new();

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(digraph, HashMap::from([(0, HashSet::from([(1, 2)]))]));

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            HashMap::from([(0, HashSet::from([(1, 2), (2, 1)]))])
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([(1, 2), (2, 1)])),
                (1, HashSet::from([(2, 4)])),
            ])
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([(1, 2), (2, 1)])),
                (1, HashSet::from([(2, 4), (0, -2)])),
            ])
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashSet::from([(1, 2), (2, 1)])),
                (1, HashSet::from([(2, 4), (0, -2)])),
                (2, HashSet::from([(0, 3)])),
            ])
        );
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::from([
            (0, HashMap::new()),
            (1, HashMap::new()),
            (2, HashMap::new()),
        ]);

        digraph.add_weighted_arc(0, 1, 2);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::from([(1, 2)])),
                (1, HashMap::new()),
                (2, HashMap::new())
            ])
        );

        digraph.add_weighted_arc(0, 2, 1);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::new()),
                (2, HashMap::new())
            ])
        );

        digraph.add_weighted_arc(1, 2, 4);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4)])),
                (2, HashMap::new())
            ])
        );

        digraph.add_weighted_arc(1, 0, -2);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4), (0, -2)])),
                (2, HashMap::new())
            ])
        );

        digraph.add_weighted_arc(2, 0, 3);

        assert_eq!(
            digraph,
            HashMap::from([
                (0, HashMap::from([(1, 2), (2, 1)])),
                (1, HashMap::from([(2, 4), (0, -2)])),
                (2, HashMap::from([(0, 3)])),
            ])
        );
    }
}
