//! Return a digraph's vertices.
//!
//! # Example
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

/// Return a digraph's vertices.
///
/// # Implementing [`Vertices`] for a custom type
///
/// Provide an implementation of [`vertices`](Vertices::vertices) that returns
/// an iterator over all vertices in the digraph.
///
/// ```
/// use {
///     graaf::Vertices,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     vertices: Vec<BTreeSet<usize>>,
/// }
///
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.vertices.len()
///     }
/// }
///
/// let digraph = AdjacencyList {
///     vertices: vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
/// };
///
/// assert!(digraph.vertices().eq(0..3));
/// ```
pub trait Vertices {
    /// Return an iterator over the vertices.
    ///
    /// # Example
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
