//! Determine whether a vertex in a digraph is isolated.
//!
//! A vertex is isolated if it has no incoming or outgoing arcs.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsIsolated,
//!     std::collections::HashSet,
//! };
//!
//! let digraph = vec![
//!     HashSet::from([1, 2]),
//!     HashSet::from([0]),
//!     HashSet::new(),
//!     HashSet::new(),
//! ];
//!
//! assert!(!digraph.is_isolated(0));
//! assert!(!digraph.is_isolated(1));
//! assert!(!digraph.is_isolated(2));
//! assert!(digraph.is_isolated(3));
//! ```

use super::{
    Indegree,
    Outdegree,
};

/// Determine whether a vertex is isolated.
///
/// # How can I implement `IsIsolated`?
///
/// Provide an implementation of `is_isolated` that returns `true` if the vertex
/// is isolated and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IsIsolated,
///         Outdegree,
///     },
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl IsIsolated for Digraph {
///     fn is_isolated(&self, s: usize) -> bool {
///         self.arcs.indegree(s) == 0 && self.arcs.outdegree(s) == 0
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![
///         HashSet::from([1, 2]),
///         HashSet::from([0]),
///         HashSet::new(),
///         HashSet::new(),
///     ],
/// };
///
/// assert!(!digraph.is_isolated(0));
/// assert!(!digraph.is_isolated(1));
/// assert!(!digraph.is_isolated(2));
/// assert!(digraph.is_isolated(3));
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::IsIsolated,
///     std::collections::HashSet,
/// };
///
/// let digraph = vec![HashSet::from([1, 2]), HashSet::from([0]), HashSet::new()];
///
/// assert!(!digraph.is_isolated(0));
/// assert!(!digraph.is_isolated(1));
/// assert!(!digraph.is_isolated(2));
/// assert!(digraph.is_isolated(3));
/// ```
pub trait IsIsolated {
    /// Returns whether the vertex is isolated in the digraph.
    fn is_isolated(&self, s: usize) -> bool;
}

impl<T> IsIsolated for T
where
    T: Indegree + Outdegree + ?Sized,
{
    fn is_isolated(&self, s: usize) -> bool {
        self.indegree(s) == 0 && self.outdegree(s) == 0
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
        std::collections::{
            BTreeMap,
            BTreeSet,
            HashMap,
            HashSet,
        },
    };

    macro_rules! test_is_isolated {
        ($digraph:expr) => {
            assert!(!$digraph.is_isolated(0));
            assert!(!$digraph.is_isolated(1));
            assert!(!$digraph.is_isolated(2));
            assert!($digraph.is_isolated(3));
        };
    }

    macro_rules! setup_unweighted {
        ($digraph:expr) => {
            $digraph.add_arc(0, 1);
            $digraph.add_arc(0, 2);
            $digraph.add_arc(1, 2);
        };
    }

    macro_rules! setup_weighted {
        ($digraph:expr) => {
            $digraph.add_weighted_arc(0, 1, 1);
            $digraph.add_weighted_arc(0, 2, 1);
            $digraph.add_weighted_arc(1, 2, 1);
        };
    }

    #[test]
    fn vec_btree_set() {
        let digraph = &mut Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = &mut Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let digraph: &mut [BTreeSet<usize>; 4] = &mut <[BTreeSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph: &mut [HashSet<usize>; 4] = &mut <[HashSet<usize>; 4]>::empty();

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn btree_map_btree_set() {
        let digraph = &mut BTreeMap::<usize, BTreeSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn hash_map_hash_set() {
        let digraph = &mut HashMap::<usize, HashSet<usize>>::empty(4);

        setup_unweighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = &mut Vec::<HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, i32>] = &mut Vec::<BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &mut [HashMap<usize, i32>] = &mut Vec::<HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = &mut <[BTreeMap<usize, i32>; 4]>::empty();

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = &mut <[HashMap<usize, i32>; 4]>::empty();

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn btree_map_btree_map() {
        let digraph = &mut BTreeMap::<usize, BTreeMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }

    #[test]
    fn hash_map_hash_map() {
        let digraph = &mut HashMap::<usize, HashMap<usize, i32>>::empty(4);

        setup_weighted!(digraph);
        test_is_isolated!(digraph);
    }
}
