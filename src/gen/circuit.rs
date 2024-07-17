//! Generate circuit digraphs.
//!
//! A circuit is an oriented cycle.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::Arcs,
//! };
//!
//! // 0 -> {}
//!
//! assert!(Digraph::circuit(1).arcs().eq([]));
//!
//! // 0 -> {1}
//! // 1 -> {0}
//!
//! assert!(Digraph::circuit(2).arcs().eq([(0, 1), (1, 0)]));
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {0}
//!
//! assert!(Digraph::circuit(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate circuit digraphs.
///
/// # How can I implement `Circuit`?
///
/// Provide an implementation of `circuit` that generates a circuit digraph with
/// `order` vertices OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Circuit,
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
/// let digraph = Digraph::circuit(3);
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
///     gen::Circuit,
///     op::Arcs,
/// };
///
/// // 0 -> {}
///
/// assert!(Digraph::circuit(1).arcs().eq([]));
///
/// // 0 -> {1}
/// // 1 -> {0}
///
/// assert!(Digraph::circuit(2).arcs().eq([(0, 1), (1, 0)]));
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {0}
///
/// assert!(Digraph::circuit(3).arcs().eq([(0, 1), (1, 2), (2, 0)]));
/// ```
pub trait Circuit {
    /// Generates a circuit digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    #[must_use]
    fn circuit(order: usize) -> Self;
}

impl<D> Circuit for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn circuit(order: usize) -> Self {
        let mut digraph = D::empty(order);

        if order == 1 {
            return digraph;
        }

        for u in 0..order - 1 {
            digraph.add_arc(u, u + 1);
        }

        digraph.add_arc(order - 1, 0);

        digraph
    }
}
