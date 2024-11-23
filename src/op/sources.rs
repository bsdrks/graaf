//! Iterate a digraph's sources.
//!
//! A source is a vertex without in-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Sources,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sources().eq([0, 3]));
//! ```

use crate::{
    Indegree,
    Vertices,
};

/// Digraph sources
pub trait Sources {
    /// Iterate the sources in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     Sources,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.sources().eq([0, 3]));
    /// ```
    #[must_use]
    fn sources(&self) -> impl Iterator<Item = usize>;
}

impl<D> Sources for D
where
    D: Indegree + Vertices,
{
    fn sources(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_source(u))
    }
}
