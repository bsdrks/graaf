//! Generate complete digraphs.
//!
//! In a complete digraph, an arc connects every ordered pair of vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Complete,
//!     op::Arcs,
//! };
//!
//! // 0 -> {}
//!
//! assert!(Digraph::complete(1).arcs().eq([]));
//!
//! // 0 -> {1}
//! // 1 -> {0}
//!
//! assert!(Digraph::complete(2).arcs().eq([(0, 1), (1, 0)]));
//!
//! // 0 -> {1, 2}
//! // 1 -> {0, 2}
//! // 2 -> {0, 1}
//!
//! assert!(Digraph::complete(3)
//!     .arcs()
//!     .eq([(0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]));
//! ```

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate complete digraphs.
///
/// # How can I implement `Complete`?
///
/// Provide an implementation of `complete` that generates a complete digraph
/// with `v` vertices OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Complete,
///             Empty,
///         },
///         op::AddArc,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl AddArc for Digraph {
///     fn add_arc(&mut self, s: usize, t: usize) {
///         self.arcs[s].insert(t);
///     }
/// }
///
/// impl Empty for Digraph {
///     fn empty(v: usize) -> Self {
///         Digraph {
///             arcs: vec![BTreeSet::new(); v],
///         }
///     }
/// }
///
/// // 0 -> {1, 2}
/// // 1 -> {0, 2}
/// // 2 -> {0, 1}
///
/// let digraph = Digraph::complete(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0, 2]),
///     BTreeSet::from([0, 1]),
/// ]));
/// ```
pub trait Complete {
    /// Generates a complete digraph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the digraph
    fn complete(v: usize) -> Self;
}

impl<D> Complete for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `v` is zero.
    fn complete(v: usize) -> Self {
        let mut digraph = D::empty(v);

        for s in 0..v {
            for t in (s + 1)..v {
                digraph.add_arc(s, t);
                digraph.add_arc(t, s);
            }
        }

        digraph
    }
}
