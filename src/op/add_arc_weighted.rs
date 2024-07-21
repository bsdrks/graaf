//! Add an arc to an arc-weighted digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArcWeighted,
//!         ArcsWeighted,
//!     },
//! };
//!
//! let mut digraph = Digraph::<isize>::empty(3);
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
/// # How can I implement `AddArcWeighted`?
///
/// Provide an implementation of `add_arc_weighted` that adds a weighted arc to
/// the digraph.
///
/// ```
/// use {
///     graaf::op::AddArcWeighted,
///     std::collections::BTreeMap,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeMap<usize, isize>>,
/// }
///
/// impl AddArcWeighted<isize> for Digraph {
///     fn add_arc_weighted(&mut self, u: usize, v: usize, w: isize) {
///         self.arcs[u].insert(v, w);
///     }
/// }
///
/// let mut digraph = Digraph {
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
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     gen::Empty,
///     op::{
///         AddArcWeighted,
///         ArcsWeighted,
///     },
/// };
///
/// let mut digraph = Digraph::<isize>::empty(3);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(0, 2, 1);
/// digraph.add_arc_weighted(1, 2, -3);
///
/// assert!(digraph
///     .arcs_weighted()
///     .eq([(0, 1, &2), (0, 2, &1), (1, 2, &-3)]));
/// ```
///
/// [`HasArc`]: crate::op::HasArc
/// [`RemoveArc`]: crate::op::RemoveArc
pub trait AddArcWeighted<W> {
    /// Adds an arc from `u` to `v` with weight `w` to the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    /// * `w`: The weight of the arc.
    ///
    /// # Panics
    ///
    /// * Should panic if `u` is out of bounds.
    /// * Should panic if `v` is out of bounds.
    /// * Should panic if `u` equals `v`.
    fn add_arc_weighted(&mut self, u: usize, v: usize, w: W);
}
