//! Generate empty digraphs.
//!
//! An empty digraph has no arcs.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate an empty digraph of order `2`.
//!
//! ![An empty digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_2.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! assert!(AdjacencyList::empty(2).arcs().eq([]));
//! ```
//!
//! ## Order 3
//!
//! Generate an empty digraph of order `3`.
//!
//! ![An empty digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_3.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! assert!(AdjacencyList::empty(3).arcs().eq([]));
//! ```
//!
//! ## Order 4
//!
//! Generate an empty digraph of order `4`.
//!
//! ![An empty digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_4.svg)
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! assert!(AdjacencyList::empty(4).arcs().eq([]));
//! ```
#![doc(alias = "edgeless")]

/// Generate empty digraphs.
///
/// An empty digraph has no arcs.
///
/// # Implementing [`Empty`] for a custom type
///
/// Provide an implementation of [`empty`](Empty::empty) that generates an
/// empty digraph with `v` vertices.
///
/// ```
/// use {
///     graaf::Empty,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Empty for AdjacencyList {
///     /// # Panics
///     ///
///     /// Panics if `order` is zero.
///     fn empty(order: usize) -> Self {
///         assert!(order > 0, "a digraph has at least one vertex");
///
///         AdjacencyList {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// assert!(AdjacencyList::empty(3).arcs.iter().eq(&[
///     BTreeSet::new(),
///     BTreeSet::new(),
///     BTreeSet::new()
/// ]));
/// ```
#[doc(alias = "Edgeless")]
pub trait Empty {
    /// Generate an empty digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate an empty digraph of order `2`.
    ///
    /// ![An empty digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::empty(2).arcs().eq([]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate an empty digraph of order `3`.
    ///
    /// ![An empty digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::empty(3).arcs().eq([]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate an empty digraph of order `4`.
    ///
    /// ![An empty digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::empty(4).arcs().eq([]));
    /// ```
    #[doc(alias = "edgeless")]
    #[must_use]
    fn empty(order: usize) -> Self;

    /// Generate a trivial digraph.
    ///
    /// A trivial digraph has one vertex and no arcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// assert!(AdjacencyList::trivial().arcs().eq([]));
    /// ```
    #[doc(alias = "singleton")]
    #[must_use]
    fn trivial() -> Self
    where
        Self: Sized,
    {
        Self::empty(1)
    }
}
