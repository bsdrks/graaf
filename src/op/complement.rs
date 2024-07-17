//! Generate the complement of a digraph.
//!
//! The complement of a digraph contains all arcs not present in the original
//! digraph.
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
//!         Complement,
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
//! // 0 -> {2, 3}
//! // 1 -> {0, 3}
//! // 2 -> {0, 1}
//! // 3 -> {1, 2}
//!
//! let converse = digraph.complement();
//!
//! assert!(converse.arcs().eq([
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (1, 3),
//!     (2, 0),
//!     (2, 1),
//!     (3, 1),
//!     (3, 2)
//! ]));
//! ```

use crate::gen::Empty;

use super::{
    AddArc,
    HasArc,
    Order,
};

/// Generate the complement of a digraph.
///
/// # How do I implement `Complement`?
///
/// Provide an implementation of `complement` that returns a digraph with all
/// arcs not present in the original digraph.
///
/// ```
/// use {
///     graaf::op::Complement,
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Complement for Digraph {
///     fn complement(&self) -> Self {
///         let order = self.arcs.len();
///         let mut arcs = vec![BTreeSet::<usize>::new(); order];
///
///         for u in 0..order {
///             for v in u + 1..order {
///                 if !self.arcs[u].contains(&v) {
///                     arcs[u].insert(v);
///                 }
///
///                 if !self.arcs[v].contains(&u) {
///                     arcs[v].insert(u);
///                 }
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
/// // 0 -> {2, 3}
/// // 1 -> {0, 3}
/// // 2 -> {0, 1}
/// // 3 -> {1, 2}
///
/// let complement = digraph.complement();
///
/// assert_eq!(
///     complement.arcs,
///     vec![
///         BTreeSet::from([2, 3]),
///         BTreeSet::from([0, 3]),
///         BTreeSet::from([0, 1]),
///         BTreeSet::from([1, 2]),
///     ],
/// );
/// ```
pub trait Complement {
    /// Generates the complement of the digraph.
    #[must_use]
    fn complement(&self) -> Self;
}

impl<D> Complement for D
where
    D: AddArc + Empty + HasArc + Order,
{
    fn complement(&self) -> Self {
        let order = self.order();
        let mut digraph = D::empty(order);

        for u in 0..order {
            for v in u + 1..order {
                if !self.has_arc(u, v) {
                    digraph.add_arc(u, v);
                }

                if !self.has_arc(v, u) {
                    digraph.add_arc(v, u);
                }
            }
        }

        digraph
    }
}
