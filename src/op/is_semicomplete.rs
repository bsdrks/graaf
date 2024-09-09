//! Check whether a digraph is semicomplete.
//!
//! A digraph is semicomplete if there is an arc between every unordered pair
//! `u`, `v` of distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsSemicomplete,
//!     RandomTournament,
//! };
//!
//! assert!(!AdjacencyList::empty(3).is_semicomplete());
//! assert!(AdjacencyList::complete(3).is_semicomplete());
//! assert!(AdjacencyList::circuit(3).is_semicomplete());
//! assert!(AdjacencyList::random_tournament(3, 0).is_semicomplete());
//! ```

use crate::{
    HasArc,
    Order,
};

/// Check whether a digraph is semicomplete.
///
/// # Implementing [`IsSemicomplete`] for a custom type
///
/// Provide an implementation of
/// [`is_semicomplete`](IsSemicomplete::is_semicomplete) that returns whether
/// the digraph is semicomplete OR implement `HasArc` and `Order`.
///
/// ```
/// use {
///     graaf::{
///         Circuit,
///         Complete,
///         Empty,
///         HasArc,
///         IsSemicomplete,
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
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0, 1]),
///     ]
/// }
/// .is_semicomplete());
///
/// assert!(AdjacencyList {
///     arcs: vec![
///         BTreeSet::from([1, 2]),
///         BTreeSet::from([0, 2]),
///         BTreeSet::from([0]),
///     ]
/// }
/// .is_semicomplete());
///
/// assert!(!AdjacencyList {
///     arcs: vec![BTreeSet::new(); 3]
/// }
/// .is_semicomplete());
/// ```
///
/// # Examples
pub trait IsSemicomplete {
    /// Check whether the digraph is semicomplete.
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsSemicomplete,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(!AdjacencyList::empty(3).is_semicomplete());
    /// assert!(AdjacencyList::complete(3).is_semicomplete());
    /// assert!(AdjacencyList::circuit(3).is_semicomplete());
    /// assert!(AdjacencyList::random_tournament(3, 0).is_semicomplete());
    /// ```
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
