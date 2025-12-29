//! Generate path digraphs.
//!
//! A path digraph is an arc chain that connects vertices in a linear sequence.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a path digraph of order `2`.
//!
//! ![A path digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(2).arcs().eq([(0, 1)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a path digraph of order `3`.
//!
//! ![A path digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(3).arcs().eq([(0, 1), (1, 2)]));
//! ```
//!
//! ## Order 4
//!
//! Generate a path digraph of order `4`.
//!
//! ![A path digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
//! ```

/// Path digraphs
pub trait Path {
    /// Generate a path digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a path digraph of order `2`.
    ///
    /// ![A path digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(2).arcs().eq([(0, 1)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a path digraph of order `3`.
    ///
    /// ![A path digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(3).arcs().eq([(0, 1), (1, 2)]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a path digraph of order `4`.
    ///
    /// ![A path digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
    /// ```
    #[must_use]
    fn path(order: usize) -> Self;
}

/// `Path` tests
#[macro_export]
macro_rules! test_path {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn path_0() {
            drop(<$type>::path(0));
        }

        #[test]
        fn path_1() {
            let digraph = <$type>::path(1);

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn path_1_complement() {
            let digraph = <$type>::path(1).complement();

            assert_eq!(digraph.order(), 1);
            assert!(digraph.arcs().eq([]));
        }

        #[test]
        fn path_2() {
            let digraph = <$type>::path(2);

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(0, 1)]));
        }

        #[test]
        fn path_2_complement() {
            let digraph = <$type>::path(2).complement();

            assert_eq!(digraph.order(), 2);
            assert!(digraph.arcs().eq([(1, 0)]));
        }

        #[test]
        fn path_3() {
            let digraph = <$type>::path(3);

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 1), (1, 2)]));
        }

        #[test]
        fn path_3_complement() {
            let digraph = <$type>::path(3).complement();

            assert_eq!(digraph.order(), 3);
            assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 0), (2, 1)]));
        }
    };
}

/// `Path` proptests
#[macro_export]
macro_rules! proptest_path {
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
            fn path_complement_size(order in 1..5_usize) {
                assert_eq!(
                    <$type>::path(order).complement().size(),
                    (order - 1).pow(2)
                );
            }

            #[test]
            fn path_degree(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let last = order - 1;

                assert!(
                    (order == 1 && digraph.degree(0) == 0)
                        || digraph.degree(0) == 1
                            && (1..last).all(|u| digraph.degree(u) == 2
                                && digraph.degree(last) == 1)
                );
            }

            #[test]
            fn path_degree_sequence(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let degree_sequence = &mut digraph.degree_sequence();

                if order == 1 {
                    assert_eq!(degree_sequence.next(), Some(0));
                } else {
                    assert_eq!(degree_sequence.next(), Some(1));
                    assert!(degree_sequence.take(order - 2).all(|d| d == 2));
                    assert_eq!(degree_sequence.next(), Some(1));
                }
            }

            #[test]
            fn path_degree_sum_equals_2size(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn path_even_number_odd_degrees(order in 1..5_usize) {
                let digraph = <$type>::path(order);

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
            fn path_has_edge(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert!((0..order)
                    .all(|u| (0..order).all(|v| !digraph.has_edge(u, v))));
            }

            #[test]
            fn path_indegree(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert_eq!(digraph.indegree(0), 0);
                assert!((1..order).all(|u| digraph.indegree(u) == 1));
            }

            #[test]
            fn path_indegree_sequence(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert_eq!(indegree_sequence.next(), Some(0));
                assert!(indegree_sequence.all(|d| d == 1));
            }

            #[test]
            fn path_is_balanced(order in 1..5_usize) {
                assert!((order == 1) == <$type>::path(order).is_balanced());
            }

            #[test]
            fn path_is_complete(order in 1..5_usize) {
                assert!((order == 1) == <$type>::path(order).is_complete());
            }

            #[test]
            fn path_is_isolated(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert!(digraph
                    .vertices()
                    .all(|u| (order == 1) == digraph.is_isolated(u)));
            }

            #[test]
            fn path_is_oriented(order in 1..5_usize) {
                assert!(<$type>::path(order).is_oriented());
            }

            #[test]
            fn path_is_pendant(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let last = order - 1;

                assert!(
                    (order == 1 && !digraph.is_pendant(0))
                        || (digraph.is_pendant(0)
                            && (1..last).all(|u| !digraph.is_pendant(u))
                            && digraph.is_pendant(last))
                );
            }

            #[test]
            fn path_is_regular(order in 1..5_usize) {
                assert!((order == 1) == <$type>::path(order).is_regular());
            }

            #[test]
            fn path_is_semicomplete(order in 1..5_usize) {
                assert!((order < 3) == <$type>::path(order).is_semicomplete());
            }

            #[test]
            fn path_is_simple(order in 1..5_usize) {
                assert!(<$type>::path(order).is_simple());
            }

            #[test]
            fn path_is_sink(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let last = order - 1;

                assert!(
                    (order == 1 && digraph.is_sink(0))
                        || ((0..last).all(|u| !digraph.is_sink(u))
                            && digraph.is_sink(last))
                );
            }

            #[test]
            fn path_is_source(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert!(digraph.is_source(0));
                assert!((1..order).all(|u| !digraph.is_source(u)));
            }

            #[test]
            fn path_is_spanning_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert!(digraph.is_spanning_subdigraph(&digraph));
            }

            #[test]
            fn path_is_subdigraph(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn path_is_superdigraph(order in 1..5_usize) {
                let digraph = <$type>::path(order);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn path_is_symmetric(order in 1..5_usize) {
                assert!((order == 1) == <$type>::path(order).is_symmetric());
            }

            #[test]
            fn path_is_tournament(order in 1..5_usize) {
                assert!((order < 3) == <$type>::path(order).is_tournament());
            }

            #[test]
            fn path_max_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::path(order).max_degree(),
                    match order {
                        1 => 0,
                        2 => 1,
                        _ => 2,
                    }
                );
            }

            #[test]
            fn path_max_indegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::path(order).max_indegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn path_max_outdegree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::path(order).max_outdegree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn path_min_degree(order in 1..5_usize) {
                assert_eq!(
                    <$type>::path(order).min_degree(),
                    if order == 1 { 0 } else { 1 }
                );
            }

            #[test]
            fn path_min_indegree(order in 1..5_usize) {
                assert_eq!(<$type>::path(order).min_indegree(), 0);
            }

            #[test]
            fn path_min_outdegree(order in 1..5_usize) {
                assert_eq!(<$type>::path(order).min_outdegree(), 0);
            }

            #[test]
            fn path_outdegree(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let last = order - 1;

                assert!((0..last).all(|u| digraph.outdegree(u) == 1));
                assert_eq!(digraph.outdegree(last), 0);
            }

            #[test]
            fn path_outdegree_sequence(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.take(order - 1).all(|d| d == 1));
                assert_eq!(outdegree_sequence.next(), Some(0));
            }

            #[test]
            fn path_semidegree_sequence(order in 1..5_usize) {
                let digraph = <$type>::path(order);
                let semidegree_sequence = &mut digraph.semidegree_sequence();

                assert!(if order == 1 {
                    semidegree_sequence.next() == Some((0, 0))
                } else {
                    semidegree_sequence.next() == Some((0, 1))
                        && semidegree_sequence
                            .take(order - 2)
                            .all(|d| d == (1, 1))
                        && semidegree_sequence.next() == Some((1, 0))
                });
            }

            #[test]
            fn path_sinks(order in 1..5_usize) {
                assert!(<$type>::path(order).sinks().eq([order - 1]));
            }

            #[test]
            fn path_size(order in 1..5_usize) {
                assert_eq!(<$type>::path(order).size(), order - 1);
            }

            #[test]
            fn path_sources(order in 1..5_usize) {
                assert!(<$type>::path(order).sources().eq([0]));
            }
        }
    };
}
