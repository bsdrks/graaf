//! Generate Erdős-Rényi digraphs.
//!
//! The Erdős-Rényi model generates a random digraph with a given number of
//! vertices.
//!
//! The time complexity is `O(v²)`, where `v` is the digraph's order.
//!
//! # Examples
//!
//! Generate a random Erdős-Rényi digraph of order `6` with a probability of
//! `0.5` and seed `0`.
//!
//! ![A random Erdős-Rényi digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/erdos_renyi_1-0.89.2.svg?)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     ErdosRenyi,
//! };
//!
//! let digraph = AdjacencyList::erdos_renyi(6, 0.5, 0);
//!
//! assert!(digraph.arcs().eq([
//!     (0, 4),
//!     (0, 5),
//!     (1, 2),
//!     (1, 3),
//!     (1, 4),
//!     (2, 0),
//!     (2, 1),
//!     (2, 4),
//!     (3, 1),
//!     (4, 0),
//!     (4, 1),
//!     (4, 2),
//!     (5, 1),
//!     (5, 3)
//! ]));
//! ```

/// Erdős-Rényi digraphs
pub trait ErdosRenyi {
    /// Generate an Erdős-Rényi digraph.
    ///
    /// The Erdős-Rényi model generates a random digraph with a given number
    /// of vertices.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    /// * `p` - The probability of an arc between two vertices.
    /// * `seed` - The seed for the random number generator.
    ///
    /// # Examples
    ///
    /// Generate a random Erdős-Rényi digraph of order `6` with a probability
    /// of `0.5` and seed `0`.
    ///
    /// ![A random Erdős-Rényi digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/erdos_renyi_1-0.89.2.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     ErdosRenyi,
    /// };
    ///
    /// let digraph = AdjacencyList::erdos_renyi(6, 0.5, 0);
    ///
    /// assert!(digraph.arcs().eq([
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 2),
    ///     (1, 3),
    ///     (1, 4),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 4),
    ///     (3, 1),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 2),
    ///     (5, 1),
    ///     (5, 3)
    /// ]));
    /// ```
    #[must_use]
    fn erdos_renyi(order: usize, p: f64, seed: u64) -> Self;
}

/// `ErdosRenyi` tests
#[macro_export]
macro_rules! test_erdos_renyi {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn erdos_renyi_0() {
            drop(<$type>::erdos_renyi(0, 0.5, 0));
        }

        #[test]
        #[should_panic(expected = "p = -0.1 must be in [0, 1]")]
        fn erdos_renyi_p_negative() {
            drop(<$type>::erdos_renyi(2, -0.1, 0));
        }

        #[test]
        #[should_panic(expected = "p = 1.1 must be in [0, 1]")]
        fn erdos_renyi_p_gt_1() {
            drop(<$type>::erdos_renyi(2, 1.1, 0));
        }
    };
}

/// `ErdosRenyi` proptests
#[macro_export]
macro_rules! proptest_erdos_renyi {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::{
                Degree,
                IsSubdigraph,
                IsSuperdigraph,
            },
        };

        proptest! {
            #[test]
            fn erdos_renyi_degree(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);
                let max_degree = (order - 1) * 2;

                assert!(digraph.vertices().all(|u| {
                    (0..=max_degree).contains(&digraph.degree(u))
                }));
            }

            #[test]
            fn erdos_renyi_degree_sum_equals_2size(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn erdos_renyi_even_number_odd_degrees(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);

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
            fn erdos_renyi_has_arc(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);

                assert!(digraph.vertices().all(|u| !digraph.has_arc(u, u) ));
            }

            #[test]
            fn erdos_renyi_indegree(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);

                assert!(digraph.vertices().all(|v| {
                    (0..order).contains(&digraph.indegree(v))
                }));
            }

            #[test]
            fn erdos_renyi_is_complete(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                if p == 0.0 {
                    assert!(!<$type>::erdos_renyi(order, p, seed).is_complete());
                } else if order == 1 {
                    assert!(<$type>::erdos_renyi(order, p, seed).is_complete());
                }
            }

            #[test]
            fn erdos_renyi_is_simple(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                assert!(<$type>::erdos_renyi(order, p, seed).is_simple());
            }

            #[test]
            fn erdos_renyi_is_subdigraph(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn erdos_renyi_is_superdigraph(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn erdos_renyi_order(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                assert_eq!(<$type>::erdos_renyi(order, p, seed).order(), order);
            }

            #[test]
            fn erdos_renyi_outdegree(
                order in 1..5_usize,
                p in 0.0..1.0,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::erdos_renyi(order, p, seed);

                assert!(digraph.vertices().all(|u| {
                    (0..order).contains(&digraph.outdegree(u))
                }));
            }

            #[test]
            fn erdos_renyi_size_p_0(order in 1..5_usize, seed: u64) {
                assert_eq!(<$type>::erdos_renyi(order, 0.0, seed).size(), 0);
            }

            #[test]
            fn erdos_renyi_size_p_1(order in 1..5_usize, seed: u64) {
                assert_eq!(
                    <$type>::erdos_renyi(order, 1.0, seed).size(),
                    order * (order - 1)
                );
            }
        }
    };
}
