//! Return the subgraph with the vertices that satisfy the predicate.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyMap,
//!     Arcs,
//!     FilterVertices,
//!     Vertices,
//!     Wheel,
//! };
//!
//! let mut digraph = AdjacencyMap::wheel(9);
//! let subgraph = digraph.filter_vertices(|u| u % 2 == 0 && u < 6);
//!
//! assert!(subgraph.arcs().eq([(0, 2), (0, 4), (2, 0), (4, 0)]));
//! assert!(subgraph.vertices().eq([0, 2, 4]));
//! ```

/// Filter vertices
pub trait FilterVertices {
    /// Return the subgraph with the vertices that satisfy the predicate.
    ///
    /// # Panics
    ///
    /// Panics if the subgraph has zero vertices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyMap,
    ///     Arcs,
    ///     FilterVertices,
    ///     Vertices,
    ///     Wheel,
    /// };
    ///
    /// let mut digraph = AdjacencyMap::wheel(9);
    /// let subgraph = digraph.filter_vertices(|u| u % 2 == 0 && u < 6);
    ///
    /// assert!(subgraph.arcs().eq([(0, 2), (0, 4), (2, 0), (4, 0)]));
    /// assert!(subgraph.vertices().eq([0, 2, 4]));
    /// ```
    #[must_use]
    fn filter_vertices<P>(&self, predicate: P) -> Self
    where
        P: Fn(usize) -> bool;
}
