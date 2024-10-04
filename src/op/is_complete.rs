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

/// Check whether a digraph is complete.
///
/// # Implementing [`IsComplete`] for a custom type
///
/// Provide an implementation of [`is_complete`](IsComplete::is_complete) that
/// returns whether the digraph is complete.
///
/// ```
/// use {
///     graaf::IsComplete,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     pub arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl IsComplete for AdjacencyList {
///     fn is_complete(&self) -> bool {
///         let order = self.arcs.len();
///
///         (0..order).all(|u| {
///             (u + 1..order).all(|v| {
///                 self.arcs[u].contains(&v) && self.arcs[v].contains(&u)
///             })
///         })
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
///
/// Implementations can be built with the [`AddArc`](crate::AddArc) and
/// [`HasEdge`](crate::HasEdge) traits.
///
/// ```
/// use {
///     graaf::{
///         Circuit,
///         Empty,
///         HasEdge,
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
/// impl IsComplete for AdjacencyList {
///     fn is_complete(&self) -> bool {
///         let order = self.order();
///
///         (0..order).all(|u| (u + 1..order).all(|v| self.has_edge(u, v)))
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
    /// Check whether the digraph is complete.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsComplete,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(AdjacencyList::complete(3).is_complete());
    /// assert!(!AdjacencyList::circuit(3).is_complete());
    /// assert!(!AdjacencyList::empty(3).is_complete());
    /// assert!(!AdjacencyList::random_tournament(3, 0).is_complete());
    /// ```
    #[must_use]
    fn is_complete(&self) -> bool;
}
