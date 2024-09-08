//! Check whether a digraph is a tournament.
//!
//! A tournament is a digraph in which there is one arc between every unordered
//! pair of distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsTournament,
//!     RandomTournament,
//! };
//!
//! assert!(!AdjacencyList::empty(3).is_tournament());
//! assert!(!AdjacencyList::complete(3).is_tournament());
//! assert!(AdjacencyList::circuit(3).is_tournament());
//! assert!(AdjacencyList::random_tournament(3, 0).is_tournament());
//! ```

use crate::{
    HasArc,
    Order,
};

/// Check whether a digraph is a tournament.
///
/// A tournament is a digraph in which there is one arc between every unordered
/// pair of distinct vertices.
///
/// # Implementing `IsTournament`
///
/// Provide an implementation of `is_tournament` that returns whether the
/// digraph is a tournament OR implement `HasArc` and `Order`.
///
/// ```
/// use {
///     graaf::{
///         Circuit,
///         Complete,
///         Empty,
///         HasArc,
///         IsTournament,
///         Order,
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for AdjacencyList {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs[u].contains(&v)
///     }
/// }
///
/// impl Order for AdjacencyList {
///     fn order(&self) -> usize {
///         self.arcs.len()
///     }
/// }
///
/// assert!(AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::new()
///     ]
/// }
/// .is_tournament());
///
/// assert!(!AdjacencyList {
///     arcs: vec![BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
/// }
/// .is_tournament());
///
/// assert!(!AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0]),
///     ]
/// }
/// .is_tournament());
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     AdjacencyList,
///     Circuit,
///     Complete,
///     Empty,
///     IsTournament,
///     RandomTournament,
/// };
///
/// assert!(!AdjacencyList::empty(3).is_tournament());
/// assert!(!AdjacencyList::complete(3).is_tournament());
/// assert!(AdjacencyList::circuit(3).is_tournament());
/// assert!(AdjacencyList::random_tournament(3, 0).is_tournament());
/// ```
pub trait IsTournament {
    /// Checks whether the digraph is a tournament.
    #[must_use]
    fn is_tournament(&self) -> bool;
}

impl<D> IsTournament for D
where
    D: HasArc + Order,
{
    fn is_tournament(&self) -> bool {
        let order = self.order();

        for u in 0..order {
            for v in (u + 1)..order {
                if usize::from(self.has_arc(u, v))
                    + usize::from(self.has_arc(v, u))
                    != 1
                {
                    return false;
                }
            }
        }

        true
    }
}
