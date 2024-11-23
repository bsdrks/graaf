//! Generate a digraph's converse.
//!
//! A digraph's converse is the digraph with all arcs reversed.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//!     Converse,
//! };
//!
//! let digraph = AdjacencyList::circuit(4);
//! let converse = digraph.converse();
//!
//! assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
//! ```

/// Digraph converse
pub trait Converse {
    /// Generate a digraph's converse.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    ///     Converse,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(4);
    /// let converse = digraph.converse();
    ///
    /// assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
    /// ```
    #[must_use]
    fn converse(&self) -> Self;
}
