//! Return a digraph's vertices.
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

/// Return a digraph's vertices.
///
/// # Implementing `Vertices`
///
/// Provide an implementation of `vertices` that returns an iterator over
/// all vertices in the digraph.
///
/// ```
/// use {
///     graaf::op::Vertices,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     vertices: Vec<BTreeSet<usize>>,
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.vertices.len()
///     }
/// }
///
/// let digraph = Digraph {
///     vertices: vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
/// };
///
/// assert!(digraph.vertices().eq(0..3));
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
