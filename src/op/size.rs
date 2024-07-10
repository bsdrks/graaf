//! Count the arcs in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Size,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 3);
//! digraph.add_arc(2, 0);
//! digraph.add_arc(2, 1);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(3, 1);
//! digraph.add_arc(3, 2);
//!
//! assert_eq!(digraph.size(), 10);
//! ```

/// Count the arcs in a digraph.
///
/// # How can I implement `Size`?
///
/// Provide an implementation of `size` that returns the number of
/// arcs in the digraph.
///
/// ```
/// use graaf::op::Size;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl Size for Digraph {
///     fn size(&self) -> usize {
///         self.arcs.iter().map(Vec::len).sum()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Cycle,
///     op::Size,
/// };
///
/// let digraph = Digraph::cycle(3);
///
/// assert_eq!(digraph.size(), 3);
/// ```
pub trait Size {
    /// Counts the arcs in the digraph.
    #[must_use]
    fn size(&self) -> usize;
}
