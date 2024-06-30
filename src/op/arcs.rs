//! Iterate over the arcs in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Cycle,
//!     op::{
//!         AddArc,
//!         Arcs,
//!     },
//! };
//!
//! let digraph = Digraph::cycle(3);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```

/// Iterate the arcs in a digraph.
///
/// # How can I implement `Arcs`?
///
/// Provide an implementation of `arcs` that returns an iterator over
/// all arcs in a digraph.
///
/// ```
/// use graaf::op::Arcs;
///
/// struct Digraph {
///     arcs: Vec<(usize, usize)>,
/// }
///
/// impl Arcs for Digraph {
///     fn arcs(&self) -> impl Iterator<Item = (usize, usize)> {
///         self.arcs.iter().copied()
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Cycle,
///     op::Arcs,
/// };
///
/// let digraph = Digraph::cycle(3);
///
/// assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait Arcs {
    /// Returns an iterator over the arcs in the digraph.
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)>;
}
