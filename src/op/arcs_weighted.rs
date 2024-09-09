//! Iterate a digraph's weighted arcs.
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

/// Iterate a digraph's weighted arcs.
///
/// # Implementing [`ArcsWeighted`] for a custom type
///
/// Provide an implementation of [`arcs_weighted`](ArcsWeighted) that returns
/// an iterator over the arcs in a digraph with their weights.
///
/// ```
/// use graaf::ArcsWeighted;
///
/// struct AdjacencyListWeighted {
///     arcs: Vec<(usize, usize, usize)>,
/// }
///
/// impl ArcsWeighted<usize> for AdjacencyListWeighted {
///     fn arcs_weighted<'a>(
///         &'a self,
///     ) -> impl Iterator<Item = (usize, usize, &'a usize)>
///     where
///         usize: 'a,
///     {
///         self.arcs.iter().map(|&(u, v, ref w)| (u, v, w))
///     }
/// }
/// ```
pub trait ArcsWeighted<W> {
    /// Return an iterator over the weighted arcs in the digraph.
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
    fn arcs_weighted<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a;
}
