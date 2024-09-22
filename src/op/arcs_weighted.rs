//! Iterate over a digraph's weighted arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     ArcsWeighted,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(1, 2, 3);
//! digraph.add_arc_weighted(2, 0, 4);
//!
//! assert!(digraph
//!     .arcs_weighted()
//!     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
//! ```

/// Iterate over a digraph's weighted arcs.
///
/// # Implementing [`ArcsWeighted`] for a custom type
///
/// Provide an implementation of [`arcs_weighted`](ArcsWeighted) that returns
/// an iterator over a digraph's arcs and their weights.
///
/// ```
/// use graaf::ArcsWeighted;
///
/// struct AdjacencyListWeighted {
///     arcs: Vec<(usize, usize, usize)>,
/// }
///
/// impl ArcsWeighted for AdjacencyListWeighted {
///     type Weight = usize;
///
///     fn arcs_weighted(
///         &self,
///     ) -> impl Iterator<Item = (usize, usize, &Self::Weight)> {
///         self.arcs.iter().map(|&(u, v, ref w)| (u, v, w))
///     }
/// }
/// ```
pub trait ArcsWeighted {
    /// The weight of an arc.
    type Weight;

    /// Return an iterator over a digraphs' weighted arcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     ArcsWeighted,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::empty(3);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(1, 2, 3);
    /// digraph.add_arc_weighted(2, 0, 4);
    ///
    /// assert!(digraph
    ///     .arcs_weighted()
    ///     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
    /// ```
    #[must_use]
    fn arcs_weighted(
        &self,
    ) -> impl Iterator<Item = (usize, usize, &Self::Weight)>;
}
