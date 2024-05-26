//! Generate random variable-sized tournaments
//!
//! A tournament is a digraph in which for every pair of distinct
//! vertices `s` and `t`, exactly one of the arcs `(s, t)` and `(t, s)` is
//! present. To generate constant-sized tournaments, see
//! [`RandomTournamentConst`].
//!
//! # Examples
//!
//! ```
//! extern crate alloc;
//!
//! use {
//!     alloc::collections::BTreeSet,
//!     graaf::{
//!         gen::RandomTournament,
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
//! let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
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
//! [`RandomTournamentConst`]: crate::gen::RandomTournamentConst

extern crate alloc;

use {
    super::prng::Xoshiro256StarStar,
    crate::{
        gen::Empty,
        op::AddArc,
    },
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

/// Generate random variable-sized tournaments
///
/// # How can I implement `RandomTournament`?
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
///             Empty,
///             RandomTournament,
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
/// pub struct Tournament {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Empty for Tournament {
///     fn empty(v: usize) -> Self {
///         Tournament {
///             arcs: vec![BTreeSet::new(); v],
///         }
///     }
/// }
///
/// impl RandomTournament for Tournament {
///     fn random_tournament(v: usize) -> Self {
///         let mut rng = Xoshiro256StarStar::new(v as u64);
///         let mut tournament = Self::empty(v);
///
///         for s in 0..v {
///             for t in (s + 1)..v {
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
/// let tournament = Tournament::random_tournament(4);
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
///         gen::RandomTournament,
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
/// let tournament = Vec::<BTreeSet<usize>>::random_tournament(4);
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
pub trait RandomTournament {
    /// Returns a random tournament.
    #[must_use]
    fn random_tournament(v: usize) -> Self;
}

macro_rules! impl_random_tournament {
    () => {
        fn random_tournament(v: usize) -> Self {
            let mut rng = Xoshiro256StarStar::new(v as u64);
            let mut arcs = Self::empty(v);

            for s in 0..v {
                for t in (s + 1)..v {
                    if rng.next_bool() {
                        arcs.add_arc(s, t);
                    } else {
                        arcs.add_arc(t, s);
                    }
                }
            }

            arcs
        }
    };
}

impl RandomTournament for Vec<Vec<usize>> {
    impl_random_tournament!();
}

impl RandomTournament for Vec<BTreeSet<usize>> {
    impl_random_tournament!();
}

impl<H> RandomTournament for Vec<HashSet<usize, H>>
where
    H: BuildHasher + Default,
    HashSet<usize, H>: Clone,
{
    impl_random_tournament!();
}

impl RandomTournament for BTreeMap<usize, Vec<usize>> {
    impl_random_tournament!();
}

impl RandomTournament for BTreeMap<usize, BTreeSet<usize>> {
    impl_random_tournament!();
}

impl<H> RandomTournament for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher + Default,
{
    impl_random_tournament!();
}

impl<H> RandomTournament for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher + Default,
{
    impl_random_tournament!();
}

#[cfg(test)]
mod tests {
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
        proptest::proptest,
    };

    fn prop_degree<T>(v: usize)
    where
        T: Degree + IterVertices + RandomTournament,
    {
        let digraph = T::random_tournament(v);
        let degree = v - 1;

        for s in digraph.iter_vertices() {
            assert_eq!(digraph.degree(s), degree);
        }
    }

    fn prop_indegree<T>(v: usize)
    where
        T: Indegree + IterVertices + RandomTournament,
    {
        let digraph = T::random_tournament(v);

        for s in digraph.iter_vertices() {
            assert!((0..v).contains(&digraph.indegree(s)));
        }
    }

    fn prop_order<T>(v: usize)
    where
        T: Order + RandomTournament,
    {
        let digraph = T::random_tournament(v);

        assert_eq!(digraph.order(), v);
    }

    fn prop_outdegree<T>(v: usize)
    where
        T: Outdegree + IterVertices + RandomTournament,
    {
        let digraph = T::random_tournament(v);

        for s in digraph.iter_vertices() {
            assert!((0..v).contains(&digraph.outdegree(s)));
        }
    }

    fn prop_size<T>(v: usize)
    where
        T: RandomTournament + Size,
    {
        let digraph = T::random_tournament(v);

        assert_eq!(digraph.size(), (0..v).sum::<usize>());
    }

    proptest! {
        #[test]
        fn degree_vec_btree_set(v in 1..100_usize) {
            prop_degree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn degree_vec_hash_set(v in 1..100_usize) {
            prop_degree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn degree_btree_map_btree_set(v in 1..100_usize) {
            prop_degree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn degree_hash_map_hash_set(v in 1..100_usize) {
            prop_degree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_btree_set(v in 1..100_usize) {
            prop_indegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_vec_hash_set(v in 1..100_usize) {
            prop_indegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn indegree_btree_map_btree_set(v in 1..100_usize) {
            prop_indegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn indegree_hash_map_hash_set(v in 1..100_usize) {
            prop_indegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn order_vec_vec(v in 1..100_usize) {
            prop_order::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn order_vec_btree_set(v in 1..100_usize) {
            prop_order::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn order_vec_hash_set(v in 1..100_usize) {
            prop_order::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_vec(v in 1..100_usize) {
            prop_outdegree::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_btree_set(v in 1..100_usize) {
            prop_outdegree::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_vec_hash_set(v in 1..100_usize) {
            prop_outdegree::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_vec(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_btree_map_btree_set(v in 1..100_usize) {
            prop_outdegree::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_vec(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn outdegree_hash_map_hash_set(v in 1..100_usize) {
            prop_outdegree::<HashMap<usize, HashSet<usize>>>(v);
        }

        #[test]
        fn size_vec_vec(v in 1..100_usize) {
            prop_size::<Vec<Vec<usize>>>(v);
        }

        #[test]
        fn size_vec_btree_set(v in 1..100_usize) {
            prop_size::<Vec<BTreeSet<usize>>>(v);
        }

        #[test]
        fn size_vec_hash_set(v in 1..100_usize) {
            prop_size::<Vec<HashSet<usize>>>(v);
        }

        #[test]
        fn size_btree_map_vec(v in 1..100_usize) {
            prop_size::<BTreeMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn size_btree_map_btree_set(v in 1..100_usize) {
            prop_size::<BTreeMap<usize, BTreeSet<usize>>>(v);
        }

        #[test]
        fn size_hash_map_vec(v in 1..100_usize) {
            prop_size::<HashMap<usize, Vec<usize>>>(v);
        }

        #[test]
        fn size_hash_map_hash_set(v in 1..100_usize) {
            prop_size::<HashMap<usize, HashSet<usize>>>(v);
        }
    }
}
