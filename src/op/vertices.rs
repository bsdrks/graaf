//! Iterate over the vertices in a digraph.
//!
//! # Example
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::Vertices,
//! };
//!
//! let digraph = Digraph::empty(4);
//!
//! assert!(digraph.vertices().eq(0..4));
//! ```

/// Iterate over the vertices in a digraph.
///
/// # How can I implement `Vertices`?
///
/// Provide an implementation of `vertices` that returns an iterator over
/// all vertices in the digraph.
///
/// ```
/// use graaf::op::Vertices;
///
/// struct Digraph {
///     vertices: Vec<usize>,
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.vertices.len()
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::Vertices,
/// };
///
/// let digraph = Digraph::empty(4);
///
/// assert!(digraph.vertices().eq(0..4));
/// ```
pub trait Vertices {
    /// Returns an iterator over the vertices.
    #[must_use]
    fn vertices(&self) -> impl Iterator<Item = usize>;
}
