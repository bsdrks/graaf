#![doc(alias = "circular")]
//! Generate cycle digraphs.
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

/// Generate cycle digraphs.
///
/// # How can I implement `Cycle`?
///
/// Provide an implementation of `cycle` that generates a cycle digraph with
/// `order` vertices OR implement `AddArc` and `Empty`.
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
///     fn add_arc(&mut self, u: usize, v: usize) {
///         self.arcs[u].insert(v);
///     }
/// }
///
/// impl Empty for Digraph {
///     fn empty(order: usize) -> Self {
///         Self {
///             arcs: vec![BTreeSet::new(); order],
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
    /// * `order` - The number of vertices in the digraph
    #[must_use]
    fn cycle(order: usize) -> Self;
}

impl<D> Cycle for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn cycle(order: usize) -> Self {
        let mut digraph = D::empty(order);

        for u in 0..order - 1 {
            digraph.add_arc(u, u + 1);
        }

        digraph.add_arc(order - 1, 0);

        digraph
    }
}
