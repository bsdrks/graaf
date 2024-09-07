//! Return a digraph's sinks.
//!
//! A sink is a vertex with no out-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         Sinks,
//!     },
//! };
//!
//! let mut digraph = Digraph::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//!
//! assert!(digraph.sinks().eq([2, 3]));
//! ```

use super::{
    Outdegree,
    Vertices,
};

/// Return a digraph's sinks.
///
/// # Implementing `Sinks`
///
/// Provide an implementation of `sinks` that returns an iterator over the
/// sinks in the digraph OR implement `Outdegree` and `Vertices`.
///
/// ```
/// use {
///     graaf::op::{
///         Outdegree,
///         Sinks,
///         Vertices,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Outdegree for Digraph {
///     fn outdegree(&self, u: usize) -> usize {
///         self.arcs[u].len()
///     }
/// }
///
/// impl Vertices for Digraph {
///     fn vertices(&self) -> impl Iterator<Item = usize> {
///         0..self.arcs.len()
///     }
/// }
///
/// let mut digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::new(),
///         BTreeSet::new(),
///     ],
/// };
///
/// assert!(digraph.sinks().eq([2, 3]));
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
///         Sinks,
///     },
/// };
///
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
///
/// assert!(digraph.sinks().eq([2, 3]));
/// ```
pub trait Sinks {
    /// Returns an iterator over the sinks in the digraph.
    fn sinks(&self) -> impl Iterator<Item = usize>;
}

impl<D> Sinks for D
where
    D: Outdegree + Vertices,
{
    fn sinks(&self) -> impl Iterator<Item = usize> {
        self.vertices().filter(move |&u| self.is_sink(u))
    }
}
