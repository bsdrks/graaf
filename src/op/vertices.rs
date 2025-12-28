//! Iterate a digraph's vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Empty,
//!     Vertices,
//! };
//!
//! let digraph = AdjacencyList::empty(4);
//!
//! assert!(digraph.vertices().eq(0..4));
//! ```

/// Digraph vertices
pub trait Vertices {
    /// Iterate the digraph's vertices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Empty,
    ///     Vertices,
    /// };
    ///
    /// let digraph = AdjacencyList::empty(4);
    ///
    /// assert!(digraph.vertices().eq(0..4));
    /// ```
    #[must_use]
    fn vertices(&self) -> impl Iterator<Item = usize>;
}
