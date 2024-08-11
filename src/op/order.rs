//! Return the number of vertices in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::Order,
//! };
//!
//! let digraph = Digraph::empty(4);
//!
//! assert_eq!(digraph.order(), 4);
//! ```

/// Return the number of vertices in a digraph.
///
/// # How can I implement `Order`?
///
/// Provides an implementation of `order` that returns the number of vertices
/// in the digraph.
///
/// ```
/// use graaf::op::Order;
///
/// struct Digraph {
///     vertices: Vec<usize>,
/// }
///
/// impl Order for Digraph {
///     fn order(&self) -> usize {
///         self.vertices.len()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::Order,
/// };
///
/// let digraph = Digraph::empty(4);
///
/// assert_eq!(digraph.order(), 4);
/// ```
pub trait Order {
    /// Counts the vertices in the digraph.
    #[must_use]
    fn order(&self) -> usize;
}
