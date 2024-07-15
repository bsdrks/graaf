//! Determine whether a digraph is complete.
//!
//! A digraph is complete if, for every pair `u`, `v` of distinct vertices,
//! there is an arc from `u` to `v` and an arc from `v` to `u`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::{
//!         Complete,
//!         Cycle,
//!         Empty,
//!         RandomTournament,
//!     },
//!     op::IsComplete,
//! };
//!
//! assert!(Digraph::complete(3).is_complete());
//! assert!(!Digraph::cycle(3).is_complete());
//! assert!(!Digraph::empty(3).is_complete());
//! assert!(!Digraph::random_tournament(3).is_complete());
//! ```

use super::{
    HasEdge,
    Order,
};

/// Determine whether a digraph is complete.
///
/// # How can I implement `IsComplete`?
///
/// Provide an implementation of `is_complete` that whether the digraph is
/// complete OR implement `HasEdge` and `Order`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Complete,
///             Cycle,
///             Empty,
///             RandomTournament,
///         },
///         op::{
///             HasArc,
///             IsComplete,
///             Order,
///         },
///     },
///     std::collections::BTreeSet,
/// };
///
/// struct Digraph {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl HasArc for Digraph {
///     fn has_arc(&self, u: usize, v: usize) -> bool {
///         self.arcs.get(u).map_or(false, |set| set.contains(&v))
///     }
/// }
///
/// impl Order for Digraph {
///     fn order(&self) -> usize {
///         self.arcs.len()
///     }
/// }
///
/// assert!(Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0, 1])
///     ]
/// }
/// .is_complete());
///
/// assert!(!Digraph {
///     arcs: vec![
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///         BTreeSet::from([0])
///     ]
/// }
/// .is_complete());
///
/// assert!(!Digraph {
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
