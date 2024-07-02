//! Check if an arc exists from one vertex to another.
//!
//! To check if an arc exists from `s` to `t` and from `t` to `s`, see
//! [`HasEdge`].
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         HasArc,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert!(digraph.has_arc(0, 1));
//! assert!(digraph.has_arc(0, 2));
//! assert!(digraph.has_arc(1, 0));
//! assert!(!digraph.has_arc(1, 2));
//! assert!(!digraph.has_arc(2, 0));
//! assert!(!digraph.has_arc(2, 1));
//! ```
//!
//! [`HasEdge`]: crate::op::HasEdge

/// Check if an arc exists from one vertex to another.
///
/// # How can I implement `HasArc`?
///
/// Provide an implementation of `has_arc` that returns `true` if there is an
/// arc from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::HasArc,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, s: usize, t: usize) -> bool {
///         self.arcs.get(s).map_or(false, |set| set.contains(&t))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![BTreeSet::from([1, 2]), BTreeSet::from([0]), BTreeSet::new()],
/// };
///
/// assert!(digraph.has_arc(0, 1));
/// assert!(digraph.has_arc(0, 2));
/// assert!(digraph.has_arc(1, 0));
/// assert!(!digraph.has_arc(1, 2));
/// assert!(!digraph.has_arc(2, 0));
/// assert!(!digraph.has_arc(2, 1));
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
///         HasArc,
///     },
/// };
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 0);
///
/// assert!(digraph.has_arc(0, 1));
/// assert!(digraph.has_arc(0, 2));
/// assert!(digraph.has_arc(1, 0));
/// assert!(!digraph.has_arc(1, 2));
/// assert!(!digraph.has_arc(2, 0));
/// assert!(!digraph.has_arc(2, 1));
/// ```
///
/// [`AddArc`]: crate::op::AddArc
/// [`AddArcWeighted`]: crate::op::AddArcWeighted
pub trait HasArc {
    /// Returns whether an arc exists from `s` to `t` in the digraph.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    ///
    /// # Panics
    ///
    /// Panics if `s` or `t` are out of bounds.
    fn has_arc(&self, s: usize, t: usize) -> bool;
}
