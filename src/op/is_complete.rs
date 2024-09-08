//! Check whether a digraph is complete.
//!
//! A digraph is complete if, for every pair `u`, `v` of distinct vertices,
//! there is an arc from `u` to `v` and an arc from `v` to `u`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsComplete,
//!     RandomTournament,
//! };
//!
//! assert!(AdjacencyList::complete(3).is_complete());
//! assert!(!AdjacencyList::circuit(3).is_complete());
//! assert!(!AdjacencyList::empty(3).is_complete());
//! assert!(!AdjacencyList::random_tournament(3, 0).is_complete());
//! ```

use crate::{
    HasEdge,
    Order,
};

/// Check whether a digraph is complete.
///
/// # Implementing `IsComplete`
///
/// Provide an implementation of `is_complete` that returns whether the digraph
/// is complete OR implement `HasEdge` and `Order`.
///
/// ```
/// use {
///     graaf::{
///         Circuit,
///         Complete,
///         Empty,
///         HasArc,
///         IsComplete,
///         Order,
///         RandomTournament,
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
///         self.arcs.get(u).map_or(false, |set| set.contains(&v))
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
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0, 1])
///     ]
/// }
/// .is_complete());
///
/// assert!(!AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0])
///     ]
/// }
/// .is_complete());
///
/// assert!(!AdjacencyList {
///     arcs: vec![BTreeSet::new(); 3]
/// }
/// .is_complete());
/// ```
pub trait IsComplete {
    /// Determines whether the digraph is complete.
    #[must_use]
    fn is_complete(&self) -> bool;
}

impl<D> IsComplete for D
where
    D: HasEdge + Order,
{
    fn is_complete(&self) -> bool {
        let order = self.order();

        for u in 0..order {
            for v in (u + 1)..order {
                if !self.has_edge(u, v) {
                    return false;
                }
            }
        }

        true
    }
}
