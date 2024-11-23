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

/// Digraph order
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
