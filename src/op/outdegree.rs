#![doc(alias = "out_degree")]
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

/// Get the outdegree of a vertex in a digraph.
///
/// # How can I implement `Outdegree`?
///
/// Provide an implementation of `outdegree` that returns the outdegree of the
/// target vertex. The implementation should not panic if the vertex is not in
/// the digraph.
///
/// ```
/// use graaf::op::Outdegree;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl Outdegree for Digraph {
///     fn outdegree(&self, s: usize) -> usize {
///         self.arcs.get(s).map_or(0, Vec::len)
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
    fn outdegree(&self, u: usize) -> usize;

    /// Returns whether a vertex is a sink of the digraph.
    ///
    /// A sink is a vertex with an outdegree of 0.
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
    fn is_sink(&self, u: usize) -> bool {
        self.outdegree(u) == 0
    }
}
