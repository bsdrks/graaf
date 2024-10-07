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

/// Check whether a digraph is semicomplete.
///
/// # Implementing [`IsSemicomplete`] for a custom type
///
/// Provide an implementation of
/// [`is_semicomplete`](IsSemicomplete::is_semicomplete) that returns whether
/// the digraph is semicomplete.
///
/// ```
/// use {
///     graaf::IsSemicomplete,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsSemicomplete for AdjacencyList {
///     fn is_semicomplete(&self) -> bool {
///         let order = self.arcs.len();
///
///         (0..order).all(|u| {
///             (u + 1..order).all(|v| {
///                 self.arcs[u].contains(&v) || self.arcs[v].contains(&u)
///             })
///         })
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
/// Implementations can be built with the [`HasArc`](crate::HasArc) and
/// [`Order`](crate::Order) traits.
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
/// impl IsSemicomplete for AdjacencyList {
///     fn is_semicomplete(&self) -> bool {
///         let order = self.order();
///
///         (0..order).all(|u| {
///             (u + 1..order)
///                 .all(|v| self.has_arc(u, v) || self.has_arc(v, u))
///         })
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
