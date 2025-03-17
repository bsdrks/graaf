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
