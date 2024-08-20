//! Return a digraph's converse.
//!
//! The converse of a digraph is a digraph with all arcs reversed.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Circuit,
//!     op::{
//!         AddArc,
//!         Arcs,
//!         Converse,
//!     },
//! };
//!
//! // 0 -> {1}
//! // 1 -> {2}
//! // 2 -> {3}
//! // 3 -> {0}
//!
//! let digraph = Digraph::circuit(4);
//!
//! // 0 -> {3}
//! // 1 -> {0}
//! // 2 -> {1}
//! // 3 -> {2}
//!
//! let converse = digraph.converse();
//!
//! assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
//! ```

/// Return a digraph's converse.
///
/// # How do I implement `Converse`
///
/// Provide an implementation of `converse` that returns a digraph with all
/// arcs reversed.
///
/// ```
/// use {
///     graaf::op::Converse,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Converse for Digraph {
///     fn converse(&self) -> Self {
///         let order = self.arcs.len();
///         let mut arcs = vec![BTreeSet::<usize>::new(); order];
///
///         for (u, set) in self.arcs.iter().enumerate() {
///             for &v in set {
///                 arcs[v].insert(u);
///             }
///         }
///
///         Self { arcs }
///     }
/// }
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {3}
/// // 3 -> {0}
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([3]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// // 0 -> {3}
/// // 1 -> {0}
/// // 2 -> {1}
/// // 3 -> {2}
///
/// let converse = digraph.converse();
///
/// assert_eq!(
///     converse.arcs,
///     vec![
///         BTreeSet::from([3]),
///         BTreeSet::from([0]),
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///     ],
/// );
/// ```
pub trait Converse {
    /// Generates the converse of the digraph.
    #[must_use]
    fn converse(&self) -> Self;
}
