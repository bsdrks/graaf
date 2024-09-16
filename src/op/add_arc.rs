//! Add an arc to a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (2, 0)]));
//! ```

/// Add an arc to a digraph.
///
/// # Implementing [`AddArc`] for a custom type
///
/// Provide an implementation of [`add_arc`](AddArc::add_arc) that adds an arc
/// to the digraph.
///
/// ```
/// use {
///     graaf::AddArc,
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
/// let mut digraph = AdjacencyList {
///     arcs: vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
/// };
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::new(),
///     BTreeSet::from([0]),
/// ]));
/// ```
pub trait AddArc {
    /// Add an arc from `u` to `v` to the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Panics
    ///
    /// * Should panic if `u` equals `v`.
    /// * Should panic if `u` isn't in the digraph.
    /// * Should panic if `v` isn't in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.arcs().eq([(0, 1), (0, 2), (2, 0)]));
    /// ```
    fn add_arc(&mut self, u: usize, v: usize);
}
