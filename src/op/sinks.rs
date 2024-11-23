//! Iterate a digraph's sinks.
//!
//! A sink is a vertex without out-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Sinks,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sinks().eq([2, 3]));
//! ```

use crate::{
    Outdegree,
    Vertices,
};

/// Digraph sinks
pub trait Sinks {
    /// Iterate a digraph's sinks.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Sinks,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.sinks().eq([2, 3]));
    /// ```
    #[must_use]
    fn sinks(&self) -> impl Iterator<Item = usize>;
}

impl<D> Sinks for D
where
    D: Outdegree + Vertices,
{
    fn sinks(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_sink(u))
    }
}
