//! Return the number of arcs in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Size,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
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

/// Return the number of arcs in a digraph.
///
/// # Implementing `Size`
///
/// Provide an implementation of `size` that returns the number of arcs in the
/// digraph.
///
/// ```
/// use {
///     graaf::Size,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Size for AdjacencyList {
///     fn size(&self) -> usize {
///         self.arcs.iter().map(BTreeSet::len).sum()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AdjacencyList,
///     Circuit,
///     Size,
/// };
///
/// let digraph = AdjacencyList::circuit(3);
///
/// assert_eq!(digraph.size(), 3);
/// ```
pub trait Size {
    /// Counts the arcs in the digraph.
    #[must_use]
    fn size(&self) -> usize;
}
