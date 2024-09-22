//! Add an arc to an arc-weighted digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     ArcsWeighted,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<isize>::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(0, 2, 1);
//! digraph.add_arc_weighted(1, 2, -3);
//!
//! assert!(digraph
//!     .arcs_weighted()
//!     .eq([(0, 1, &2), (0, 2, &1), (1, 2, &-3)]));
//! ```

/// Add an arc to an arc-weighted digraph.
///
/// # Implementing [`AddArcWeighted`] for a custom type
///
/// Provide an implementation of
/// [`add_arc_weighted`](AddArcWeighted::add_arc_weighted) that adds a weighted
/// arc to the digraph.
///
/// ```
/// use {
///     graaf::AddArcWeighted,
///     std::collections::BTreeMap,
/// };
///
/// struct AdjacencyListWeighted {
///     arcs: Vec<BTreeMap<usize, isize>>,
/// }
///
/// impl AddArcWeighted for AdjacencyListWeighted {
///     type Weight = isize;
///
///     fn add_arc_weighted(&mut self, u: usize, v: usize, w: Self::Weight) {
///         self.arcs[u].insert(v, w);
///     }
/// }
///
/// let mut digraph = AdjacencyListWeighted {
///     arcs: vec![BTreeMap::new(); 3],
/// };
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(0, 2, 1);
/// digraph.add_arc_weighted(1, 2, -3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeMap::from([(1, 2), (2, 1)]),
///     BTreeMap::from([(2, -3)]),
///     BTreeMap::new(),
/// ]));
/// ```
pub trait AddArcWeighted {
    /// The weight of an arc.
    type Weight;

    /// Add an arc from `u` to `v` with weight `w` to the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    /// * `w`: The arc's weight.
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
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     ArcsWeighted,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::<isize>::empty(3);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 1);
    /// digraph.add_arc_weighted(1, 2, -3);
    ///
    /// assert!(digraph
    ///     .arcs_weighted()
    ///     .eq([(0, 1, &2), (0, 2, &1), (1, 2, &-3)]));
    /// ```
    fn add_arc_weighted(&mut self, u: usize, v: usize, w: Self::Weight);
}
