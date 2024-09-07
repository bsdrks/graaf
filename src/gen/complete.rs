//! Generate complete digraphs.
//!
//! In a complete digraph, an arc connects every ordered pair of vertices.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a complete digraph of order `2`.
//!
//! ![Complete digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_2.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Complete,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::complete(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a complete digraph of order `3`.
//!
//! ![Complete digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_3.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Complete,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::complete(3).arcs().eq([
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
//! Generate a complete digraph of order `4`.
//!
//! ![Complete digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_4.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Complete,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::complete(4).arcs().eq([
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
//!     (3, 2)
//! ]));
//! ```

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate complete digraphs.
///
/// In a complete digraph, an arc connects every ordered pair of vertices.
///
/// # Implementing `Complete`
///
/// Provide an implementation of `complete` that generates a complete digraph
/// of a given `order` OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Complete,
///             Empty,
///         },
///         op::AddArc,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl AddArc for Digraph {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl Empty for Digraph {
///     fn empty(order: usize) -> Self {
///         Digraph {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// let digraph = Digraph::complete(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0, 1]),
/// ]));
/// ```
pub trait Complete {
    /// Generates a complete digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a complete digraph of order `2`.
    ///
    /// ![Complete digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Complete,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::complete(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a complete digraph of order `3`.
    ///
    /// ![Complete digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Complete,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::complete(3).arcs().eq([
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
    /// Generate a complete digraph of order `4`.
    ///
    /// ![Complete digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/complete_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Complete,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::complete(4).arcs().eq([
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
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn complete(order: usize) -> Self;
}

impl<D> Complete for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn complete(order: usize) -> Self {
        let mut digraph = D::empty(order);

        for u in 0..order {
            for v in (u + 1)..order {
                digraph.add_arc(u, v);
                digraph.add_arc(v, u);
            }
        }

        digraph
    }
}
