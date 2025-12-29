//! Generate biclique digraphs.
//!
//! Bicliques are also known as complete bipartite digraphs.
//!
//! # Examples
//!
//! ## m = 2, n = 3
//!
//! Generate a biclique digraph with `m = 2` and `n = 3`.
//!
//! ![A biclique digraph with m = 2 and n = 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_2_3.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Biclique,
//! };
//!
//! assert!(AdjacencyList::biclique(2, 3).arcs().eq([
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (1, 2),
//!     (1, 3),
//!     (1, 4),
//!     (2, 0),
//!     (2, 1),
//!     (3, 0),
//!     (3, 1),
//!     (4, 0),
//!     (4, 1),
//! ]));
//! ```
//!
//! ## m = 4, n = 2
//!
//! Generate a biclique digraph with `m = 4` and `n = 2`.
//!
//! ![A biclique digraph with m = 4 and n = 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_4_2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Biclique,
//! };
//!
//! assert!(AdjacencyList::biclique(4, 2).arcs().eq([
//!     (0, 4),
//!     (0, 5),
//!     (1, 4),
//!     (1, 5),
//!     (2, 4),
//!     (2, 5),
//!     (3, 4),
//!     (3, 5),
//!     (4, 0),
//!     (4, 1),
//!     (4, 2),
//!     (4, 3),
//!     (5, 0),
//!     (5, 1),
//!     (5, 2),
//!     (5, 3),
//! ]));
//! ```
#![doc(alias = "complete_bipartite")]

/// Biclique digraphs
#[doc(alias = "CompleteBipartite")]
pub trait Biclique {
    /// Generate a biclique digraph.
    ///
    /// # Arguments
    ///
    /// * `m` - The number of vertices in the first partition.
    /// * `n` - The number of vertices in the second partition.
    ///
    /// # Examples
    ///
    /// ## m = 2, n = 3
    ///
    /// Generate a biclique digraph with `m = 2` and `n = 3`.
    ///
    /// ![A biclique digraph with m = 2 and n = 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_2_3.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Biclique,
    /// };
    ///
    /// assert!(AdjacencyList::biclique(2, 3).arcs().eq([
    ///     (0, 2),
    ///     (0, 3),
    ///     (0, 4),
    ///     (1, 2),
    ///     (1, 3),
    ///     (1, 4),
    ///     (2, 0),
    ///     (2, 1),
    ///     (3, 0),
    ///     (3, 1),
    ///     (4, 0),
    ///     (4, 1),
    /// ]));
    /// ```
    ///
    /// ## m = 4, n = 2
    ///
    /// Generate a biclique digraph with `m = 4` and `n = 2`.
    ///
    /// ![A biclique digraph with m = 4 and n = 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_4_2.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Biclique,
    /// };
    ///
    /// assert!(AdjacencyList::biclique(4, 2).arcs().eq([
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 4),
    ///     (1, 5),
    ///     (2, 4),
    ///     (2, 5),
    ///     (3, 4),
    ///     (3, 5),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 2),
    ///     (4, 3),
    ///     (5, 0),
    ///     (5, 1),
    ///     (5, 2),
    ///     (5, 3),
    /// ]));
    /// ```
    #[doc(alias = "complete_bipartite")]
    #[must_use]
    fn biclique(m: usize, n: usize) -> Self;

    /// Generate a claw digraph.
    ///
    /// The claw digraph is also known as K{1, 3}.
    ///
    /// ![The claw digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_claw.svg)
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Biclique,
    /// };
    ///
    /// assert!(AdjacencyList::claw().arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 0)
    /// ]));
    /// ```
    #[must_use]
    fn claw() -> Self
    where
        Self: Sized,
    {
        Self::biclique(1, 3)
    }

    /// Generate a utility digraph.
    ///
    /// The utility digraph is also known as K{3, 3}.
    ///
    /// ![The utility digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/biclique_utility.svg)
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Biclique,
    /// };
    ///
    /// assert!(AdjacencyList::utility().arcs().eq([
    ///     (0, 3),
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 3),
    ///     (1, 4),
    ///     (1, 5),
    ///     (2, 3),
    ///     (2, 4),
    ///     (2, 5),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 2),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 2),
    ///     (5, 0),
    ///     (5, 1),
    ///     (5, 2),
    /// ]));
    /// ```
    #[must_use]
    fn utility() -> Self
    where
        Self: Sized,
    {
        Self::biclique(3, 3)
    }
}

/// `Biclique` tests
#[macro_export]
macro_rules! test_biclique {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "m = 0 must be greater than zero")]
        fn biclique_0_1() {
            drop(<$type>::biclique(0, 1));
        }

        #[test]
        #[should_panic(expected = "n = 0 must be greater than zero")]
        fn biclique_1_0() {
            drop(<$type>::biclique(1, 0));
        }

        #[test]
        fn biclique_1_1() {
            let digraph = <$type>::biclique(1, 1);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn biclique_1_1_complement() {
            let digraph = <$type>::biclique(1, 1).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn biclique_1_2() {
            let digraph = <$type>::biclique(1, 2);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
        }

        #[test]
        fn biclique_1_2_complement() {
            let digraph = <$type>::biclique(1, 2).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(1, 2), (2, 1)]));
        }

        #[test]
        fn biclique_2_1() {
            let digraph = <$type>::biclique(2, 1);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 2), (1, 2), (2, 0), (2, 1)]));
        }

        #[test]
        fn biclique_2_1_complement() {
            let digraph = <$type>::biclique(2, 1).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn biclique_2_2() {
            let digraph = <$type>::biclique(2, 2);

            assert_eq!(digraph.order(), 4);

            assert!(digraph.arcs().eq([
                (0, 2),
                (0, 3),
                (1, 2),
                (1, 3),
                (2, 0),
                (2, 1),
                (3, 0),
                (3, 1)
            ]));
        }

        #[test]
        fn biclique_2_2_complement() {
            let digraph = <$type>::biclique(2, 2).complement();

            assert_eq!(digraph.order(), 4);
            assert!(digraph.arcs().eq([(0, 1), (1, 0), (2, 3), (3, 2)]));
        }

        #[test]
        fn biclique_claw() {
            let digraph = <$type>::claw();

            assert_eq!(digraph.order(), 4);

            assert!(digraph.arcs().eq([
                (0, 1),
                (0, 2),
                (0, 3),
                (1, 0),
                (2, 0),
                (3, 0)
            ]));
        }

        #[test]
        fn biclique_utility() {
            let digraph = <$type>::utility();

            assert_eq!(digraph.order(), 6);

            assert!(<$type>::utility().arcs().eq([
                (0, 3),
                (0, 4),
                (0, 5),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 0),
                (3, 1),
                (3, 2),
                (4, 0),
                (4, 1),
                (4, 2),
                (5, 0),
                (5, 1),
                (5, 2)
            ]));
        }
    };
}

/// `Biclique` proptests
#[macro_export]
macro_rules! proptest_biclique {
    ($type:ty) => {
        use {
            $crate::{
                Degree,
                IsBalanced,
                IsComplete,
                IsIsolated,
                IsOriented,
                IsPendant,
                IsSpanningSubdigraph,
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
                OutdegreeSequence,
                Sinks,
                Sources
            },
            proptest::proptest,
        };

        proptest! {
            #[test]
            fn biclique_1_n_equals_star_n_plus_1(n in 1..5_usize) {
                assert_eq!(<$type>::biclique(1, n), <$type>::star(n + 1));
            }

            #[test]
            fn biclique_complement_size(m in 1..5_usize, n in 1..5_usize) {
                assert_eq!(
                    <$type>::biclique(m, n).complement().size(),
                    m * (m - 1) + n * (n - 1)
                );
            }

            #[test]
            fn biclique_degree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let clique_1_degree = n * 2;
                let clique_2_degree = m * 2;

                assert!((0..m).all(|u| digraph.degree(u) == clique_1_degree));
                assert!((m..m + n).all(|u| digraph.degree(u) == clique_2_degree));
            }

            #[test]
            fn biclique_degree_sequence(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let degree_sequence = &mut digraph.degree_sequence();
                let clique_1_degree = n * 2;
                let clique_2_degree = m * 2;

                assert!(degree_sequence.take(m).all(|d| d == clique_1_degree));
                assert!(degree_sequence.all(|d| d == clique_2_degree));
            }

            #[test]
            fn biclique_degree_sum_equals_2size(
                m in 1..5_usize,
                n in 1..5_usize
            ) {
                let digraph = <$type>::biclique(m, n);

                assert_eq!(
                    digraph.vertices().fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn biclique_even_number_odd_degrees(
                m in 1..5_usize,
                n in 1..5_usize
            ) {
                let digraph = <$type>::biclique(m, n);

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
            fn biclique_has_arc(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| (0..m).all(|v| !digraph.has_arc(u, v)))
                        && (m..order)
                            .all(|u| (m..order).all(|v| !digraph.has_arc(u, v)))
                        && (0..m)
                            .all(|u| (m..order).all(|v| digraph.has_arc(u, v)))
                );
            }

            #[test]
            fn biclique_has_edge(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| (0..m).all(|v| !digraph.has_edge(u, v)))
                        && (m..order)
                            .all(|u| (m..order).all(|v| !digraph.has_edge(u, v)))
                        && (0..m)
                            .all(|u| (m..order).all(|v| digraph.has_edge(u, v)))
                );
            }

            #[test]
            fn biclique_in_neighbors(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| digraph.in_neighbors(u).eq(m..order))
                        && (m..order).all(|u| digraph.in_neighbors(u).eq(0..m))
                );
            }

            #[test]
            fn biclique_indegree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert!(
                    (0..m).all(|u| digraph.indegree(u) == n)
                        && (m..m + n).all(|u| digraph.indegree(u) == m)
                );
            }

            #[test]
            fn biclique_indegree_sequence(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert!(indegree_sequence.take(m).all(|d| d == n));
                assert!(indegree_sequence.all(|d| d == m));
            }

            #[test]
            fn biclique_is_balanced(m in 1..5_usize, n in 1..5_usize) {
                assert!(<$type>::biclique(m, n).is_balanced());
            }

            #[test]
            fn biclique_is_complete(m in 1..5_usize, n in 1..5_usize) {
                assert!(
                    ((m, n) == (1, 1)) == <$type>::biclique(m, n).is_complete()
                );
            }

            #[test]
            fn biclique_is_isolated(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert!(digraph.vertices().all(|u| !digraph.is_isolated(u)));
            }

            #[test]
            fn biclique_is_oriented(m in 1..5_usize, n in 1..5_usize) {
                assert!(!<$type>::biclique(m, n).is_oriented());
            }

            #[test]
            fn biclique_is_pendant(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn biclique_is_regular(m in 1..5_usize, n in 1..5_usize) {
                assert!(<$type>::biclique(m, n).is_regular() == (m == n));
            }

            #[test]
            fn biclique_is_semicomplete(m in 1..5_usize, n in 1..5_usize) {
                assert!(
                    ((m, n) == (1, 1))
                        == <$type>::biclique(m, n).is_semicomplete()
                );
            }

            #[test]
            fn biclique_is_simple(m in 1..5_usize, n in 1..5_usize) {
                assert!(<$type>::biclique(m, n).is_simple());
            }

            #[test]
            fn biclique_is_spanning_subdigraph(
                m in 1..5_usize,
                n in 1..5_usize
            ) {
                let digraph = <$type>::biclique(m, n);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn biclique_is_subdigraph(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn biclique_is_superdigraph(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn biclique_is_symmetric(m in 1..5_usize, n in 1..5_usize) {
                assert!(<$type>::biclique(m, n).is_symmetric());
            }

            #[test]
            fn biclique_is_tournament(m in 1..5_usize, n in 1..5_usize) {
                assert!(!<$type>::biclique(m, n).is_tournament());
            }

            #[test]
            fn biclique_max_degree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert_eq!(digraph.max_degree(), m.max(n) * 2);
            }

            #[test]
            fn biclique_max_indegree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert_eq!(digraph.max_indegree(), m.max(n));
            }

            #[test]
            fn biclique_max_outdegree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert_eq!(digraph.max_outdegree(), m.max(n));
            }

            #[test]
            fn biclique_min_degree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert_eq!(digraph.min_degree(), m.min(n) * 2);
            }

            #[test]
            fn biclique_min_indegree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert_eq!(digraph.min_indegree(), m.min(n));
            }

            #[test]
            fn biclique_min_outdegree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert_eq!(digraph.min_outdegree(), m.min(n));
            }

            #[test]
            fn biclique_order(m in 1..5_usize, n in 1..5_usize) {
                assert_eq!(<$type>::biclique(m, n).order(), m + n);
            }

            #[test]
            fn biclique_out_neighbors(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let order = m + n;

                assert!(
                    (0..m).all(|u| digraph.out_neighbors(u).eq(m..order))
                        && (m..order).all(|u| digraph.out_neighbors(u).eq(0..m))
                );
            }

            #[test]
            fn biclique_outdegree(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);

                assert!(
                    (0..m).all(|u| digraph.outdegree(u) == n)
                        && (m..m + n).all(|u| digraph.outdegree(u) == m)
                );
            }

            #[test]
            fn biclique_outdegree_sequence(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.take(m).all(|d| d == n));
                assert!(outdegree_sequence.all(|d| d == m));
            }

            #[test]
            fn biclique_semidegree_sequence(m in 1..5_usize, n in 1..5_usize) {
                let digraph = <$type>::biclique(m, n);
                let semidegree_sequence = &mut digraph.semidegree_sequence();

                assert!(semidegree_sequence.take(m).all(|d| d == (n, n)));
                assert!(semidegree_sequence.all(|d| d == (m, m)));
            }

            #[test]
            fn biclique_sinks(m in 1..5_usize, n in 1..5_usize) {
                assert!(<$type>::biclique(m, n).sinks().eq([]));
            }

            #[test]
            fn biclique_size(m in 1..5_usize, n in 1..5_usize) {
                assert_eq!(<$type>::biclique(m, n).size(), m * n * 2);
            }

            #[test]
            fn biclique_sources(m in 1..5_usize, n in 1..5_usize) {
                assert!(<$type>::biclique(m, n).sources().eq([]));
            }
        }
    }
}
