//! Get the weight of an arc.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list_weighted::Digraph,
//!     gen::Empty,
//!     op::{
//!         AddArcWeighted,
//!         ArcWeight,
//!     },
//! };
//!
//! let mut digraph = Digraph::<usize>::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(0, 2, 3);
//! digraph.add_arc_weighted(1, 0, 4);
//! digraph.add_arc_weighted(2, 0, 7);
//! digraph.add_arc_weighted(2, 1, 8);
//!
//! assert_eq!(digraph.arc_weight(0, 0), None);
//! assert_eq!(digraph.arc_weight(0, 1), Some(&2));
//! assert_eq!(digraph.arc_weight(0, 2), Some(&3));
//! assert_eq!(digraph.arc_weight(1, 0), Some(&4));
//! assert_eq!(digraph.arc_weight(1, 1), None);
//! assert_eq!(digraph.arc_weight(2, 0), Some(&7));
//! assert_eq!(digraph.arc_weight(2, 1), Some(&8));
//! assert_eq!(digraph.arc_weight(2, 2), None);
//! ```

/// Get the weight of an arc.
///
/// # How can I implement `ArcWeight`?
///
/// Provide an implementation of `arc_weight` that returns the weight of the
/// arc from `s` to `t`.
///
/// ```
/// use {
///     graaf::op::ArcWeight,
///     std::collections::BTreeMap,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeMap<usize, usize>>,
/// }
///
/// impl ArcWeight<usize> for Digraph {
///     fn arc_weight(&self, s: usize, t: usize) -> Option<&usize> {
///         self.arcs.get(s).and_then(|m| m.get(&t))
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
///         ArcWeight,
///     },
/// };
///
/// let mut digraph = Digraph::<usize>::empty(3);
///
/// digraph.add_arc_weighted(0, 1, 2);
/// digraph.add_arc_weighted(0, 2, 3);
/// digraph.add_arc_weighted(1, 0, 4);
/// digraph.add_arc_weighted(2, 0, 7);
/// digraph.add_arc_weighted(2, 1, 8);
///
/// assert_eq!(digraph.arc_weight(0, 0), None);
/// assert_eq!(digraph.arc_weight(0, 1), Some(&2));
/// assert_eq!(digraph.arc_weight(0, 2), Some(&3));
/// assert_eq!(digraph.arc_weight(1, 0), Some(&4));
/// assert_eq!(digraph.arc_weight(1, 1), None);
/// assert_eq!(digraph.arc_weight(2, 0), Some(&7));
/// assert_eq!(digraph.arc_weight(2, 1), Some(&8));
/// assert_eq!(digraph.arc_weight(2, 2), None);
/// ```
pub trait ArcWeight<W> {
    /// Returns the weight of the arc from `s` to `t` if it exists in the
    /// digraph.
    ///
    /// # Arguments
    ///
    /// * `s`: The head vertex.
    /// * `t`: The tail vertex.
    fn arc_weight(&self, s: usize, t: usize) -> Option<&W>;
}
