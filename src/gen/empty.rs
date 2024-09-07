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
//! ![Empty digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_2.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::empty(2).arcs().eq([]));
//! ```
//!
//! ## Order 3
//!
//! Generate an empty digraph of order `3`.
//!
//! ![Empty digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_3.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::empty(3).arcs().eq([]));
//! ```
//!
//! ## Order 4
//!
//! Generate an empty digraph of order `4`.
//!
//! ![Empty digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_4.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::empty(4).arcs().eq([]));
//! ```
#![doc(alias = "edgeless")]

/// Generate empty digraphs.
///
/// An empty digraph has no arcs.
///
/// # Implementing `Empty`
///
/// Provide an implementation of `empty` that generates an empty digraph with
/// `v` vertices.
///
/// ```
/// use {
///     graaf::gen::Empty,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Empty for Digraph {
///     /// # Panics
///     ///
///     /// Panics if `order` is zero.
///     fn empty(order: usize) -> Self {
///         assert!(order > 0, "a digraph must have at least one vertex");
///
///         Digraph {
///             arcs: vec![BTreeSet::new(); order],
///         }
///     }
/// }
///
/// assert!(Digraph::empty(3).arcs.iter().eq(&[
///     BTreeSet::new(),
///     BTreeSet::new(),
///     BTreeSet::new()
/// ]));
/// ```
#[doc(alias = "Edgeless")]
pub trait Empty {
    /// Generates an empty digraph.
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
    /// ![Empty digraph of order `2`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Empty,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::empty(2).arcs().eq([]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate an empty digraph of order `3`.
    ///
    /// ![Empty digraph of order `3`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Empty,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::empty(3).arcs().eq([]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate an empty digraph of order `4`.
    ///
    /// ![Empty digraph of order `4`](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/empty_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Empty,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::empty(4).arcs().eq([]));
    /// ```
    #[doc(alias = "edgeless")]
    #[must_use]
    fn empty(order: usize) -> Self;

    /// Generates a trivial digraph.
    ///
    /// A trivial digraph has one vertex and no arcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Empty,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::trivial().arcs().eq([]));
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
