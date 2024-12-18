//! Remove an arc from a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(2, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 1)]));
//!
//! assert!(digraph.remove_arc(0, 1));
//! assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
//!
//! assert!(digraph.remove_arc(0, 2));
//! assert!(digraph.arcs().eq([(1, 0), (2, 1)]));
//!
//! assert!(digraph.remove_arc(1, 0));
//! assert!(digraph.arcs().eq([(2, 1)]));
//!
//! assert!(digraph.remove_arc(2, 1));
//! assert!(digraph.arcs().eq([]));
//! ```

/// Remove an arc from a digraph.
pub trait RemoveArc {
    /// Remove the arc from the digraph. Return whether the arc was removed.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    ///     RemoveArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(2, 1);
    ///
    /// assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 1)]));
    ///
    /// assert!(digraph.remove_arc(0, 1));
    /// assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
    ///
    /// assert!(digraph.remove_arc(0, 2));
    /// assert!(digraph.arcs().eq([(1, 0), (2, 1)]));
    ///
    /// assert!(digraph.remove_arc(1, 0));
    /// assert!(digraph.arcs().eq([(2, 1)]));
    ///
    /// assert!(digraph.remove_arc(2, 1));
    /// assert!(digraph.arcs().eq([]));
    /// ```
    #[must_use]
    fn remove_arc(&mut self, u: usize, v: usize) -> bool;
}
