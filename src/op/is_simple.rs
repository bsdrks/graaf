//! A trait to determine whether a graph is simple
//!
//! A graph is simple if it has no self-loops or parallel edges.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::op::IsSimple,
//!     std::collections::HashSet,
//! };
//!
//! let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
//!
//! assert!(graph.is_simple());
//!
//! let graph = vec![vec![0, 1, 2], vec![0, 2], vec![0]];
//!
//! assert!(!graph.is_simple());
//! ```

/// A trait to determine whether a graph is simple
///
/// # How can I implement `IsSimple`?
///
/// Provide an implementation of `is_simple` that returns `true` if the graph is
/// simple and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::IsSimple,
///     std::collections::HashSet,
/// };
///
/// struct Graph {
///     edges: Vec<HashSet<usize>>,
/// }
///
/// impl IsSimple for Graph {
///     fn is_simple(&self) -> bool {
///         self.edges
///             .iter()
///             .enumerate()
///             .all(|(s, set)| !set.contains(&s))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use {
///     graaf::op::IsSimple,
///     std::collections::HashSet,
/// };
///
/// let graph = vec![vec![1, 2], vec![0, 2, 3], vec![0, 1, 3], vec![1, 2]];
///
/// assert!(graph.is_simple());
///
/// let graph = vec![vec![0, 1, 2], vec![0, 2], vec![0]];
///
/// assert!(!graph.is_simple());
/// ```
pub trait IsSimple {
    /// Determine whether the graph is simple.
    fn is_simple(&self) -> bool;
}
