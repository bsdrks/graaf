//! Iterates a digraph's arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! let digraph = AdjacencyList::circuit(3);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```

/// Iterates a digraph's arcs.
///
/// # Implementing `Arcs`
///
/// Provide an implementation of `arcs` that returns an iterator over the arcs
/// in a digraph.
///
/// ```
/// use graaf::Arcs;
///
/// struct AdjacencyList {
///     arcs: Vec<(usize, usize)>,
/// }
///
/// impl Arcs for AdjacencyList {
///     fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.arcs.iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AdjacencyList,
///     Arcs,
///     Circuit,
/// };
///
/// let digraph = AdjacencyList::circuit(3);
///
/// assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait Arcs {
    /// Returns an iterator over the arcs in the digraph.
    #[must_use]
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)>;
}
