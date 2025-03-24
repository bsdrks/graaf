//! Generate a random recursive tree.
//!
//! A Random Recursive Tree is a directed acyclic graph constructed
//! incrementally. Starting from a single vertex, at each step a new vertex is
//! added along with an arc from the new vertex to one of the existing
//! vertices, chosen uniformly at random. This process yields a tree structure
//! where the order of vertex addition naturally reflects its recursive growth.
//!
//! # Examples
//!
//! Generate a random recursive tree of order `6`.
//!
//! ![A random recursive tree of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_recursive_tree_1-0.89.3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     RandomRecursiveTree,
//! };
//!
//! assert!(AdjacencyList::random_recursive_tree(6, 0).arcs().eq([
//!     (1, 0),
//!     (2, 0),
//!     (3, 1),
//!     (4, 0),
//!     (5, 2)
//! ]));
//! ```

/// random recursive trees
pub trait RandomRecursiveTree {
    /// Generate a random recursive tree.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    /// * `seed` - The seed for the random number generator.
    ///
    /// # Panics
    ///
    /// * Panics if `order` is zero.
    ///
    /// # Examples
    ///
    /// Generate a random recursive tree of order `6`.
    ///
    /// ![A random recursive tree of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/random_recursive_tree_1-0.89.3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     RandomRecursiveTree,
    /// };
    ///
    /// assert!(AdjacencyList::random_recursive_tree(6, 0).arcs().eq([
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 1),
    ///     (4, 0),
    ///     (5, 2)
    /// ]));
    /// ```
    #[must_use]
    fn random_recursive_tree(order: usize, seed: u64) -> Self;
}

/// `RandomRecursiveTree` tests
#[macro_export]
macro_rules! test_random_recursive_tree {
    ($type:ty) => {
        #[test]
        #[should_panic(expected = "a digraph has at least one vertex")]
        fn random_recursive_tree_0() {
            let _ = <$type>::random_recursive_tree(0, 0);
        }
    };
}

/// `RandomRecursiveTree` proptests
#[macro_export]
macro_rules! proptest_random_recursive_tree {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::{
                Degree,
                IsSubdigraph,
                IsSuperdigraph,
                OutdegreeSequence,
            },
        };

        proptest! {
            #[test]
            fn random_recursive_tree_degree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.degree(u) <= order - u
                }));
            }

            #[test]
            fn random_recursive_tree_degree_sequence(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);
                let degree_sequence = &mut digraph.degree_sequence();

                assert!(degree_sequence.all(|d| d < order));
            }

            #[test]
            fn random_recursive_tree_degree_sum_equals_2size(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);

                assert_eq!(
                    digraph
                        .vertices()
                        .fold(0, |acc, u| acc + digraph.degree(u)),
                    2 * digraph.size()
                );
            }

            #[test]
            fn random_recursive_tree_even_number_odd_degrees(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);

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
            fn random_recursive_tree_has_edge(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.vertices().all(|v| !digraph.has_edge(u, v))
                }));
            }

            #[test]
            fn random_recursive_tree_indegree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);

                assert!(digraph.vertices().all(|v| {
                    digraph.indegree(v) <= order - v
                }));
            }

            #[test]
            fn random_recursive_tree_indegree_sequence(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);
                let indegree_sequence = &mut digraph.indegree_sequence();

                assert!(indegree_sequence.all(|d| d < order));
            }

            #[test]
            fn random_recursive_tree_is_complete(order in 1..5_usize, seed: u64) {
                assert!((order == 1) == <$type>::random_recursive_tree(order, seed).is_complete());
            }

            #[test]
            fn random_recursive_tree_is_simple(order in 1..5_usize, seed: u64) {
                assert!(<$type>::random_recursive_tree(order, seed).is_simple());
            }

            #[test]
            fn random_recursive_tree_is_subdigraph(order in 1..5_usize, seed: u64) {
                let digraph = <$type>::random_recursive_tree(order, seed);

                assert!(digraph.is_subdigraph(&digraph));
            }

            #[test]
            fn random_recursive_tree_is_superdigraph(order in 1..5_usize, seed: u64) {
                let digraph = <$type>::random_recursive_tree(order, seed);

                assert!(digraph.is_superdigraph(&digraph));
            }

            #[test]
            fn random_recursive_tree_order(order in 1..5_usize, seed: u64) {
                assert_eq!(<$type>::random_recursive_tree(order, seed).order(), order);
            }

            #[test]
            fn random_recursive_tree_outdegree(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);

                assert!(digraph.vertices().all(|u| {
                    digraph.outdegree(u) <= order - u
                }));
            }

            #[test]
            fn random_recursive_tree_outdegree_sequence(
                order in 1..5_usize,
                seed in 0..1000_u64
            ) {
                let digraph = <$type>::random_recursive_tree(order, seed);
                let outdegree_sequence = &mut digraph.outdegree_sequence();

                assert!(outdegree_sequence.all(|d| d < order));
            }

            #[test]
            fn random_recursive_tree_size(order in 1..5_usize, seed: u64) {
                assert_eq!(<$type>::random_recursive_tree(order, seed).size(), order - 1);
            }
        }
    }
}
