//! Determine whether a digraph is simple.
//!
//! A digraph is simple if it has no self-loops or parallel arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         IsSimple,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.is_simple());
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 0);
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_simple());
//! ```

/// Determine whether a digraph is simple.
///
/// # How can I implement `IsSimple`?
///
/// Provide an implementation of `is_simple` that returns `true` if the digraph
/// is simple and `false` otherwise.
///
/// ```
/// use {
///     graaf::op::IsSimple,
///     std::collections::HashSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl IsSimple for Digraph {
///     fn is_simple(&self) -> bool {
///         self.arcs
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
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         IsSimple,
///     },
/// };
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
///
/// assert!(digraph.is_simple());
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 0);
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(!digraph.is_simple());
/// ```
pub trait IsSimple {
    /// Returns whether the digraph is simple.
    fn is_simple(&self) -> bool;
}
