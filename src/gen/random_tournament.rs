//! Generate random tournaments.
//!
//! A tournament is a digraph in which an arc connects every unordered pair of
//! distinct vertices.
//!
//! # Examples
//!
//! Generate a random tournament of order `6`.
//!
//! ![A random tournament of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_tournament_1-0.89.2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     RandomTournament,
//! };
//!
//! let tournament = AdjacencyList::random_tournament(6, 0);
//!
//! assert!(tournament.arcs().eq([
//!     (0, 5),
//!     (1, 0),
//!     (1, 4),
//!     (1, 5),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (2, 5),
//!     (3, 0),
//!     (3, 1),
//!     (3, 5),
//!     (4, 0),
//!     (4, 2),
//!     (4, 3),
//!     (5, 4)
//! ]));
//! ```

/// Random tournaments
pub trait RandomTournament {
    /// Generate a random tournament.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the tournament.
    /// * `seed` - The seed for the random number generator.
    ///
    /// # Examples
    ///
    /// Generate a random tournament of order `6`.
    ///
    /// ![A random tournament of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_tournament_1-0.89.2.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     RandomTournament,
    /// };
    ///
    /// let tournament = AdjacencyList::random_tournament(6, 0);
    ///
    /// assert!(tournament.arcs().eq([
    ///     (0, 5),
    ///     (1, 0),
    ///     (1, 4),
    ///     (1, 5),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (2, 5),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 5),
    ///     (4, 0),
    ///     (4, 2),
    ///     (4, 3),
    ///     (5, 4)
    /// ]));
    /// ```
    #[must_use]
    fn random_tournament(order: usize, seed: u64) -> Self;
}

/// `RandomTournament` tests
#[macro_export]
macro_rules! test_random_tournament {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn random_tournament_0() {
            let _ = <$type>::random_tournament(0, 0);
        }
    };
}

/// `RandomTournament` proptests
#[macro_export]
macro_rules! proptest_random_tournament {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::{
                Degree,
                IsIsolated,
                IsOriented,
                IsPendant,
                IsSpanningSubdigraph,
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
                OutdegreeSequence,
            },
        };

        proptest! {
            #[test]
            fn random_tournament_complement_size(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    <$type>::random_tournament(order, seed).complement().size(),
                    order * (order - 1) / 2
                );
            }

            #[test]
            fn random_tournament_degree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);
                let degree = order - 1;

                assert!(digraph
                    .vertices()
                    .all(|u| digraph.degree(u) == degree));
            }

            #[test]
            fn random_tournament_degree_sequence(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let degree = order - 1;

                assert!(<$type>::random_tournament(order, seed)
                    .degree_sequence()
                    .all(|d| d == degree));
            }

            #[test]
            fn random_tournament_degree_sum_equals_2size(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn random_tournament_even_number_odd_degrees(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert_eq!(
                    digraph
                        .vertices()
                        .filter(|&u| digraph.degree(u) % 2 == 1)
                        .count()
                        % 2,
                    0
                );
            }

            #[test]
            fn random_tournament_has_arc(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| {
                        u == v || digraph.has_arc(u, v) ^ digraph.has_arc(v, u)
                    })
                }));
            }

            #[test]
            fn random_tournament_has_edge(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_edge(u, v))
                }));
            }

            #[test]
            fn random_tournament_indegree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (0..order).contains(&digraph.indegree(u))));
            }

            #[test]
            fn random_tournament_indegree_sequence(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert!(indegree_sequence.all(|d| (0..order).contains(&d)));
            }

            #[test]
            fn random_tournament_is_complete(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert!(
                    (order == 1)
                        == <$type>::random_tournament(order, seed).is_complete()
                );
            }

            #[test]
            fn random_tournament_is_isolated(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn random_tournament_is_oriented(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert!(<$type>::random_tournament(order, seed).is_oriented());
            }

            #[test]
            fn random_tournament_is_pendant(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 2) == digraph.is_pendant(u)));
            }

            #[test]
            fn random_tournament_is_semicomplete(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert!(
                    <$type>::random_tournament(order, seed).is_semicomplete()
                );
            }

            #[test]
            fn random_tournament_is_simple(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert!(<$type>::random_tournament(order, seed).is_simple());
            }

            #[test]
            fn random_tournament_is_spanning_subdigraph(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn random_tournament_is_subdigraph(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn random_tournament_is_superdigraph(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn random_tournament_is_symmetric(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert!(
                    (order == 1)
                        == <$type>::random_tournament(order, seed).is_symmetric()
                );
            }

            #[test]
            fn random_tournament_is_tournament(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert!(<$type>::random_tournament(order, seed).is_tournament());
            }

            #[test]
            fn random_tournament_max_degree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    <$type>::random_tournament(order, seed).max_degree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn random_tournament_min_degree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    <$type>::random_tournament(order, seed).min_degree(),
                    if order == 1 { 0 } else { order - 1 }
                );
            }

            #[test]
            fn random_tournament_order(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    <$type>::random_tournament(order, seed).order(), order
                );
            }

            #[test]
            fn random_tournament_outdegree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);

                assert!(digraph
                    .vertices()
                    .all(|u| (0..order).contains(&digraph.outdegree(u))));
            }

            #[test]
            fn random_tournament_outdegree_sequence(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_tournament(order, seed);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.all(|d| (0..order).contains(&d)));
            }

            #[test]
            fn random_tournament_semidegree_sequence(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    <$type>::random_tournament(order, seed)
                        .semidegree_sequence()
                        .fold(0, |acc, (indegree, outdegree)| acc
                            + indegree
                            + outdegree),
                    order * (order - 1)
                );
            }

            #[test]
            fn random_tournament_size(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                assert_eq!(
                    <$type>::random_tournament(order, seed).size(),
                    order * (order - 1) / 2
                );
            }
        }
    }
}
