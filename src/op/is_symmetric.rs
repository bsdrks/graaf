//! Check whether a digraph is symmetric.
//!
//! A digraph is symmetric if for every arc `(u, v)` there is an arc
//! `(v, u)`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSymmetric,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::empty(2);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 0);
//!
//! assert!(digraph.is_symmetric());
//!
//! digraph.remove_arc(1, 0);
//!
//! assert!(!digraph.is_symmetric());
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_symmetric());
//! ```

use crate::{
    Arcs,
    HasArc,
};

/// Check whether a digraph is symmetric.
pub trait IsSymmetric {
    /// Check whether the digraph is symmetric.
    ///
    /// # Examples
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSymmetric,
    ///     RemoveArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(2);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(digraph.is_symmetric());
    ///
    /// digraph.remove_arc(1, 0);
    ///
    /// assert!(!digraph.is_symmetric());
    /// ```
    #[must_use]
    fn is_symmetric(&self) -> bool;
}

impl<D> IsSymmetric for D
where
    D: Arcs + HasArc,
{
    fn is_symmetric(&self) -> bool {
        self.arcs().all(|(u, v)| self.has_arc(v, u))
    }
}
