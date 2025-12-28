//! Iterate a vertex's weighted out-neighbours.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     Empty,
//!     OutNeighborsWeighted,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::empty(6);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(0, 2, 3);
//! digraph.add_arc_weighted(0, 3, 4);
//! digraph.add_arc_weighted(1, 2, 3);
//! digraph.add_arc_weighted(1, 3, 4);
//! digraph.add_arc_weighted(1, 4, 5);
//! digraph.add_arc_weighted(2, 3, 4);
//! digraph.add_arc_weighted(2, 4, 5);
//! digraph.add_arc_weighted(2, 5, 6);
//!
//! assert!(
//!     digraph
//!         .out_neighbors_weighted(0)
//!         .eq([(1, &2), (2, &3), (3, &4)])
//! );
//!
//! assert!(
//!     digraph
//!         .out_neighbors_weighted(1)
//!         .eq([(2, &3), (3, &4), (4, &5)])
//! );
//!
//! assert!(
//!     digraph
//!         .out_neighbors_weighted(2)
//!         .eq([(3, &4), (4, &5), (5, &6)])
//! );
//! ```
#![doc(alias = "weighted_out_neighbours")]

/// Vertex weighted out-neighbours
#[doc(alias = "OutNeighboursWeighted")]
pub trait OutNeighborsWeighted {
    /// The weight of an arc.
    type Weight;

    /// Iterate the vertex's weighted out-neighbors.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     Empty,
    ///     OutNeighborsWeighted,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::empty(6);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(0, 3, 4);
    /// digraph.add_arc_weighted(1, 2, 3);
    /// digraph.add_arc_weighted(1, 3, 4);
    /// digraph.add_arc_weighted(1, 4, 5);
    /// digraph.add_arc_weighted(2, 3, 4);
    /// digraph.add_arc_weighted(2, 4, 5);
    /// digraph.add_arc_weighted(2, 5, 6);
    ///
    /// assert!(
    ///     digraph
    ///         .out_neighbors_weighted(0)
    ///         .eq([(1, &2), (2, &3), (3, &4)])
    /// );
    ///
    /// assert!(
    ///     digraph
    ///         .out_neighbors_weighted(1)
    ///         .eq([(2, &3), (3, &4), (4, &5)])
    /// );
    ///
    /// assert!(
    ///     digraph
    ///         .out_neighbors_weighted(2)
    ///         .eq([(3, &4), (4, &5), (5, &6)])
    /// );
    /// ```
    #[doc(alias = "out_neighbours_weighted")]
    #[must_use]
    fn out_neighbors_weighted(
        &self,
        u: usize,
    ) -> impl Iterator<Item = (usize, &Self::Weight)>;
}
