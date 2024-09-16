//! Generate star digraphs.
//!
//! A star digraph is a digraph with a single vertex connected to all others.
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
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(2).arcs().eq([(0, 1), (1, 0)]));
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
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(3).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (1, 0),
//!     (2, 0)
//! ]));
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
//!     AdjacencyList,
//!     Arcs,
//!     Star,
//! };
//!
//! assert!(AdjacencyList::star(4).arcs().eq([
//!     (0, 1),
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (2, 0),
//!     (3, 0)
//! ]));
//! ```

use crate::{
    AddArc,
    Empty,
};

/// Generate star digraphs.
///
/// A star digraph is a digraph with a single vertex connected to all others.
///
/// # Implementing [`Star`] for a custom type
///
/// Provide an implementation of [`star`](Star::star) that generates a star
/// digraph of a given `order` OR implement [`AddArc`] and [`Empty`].
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         Empty,
///         Star,
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
/// let digraph = AdjacencyList::star(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0]),
///     BTreeSet::from([0])
/// ]));
/// ```
pub trait Star {
    /// Generate a star digraph.
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
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(2).arcs().eq([(0, 1), (1, 0)]));
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
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(3).arcs().eq([
    ///     (0, 1),
    ///     (0, 2),
    ///     (1, 0),
    ///     (2, 0)
    /// ]));
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
    ///     AdjacencyList,
    ///     Arcs,
    ///     Star,
    /// };
    ///
    /// assert!(AdjacencyList::star(4).arcs().eq([
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
