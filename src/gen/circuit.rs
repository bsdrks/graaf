//! Generate circuit digraphs.
//!
//! A circuit is an oriented cycle.
//!
//! # Examples
//!
//! ## Order 2
//!
//! Generate a circuit digraph of order 2.
//!
//! ![Circuit digraph of order 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_2.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::circuit(2).arcs().eq([(0, 1), (1, 0)]));
//! ```
//!
//! ## Order 3
//!
//! Generate a circuit digraph of order 3.
//!
//! ![Circuit digraph of order 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_3.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::circuit(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```
//!
//! ## Order 4
//!
//! Generate a circuit digraph of order 4.
//!
//! ![Circuit digraph of order 4](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_4.svg)
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::Arcs,
//! };
//!
//! assert!(Digraph::circuit(4)
//!     .arcs()
//!     .eq([(0, 1), (1, 2), (2, 3), (3, 0)]));
//! ```

/// Generate circuit digraphs.
///
/// # Implementing `Circuit`
///
/// Provide an implementation of `circuit` that generates a circuit digraph of
/// a given `order`.
///
/// ```
/// use {
///     graaf::gen::Circuit,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Circuit for Digraph {
///     /// # Panics
///     ///
///     /// Panics if `order` is zero.
///     fn circuit(order: usize) -> Self {
///         if order == 1 {
///             return Self {
///                 arcs: vec![BTreeSet::new()],
///             };
///         }
///
///         Self {
///             arcs: (0..order)
///                 .map(|u| BTreeSet::from([(u + 1) % order]))
///                 .collect::<Vec<_>>(),
///         }
///     }
/// }
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// let digraph = Digraph::circuit(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1]),
///     BTreeSet::from([2]),
///     BTreeSet::from([0])
/// ]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Circuit,
///     op::Arcs,
/// };
///
/// // 0 -> {}
///
/// assert!(Digraph::circuit(1).arcs().eq([]));
///
/// // 0 -> {1}
/// // 1 -> {0}
///
/// assert!(Digraph::circuit(2).arcs().eq([(0, 1), (1, 0)]));
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// assert!(Digraph::circuit(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait Circuit {
    /// Generates a circuit digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ## Order 2
    ///
    /// Generate a circuit digraph of order 2.
    ///
    /// ![Circuit digraph of order 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_2.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Circuit,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::circuit(2).arcs().eq([(0, 1), (1, 0)]));
    /// ```
    ///
    /// ## Order 3
    ///
    /// Generate a circuit digraph of order 3.
    ///
    /// ![Circuit digraph of order 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_3.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Circuit,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::circuit(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
    /// ```
    ///
    /// ## Order 4
    ///
    /// Generate a circuit digraph of order 4.
    ///
    /// ![Circuit digraph of order 4](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/circuit_4.svg)
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Circuit,
    ///     op::Arcs,
    /// };
    ///
    /// assert!(Digraph::circuit(4)
    ///     .arcs()
    ///     .eq([(0, 1), (1, 2), (2, 3), (3, 0)]));
    /// ```
    #[must_use]
    fn circuit(order: usize) -> Self;
}
