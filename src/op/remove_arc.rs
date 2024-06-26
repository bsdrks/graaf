//! Remove an arc from a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Arcs,
//!         RemoveArc,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(2, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 1)]));
//! assert!(digraph.remove_arc(0, 1));
//! assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
//! assert!(digraph.remove_arc(0, 2));
//! assert!(digraph.arcs().eq([(1, 0), (2, 1)]));
//! assert!(digraph.remove_arc(1, 0));
//! assert!(digraph.arcs().eq([(2, 1)]));
//! assert!(digraph.remove_arc(2, 1));
//! assert!(digraph.arcs().eq([]));
//! ```

/// Remove an arc from a digraph.
///
/// # How can I implement `RemoveArc`?
///
/// Provide an implementation of `remove_arc` that removes the arc from `s` to
/// `t` from a digraph. Return whether the arc was removed.
///
/// ```
/// use {
///     graaf::op::RemoveArc,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl RemoveArc for Digraph {
///     fn remove_arc(&mut self, s: usize, t: usize) -> bool {
///         self.arcs[s].remove(&t)
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         Arcs,
///         RemoveArc,
///     },
/// };
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(2, 1);
///
/// assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 1)]));
/// assert!(digraph.remove_arc(0, 1));
/// assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
/// assert!(digraph.remove_arc(0, 2));
/// assert!(digraph.arcs().eq([(1, 0), (2, 1)]));
/// assert!(digraph.remove_arc(1, 0));
/// assert!(digraph.arcs().eq([(2, 1)]));
/// assert!(digraph.remove_arc(2, 1));
/// assert!(digraph.arcs().eq([]));
/// ```
///
/// [`AddArc`]: crate::op::AddArc
/// [`AddArcWeighted`]: crate::op::AddArcWeighted
pub trait RemoveArc {
    /// Removes the arc from `s` to `t` from the digraph. Returns whether the
    /// arc was removed.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    fn remove_arc(&mut self, s: usize, t: usize) -> bool;
}
