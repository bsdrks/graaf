//! Generate the converse of a digraph.
//!
//! The converse of a digraph is a digraph with all arcs reversed.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::Cycle,
//!     op::{
//!         AddArc,
//!         Arcs,
//!         Converse,
//!     },
//! };
//!
//! let digraph = Digraph::cycle(4);
//! let converse = digraph.converse();
//!
//! assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
//! ```

/// Generate the converse of a digraph.
///
/// # How do I implement `Converse`?
///
/// Provide an implementation of `converse` that returns a digraph with all arcs
/// reversed.
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
///         let v = self.arcs.len();
///         let mut arcs = vec![BTreeSet::<usize>::new(); v];
///
///         for (s, set) in self.arcs.iter().enumerate() {
///             for &t in set {
///                 arcs[t].insert(s);
///             }
///         }
///
///         Self { arcs }
///     }
/// }
///
/// let digraph = Digraph {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// let converse = digraph.converse();
///
/// assert_eq!(
///     converse.arcs,
///     vec![
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///         BTreeSet::from([1]),
///     ],
/// );
/// ```
pub trait Converse {
    /// Generates the converse of the digraph
    #[must_use]
    fn converse(&self) -> Self;
}
