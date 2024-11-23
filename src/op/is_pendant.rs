//! Check whether a vertex is pendant.
//!
//! A pendant vertex has a degree of one.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsPendant,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(3, 0);
//!
//! assert!(!digraph.is_pendant(0));
//! assert!(!digraph.is_pendant(1));
//! assert!(digraph.is_pendant(2));
//! assert!(digraph.is_pendant(3));
//! ```

use crate::Degree;

/// Check whether a vertex is pendant.
pub trait IsPendant {
    /// Check whether the vertex is pendant.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsPendant,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(!digraph.is_pendant(0));
    /// assert!(!digraph.is_pendant(1));
    /// assert!(digraph.is_pendant(2));
    /// assert!(digraph.is_pendant(3));
    /// ```
    #[must_use]
    fn is_pendant(&self, u: usize) -> bool;
}

impl<D> IsPendant for D
where
    D: Degree,
{
    fn is_pendant(&self, u: usize) -> bool {
        self.degree(u) == 1
    }
}
