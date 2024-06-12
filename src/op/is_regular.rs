//! Determine whether a digraph is regular.
//!
//! A digraph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         gen::Cycle,
//!         op::{
//!             IsRegular,
//!             RemoveArc,
//!         },
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let mut digraph = Vec::<BTreeSet<usize>>::cycle(7);
//!
//! assert!(digraph.is_regular());
//!
//! digraph.remove_arc(6, 0);
//!
//! assert!(!digraph.is_regular());
//! ```

use crate::op::{
    Indegree,
    IterVertices,
    Outdegree,
};

/// Determine whether a digraph is regular.
///
/// # How can I implement `IsRegular`?
///
/// Provide an implementation of `is_regular` that returns `true` if the digraph
/// is regular and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IsRegular,
///         IterVertices,
///         Outdegree,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsRegular for Digraph {
///     /// # Panics
///     ///
///     /// Panics if the digraph has no vertices.
///     fn is_regular(&self) -> bool {
///         let mut vertices = self.arcs.iter_vertices();
///
///         let v = vertices
///             .next()
///             .expect("a graph must have at least one vertex");
///
///         let indegree = self.arcs.indegree(v);
///         let outdegree = self.arcs.outdegree(v);
///
///         vertices
///             .all(|v| self.arcs.indegree(v) == indegree && self.arcs.outdegree(v) == outdegree)
///     }
/// }
/// ```
pub trait IsRegular {
    /// Returns whether the digraph is regular.
    fn is_regular(&self) -> bool;
}

impl<T> IsRegular for T
where
    T: Indegree + IterVertices + Outdegree + ?Sized,
{
    /// # Panics
    ///
    /// Panics if the digraph has no vertices.
    fn is_regular(&self) -> bool {
        let mut vertices = self.iter_vertices();

        let v = vertices
            .next()
            .expect("a graph must have at least one vertex");

        let indegree = self.indegree(v);
        let outdegree = self.outdegree(v);

        vertices.all(|v| self.indegree(v) == indegree && self.outdegree(v) == outdegree)
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
                Empty,
                EmptyConst,
            },
            op::{
                AddWeightedArc,
                RemoveArc,
            },
        },
        proptest::proptest,
        std::collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
            HashSet,
        },
    };

    macro_rules! test_is_regular {
        ($digraph:expr) => {
            assert!($digraph.is_regular());
            assert!($digraph.remove_arc(2, 0));
            assert!(!$digraph.is_regular());
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {{
            $digraph.add_weighted_arc(0, 1, 4);
            $digraph.add_weighted_arc(1, 2, 3);
            $digraph.add_weighted_arc(2, 0, 3);
        }};
    }

    proptest! {
        #[test]
        fn empty_vec_btree_set_unweighted(v in 1..100_usize) {
            let digraph = Vec::<BTreeSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_vec_hash_set_unweighted(v in 1..100_usize) {
            let digraph = Vec::<HashSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_btree_map_btree_set_unweighted(v in 1..100_usize) {
            let digraph = BTreeMap::<usize, BTreeSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_hash_map_hash_set_unweighted(v in 1..100_usize) {
            let digraph = HashMap::<usize, HashSet<usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_vec_btree_map(v in 1..100_usize) {
            let digraph = Vec::<BTreeMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_vec_hash_map(v in 1..100_usize) {
            let digraph = Vec::<HashMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_btree_map_btree_map(v in 1..100_usize) {
            let digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }

        #[test]
        fn empty_hash_map_hash_map(v in 1..100_usize) {
            let digraph = HashMap::<usize, HashMap<usize, usize>>::empty(v);

            assert!(digraph.is_regular());
        }
    }

    #[test]
    fn vec_btree_set_unweighted() {
        let digraph = &mut <Vec<BTreeSet<usize>>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn vec_hash_set_unweighted() {
        let digraph = &mut <Vec<HashSet<usize>>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn slice_btree_set_unweighted() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn slice_hash_set_unweighted() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn arr_btree_set_unweighted() {
        let digraph = &mut <[BTreeSet<usize>; 3]>::cycle();

        test_is_regular!(digraph);
    }

    #[test]
    fn arr_hash_set_unweighted() {
        let digraph = &mut <[HashSet<usize>; 3]>::cycle();

        test_is_regular!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let mut digraph = BTreeMap::<usize, BTreeSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let mut digraph = HashMap::<usize, HashSet<usize>>::cycle(3);

        test_is_regular!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let mut digraph = Vec::<BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_is_regular!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let mut digraph = Vec::<HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_is_regular!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let mut digraph = <[BTreeMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_is_regular!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let mut digraph = <[HashMap<usize, usize>; 3]>::empty();

        setup_weighted!(digraph);
        test_is_regular!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let mut digraph = BTreeMap::<usize, BTreeMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_is_regular!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let mut digraph = HashMap::<usize, HashMap<usize, usize>>::empty(3);

        setup_weighted!(digraph);
        test_is_regular!(digraph);
    }
}
