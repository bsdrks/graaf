//! Add an arc to a digraph.
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
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {}
//! // 2 -> {0}
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (2, 0)]));
//! ```

/// Add an arc to a digraph.
///
/// # How can I implement `AddArc`?
///
/// Provide an implementation of `add_arc` that adds an arc to the digraph.
///
/// ```
/// use {
///     graaf::op::AddArc,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl AddArc for Digraph {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {}
/// // 2 -> {0}
///
/// let mut digraph = Digraph {
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
///     },
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {}
/// // 2 -> {0}
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(digraph.arcs().eq([(0, 1), (0, 2), (2, 0)]));
/// ```
///
/// [`HasArc`]: crate::op::HasArc
/// [`RemoveArc`]: crate::op::RemoveArc
pub trait AddArc {
    /// Adds an arc from `u` to `v` to the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Panics
    ///
    /// * Should panic if `u` equals `v`.
    /// * Should panic if `u` is out of bounds.
    /// * Should panic if `v` is out of bounds.
    fn add_arc(&mut self, u: usize, v: usize);
}
