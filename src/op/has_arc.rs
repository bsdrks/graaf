//! Check whether a digraph contains an edge.
//!
//! To check whether an arc exists from `u` to `v` and from `v` to `u`, see
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

/// Check whether a digraph contains an arc.
///
/// # Implementing `HasArc`
///
/// Provide an implementation of `has_arc` that returns `true` if there is an
/// arc from `u` to `v`.
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
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs.get(u).map_or(false, |set| set.contains(&v))
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0]),
///         BTreeSet::new(),
///     ],
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
    /// Checks whether an arc exists from `u` to `v` in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Panics
    ///
    /// `has_arc` may not panic if `u` and `v` are out of bounds.
    #[must_use]
    fn has_arc(&self, u: usize, v: usize) -> bool;
}
