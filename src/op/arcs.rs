//! Iterate a digraph's arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! let digraph = AdjacencyList::circuit(3);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```

/// Digraph arcs
pub trait Arcs {
    /// Iterate the digraph's arcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(3);
    ///
    /// assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
    /// ```
    #[must_use]
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)>;
}
