//! Generate random constant-sized tournaments.
//!
//! A tournament is a digraph in which, for every pair of distinct
//! vertices `s` and `t`, exactly one of the arcs `(s, t)` and `(t, s)` is
//! present. To generate variable-sized tournaments, see [`RandomTournament`].
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::RandomTournamentConst,
//!         op::{
//!             Degree,
//!             Indegree,
//!             IterVertices,
//!             Order,
//!             Outdegree,
//!             Size,
//!         },
//!     },
//! };
//!
//! let tournament = <[BTreeSet<usize>; 4]>::random_tournament();
//!
//! assert_eq!(tournament.size(), 6);
//! assert_eq!(tournament.order(), 4);
//!
//! for s in tournament.iter_vertices() {
//!     assert_eq!(tournament.degree(s), 3);
//!     assert!((0..3).contains(&tournament.outdegree(s)));
//!     assert!((0..3).contains(&tournament.indegree(s)));
//! }
//! ```
//!
//! [`RandomTournament`]: crate::gen::RandomTournament

use {
    super::prng::Xoshiro256StarStar,
    crate::{
        gen::EmptyConst,
        op::AddArc,
    },
};

/// Generate random constant-sized tournaments.
///
/// # How can I implement `RandomTournamentConst`?
///
/// Provide an implementation of `random_tournament` that returns a random
/// tournament.
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::{
///             prng::Xoshiro256StarStar,
///             EmptyConst,
///             RandomTournamentConst,
///         },
///         op::{
///             AddArc,
///             Degree,
///             Indegree,
///             IterVertices,
///             Order,
///             Outdegree,
///             Size,
///         },
///     },
/// };
///
/// pub struct Tournament<const V: usize> {
///     pub arcs: [BTreeSet<usize>; V],
/// }
///
/// impl<const V: usize> EmptyConst for Tournament<V> {
///     fn empty() -> Self {
///         Tournament {
///             arcs: <[BTreeSet<usize>; V]>::empty(),
///         }
///     }
/// }
///
/// impl<const V: usize> RandomTournamentConst for Tournament<V> {
///     fn random_tournament() -> Self {
///         let mut rng = Xoshiro256StarStar::new(V as u64);
///         let mut tournament = Self::empty();
///
///         for s in 0..V {
///             for t in (s + 1)..V {
///                 if rng.next_bool() {
///                     tournament.arcs.add_arc(s, t);
///                 } else {
///                     tournament.arcs.add_arc(t, s);
///                 }
///             }
///         }
///
///         tournament
///     }
/// }
///
/// let tournament = Tournament::<4>::random_tournament();
///
/// assert_eq!(tournament.arcs.size(), 6);
/// assert_eq!(tournament.arcs.order(), 4);
///
/// for s in tournament.arcs.iter_vertices() {
///     assert_eq!(tournament.arcs.degree(s), 3);
///     assert!((0..3).contains(&tournament.arcs.outdegree(s)));
///     assert!((0..3).contains(&tournament.arcs.indegree(s)));
/// }
/// ```
///
/// # Examples
///
/// ```
/// extern crate alloc;
///
/// use {
///     alloc::collections::BTreeSet,
///     graaf::{
///         gen::RandomTournamentConst,
///         op::{
///             Degree,
///             Indegree,
///             IterVertices,
///             Order,
///             Outdegree,
///             Size,
///         },
///     },
/// };
///
/// let tournament = <[BTreeSet<usize>; 4]>::random_tournament();
///
/// assert_eq!(tournament.size(), 6);
/// assert_eq!(tournament.order(), 4);
///
/// for s in tournament.iter_vertices() {
///     assert_eq!(tournament.degree(s), 3);
///     assert!((0..3).contains(&tournament.outdegree(s)));
///     assert!((0..3).contains(&tournament.indegree(s)));
/// }
/// ```
pub trait RandomTournamentConst {
    /// Returns a random tournament.
    #[must_use]
    fn random_tournament() -> Self;
}

impl<const V: usize, D> RandomTournamentConst for [D; V]
where
    [D; V]: AddArc + EmptyConst,
{
    fn random_tournament() -> Self {
        let mut rng = Xoshiro256StarStar::new(V as u64);
        let mut arcs = Self::empty();

        for s in 0..V {
            for t in (s + 1)..V {
                if rng.next_bool() {
                    arcs.add_arc(s, t);
                } else {
                    arcs.add_arc(t, s);
                }
            }
        }

        arcs
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use {
        super::*,
        crate::op::{
            Degree,
            Indegree,
            IterVertices,
            Order,
            Outdegree,
            Size,
        },
        alloc::collections::BTreeSet,
        std::collections::HashSet,
    };

    macro_rules! prop_arr_vec {
        ($V:expr) => {
            prop_order::<$V, [Vec<usize>; $V]>();
            prop_outdegree::<$V, [Vec<usize>; $V]>();
            prop_size::<$V, [Vec<usize>; $V]>();
        };
    }

    macro_rules! prop_arr_btree_set {
        ($V:expr) => {
            prop_degree::<$V, [BTreeSet<usize>; $V]>();
            prop_indegree::<$V, [BTreeSet<usize>; $V]>();
            prop_order::<$V, [BTreeSet<usize>; $V]>();
            prop_outdegree::<$V, [BTreeSet<usize>; $V]>();
            prop_size::<$V, [BTreeSet<usize>; $V]>();
        };
    }

    macro_rules! prop_arr_hash_set {
        ($V:expr) => {
            prop_degree::<$V, [HashSet<usize>; $V]>();
            prop_indegree::<$V, [HashSet<usize>; $V]>();
            prop_order::<$V, [HashSet<usize>; $V]>();
            prop_outdegree::<$V, [HashSet<usize>; $V]>();
            prop_size::<$V, [HashSet<usize>; $V]>();
        };
    }

    fn prop_degree<const V: usize, T>()
    where
        T: Degree + IterVertices + RandomTournamentConst,
    {
        let digraph = T::random_tournament();
        let degree = V - 1;

        for s in digraph.iter_vertices() {
            assert_eq!(digraph.degree(s), degree);
        }
    }

    fn prop_indegree<const V: usize, T>()
    where
        T: Indegree + IterVertices + RandomTournamentConst,
    {
        let digraph = T::random_tournament();

        for s in digraph.iter_vertices() {
            assert!((0..V).contains(&digraph.indegree(s)));
        }
    }

    fn prop_order<const V: usize, T>()
    where
        T: Order + RandomTournamentConst,
    {
        let digraph = T::random_tournament();

        assert_eq!(digraph.order(), V);
    }

    fn prop_outdegree<const V: usize, T>()
    where
        T: Outdegree + IterVertices + RandomTournamentConst,
    {
        let digraph = T::random_tournament();

        for s in digraph.iter_vertices() {
            assert!((0..V).contains(&digraph.outdegree(s)));
        }
    }

    fn prop_size<const V: usize, T>()
    where
        T: RandomTournamentConst + Size,
    {
        let digraph = T::random_tournament();

        assert_eq!(digraph.size(), (0..V).sum::<usize>());
    }

    #[test]
    fn arr_vec() {
        prop_arr_vec!(2);
        prop_arr_vec!(3);
        prop_arr_vec!(4);
        prop_arr_vec!(5);
        prop_arr_vec!(6);
        prop_arr_vec!(7);
    }

    #[test]
    fn arr_btree_set() {
        prop_arr_btree_set!(2);
        prop_arr_btree_set!(3);
        prop_arr_btree_set!(4);
        prop_arr_btree_set!(5);
        prop_arr_btree_set!(6);
        prop_arr_btree_set!(7);
    }

    #[test]
    fn arr_hash_set() {
        prop_arr_hash_set!(2);
        prop_arr_hash_set!(3);
        prop_arr_hash_set!(4);
        prop_arr_hash_set!(5);
        prop_arr_hash_set!(6);
        prop_arr_hash_set!(7);
    }
}
