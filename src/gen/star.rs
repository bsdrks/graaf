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
//! ![A star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
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
//! ![A star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
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
//! ![A star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
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

/// Generate star digraphs.
///
/// A star digraph is a digraph with a single vertex connected to all others.
///
/// # Implementing [`Star`] for a custom type
///
/// Provide an implementation of [`star`](Star::star) that generates a star
/// digraph of a given `order`.
///
/// ```
/// use {
///     graaf::Star,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Star for AdjacencyList {
///     fn star(order: usize) -> Self {
///         let mut arcs = Vec::with_capacity(order);
///
///         arcs.push((1..order).collect());
///
///         for _ in 1..order {
///             arcs.push(BTreeSet::from([0]));
///         }
///
///         Self { arcs }
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
///
/// Implementations can be built with the [`AddArc`](crate::AddArc) and
/// [`Empty`](crate::Empty) traits.
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
/// impl Empty for AdjacencyList {
///     fn empty(order: usize) -> Self {
///         Self {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// impl AddArc for AdjacencyList {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl Star for AdjacencyList {
///     fn star(order: usize) -> Self {
///         let mut digraph = Self::empty(order);
///
///         for u in 1..order {
///             digraph.add_arc(u, 0);
///             digraph.add_arc(0, u);
///         }
///
///         digraph
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
    /// ![A star digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_2.svg)
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
    /// ![A star digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_3.svg)
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
    /// ![A star digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/star_4.svg)
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
