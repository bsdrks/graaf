//! Return a digraph's arcs and their weights.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArcWeighted,
//!         ArcsWeighted,
//!     },
//! };
//!
//! // 0 -> {1 (2)}
//! // 1 -> {2 (3)}
//! // 2 -> {0 (4)}
//!
//! let mut digraph = Digraph::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(1, 2, 3);
//! digraph.add_arc_weighted(2, 0, 4);
//!
//! assert!(digraph
//!     .arcs_weighted()
//!     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
//! ```

/// Return a digraph's arcs and their weights.
///
/// # How can I implement `ArcsWeighted`?
///
/// Provide an implementation of `arcs_weighted` that returns an iterator over
/// the arcs in a digraph with their weights.
///
/// ```
/// use graaf::op::ArcsWeighted;
///
/// struct Digraph {
///     arcs: Vec<(usize, usize, usize)>,
/// }
///
/// impl ArcsWeighted<usize> for Digraph {
///     fn arcs_weighted<'a>(
///         &'a self,
///     ) -> impl Iterator<Item = (usize, usize, &'a usize)>
///     where
///         usize: 'a,
///     {
///         self.arcs.iter().map(|&(u, v, ref w)| (u, v, w))
///     }
/// }
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list_weighted::Digraph,
///     gen::Empty,
///     op::{
///         AddArcWeighted,
///         ArcsWeighted,
///     },
/// };
///
/// // 0 -> {1 (2)}
/// // 1 -> {2 (3)}
/// // 2 -> {0 (4)}
///
/// let mut digraph = Digraph::empty(3);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(1, 2, 3);
/// digraph.add_arc_weighted(2, 0, 4);
///
/// assert!(digraph
///     .arcs_weighted()
///     .eq([(0, 1, &2), (1, 2, &3), (2, 0, &4)]));
/// ```
pub trait ArcsWeighted<W> {
    /// Returns an iterator over the weighted arcs in the digraph.
    #[must_use]
    fn arcs_weighted<'a>(
        &'a self,
    ) -> impl Iterator<Item = (usize, usize, &'a W)>
    where
        W: 'a;
}
