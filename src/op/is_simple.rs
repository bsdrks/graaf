//! Check whether a digraph is simple.
//!
//! A digraph is simple if it has no self-loops or parallel arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSimple,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.is_simple());
//! ```

/// Check whether a digraph is simple.
///
/// # Implementing [`IsSimple`] for a custom type
///
/// Provide an implementation of [`is_simple`](IsSimple::is_simple) that
/// returns whether the digraph is simple.
///
/// ```
/// use {
///     graaf::IsSimple,
///     std::collections::HashSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl IsSimple for AdjacencyList {
///     fn is_simple(&self) -> bool {
///         self.arcs
///             .iter()
///             .enumerate()
///             .all(|(u, set)| !set.contains(&u))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AddArc,
///     AdjacencyList,
///     Empty,
///     IsSimple,
/// };
///
/// let mut digraph = AdjacencyList::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
///
/// assert!(digraph.is_simple());
/// ```
pub trait IsSimple {
    /// Check whether the digraph is simple.
    #[must_use]
    fn is_simple(&self) -> bool;
}
