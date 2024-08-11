//! Check whether a digraph is semicomplete.
//!
//! A digraph is semicomplete if there is an arc between every unordered pair
//! `u`, `v` of distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     adjacency_list::Digraph,
//!     gen::{
//!         Circuit,
//!         Complete,
//!         Empty,
//!         RandomTournament,
//!     },
//!     op::IsSemicomplete,
//! };
//!
//! assert!(!Digraph::empty(3).is_semicomplete());
//! assert!(Digraph::complete(3).is_semicomplete());
//! assert!(Digraph::circuit(3).is_semicomplete());
//! assert!(Digraph::random_tournament(3, 0).is_semicomplete());
//! ```

use super::{
    HasArc,
    Order,
};

/// Check whether a digraph is semicomplete.
///
/// # How can I implement `IsSemicomplete`?
///
/// Provide an implementation of `is_semicomplete` that returns whether the
/// digraph is semicomplete OR implement `HasArc` and `Order`.
///
/// ```
/// use {
///     graaf::{
///         gen::{
///             Circuit,
///             Complete,
///             Empty,
///         },
///         op::{
///             HasArc,
///             IsSemicomplete,
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
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0, 1]),
///     ]
/// }
/// .is_semicomplete());
///
/// assert!(Digraph {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0]),
///     ]
/// }
/// .is_semicomplete());
///
/// assert!(!Digraph {
///     arcs: vec![BTreeSet::new(); 3]
/// }
/// .is_semicomplete());
/// ```
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     gen::{
///         Circuit,
///         Complete,
///         Empty,
///         RandomTournament,
///     },
///     op::IsSemicomplete,
/// };
///
/// assert!(!Digraph::empty(3).is_semicomplete());
/// assert!(Digraph::complete(3).is_semicomplete());
/// assert!(Digraph::circuit(3).is_semicomplete());
/// assert!(Digraph::random_tournament(3, 0).is_semicomplete());
/// ```
pub trait IsSemicomplete {
    /// Checks whether the digraph is semicomplete.
    #[must_use]
    fn is_semicomplete(&self) -> bool;
}

impl<D> IsSemicomplete for D
where
    D: HasArc + Order,
{
    fn is_semicomplete(&self) -> bool {
        let order = self.order();

        for u in 0..order {
            for v in (u + 1)..order {
                if !(self.has_arc(u, v) || self.has_arc(v, u)) {
                    return false;
                }
            }
        }

        true
    }
}
