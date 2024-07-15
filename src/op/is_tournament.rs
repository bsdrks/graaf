//! Determine whether a digraph is a tournament.
//!
//! A tournament is a digraph in which there is an arc between every unordered
//! pair of distinct vertices.
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
//!     op::IsTournament,
//! };
//!
//! assert!(!Digraph::empty(3).is_tournament());
//! assert!(!Digraph::complete(3).is_tournament());
//! assert!(Digraph::cycle(3).is_tournament());
//! assert!(Digraph::random_tournament(3).is_tournament());
//! ```

use super::{
    HasArc,
    Order,
};

/// Determine whether a digraph is a tournament.
///
/// # How can I implement `IsTournament`?
///
/// Provide an implementation of `is_tournament` that returns whether the
/// digraph is a tournament OR implement `HasArc` and `Order`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Complete,
///             Cycle,
///             Empty,
///         },
///         op::{
///             HasArc,
///             IsTournament,
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
///         self.arcs[u].contains(&v)
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
///     arcs: vec![BTreeSet::from([1, 2]), BTreeSet::from([2]), BTreeSet::new()]
/// }
/// .is_tournament());
///
/// assert!(!Digraph {
///     arcs: vec![BTreeSet::from([1, 2]), BTreeSet::new(), BTreeSet::new()]
/// }
/// .is_tournament());
///
/// assert!(!Digraph {
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
///     adjacency_list::Digraph,
///     gen::{
///         Complete,
///         Cycle,
///         Empty,
///         RandomTournament,
///     },
///     op::IsTournament,
/// };
///
/// assert!(!Digraph::empty(3).is_tournament());
/// assert!(!Digraph::complete(3).is_tournament());
/// assert!(Digraph::cycle(3).is_tournament());
/// assert!(Digraph::random_tournament(3).is_tournament());
/// ```
pub trait IsTournament {
    /// Returns whether the digraph is a tournament.
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
                if usize::from(self.has_arc(u, v)) + usize::from(self.has_arc(v, u)) != 1 {
                    return false;
                }
            }
        }

        true
    }
}
