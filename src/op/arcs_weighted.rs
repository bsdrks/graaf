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
//! assert!(
//!     digraph
//!         .arcs_weighted()
//!         .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)])
//! );
//! ```

/// Weighted arcs
pub trait ArcsWeighted {
    /// The weight of an arc.
    type Weight;

    /// Iterate the digraph's weighted arcs.
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
    /// assert!(
    ///     digraph
    ///         .arcs_weighted()
    ///         .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)])
    /// );
    /// ```
    #[must_use]
    fn arcs_weighted(
        &self,
    ) -> impl Iterator<Item = (usize, usize, &Self::Weight)>;
}
