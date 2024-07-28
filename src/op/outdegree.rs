//! Get the outdegree of a vertex in a digraph.
//!
//! The outdegree is the number of arcs incident out of a vertex.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Outdegree,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert_eq!(digraph.outdegree(0), 2);
//! assert_eq!(digraph.outdegree(1), 1);
//! assert_eq!(digraph.outdegree(2), 0);
//! ```
#![doc(alias = "out_degree")]

/// Get the outdegree of a vertex in a digraph.
///
/// # How can I implement `Outdegree`?
///
/// Provide an implementation of `outdegree` that returns the outdegree of the
/// target vertex. The implementation should not panic if the vertex is not in
/// the digraph.
///
/// ```
/// use {
///     graaf::op::Outdegree,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Outdegree for Digraph {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs.get(u).map_or(0, BTreeSet::len)
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
///         Outdegree,
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
/// assert_eq!(digraph.outdegree(0), 2);
/// assert_eq!(digraph.outdegree(1), 1);
/// assert_eq!(digraph.outdegree(2), 1);
/// ```
pub trait Outdegree {
    /// Returns the outdegree of a vertex in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    #[doc(alias = "out_degree")]
    #[must_use]
    fn outdegree(&self, u: usize) -> usize;

    /// Returns whether a vertex is a sink of the digraph.
    ///
    /// A sink is a vertex with no out-neighbors.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     adjacency_list::Digraph,
    ///     gen::Empty,
    ///     op::{
    ///         AddArc,
    ///         Outdegree,
    ///     },
    /// };
    ///
    /// let mut digraph = Digraph::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(!digraph.is_sink(0));
    /// assert!(!digraph.is_sink(1));
    /// assert!(digraph.is_sink(2));
    /// ```
    #[must_use]
    fn is_sink(&self, u: usize) -> bool {
        self.outdegree(u) == 0
    }
}
