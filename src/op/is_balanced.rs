#![doc(alias = "isograph")]
#![doc(alias = "pseudosymmetric")]
//! Determine whether a digraph is balanced.
//!
//! A digraph is balanced if the indegree of each vertex is equal to its
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsBalanced,
//!     std::collections::BTreeSet,
//! };
//!
//! let digraph: [BTreeSet<usize>; 3] = [
//!     BTreeSet::from([1, 2]),
//!     BTreeSet::from([0, 2]),
//!     BTreeSet::from([0, 1]),
//! ];
//!
//! assert!(digraph.is_balanced());
//!
//! let digraph: [BTreeSet<usize>; 3] = [
//!     BTreeSet::from([1, 2]),
//!     BTreeSet::from([0, 2]),
//!     BTreeSet::from([0]),
//! ];
//!
//! assert!(!digraph.is_balanced());
//! ```

use super::{
    Indegree,
    IterVertices,
    Outdegree,
};

/// Determine whether a digraph is balanced.
///
/// # How can I implement `IsBalanced`?
///
/// Provide an implementation of `is_balanced` that returns `true` if the
/// digraph is balanced and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::{
///         Indegree,
///         IsBalanced,
///         IterVertices,
///         Outdegree,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph<const V: usize> {
///     pub arcs: [BTreeSet<usize>; V],
/// }
///
/// impl<const V: usize> IsBalanced for Digraph<V> {
///     fn is_balanced(&self) -> bool {
///         self.arcs
///             .iter_vertices()
///             .all(|s| self.arcs.indegree(s) == self.arcs.outdegree(s))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::IsBalanced,
///     std::collections::BTreeSet,
/// };
///
/// let digraph: [BTreeSet<usize>; 3] = [
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0, 1]),
/// ];
///
/// assert!(digraph.is_balanced());
///
/// let digraph: [BTreeSet<usize>; 3] = [
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0]),
/// ];
///
/// assert!(!digraph.is_balanced());
/// ```
pub trait IsBalanced {
    /// Returns whether the digraph is balanced.
    fn is_balanced(&self) -> bool;
}

impl<T> IsBalanced for T
where
    T: Indegree + IterVertices + Outdegree + ?Sized,
{
    fn is_balanced(&self) -> bool {
        self.iter_vertices()
            .all(|s| self.indegree(s) == self.outdegree(s))
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

    macro_rules! test_unweighted {
        ($digraph:expr) => {
            assert!($digraph.is_balanced());

            $digraph.add_arc(0, 1);

            assert!(!$digraph.is_balanced());

            $digraph.add_arc(1, 0);

            assert!($digraph.is_balanced());

            $digraph.add_arc(0, 2);

            assert!(!$digraph.is_balanced());

            $digraph.add_arc(1, 2);

            assert!(!$digraph.is_balanced());

            $digraph.add_arc(2, 0);

            assert!(!$digraph.is_balanced());

            $digraph.add_arc(2, 1);

            assert!($digraph.is_balanced());
        };
    }

    macro_rules! test_weighted {
        ($digraph:expr) => {
            assert!($digraph.is_balanced());

            $digraph.add_weighted_arc(0, 1, 1);

            assert!(!$digraph.is_balanced());

            $digraph.add_weighted_arc(1, 0, -3);

            assert!($digraph.is_balanced());

            $digraph.add_weighted_arc(0, 2, 2);

            assert!(!$digraph.is_balanced());

            $digraph.add_weighted_arc(1, 2, 0);

            assert!(!$digraph.is_balanced());

            $digraph.add_weighted_arc(2, 0, 1);

            assert!(!$digraph.is_balanced());

            $digraph.add_weighted_arc(2, 1, 1);

            assert!($digraph.is_balanced());
        };
    }

    #[test]
    fn vec_btree_set() {
        let digraph = &mut <Vec<BTreeSet<usize>>>::empty(3);

        test_unweighted!(digraph);
    }

    #[test]
    fn vec_hash_set() {
        let digraph = &mut <Vec<HashSet<usize>>>::empty(3);

        test_unweighted!(digraph);
    }

    #[test]
    fn slice_btree_set() {
        let digraph: &mut [BTreeSet<usize>] = &mut Vec::<BTreeSet<usize>>::empty(3);

        test_unweighted!(digraph);
    }

    #[test]
    fn slice_hash_set() {
        let digraph: &mut [HashSet<usize>] = &mut Vec::<HashSet<usize>>::empty(3);

        test_unweighted!(digraph);
    }

    #[test]
    fn arr_btree_set() {
        let digraph = &mut <[BTreeSet<usize>; 3]>::empty();

        test_unweighted!(digraph);
    }

    #[test]
    fn arr_hash_set() {
        let digraph = &mut <[HashSet<usize>; 3]>::empty();

        test_unweighted!(digraph);
    }

    #[test]
    fn vec_btree_map() {
        let digraph = &mut <Vec<BTreeMap<usize, i32>>>::empty(3);

        test_weighted!(digraph);
    }

    #[test]
    fn vec_hash_map() {
        let digraph = &mut <Vec<HashMap<usize, i32>>>::empty(3);

        test_weighted!(digraph);
    }

    #[test]
    fn slice_btree_map() {
        let digraph: &mut [BTreeMap<usize, i32>] = &mut Vec::<BTreeMap<usize, i32>>::empty(3);

        test_weighted!(digraph);
    }

    #[test]
    fn slice_hash_map() {
        let digraph: &mut [HashMap<usize, i32>] = &mut Vec::<HashMap<usize, i32>>::empty(3);

        test_weighted!(digraph);
    }

    #[test]
    fn arr_btree_map() {
        let digraph = &mut <[BTreeMap<usize, i32>; 3]>::empty();

        test_weighted!(digraph);
    }

    #[test]
    fn arr_hash_map() {
        let digraph = &mut <[HashMap<usize, i32>; 3]>::empty();

        test_weighted!(digraph);
    }
}
