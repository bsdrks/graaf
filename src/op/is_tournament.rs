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

/// Check whether a digraph is a tournament.
///
/// A tournament is a digraph in which there is one arc between every unordered
/// pair of distinct vertices.
///
/// # Implementing [`IsTournament`] for a custom type
///
/// Provide an implementation of [`is_tournament`](IsTournament::is_tournament)
/// that returns whether the digraph is a tournament.
///
/// ```
/// use {
///     graaf::IsTournament,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsTournament for AdjacencyList {
///     fn is_tournament(&self) -> bool {
///         let order = self.arcs.len();
///
///         (0..order).all(|u| {
///             (u + 1..order).all(|v| {
///                 self.arcs[u].contains(&v) ^ self.arcs[v].contains(&u)
///             })
///         })
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
/// Implementations can be built with the [`HasArc`](crate::HasArc) and
/// [`Order`](crate::Order) traits.
///
/// ```
/// use {
///     graaf::{
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
/// impl IsTournament for AdjacencyList {
///     fn is_tournament(&self) -> bool {
///         let order = self.order();
///
///         (0..order).all(|u| {
///             (u + 1..order).all(|v| self.has_arc(u, v) ^ self.has_arc(v, u))
///         })
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
pub trait IsTournament {
    /// Check whether the digraph is a tournament.
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
    #[must_use]
    fn is_tournament(&self) -> bool;
}
