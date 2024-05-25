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
/// struct Digraph {
///     arcs: Vec<Vec<(usize, i32)>>,
/// }
///
/// impl AddWeightedArc<i32> for Digraph {
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
    W: Eq + Hash,
    H: BuildHasher,
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

impl<W> AddWeightedArc<W> for Vec<(usize, usize, W)> {
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        self.push((s, t, w));
    }
}

impl<W> AddWeightedArc<W> for BTreeSet<(usize, usize, W)>
where
    W: Ord,
{
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self.insert((s, t, w));
    }
}

impl<W, H> AddWeightedArc<W> for HashSet<(usize, usize, W), H>
where
    W: Eq + Hash,
    H: BuildHasher,
{
    fn add_weighted_arc(&mut self, s: usize, t: usize, w: W) {
        let _ = self.insert((s, t, w));
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
            op::IterAllWeightedArcs,
        },
    };

    macro_rules! test_add_weighted_arc {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 2);
            $digraph.add_weighted_arc(0, 2, 1);
            $digraph.add_weighted_arc(1, 0, -2);
            $digraph.add_weighted_arc(2, 0, 3);

            let mut iter = $digraph.iter_all_weighted_arcs();

            assert!(matches!(
                iter.next(),
                Some((0, 1, 2) | (0, 2, 1) | (1, 0, -2) | (2, 0, 3))
            ));

            assert!(matches!(
                iter.next(),
                Some((0, 1, 2) | (0, 2, 1) | (1, 0, -2) | (2, 0, 3))
            ));

            assert!(matches!(
                iter.next(),
                Some((0, 1, 2) | (0, 2, 1) | (1, 0, -2) | (2, 0, 3))
            ));

            assert!(matches!(
                iter.next(),
                Some((0, 1, 2) | (0, 2, 1) | (1, 0, -2) | (2, 0, 3))
            ));

            assert_eq!(iter.next(), None);
        };
    }

    macro_rules! test_add_weighted_arc_const {
        ($ty:ty) => {
            let mut digraph = <$ty>::empty();

            test_add_weighted_arc!(digraph);
        };
    }

    macro_rules! test_add_weighted_arc_dynamic {
        ($ty:ty) => {
            let mut digraph = <$ty>::empty(3);

            test_add_weighted_arc!(digraph);
        };
    }

    #[test]
    fn vec_vec() {
        test_add_weighted_arc_dynamic!(Vec::<Vec<(usize, i32)>>);
    }

    #[test]
    fn vec_btree_set() {
        test_add_weighted_arc_dynamic!(Vec::<BTreeSet<(usize, i32)>>);
    }

    #[test]
    fn vec_hash_set() {
        test_add_weighted_arc_dynamic!(Vec::<HashSet<(usize, i32)>>);
    }

    #[test]
    fn vec_btree_map() {
        test_add_weighted_arc_dynamic!(Vec::<BTreeMap<usize, i32>>);
    }

    #[test]
    fn vec_hash_map() {
        test_add_weighted_arc_dynamic!(Vec::<HashMap<usize, i32>>);
    }

    #[test]
    fn arr_vec() {
        test_add_weighted_arc_const!([Vec<(usize, i32)>; 3]);
    }

    #[test]
    fn arr_btree_set() {
        test_add_weighted_arc_const!([BTreeSet<(usize, i32)>; 3]);
    }

    #[test]
    fn arr_hash_set() {
        test_add_weighted_arc_const!([HashSet<(usize, i32)>; 3]);
    }

    #[test]
    fn arr_btree_map() {
        test_add_weighted_arc_const!([BTreeMap<usize, i32>; 3]);
    }

    #[test]
    fn arr_hash_map() {
        test_add_weighted_arc_const!([HashMap<usize, i32>; 3]);
    }

    #[test]
    fn btree_map_vec() {
        test_add_weighted_arc_dynamic!(BTreeMap::<usize, Vec<(usize, i32)>>);
    }

    #[test]
    fn btree_map_btree_set() {
        test_add_weighted_arc_dynamic!(BTreeMap::<usize, BTreeSet<(usize, i32)>>);
    }

    #[test]
    fn btree_map_btree_map() {
        test_add_weighted_arc_dynamic!(BTreeMap::<usize, BTreeMap<usize, i32>>);
    }

    #[test]
    fn hash_map_vec() {
        test_add_weighted_arc_dynamic!(HashMap::<usize, Vec<(usize, i32)>>);
    }

    #[test]
    fn hash_map_hash_set() {
        test_add_weighted_arc_dynamic!(HashMap::<usize, HashSet<(usize, i32)>>);
    }

    #[test]
    fn hash_map_hash_map() {
        test_add_weighted_arc_dynamic!(HashMap::<usize, HashMap<usize, i32>>);
    }

    #[test]
    fn vec_tuple() {
        test_add_weighted_arc_dynamic!(Vec::<(usize, usize, i32)>);
    }

    #[test]
    fn btree_set_tuple() {
        test_add_weighted_arc_dynamic!(BTreeSet::<(usize, usize, i32)>);
    }

    #[test]
    fn hash_set_tuple() {
        test_add_weighted_arc_dynamic!(HashSet::<(usize, usize, i32)>);
    }
}
