//! Add an arc to a weighted digraph.
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

/// Add an arc to a weighted digraph.
///
/// # How can I implement `AddArcWeighted`?
///
/// Provide an implementation of `add_arc_weighted` that adds an arc from `s`
/// to `t` to the digraph with weight `w`.
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
///     fn add_arc_weighted(&mut self, s: usize, t: usize, w: isize) {
///         self.arcs[s].insert(t, w);
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
    /// Adds an arc from `s` to `t` with weight `w` to the digraph.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    /// * `w`: The weight of the arc.
    fn add_arc_weighted(&mut self, s: usize, t: usize, w: W);
}
