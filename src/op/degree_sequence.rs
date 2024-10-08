//! Return a digraph's degree sequence.
//!
//! The degree sequence iterates over a digraph's degrees.
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

/// Return a digraph's degree sequence.
///
/// # Implementing [`DegreeSequence`] for a custom type
///
/// Provide an implementation of
/// [`degree_sequence`](DegreeSequence::degree_sequence) that returns a
/// digraph's degree sequence OR implement [`Degree`] and [`Vertices`].
///
/// ```
/// use {
///     graaf::{
///         Degree,
///         DegreeSequence,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Degree for AdjacencyList {
///     fn degree(&self, v: usize) -> usize {
///         self.arcs[v].len()
///             + self.arcs.iter().filter(|a| a.contains(&v)).count()
///     }
/// }
///
/// impl Vertices for AdjacencyList {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let mut digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// assert!(digraph.degree_sequence().eq([3, 2, 3]));
/// ```
pub trait DegreeSequence {
    /// Return a digraph's degree sequence.
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
