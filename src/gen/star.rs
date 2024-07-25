//! Generate star digraphs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Star,
//!     op::Arcs,
//! };
//!
//! // 0 -> {}
//!
//! assert!(Digraph::star(1).arcs().eq([]));
//!
//! // 0 -> {1}
//! // 1 -> {0}
//!
//! assert!(Digraph::star(2).arcs().eq([(0, 1), (1, 0)]));
//!
//! // 0 -> {1, 2}
//! // 1 -> {0}
//! // 2 -> {0}
//!
//! assert!(Digraph::star(3).arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
//! ```

use crate::{
    gen::Empty,
    op::AddArc,
};

/// Generate star digraphs.
///
/// # How can I implement `Star`?
///
/// Provide an implementation of `star` that generates a star digraph of a
/// given `order` OR implement `AddArc` and `Empty`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Empty,
///             Star,
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
/// // 0 -> {1, 2}
/// // 1 -> {0}
/// // 2 -> {0}
///
/// let digraph = Digraph::star(3);
///
/// assert!(digraph.arcs.iter().eq(&[
///     BTreeSet::from([1, 2]),
///     BTreeSet::from([0]),
///     BTreeSet::from([0])
/// ]));
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::Star,
///     op::Arcs,
/// };
///
/// // 0 -> {0}
///
/// assert!(Digraph::star(1).arcs().eq([]));
///
/// // 0 -> {1}
/// // 1 -> {0}
///
/// assert!(Digraph::star(2).arcs().eq([(0, 1), (1, 0)]));
///
/// // 0 -> {1, 2}
/// // 1 -> {0}
/// // 2 -> {0}
///
/// assert!(Digraph::star(3).arcs().eq([(0, 1), (0, 2), (1, 0), (2, 0)]));
/// ```
pub trait Star {
    /// Generates a star digraph.
    ///
    /// # Arguments
    ///
    /// * `order` - The number of vertices in the digraph.
    #[must_use]
    fn star(order: usize) -> Self;
}

impl<D> Star for D
where
    D: AddArc + Empty,
{
    /// # Panics
    ///
    /// Panics if `order` is zero.
    fn star(order: usize) -> Self {
        let mut digraph = D::empty(order);

        for u in 1..order {
            digraph.add_arc(0, u);
            digraph.add_arc(u, 0);
        }

        digraph
    }
}
