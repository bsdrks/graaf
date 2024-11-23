//! Iterate a digraph's degree sequence.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     DegreeSequence,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.degree_sequence().eq([3, 2, 3]));
//! ```

use crate::{
    Degree,
    Vertices,
};

/// Digraph degree sequence
pub trait DegreeSequence {
    /// Iterate the digraph's degree sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     DegreeSequence,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.degree_sequence().eq([3, 2, 3]));
    /// ```
    #[must_use]
    fn degree_sequence(&self) -> impl Iterator<Item = usize>;
}

impl<D> DegreeSequence for D
where
    D: Degree + Vertices,
{
    fn degree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.degree(v))
    }
}
