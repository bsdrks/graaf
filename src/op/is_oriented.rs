//! Check whether a digraph is oriented.
//!
//! An oriented digraph has no cycle of length 2.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     IsOriented,
//! };
//!
//! assert!(!AdjacencyList::circuit(2).is_oriented());
//! assert!(AdjacencyList::circuit(3).is_oriented());
//! ```

use crate::{
    Arcs,
    HasArc,
};

/// Check whether a digraph is oriented.
pub trait IsOriented {
    /// Check whether the digraph is oriented.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     IsOriented,
    /// };
    ///
    /// assert!(!AdjacencyList::circuit(2).is_oriented());
    /// assert!(AdjacencyList::circuit(3).is_oriented());
    /// ```
    #[must_use]
    fn is_oriented(&self) -> bool;
}

impl<D> IsOriented for D
where
    D: Arcs + HasArc,
{
    fn is_oriented(&self) -> bool {
        self.arcs().all(|(u, v)| !self.has_arc(v, u))
    }
}
