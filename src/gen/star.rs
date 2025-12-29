//! Generate star digraphs.
//!
//! A star digraph is a digraph with a single vertex connected to all others.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a star digraph of order `2`.
//!
//! ![A star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a star digraph of order `3`.
//!
//! ![A star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(3).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (2, 0)
//! ]));
//! ```
//!
//! ## Order 4
//!
//! Generate a star digraph of order `4`.
//!
//! ![A star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (2, 0),
//!     (3, 0)
//! ]));
//! ```

/// Star digraphs
pub trait Star {
    /// Generate a star digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a star digraph of order `2`.
    ///
    /// ![A star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a star digraph of order `3`.
    ///
    /// ![A star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(3).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (1, 0),
    ///     (2, 0)
    /// ]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a star digraph of order `4`.
    ///
    /// ![A star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 0)
    /// ]));
    /// ```
    #[must_use]
    fn star(order: usize) -> Self;
}

/// `Star` tests
#[macro_export]
macro_rules! test_star {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn star_0() {
            drop(<$type>::star(0));
        }

        #[test]
        fn star_1() {
            let digraph = <$type>::star(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn star_1_complement() {
            let digraph = <$type>::star(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn star_2() {
            let digraph = <$type>::star(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1), (1, 0)]));
        }

        #[test]
        fn star_2_complement() {
            let digraph = <$type>::star(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn star_3() {
            let digraph = <$type>::star(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
        }

        #[test]
        fn star_3_complement() {
            let digraph = <$type>::star(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(1, 2), (2, 1)]));
        }
    };
}

/// `Star` proptests
#[macro_export]
macro_rules! proptest_star {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::{
                Degree,
                IsBalanced,
                IsIsolated,
                IsOriented,
                IsPendant,
                IsSpanningSubdigraph,
                IsSubdigraph,
                IsSuperdigraph,
                IsSymmetric,
                OutdegreeSequence,
                Sinks,
                Sources,
            },
        };

        proptest! {
            #[test]
            fn star_complement_size(order in 1..5_usize) {
                assert_eq!(
                    <$type>::star(order).complement().size(),
                    (order - 1) * order.saturating_sub(2)
                );
            }

            #[test]
            fn star_degree(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert_eq!(digraph.degree(0), (order - 1) * 2);
                assert!((1..order).all(|u| digraph.degree(u) == 2));
            }

            #[test]
            fn star_degree_sequence(order in 1..5_usize) {
                let digraph = <$type>::star(order);
                let degree_sequence = &mut digraph.degree_sequence();

                assert_eq!(degree_sequence.next(), Some((order - 1) * 2));
                assert!(degree_sequence.all(|d| d == 2));
            }

            #[test]
            fn star_degree_sum_equals_2size(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn star_even_number_odd_degrees(order in 1..5_usize) {
                let digraph = <$type>::star(order);

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
            fn star_has_edge(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!((1..order).all(|u| digraph.has_edge(0, u)
                    && (u..order).all(|v| !digraph.has_edge(u, v))));
            }

            #[test]
            fn star_indegree(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert_eq!(digraph.indegree(0), order - 1);
                assert!((1..order).all(|u| digraph.indegree(u) == 1));
            }

            #[test]
            fn star_indegree_sequence(order in 1..5_usize) {
                let digraph = <$type>::star(order);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert_eq!(indegree_sequence.next(), Some(order - 1));
                assert!(indegree_sequence.all(|d| d == 1));
            }

            #[test]
            fn star_is_balanced(order in 1..5_usize) {
                assert!(<$type>::star(order).is_balanced());
            }

            #[test]
            fn star_is_complete(order in 1..5_usize) {
                assert!((order < 3) == <$type>::star(order).is_complete());
            }

            #[test]
            fn star_is_isolated(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn star_is_oriented(order in 1..5_usize) {
                assert!((order == 1) == <$type>::star(order).is_oriented());
            }

            #[test]
            fn star_is_pendant(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!(digraph.vertices().all(|u| !digraph.is_pendant(u)));
            }

            #[test]
            fn star_is_regular(order in 1..5_usize) {
                assert!((order < 3) == <$type>::star(order).is_regular());
            }

            #[test]
            fn star_is_semicomplete(order in 1..5_usize) {
                assert!((order < 3) == <$type>::star(order).is_semicomplete());
            }

            #[test]
            fn star_is_simple(order in 1..5_usize) {
                assert!(<$type>::star(order).is_simple());
            }

            #[test]
            fn star_is_sink(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_sink(u)));
            }

            #[test]
            fn star_is_source(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_source(u)));
            }

            #[test]
            fn star_is_spanning_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn star_is_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn star_is_superdigraph(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn star_is_symmetric(order in 1..5_usize) {
                assert!(<$type>::star(order).is_symmetric());
            }

            #[test]
            fn star_is_tournament(order in 1..5_usize) {
                assert!((order == 1) == <$type>::star(order).is_tournament());
            }

            #[test]
            fn star_max_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::star(order).max_degree(),
                    (order - 1) * 2
                );
            }

            #[test]
            fn star_max_indegree(order in 1..5_usize) {
                assert_eq!(<$type>::star(order).max_indegree(), order - 1);
            }

            #[test]
            fn star_max_outdegree(order in 1..5_usize) {
                assert_eq!(<$type>::star(order).max_outdegree(), order - 1);
            }

            #[test]
            fn star_min_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::star(order).min_degree(),
                    if order == 1 { 0 } else { 2 }
                );
            }

            #[test]
            fn star_min_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::star(order).min_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn star_min_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::star(order).min_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn star_outdegree(order in 1..5_usize) {
                let digraph = <$type>::star(order);

                assert_eq!(digraph.outdegree(0), order - 1);
                assert!((1..order).all(|u| digraph.outdegree(u) == 1));
            }

            #[test]
            fn star_outdegree_sequence(order in 1..5_usize) {
                let digraph = <$type>::star(order);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert_eq!(outdegree_sequence.next(), Some(order - 1));
                assert!(outdegree_sequence.all(|d| d == 1));
            }

            #[test]
            fn star_semidegree_sequence(order in 1..5_usize) {
                let digraph = <$type>::star(order);
                let mut semidegree_sequence = digraph.semidegree_sequence();

                assert_eq!(
                    semidegree_sequence.next(),
                    Some((order - 1, order - 1))
                );

                assert!(semidegree_sequence.all(|d| d == (1, 1)));
            }

            #[test]
            fn star_sinks(order in 1..5_usize) {
                assert!(if order == 1 {
                    <$type>::star(order).sinks().eq([0])
                } else {
                    <$type>::star(order).sinks().eq([])
                });
            }

            #[test]
            fn star_size(order in 1..5_usize) {
                assert_eq!(<$type>::star(order).size(), (order - 1) * 2);
            }

            #[test]
            fn star_sources(order in 1..5_usize) {
                assert!(if order == 1 {
                    <$type>::star(order).sources().eq([0])
                } else {
                    <$type>::star(order).sources().eq([])
                });
            }
        }
    };
}
