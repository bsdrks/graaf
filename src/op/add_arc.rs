//! Add an arc to an unweighted digraph.
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

use {
    core::hash::BuildHasher,
    std::collections::{
        BTreeMap,
        BTreeSet,
        HashMap,
        HashSet,
    },
};

/// Add an arc to an unweighted digraph.
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
    /// Adds an arc from `s` to `t` to the digraph.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    fn add_arc(&mut self, s: usize, t: usize);
}

macro_rules! impl_index_push {
    () => {
        /// # Panics
        ///
        /// Panics if `s` is not in the digraph.
        fn add_arc(&mut self, s: usize, t: usize) {
            self[s].push(t);
        }
    };
}

macro_rules! impl_index_insert {
    () => {
        /// # Panics
        ///
        /// Panics if `s` is not in the digraph.
        fn add_arc(&mut self, s: usize, t: usize) {
            let _ = self[s].insert(t);
        }
    };
}

macro_rules! impl_entry_push {
    () => {
        fn add_arc(&mut self, s: usize, t: usize) {
            self.entry(s).or_default().push(t);
        }
    };
}

macro_rules! impl_entry_insert {
    () => {
        fn add_arc(&mut self, s: usize, t: usize) {
            let _ = self.entry(s).or_default().insert(t);
        }
    };
}

impl AddArc for Vec<Vec<usize>> {
    impl_index_push!();
}

impl AddArc for Vec<BTreeSet<usize>> {
    impl_index_insert!();
}

impl<H> AddArc for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    impl_index_insert!();
}

impl AddArc for [Vec<usize>] {
    impl_index_push!();
}

impl AddArc for [BTreeSet<usize>] {
    impl_index_insert!();
}

impl<H> AddArc for [HashSet<usize, H>]
where
    H: BuildHasher,
{
    impl_index_insert!();
}

impl<const V: usize> AddArc for [Vec<usize>; V] {
    impl_index_push!();
}

impl<const V: usize> AddArc for [BTreeSet<usize>; V] {
    impl_index_insert!();
}

impl<H, const V: usize> AddArc for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    impl_index_insert!();
}

impl AddArc for BTreeMap<usize, Vec<usize>> {
    impl_entry_push!();
}

impl AddArc for BTreeMap<usize, BTreeSet<usize>> {
    impl_entry_insert!();
}

impl<H> AddArc for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    impl_entry_push!();
}

impl<H> AddArc for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
    HashSet<usize, H>: Default,
{
    impl_entry_insert!();
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

    #[test]
    fn vec_vec() {
        let mut digraph = Vec::<Vec<usize>>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn vec_btree_set() {
        let mut digraph = Vec::<BTreeSet<usize>>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let mut digraph = Vec::<HashSet<usize>>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn arr_vec() {
        let mut digraph = <[Vec<usize>; 3]>::empty();

        test_add_arc!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let mut digraph = <[BTreeSet<usize>; 3]>::empty();

        test_add_arc!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let mut digraph = <[HashSet<usize>; 3]>::empty();

        test_add_arc!(digraph);
    }

    #[test]
    fn btree_map_vec() {
        let mut digraph = BTreeMap::<usize, Vec<usize>>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn hash_map_vec() {
        let mut digraph = HashMap::<usize, Vec<usize>>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn vec_tuple() {
        let mut digraph = Vec::<(usize, usize)>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn btree_set_tuple() {
        let mut digraph = BTreeSet::<(usize, usize)>::empty(3);

        test_add_arc!(digraph);
    }

    #[test]
    fn hash_set_tuple() {
        let mut digraph = HashSet::<(usize, usize)>::empty(3);

        test_add_arc!(digraph);
    }
}
