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

/// Generate cycle digraphs.
///
/// A cycle is a digraph with a single bidirectional cycle.
///
/// # Implementing [`Cycle`] for a custom type
///
/// Provide an implementation of [`cycle`](Cycle::cycle) that generates a cycle
/// digraph of a given `order`.
///
/// ```
/// use {
///     graaf::Cycle,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Cycle for AdjacencyList {
///     /// # Panics
///     ///
///     /// Panics if `order` is zero.
///     fn cycle(order: usize) -> Self {
///         assert!(order > 0, "a digraph must have at least one vertex");
///
///         if order == 1 {
///             return Self {
///                 arcs: vec![BTreeSet::new()],
///             };
///         }
///
///         Self {
///             arcs: (0..order)
///                 .map(|u| {
///                     BTreeSet::from([
///                         (u + order - 1) % order,
///                         (u + 1) % order,
///                     ])
///                 })
///                 .collect(),
///         }
///     }
/// }
///
/// let digraph = AdjacencyList::cycle(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([2, 1]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([1, 0]),
/// ]));
/// ```
///
/// Implementations can be built with the [`AddArc`] and [`Empty`] traits.
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
/// impl Cycle for AdjacencyList {
///     /// # Panics
///     ///
///     /// Panics if `order` is zero.
///     fn cycle(order: usize) -> Self {
///         let mut digraph = Self::empty(order);
///
///         for u in 0..order {
///             digraph.add_arc(u, (u + 1) % order);
///             digraph.add_arc(u, (u + order - 1) % order);
///         }
///
///         digraph
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
