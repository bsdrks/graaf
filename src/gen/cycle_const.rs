#![doc(alias = "circular")]
//! A trait to generate constant-sized directed cycle digraphs
//!
//! Cycle graphs are also known as circular graphs. To generate variable-sized
//! cycle digraphs, see [`Cycle`].
//!
//! # Examples
//!
//! ```
//! use graaf::gen::CycleConst;
//!
//! assert_eq!(<[Vec::<usize>; 1]>::cycle(), [vec![0]]);
//! assert_eq!(<[Vec::<usize>; 2]>::cycle(), [vec![1], vec![0]]);
//! assert_eq!(<[Vec::<usize>; 3]>::cycle(), [vec![1], vec![2], vec![0]]);
//! ```
//!
//! [`Cycle`]: crate::gen::Cycle

extern crate alloc;

use {
    crate::gen::EmptyConst,
    alloc::collections::BTreeSet,
    core::{
        array::from_fn,
        hash::BuildHasher,
    },
    std::collections::HashSet,
};

/// A trait to generate constant-sized directed cycle digraphs
///
/// # How can I implement `CycleConst`?
///
/// Provide an implementation of `cycle` that generates a cycle digraph with `V`
/// vertices.
///
/// ```
/// use {
///     core::{
///         array::from_fn,
///         hash::BuildHasher,
///     },
///     graaf::gen::CycleConst,
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
/// impl<const V: usize, H> CycleConst for Digraph<V, H>
/// where
///     H: BuildHasher + Default,
/// {
///     /// # Panics
///     ///
///     /// Panics if `V` is zero.
///     fn cycle() -> Self {
///         assert!(V > 0, "a graph must have at least one vertex");
///
///         let mut arcs = from_fn(|_| HashSet::with_hasher(H::default()));
///
///         for (s, set) in arcs.iter_mut().enumerate().take(V - 1) {
///             let _ = set.insert(s + 1);
///         }
///
///         let _ = arcs[V - 1].insert(0);
///
///         Digraph { arcs }
///     }
/// }
///
/// let digraph = Digraph::<3, RandomState>::cycle();
///
/// assert_eq!(
///     digraph.arcs,
///     [HashSet::from([1]), HashSet::from([2]), HashSet::from([0])]
/// );
/// ```
pub trait CycleConst {
    /// Generates a cycle digraph.
    #[must_use]
    fn cycle() -> Self;
}

impl<const V: usize> CycleConst for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        let mut digraph = Self::empty();

        for (s, vec) in digraph.iter_mut().enumerate().take(V - 1) {
            vec.push(s + 1);
        }

        digraph[V - 1].push(0);

        digraph
    }
}

impl<const V: usize> CycleConst for [BTreeSet<usize>; V] {
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        let mut digraph = Self::empty();

        for (s, set) in digraph.iter_mut().enumerate().take(V - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = digraph[V - 1].insert(0);

        digraph
    }
}

impl<const V: usize, H> CycleConst for [HashSet<usize, H>; V]
where
    H: BuildHasher + Default,
{
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        let mut digraph = Self::empty();

        for (s, set) in digraph.iter_mut().enumerate().take(V - 1) {
            let _ = set.insert(s + 1);
        }

        let _ = digraph[V - 1].insert(0);

        digraph
    }
}

impl<const V: usize> CycleConst for [(usize, usize); V] {
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle() -> Self {
        assert!(V > 0, "a graph must have at least one vertex");

        from_fn(|i| (i, (i + 1) % V))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            op::IsSimple,
            prop::sum_indegrees_eq_sum_outdegrees,
        },
    };

    #[test]
    fn arr_vec() {
        assert_eq!(<[Vec::<usize>; 1]>::cycle(), [vec![0]]);
        assert_eq!(<[Vec::<usize>; 2]>::cycle(), [vec![1], vec![0]]);
        assert_eq!(<[Vec::<usize>; 3]>::cycle(), [vec![1], vec![2], vec![0]]);
    }

    #[test]
    fn arr_btree_set() {
        assert_eq!(<[BTreeSet::<usize>; 1]>::cycle(), [BTreeSet::from([0])]);

        assert_eq!(
            <[BTreeSet::<usize>; 2]>::cycle(),
            [BTreeSet::from([1]), BTreeSet::from([0])]
        );

        assert_eq!(
            <[BTreeSet::<usize>; 3]>::cycle(),
            [
                BTreeSet::from([1]),
                BTreeSet::from([2]),
                BTreeSet::from([0])
            ]
        );
    }

    #[test]
    fn arr_hash_set() {
        assert_eq!(<[HashSet::<usize>; 1]>::cycle(), [HashSet::from([0])]);

        assert_eq!(
            <[HashSet::<usize>; 2]>::cycle(),
            [HashSet::from([1]), HashSet::from([0])]
        );

        assert_eq!(
            <[HashSet::<usize>; 3]>::cycle(),
            [HashSet::from([1]), HashSet::from([2]), HashSet::from([0])]
        );
    }

    #[test]
    fn arr_tuple() {
        assert_eq!(<[(_, _); 1]>::cycle(), [(0, 0)]);
        assert_eq!(<[(_, _); 2]>::cycle(), [(0, 1), (1, 0)]);
        assert_eq!(<[(_, _); 3]>::cycle(), [(0, 1), (1, 2), (2, 0)]);
    }

    #[test]
    fn is_simple_arr_btree_set() {
        assert!(!<[BTreeSet<usize>; 1]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 2]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 3]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 4]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 5]>::cycle().is_simple());
        assert!(<[BTreeSet<usize>; 6]>::cycle().is_simple());
    }

    #[test]
    fn is_simple_arr_hash_set() {
        assert!(!<[HashSet<usize>; 1]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 2]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 3]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 4]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 5]>::cycle().is_simple());
        assert!(<[HashSet<usize>; 6]>::cycle().is_simple());
    }

    #[test]
    fn is_simple_arr_tuple() {
        assert!(!<[(_, _); 1]>::cycle().is_simple());
        assert!(<[(_, _); 2]>::cycle().is_simple());
        assert!(<[(_, _); 3]>::cycle().is_simple());
        assert!(<[(_, _); 4]>::cycle().is_simple());
        assert!(<[(_, _); 5]>::cycle().is_simple());
        assert!(<[(_, _); 6]>::cycle().is_simple());
    }

    #[test]
    fn sum_indegrees_eq_sum_outdegrees_arr_btree_set() {
        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[BTreeSet<usize>; 1]>::cycle()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[BTreeSet<usize>; 2]>::cycle()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[BTreeSet<usize>; 3]>::cycle()
        ));
    }

    #[test]
    fn sum_indegrees_eq_sum_outdegrees_arr_hash_set() {
        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[HashSet<usize>; 1]>::cycle()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[HashSet<usize>; 2]>::cycle()
        ));

        assert!(sum_indegrees_eq_sum_outdegrees(
            &<[HashSet<usize>; 3]>::cycle()
        ));
    }
    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_vec_panic() {
        let _ = <[Vec<usize>; 0]>::cycle();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_btree_set_panic() {
        let _ = <[BTreeSet<usize>; 0]>::cycle();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_hash_set_panic() {
        let _ = <[HashSet<usize>; 0]>::cycle();
    }

    #[test]
    #[should_panic(expected = "a graph must have at least one vertex")]
    fn arr_tuple_panic() {
        let _ = <[(usize, usize); 0]>::cycle();
    }
}
