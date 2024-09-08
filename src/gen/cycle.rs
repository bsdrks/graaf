//! Generate cycle digraphs.
//!
//! A cycle is a digraph with a single bidirectional cycle.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a cycle digraph of order `2`.
//!
//! ![Cycle digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a cycle digraph of order `3`.
//!
//! ![Cycle digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(3).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (1, 2),
//!     (2, 0),
//!     (2, 1)
//! ]));
//! ```
//!
//! ## Order 4
//!
//! Generate a cycle digraph of order `4`.
//!
//! ![Cycle digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Cycle,
//! };
//!
//! assert!(AdjacencyList::cycle(4).arcs().eq([
//!     (0, 1),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2)
//! ]));
//! ```

use crate::{
    AddArc,
    Empty,
};

/// Generate cycle digraphs.
///
/// A cycle is a digraph with a single bidirectional cycle.
///
/// # Implementing [`Cycle`] for a custom type
///
/// Provide an implementation of [`cycle`](Cycle::cycle) that generates a cycle
/// digraph of a given `order` OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         Cycle,
///         Empty,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl AddArc for AdjacencyList {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl Empty for AdjacencyList {
///     fn empty(order: usize) -> Self {
///         Self {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// let digraph = AdjacencyList::cycle(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0, 1])
/// ]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AdjacencyList,
///     Arcs,
///     Cycle,
/// };
///
/// assert!(AdjacencyList::cycle(1).arcs().eq([]));
/// assert!(AdjacencyList::cycle(2).arcs().eq([(0, 1), (1, 0)]));
///
/// assert!(AdjacencyList::cycle(3).arcs().eq([
///     (0, 1),
///     (0, 2),
///     (1, 0),
///     (1, 2),
///     (2, 0),
///     (2, 1)
/// ]));
/// ```
pub trait Cycle {
    /// Generate a cycle digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a cycle digraph of order `2`.
    ///
    /// ![Cycle digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a cycle digraph of order `3`.
    ///
    /// ![Cycle digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(3).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (1, 0),
    ///     (1, 2),
    ///     (2, 0),
    ///     (2, 1)
    /// ]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a cycle digraph of order `4`.
    ///
    /// ![Cycle digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/cycle_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Cycle,
    /// };
    ///
    /// assert!(AdjacencyList::cycle(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 2),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn cycle(order: usize) -> Self;
}

impl<D> Cycle for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn cycle(order: usize) -> Self {
        let mut digraph = D::empty(order);

        if order == 1 {
            return digraph;
        }

        for u in 0..order - 1 {
            let v = u + 1;

            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }

        let u = order - 1;

        digraph.add_arc(u, 0);
        digraph.add_arc(0, u);

        digraph
    }
}
