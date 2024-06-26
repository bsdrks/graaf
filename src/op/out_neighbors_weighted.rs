//! Iterate over the weighted arcs going out from a vertex in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArcWeighted,
//!         OutNeighborsWeighted,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
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
//! assert!(digraph
//!     .out_neighbors_weighted(0)
//!     .eq([(1, &2), (2, &3), (3, &4)]));
//!
//! assert!(digraph
//!     .out_neighbors_weighted(1)
//!     .eq([(2, &3), (3, &4), (4, &5)]));
//!
//! assert!(digraph
//!     .out_neighbors_weighted(2)
//!     .eq([(3, &4), (4, &5), (5, &6)]));
//! ```
#![doc(alias = "weighted_out_neighbours")]

/// Iterate over the weighted arcs going out from a vertex in a digraph.
///
/// # How can I implement `OutNeighborsWeighted`?
///
/// Provide an implementation of `out_neighbors_weighted` that returns an
/// iterator over all weighted arcs with the source vertex `s`.
///
/// ```
/// use graaf::op::OutNeighborsWeighted;
///
/// struct Digraph {
///     arcs: Vec<Vec<(usize, usize)>>,
/// }
///
/// impl OutNeighborsWeighted<usize> for Digraph {
///     fn out_neighbors_weighted<'a>(
///         &'a self,
///         s: usize,
///     ) -> impl Iterator<Item = (usize, &'a usize)>
///     where
///         usize: 'a,
///     {
///         self.arcs[s].iter().map(|(t, w)| (*t, w))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     gen::Empty,
///     op::{
///         AddArcWeighted,
///         OutNeighborsWeighted,
///     },
/// };
///
/// let mut digraph = Digraph::empty(3);
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
/// assert!(digraph
///     .out_neighbors_weighted(0)
///     .eq([(1, &2), (2, &3), (3, &4)]));
///
/// assert!(digraph
///     .out_neighbors_weighted(1)
///     .eq([(2, &3), (3, &4), (4, &5)]));
///
/// assert!(digraph
///     .out_neighbors_weighted(2)
///     .eq([(3, &4), (4, &5), (5, &6)]));
/// ```
#[doc(alias = "OutNeighboursWeighted")]
pub trait OutNeighborsWeighted<W> {
    /// Returns an iterator over the out-neighbors of the head vertex and their
    /// weights.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    fn out_neighbors_weighted<'a>(&'a self, s: usize) -> impl Iterator<Item = (usize, &'a W)>
    where
        W: 'a;
}
