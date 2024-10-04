//! Iterate over a digraph's arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! let digraph = AdjacencyList::circuit(3);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```

/// Iterate over a digraph's arcs.
///
/// # Implementing [`Arcs`] for a custom type
///
/// Provide an implementation of [`arcs`](Arcs::arcs) that returns an iterator
/// over a digraph's arcs.
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
pub trait Arcs {
    /// Return an iterator over a digraph's arcs.
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
    #[must_use]
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)>;
}
