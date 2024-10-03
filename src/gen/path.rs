//! Generate path digraphs.
//!
//! A path digraph is an arc chain that connects vertices in a linear sequence.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a path digraph of order `2`.
//!
//! ![A path digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(2).arcs().eq([(0, 1)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a path digraph of order `3`.
//!
//! ![A path digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(3).arcs().eq([(0, 1), (1, 2)]));
//! ```
//!
//! ## Order 4
//!
//! Generate a path digraph of order `4`.
//!
//! ![A path digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Path,
//! };
//!
//! assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
//! ```

/// Generate path digraphs.
///
/// A path digraph is an arc chain that connects vertices in a linear sequence.
///
/// # Implementing [`Path`] for a custom type
///
/// Provide an implementation of [`path`](Path::path) that generates a path
/// digraph of a given `order`.
///
/// ```
/// use {
///     graaf::Path,
///     std::{
///         collections::BTreeSet,
///         iter::once,
///     },
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Path for AdjacencyList {
///     fn path(order: usize) -> Self {
///         Self {
///             arcs: (0..order - 1)
///                 .map(|u| BTreeSet::from([u + 1]))
///                 .chain(once(BTreeSet::new()))
///                 .collect(),
///         }
///     }
/// }
///
/// let digraph = AdjacencyList::path(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1]),
///     BTreeSet::from([2]),
///     BTreeSet::new()
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
///         Path,
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
/// impl Path for AdjacencyList {
///     fn path(order: usize) -> Self {
///         let mut digraph = Self::empty(order);
///
///         for u in 0..order - 1 {
///             digraph.add_arc(u, u + 1);
///         }
///
///         digraph
///     }
/// }
///
/// let digraph = AdjacencyList::path(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1]),
///     BTreeSet::from([2]),
///     BTreeSet::new()
/// ]));
/// ```
pub trait Path {
    /// Generate a path digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a path digraph of order `2`.
    ///
    /// ![A path digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(2).arcs().eq([(0, 1)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a path digraph of order `3`.
    ///
    /// ![A path digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(3).arcs().eq([(0, 1), (1, 2)]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a path digraph of order `4`.
    ///
    /// ![A path digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/path_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Path,
    /// };
    ///
    /// assert!(AdjacencyList::path(4).arcs().eq([(0, 1), (1, 2), (2, 3)]));
    /// ```
    #[must_use]
    fn path(order: usize) -> Self;
}
