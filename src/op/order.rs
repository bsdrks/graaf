//! Return the number of vertices in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Empty,
//!     Order,
//! };
//!
//! let digraph = AdjacencyList::empty(4);
//!
//! assert_eq!(digraph.order(), 4);
//! ```

/// Return the number of vertices in a digraph.
///
/// # Implementing [`Order`] for a custom type
///
/// Provides an implementation of [`order`](Order::order) that returns the
/// number of vertices in the digraph.
///
/// ```
/// use graaf::Order;
///
/// struct AdjacencyList {
///     vertices: Vec<usize>,
/// }
///
/// impl Order for AdjacencyList {
///     fn order(&self) -> usize {
///         self.vertices.len()
///     }
/// }
///
/// let digraph = AdjacencyList {
///     vertices: vec![0, 1, 2, 3],
/// };
///
/// assert_eq!(digraph.order(), 4);
/// ```
pub trait Order {
    /// Count the vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Empty,
    ///     Order,
    /// };
    ///
    /// let digraph = AdjacencyList::empty(4);
    ///
    /// assert_eq!(digraph.order(), 4);
    /// ```
    #[must_use]
    fn order(&self) -> usize;
}
