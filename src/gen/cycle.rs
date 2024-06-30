#![doc(alias = "circular")]
//! Generate directed cycle digraphs.
//!
//! Cycle graphs are also known as circular graphs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Cycle,
//!     op::Arcs,
//! };
//!
//! // 0 -> {0}
//!
//! assert!(Digraph::cycle(1).arcs().eq([(0, 0)]));
//!
//! // 0 -> {1}
//! // 1 -> {0}
//!
//! assert!(Digraph::cycle(2).arcs().eq([(0, 1), (1, 0)]));
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {0}
//!
//! assert!(Digraph::cycle(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate directed cycle digraphs.
///
/// # How can I implement `Cycle`?
///
/// Provide an implementation of `cycle` that generates a cycle digraph with `v`
/// vertices OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Cycle,
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
///         Self {
///             arcs: vec![BTreeSet::new(); v],
///         }
///     }
/// }
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// let digraph = Digraph::cycle(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1]),
///     BTreeSet::from([2]),
///     BTreeSet::from([0])
/// ]));
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
/// // 0 -> {0}
///
/// assert!(Digraph::cycle(1).arcs().eq([(0, 0)]));
///
/// // 0 -> {1}
/// // 1 -> {0}
///
/// assert!(Digraph::cycle(2).arcs().eq([(0, 1), (1, 0)]));
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// assert!(Digraph::cycle(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait Cycle {
    /// Generates a cycle digraph.
    ///
    /// # Arguments
    ///
    /// * `v` - The number of vertices in the digraph
    #[must_use]
    fn cycle(v: usize) -> Self;
}

impl<D> Cycle for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `V` is zero.
    fn cycle(v: usize) -> Self {
        let mut digraph = D::empty(v);

        for i in 0..v - 1 {
            digraph.add_arc(i, i + 1);
        }

        digraph.add_arc(v - 1, 0);

        digraph
    }
}
