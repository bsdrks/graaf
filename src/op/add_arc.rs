//! Add an arc to an unweighted digraph.
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
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (2, 0)]));
//! ```

/// Add an arc to an unweighted digraph.
///
/// # How can I implement `AddArc`?
///
/// Provide an implementation of `add_arc` that adds an unweighted arc from
/// `u` to `v` to the digraph.
///
/// ```
/// use graaf::op::AddArc;
///
/// struct Digraph {
///     arcs: Vec<Vec<usize>>,
/// }
///
/// impl AddArc for Digraph {
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].push(v);
///     }
/// }
///
/// let mut digraph = Digraph {
///     arcs: vec![Vec::new(); 3],
/// };
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(digraph.arcs.iter().eq(&[vec![1, 2], Vec::new(), vec![0]]));
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
    /// * `u`: The head vertex.
    /// * `v`: The tail vertex.
    fn add_arc(&mut self, u: usize, v: usize);
}
