//! Return the outdegree sequence of a digraph.
//!
//! The outdegree sequence is an iterator over the outdegrees of the vertices of
//! a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArc,
//!         OutdegreeSequence,
//!     },
//! };
//!
//! // 0 -> {1, 2}
//! // 1 -> {2}
//! // 2 -> {0}
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
//! ```

use super::{
    Outdegree,
    Vertices,
};

/// Return the outdegree sequence of the digraph.
///
/// The outdegree sequence is a list of the outdegrees of the vertices in the
/// digraph.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Empty,
///     op::{
///         AddArc,
///         OutdegreeSequence,
///     },
/// };
///
/// // 0 -> {1, 2}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(0, 2);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
///
/// assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
/// ```
pub trait OutdegreeSequence {
    /// Returns the outdegree sequence of the digraph.
    #[must_use]
    fn outdegree_sequence(&self) -> impl Iterator<Item = usize>;
}

impl<D> OutdegreeSequence for D
where
    D: Outdegree + Vertices,
{
    fn outdegree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.outdegree(v))
    }
}
