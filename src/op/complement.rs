//! Generate a digraph's complement.
//!
//! A digraph's complement contains all arcs not in the original digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//!     Complement,
//! };
//!
//! let digraph = AdjacencyList::circuit(4);
//! let converse = digraph.complement();
//!
//! assert!(converse.arcs().eq([
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (1, 3),
//!     (2, 0),
//!     (2, 1),
//!     (3, 1),
//!     (3, 2)
//! ]));
//! ```

/// Digraph complement
pub trait Complement {
    /// Generate the digraph's complement.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    ///     Complement,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(4);
    /// let converse = digraph.complement();
    ///
    /// assert!(converse.arcs().eq([
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 3),
    ///     (2, 0),
    ///     (2, 1),
    ///     (3, 1),
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn complement(&self) -> Self;
}
