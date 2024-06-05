//! Generate constant-sized symmetric complete digraphs.
//!
//! The generated digraphs are simple; they contain no self-loops. To generate
//! variable-sized complete digraphs, see [`Complete`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::CompleteConst;
//!
//! // 0 -> {}
//!
//! assert_eq!(<[Vec::<usize>; 1]>::complete(), [Vec::new()]);
//!
//! // 0 -> {1}
//! // 1 -> {0}
//!
//! assert_eq!(<[Vec::<usize>; 2]>::complete(), [vec![1], vec![0]]);
//!
//! // 0 -> {1, 2}
//! // 1 -> {0, 2}
//! // 2 -> {0, 1}
//!
//! assert_eq!(
//!     <[Vec::<usize>; 3]>::complete(),
//!     [vec![1, 2], vec![0, 2], vec![0, 1]]
//! );
//! ```
//!
//! [`Complete`]: crate::gen::Complete

use {
    super::EmptyConst,
    crate::op::AddArc,
};

/// Generate constant-sized symmetric complete digraphs.
///
/// # How can I implement `CompleteConst`?
///
/// Provide an implementation of `complete` that generates a symmetric complete
/// digraph with `V` vertices.
///
/// ```
/// use {
///     core::hash::BuildHasher,
///     graaf::gen::{
///         CompleteConst,
///         EmptyConst,
///     },
///     std::collections::{
///         hash_map::RandomState,
///         HashSet,
///     },
/// };
///
/// struct Digraph<const V: usize, H>
/// where
///     H: BuildHasher,
/// {
///     arcs: [HashSet<usize, H>; V],
/// }
///
/// impl<const V: usize, H> CompleteConst for Digraph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     /// # Panics
///     ///
///     /// Panics if `V` is zero.
///     fn complete() -> Self {
///         assert!(V > 0, "a graph must have at least one vertex");
///
///         let mut arcs = <[HashSet<usize, H>; V]>::empty();
///
///         for (s, set) in arcs.iter_mut().enumerate() {
///             for t in 0..V {
///                 if s != t {
///                     let _ = set.insert(t);
///                 }
///             }
///         }
///
///         Digraph { arcs }
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0, 1}
///
/// let digraph = Digraph::<3, RandomState>::complete();
///
/// assert_eq!(
///     digraph.arcs,
///     [
///         HashSet::from([1, 2]),
///         HashSet::from([0, 2]),
///         HashSet::from([0, 1])
///     ]
/// );
/// ```
///
/// # Examples
///
/// ```
/// use graaf::gen::CompleteConst;
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0, 1}
///
/// assert_eq!(
///     <[Vec<usize>; 3]>::complete(),
///     [vec![1, 2], vec![0, 2], vec![0, 1]]
/// );
/// ```
pub trait CompleteConst {
    /// Generates a complete digraph.
    fn complete() -> Self;
}

impl<const V: usize, D> CompleteConst for [D; V]
where
    [D; V]: AddArc + EmptyConst,
{
    /// # Panics
    ///
    /// Panics if `v` is 0.
    fn complete() -> Self {
        let mut digraph = <[D; V]>::empty();

        for s in 0..V {
            for t in (s + 1)..V {
                digraph.add_arc(s, t);
                digraph.add_arc(t, s);
            }
        }

        digraph
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::{
            op::{
                Indegree,
                IsSimple,
                Order,
                Outdegree,
                Size,
            },
            prop::sum_indegrees_eq_sum_outdegrees,
        },
        alloc::collections::BTreeSet,
        std::collections::HashSet,
    };

    #[test]
    fn arr_vec() {
        assert_eq!(<[Vec::<usize>; 1]>::complete(), [Vec::new()]);
        assert_eq!(<[Vec::<usize>; 2]>::complete(), [vec![1], vec![0]]);

        assert_eq!(
            <[Vec::<usize>; 3]>::complete(),
            [vec![1, 2], vec![0, 2], vec![0, 1]]
        );
    }

    #[test]
    fn arr_btree_set() {
        assert_eq!(<[BTreeSet::<usize>; 1]>::complete(), [BTreeSet::new()]);

        assert_eq!(
            <[BTreeSet::<usize>; 2]>::complete(),
            [BTreeSet::from([1]), BTreeSet::from([0])]
        );

        assert_eq!(
            <[BTreeSet::<usize>; 3]>::complete(),
            [
                BTreeSet::from([1, 2]),
                BTreeSet::from([0, 2]),
                BTreeSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        assert_eq!(<[HashSet::<usize>; 1]>::complete(), [HashSet::new()]);

        assert_eq!(
            <[HashSet::<usize>; 2]>::complete(),
            [HashSet::from([1]), HashSet::from([0])]
        );

        assert_eq!(
            <[HashSet::<usize>; 3]>::complete(),
            [
                HashSet::from([1, 2]),
                HashSet::from([0, 2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn size_arr_vec() {
        assert_eq!(<[Vec<usize>; 1]>::complete().size(), 0);
        assert_eq!(<[Vec<usize>; 2]>::complete().size(), 2);
        assert_eq!(<[Vec<usize>; 3]>::complete().size(), 6);
        assert_eq!(<[Vec<usize>; 4]>::complete().size(), 12);
        assert_eq!(<[Vec<usize>; 5]>::complete().size(), 20);
    }

    #[test]
    fn size_arr_btree_set() {
        assert_eq!(<[BTreeSet<usize>; 1]>::complete().size(), 0);
        assert_eq!(<[BTreeSet<usize>; 2]>::complete().size(), 2);
        assert_eq!(<[BTreeSet<usize>; 3]>::complete().size(), 6);
        assert_eq!(<[BTreeSet<usize>; 4]>::complete().size(), 12);
        assert_eq!(<[BTreeSet<usize>; 5]>::complete().size(), 20);
    }

    #[test]
    fn size_arr_hash_set() {
        assert_eq!(<[HashSet<usize>; 1]>::complete().size(), 0);
        assert_eq!(<[HashSet<usize>; 2]>::complete().size(), 2);
        assert_eq!(<[HashSet<usize>; 3]>::complete().size(), 6);
        assert_eq!(<[HashSet<usize>; 4]>::complete().size(), 12);
        assert_eq!(<[HashSet<usize>; 5]>::complete().size(), 20);
    }

    #[test]
    fn order_arr_vec() {
        assert_eq!(<[Vec<usize>; 1]>::complete().order(), 1);
        assert_eq!(<[Vec<usize>; 2]>::complete().order(), 2);
        assert_eq!(<[Vec<usize>; 3]>::complete().order(), 3);
        assert_eq!(<[Vec<usize>; 4]>::complete().order(), 4);
        assert_eq!(<[Vec<usize>; 5]>::complete().order(), 5);
    }

    #[test]
    fn order_arr_btree_set() {
        assert_eq!(<[BTreeSet<usize>; 1]>::complete().order(), 1);
        assert_eq!(<[BTreeSet<usize>; 2]>::complete().order(), 2);
        assert_eq!(<[BTreeSet<usize>; 3]>::complete().order(), 3);
        assert_eq!(<[BTreeSet<usize>; 4]>::complete().order(), 4);
        assert_eq!(<[BTreeSet<usize>; 5]>::complete().order(), 5);
    }

    #[test]
    fn order_arr_hash_set() {
        assert_eq!(<[HashSet<usize>; 1]>::complete().order(), 1);
        assert_eq!(<[HashSet<usize>; 2]>::complete().order(), 2);
        assert_eq!(<[HashSet<usize>; 3]>::complete().order(), 3);
        assert_eq!(<[HashSet<usize>; 4]>::complete().order(), 4);
        assert_eq!(<[HashSet<usize>; 5]>::complete().order(), 5);
    }

    #[test]
    fn indegree_arr_btree_set() {
        let digraph = <[BTreeSet<usize>; 1]>::complete();

        assert_eq!(digraph.indegree(0), 0);

        let digraph = <[BTreeSet<usize>; 2]>::complete();

        assert_eq!(digraph.indegree(0), 1);
        assert_eq!(digraph.indegree(1), 1);

        let digraph = <[BTreeSet<usize>; 3]>::complete();

        assert_eq!(digraph.indegree(0), 2);
        assert_eq!(digraph.indegree(1), 2);
        assert_eq!(digraph.indegree(2), 2);
    }

    #[test]
    fn indegree_arr_hash_set() {
        let digraph = <[HashSet<usize>; 1]>::complete();

        assert_eq!(digraph.indegree(0), 0);

        let digraph = <[HashSet<usize>; 2]>::complete();

        assert_eq!(digraph.indegree(0), 1);
        assert_eq!(digraph.indegree(1), 1);

        let digraph = <[HashSet<usize>; 3]>::complete();

        assert_eq!(digraph.indegree(0), 2);
        assert_eq!(digraph.indegree(1), 2);
        assert_eq!(digraph.indegree(2), 2);
    }

    #[test]
    fn is_simple_arr_btree_set() {
        assert!(<[BTreeSet<usize>; 1]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 2]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 3]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 4]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 5]>::complete().is_simple());
        assert!(<[BTreeSet<usize>; 6]>::complete().is_simple());
    }

    #[test]
    fn is_simple_arr_hash_set() {
        assert!(<[HashSet<usize>; 1]>::complete().is_simple());
        assert!(<[HashSet<usize>; 2]>::complete().is_simple());
        assert!(<[HashSet<usize>; 3]>::complete().is_simple());
        assert!(<[HashSet<usize>; 4]>::complete().is_simple());
        assert!(<[HashSet<usize>; 5]>::complete().is_simple());
        assert!(<[HashSet<usize>; 6]>::complete().is_simple());
    }

    #[test]
    fn outdegree_arr_vec() {
        let digraph = <[Vec<usize>; 1]>::complete();

        assert_eq!(digraph.outdegree(0), 0);

        let digraph = <[Vec<usize>; 2]>::complete();

        assert_eq!(digraph.outdegree(0), 1);
        assert_eq!(digraph.outdegree(1), 1);

        let digraph = <[Vec<usize>; 3]>::complete();

        assert_eq!(digraph.outdegree(0), 2);
        assert_eq!(digraph.outdegree(1), 2);
        assert_eq!(digraph.outdegree(2), 2);
    }

    #[test]
    fn outdegree_arr_btree_set() {
        let digraph = <[BTreeSet<usize>; 1]>::complete();

        assert_eq!(digraph.outdegree(0), 0);

        let digraph = <[BTreeSet<usize>; 2]>::complete();

        assert_eq!(digraph.outdegree(0), 1);
        assert_eq!(digraph.outdegree(1), 1);

        let digraph = <[BTreeSet<usize>; 3]>::complete();

        assert_eq!(digraph.outdegree(0), 2);
        assert_eq!(digraph.outdegree(1), 2);
        assert_eq!(digraph.outdegree(2), 2);
    }

    #[test]
    fn outdegree_arr_hash_set() {
        let digraph = <[HashSet<usize>; 1]>::complete();

        assert_eq!(digraph.outdegree(0), 0);

        let digraph = <[HashSet<usize>; 2]>::complete();

        assert_eq!(digraph.outdegree(0), 1);
        assert_eq!(digraph.outdegree(1), 1);

        let digraph = <[HashSet<usize>; 3]>::complete();

        assert_eq!(digraph.outdegree(0), 2);
        assert_eq!(digraph.outdegree(1), 2);
        assert_eq!(digraph.outdegree(2), 2);
    }

    #[test]
    fn sum_indegrees_eq_sum_outdegrees_arr_btree_set() {
        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[BTreeSet<usize>; 1]>::complete()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[BTreeSet<usize>; 2]>::complete()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[BTreeSet<usize>; 3]>::complete()
        ));
    }

    #[test]
    fn sum_indegrees_eq_sum_outdegrees_arr_hash_set() {
        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[HashSet<usize>; 1]>::complete()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[HashSet<usize>; 2]>::complete()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[HashSet<usize>; 3]>::complete()
        ));
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_vec_panic() {
        let _ = <[Vec<usize>; 0]>::complete();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_btree_set_panic() {
        let _ = <[BTreeSet<usize>; 0]>::complete();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_hash_set_panic() {
        let _ = <[HashSet<usize>; 0]>::complete();
    }
}
