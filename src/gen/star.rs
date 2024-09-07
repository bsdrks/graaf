//! Generate star digraphs.
//!
//! A star digraph is a digraph with a single vertex that is connected to all
//! other vertices.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a star digraph of order `2`.
//!
//! ![Star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Star,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::star(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a star digraph of order `3`.
//!
//! ![Star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Star,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::star(3).arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
//! ```
//!
//! ## Order 4
//!
//! Generate a star digraph of order `4`.
//!
//! ![Star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Star,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::star(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (2, 0),
//!     (3, 0)
//! ]));
//! ```

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate star digraphs.
///
/// A star digraph is a digraph with a single vertex that is connected to all
/// other vertices.
///
/// # Implementing `Star`
///
/// Provide an implementation of `star` that generates a star digraph of a
/// given `order` OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Empty,
///             Star,
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
///         Self {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// let digraph = Digraph::star(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0]),
///     BTreeSet::from([0])
/// ]));
/// ```
pub trait Star {
    /// Generates a star digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a star digraph of order `2`.
    ///
    /// ![Star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Star,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::star(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a star digraph of order `3`.
    ///
    /// ![Star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Star,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::star(3).arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a star digraph of order `4`.
    ///
    /// ![Star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Star,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::star(4).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (2, 0),
    ///     (3, 0)
    /// ]));
    /// ```
    #[must_use]
    fn star(order: usize) -> Self;
}

impl<D> Star for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn star(order: usize) -> Self {
        let mut digraph = D::empty(order);

        for u in 1..order {
            digraph.add_arc(0, u);
            digraph.add_arc(u, 0);
        }

        digraph
    }
}
