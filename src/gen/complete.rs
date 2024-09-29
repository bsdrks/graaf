//! Generate complete digraphs.
//!
//! In a complete digraph, an arc connects every ordered vertex pair.
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
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(2).arcs().eq([(0, 1), (1, 0)]));
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
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(3).arcs().eq([
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
//!     AdjacencyList,
//!     Arcs,
//!     Complete,
//! };
//!
//! assert!(AdjacencyList::complete(4).arcs().eq([
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

/// Generate complete digraphs.
///
/// In a complete digraph, an arc connects every ordered vertex pair.
///
/// # Implementing [`Complete`] for a custom type
///
/// Provide an implementation of [`complete`](Complete::complete) that
/// generates a complete digraph of a given `order`.
///
/// ```
/// use {
///     graaf::Complete,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Complete for AdjacencyList {
///     fn complete(order: usize) -> Self {
///         Self {
///             arcs: (0..order)
///                 .map(|u| (0..order).filter(|&v| u != v).collect())
///                 .collect(),
///         }
///     }
/// }
///
/// let digraph = AdjacencyList::complete(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0, 1]),
/// ]));
/// ```
///
/// Implementations can be built with the [`AddArc`] and [`Empty`] traits.
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         Complete,
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
///         AdjacencyList {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// impl Complete for AdjacencyList {
///     fn complete(order: usize) -> Self {
///         let mut digraph = Self::empty(order);
///
///         for u in 0..order {
///             for v in (u + 1)..order {
///                 digraph.add_arc(u, v);
///                 digraph.add_arc(v, u);
///             }
///         }
///
///         digraph
///     }
/// }
///
/// let digraph = AdjacencyList::complete(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0, 1]),
/// ]));
/// ```
pub trait Complete {
    /// Generate a complete digraph.
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
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(2).arcs().eq([(0, 1), (1, 0)]));
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
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(3).arcs().eq([
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
    ///     AdjacencyList,
    ///     Arcs,
    ///     Complete,
    /// };
    ///
    /// assert!(AdjacencyList::complete(4).arcs().eq([
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
