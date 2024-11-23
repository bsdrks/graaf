//! Count the arcs in a digraph.
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

/// Digraph size
pub trait Size {
    /// Count the arcs in the digraph.
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
    #[must_use]
    fn size(&self) -> usize;
}
