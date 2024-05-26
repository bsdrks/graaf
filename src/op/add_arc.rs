//! Add an arc to a unweighted digraph
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

/// Add an arc to a unweighted digraph
///
/// # How can I implement `AddArc`?
///
/// Provide an implementation of `add_arc` that adds an unweighted arc from
/// `s` to `t` to the digraph.
///
/// ```
/// use graaf::op::AddArc;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl AddArc for Digraph {
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

impl AddArc for Vec<(usize, usize)> {
    fn add_arc(&mut self, s: usize, t: usize) {
        self.push((s, t));
    }
}

impl AddArc for BTreeSet<(usize, usize)> {
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self.insert((s, t));
    }
}

impl<H> AddArc for HashSet<(usize, usize), H>
where
    H: BuildHasher,
{
    fn add_arc(&mut self, s: usize, t: usize) {
        let _ = self.insert((s, t));
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
            op::IterArcs,
        },
    };

    macro_rules! test_add_arc {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(2, 1);

            let mut iter = $digraph.iter_arcs();

            assert!(matches!(iter.next(), Some((0, 1 | 2) | (2, 1))));
            assert!(matches!(iter.next(), Some((0, 1 | 2) | (2, 1))));
            assert!(matches!(iter.next(), Some((0, 1 | 2) | (2, 1))));

            assert_eq!(iter.next(), None);
        };
    }

    macro_rules! test_add_arc_const {
        ($ty:ty) => {
            let mut digraph = <$ty>::empty();

            test_add_arc!(digraph);
        };
    }

    macro_rules! test_add_arc_dynamic {
        ($ty:ty) => {
            let mut digraph = <$ty>::empty(3);

            test_add_arc!(digraph);
        };
    }

    #[test]
    fn vec_vec() {
        test_add_arc_dynamic!(Vec<Vec<usize>>);
    }

    #[test]
    fn vec_btree_set() {
        test_add_arc_dynamic!(Vec<BTreeSet<usize>>);
    }

    #[test]
    fn vec_hash_set() {
        test_add_arc_dynamic!(Vec<HashSet<usize>>);
    }

    #[test]
    fn arr_vec() {
        test_add_arc_const!([Vec<usize>; 3]);
    }

    #[test]
    fn arr_btree_set() {
        test_add_arc_const!([BTreeSet<usize>; 3]);
    }

    #[test]
    fn arr_hash_set() {
        test_add_arc_const!([HashSet<usize>; 3]);
    }

    #[test]
    fn btree_map_vec() {
        test_add_arc_dynamic!(BTreeMap < usize, Vec<usize>>);
    }

    #[test]
    fn btree_map_btree_set() {
        test_add_arc_dynamic!(BTreeMap < usize, BTreeSet<usize>>);
    }

    #[test]
    fn hash_map_vec() {
        test_add_arc_dynamic!(HashMap < usize, Vec<usize>>);
    }

    #[test]
    fn hash_map_hash_set() {
        test_add_arc_dynamic!(HashMap < usize, HashSet<usize>>);
    }

    #[test]
    fn vec_tuple() {
        test_add_arc_dynamic!(Vec<(usize, usize)>);
    }

    #[test]
    fn btree_set_tuple() {
        test_add_arc_dynamic!(BTreeSet<(usize, usize)>);
    }

    #[test]
    fn hash_set_tuple() {
        test_add_arc_dynamic!(HashSet<(usize, usize)>);
    }
}
