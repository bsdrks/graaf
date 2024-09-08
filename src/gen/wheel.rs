//! Generate wheel digraphs.
//!
//! A wheel digraph is a wheel digraph with an additional vertex that is
//! connected to all other vertices. A wheel digraph has `4` or more vertices.
//!
//! # Examples
//!
//! ## Order < 4
//!
//! A wheel digraph has at least four vertices.
//!
//! ```should_panic
//! use graaf::{
//!     AdjacencyList,
//!     Wheel,
//! };
//!
//! let _ = AdjacencyList::wheel(3);
//! ```
//!
//! ## Order 4
//!
//! Generate a wheel digraph of order `4`.
//!
//! ![Wheel digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_4-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (1, 2),
//!     (1, 3),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 1),
//!     (3, 2),
//! ]));
//! ```
//!
//! ## Order 5
//!
//! Generate a wheel digraph of order `5`.
//!
//! ![Wheel digraph of order `5`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_5-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(5).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (1, 0),
//!     (1, 2),
//!     (1, 4),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2),
//!     (3, 4),
//!     (4, 0),
//!     (4, 1),
//!     (4, 3),
//! ]));
//! ```
//!
//! ## Order 6
//!
//! Generate a wheel digraph of order `6`.
//!
//! ![Wheel digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_6-0.87.5.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Wheel,
//! };
//!
//! assert!(AdjacencyList::wheel(6).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (0, 4),
//!     (0, 5),
//!     (1, 0),
//!     (1, 2),
//!     (1, 5),
//!     (2, 0),
//!     (2, 1),
//!     (2, 3),
//!     (3, 0),
//!     (3, 2),
//!     (3, 4),
//!     (4, 0),
//!     (4, 3),
//!     (4, 5),
//!     (5, 0),
//!     (5, 1),
//!     (5, 4),
//! ]));
//! ```

use crate::{
    AddArc,
    Empty,
};

/// Generate wheel digraphs.
///
/// A wheel is a digraph with a single bidirectional wheel.
///
/// # Implementing [`Wheel`] for a custom type
///
/// Provide an implementation of [`wheel`](Wheel::wheel) that generates a wheel
/// digraph of a given `order` OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         Empty,
///         Wheel,
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
/// let digraph = AdjacencyList::wheel(4);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2, 3]),
///     BTreeSet::from([0, 2, 3]),
///     BTreeSet::from([0, 1, 3]),
///     BTreeSet::from([0, 1, 2])
/// ]));
/// ```
pub trait Wheel {
    /// Generate a wheel digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 4
    ///
    /// Generate a wheel digraph of order `4`.
    ///
    /// ![Wheel digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_4-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 3),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 1),
    ///     (3, 2),
    /// ]));
    /// ```
    ///
    /// ## Order 5
    ///
    /// Generate a wheel digraph of order `5`.
    ///
    /// ![Wheel digraph of order `5`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_5-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(5).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (0, 4),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 4),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2),
    ///     (3, 4),
    ///     (4, 0),
    ///     (4, 1),
    ///     (4, 3),
    /// ]));
    /// ```
    ///
    /// ## Order 6
    ///
    /// Generate a wheel digraph of order `6`.
    ///
    /// ![Wheel digraph of order `6`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/wheel_6-0.87.5.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Wheel,
    /// };
    ///
    /// assert!(AdjacencyList::wheel(6).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (0, 4),
    ///     (0, 5),
    ///     (1, 0),
    ///     (1, 2),
    ///     (1, 5),
    ///     (2, 0),
    ///     (2, 1),
    ///     (2, 3),
    ///     (3, 0),
    ///     (3, 2),
    ///     (3, 4),
    ///     (4, 0),
    ///     (4, 3),
    ///     (4, 5),
    ///     (5, 0),
    ///     (5, 1),
    ///     (5, 4),
    /// ]));
    /// ```
    #[must_use]
    fn wheel(order: usize) -> Self;
}

impl<D> Wheel for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn wheel(order: usize) -> Self {
        assert!(order > 3, "a wheel digraph has at least four vertices");

        let mut digraph = D::empty(order);

        for u in 1..order - 1 {
            let v = u + 1;

            digraph.add_arc(u, v);
            digraph.add_arc(v, u);
        }

        let u = order - 1;

        digraph.add_arc(u, 1);
        digraph.add_arc(1, u);

        for u in 1..order {
            digraph.add_arc(0, u);
            digraph.add_arc(u, 0);
        }

        digraph
    }
}
