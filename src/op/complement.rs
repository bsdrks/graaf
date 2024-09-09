//! Return a digraph's complement.
//!
//! The complement of a digraph contains all arcs not present in the original
//! digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//!     Complement,
//! };
//!
//! let digraph = AdjacencyList::circuit(4);
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

use crate::{
    AddArc,
    Empty,
    HasArc,
    Order,
};

/// Return a digraph's complement.
///
/// # How do I implement `Complement`
///
/// Provide an implementation of `complement` that returns a digraph with all
/// arcs not present in the original digraph.
///
/// ```
/// use {
///     graaf::Complement,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Complement for AdjacencyList {
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
/// let digraph = AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([3]),
///         BTreeSet::from([0]),
///     ],
/// };
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
    /// Generate the complement of the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    ///     Complement,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(4);
    /// let converse = digraph.complement();
    ///
    /// assert!(converse.arcs().eq([
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 3),
    ///     (2, 0),
    ///     (2, 1),
    ///     (3, 1),
    ///     (3, 2)
    /// ]));
    /// ```
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
