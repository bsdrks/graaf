//! Return a digraph's converse.
//!
//! A digraph's converse is the digraph with all arcs reversed.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//!     Converse,
//! };
//!
//! let digraph = AdjacencyList::circuit(4);
//! let converse = digraph.converse();
//!
//! assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
//! ```

/// Return a digraph's converse.
///
/// # Implementing [`Converse`] for a digraph type
///
/// Provide an implementation of [`converse`](Converse::converse) that returns
/// the digraph with all arcs reversed.
///
/// ```
/// use {
///     graaf::Converse,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Converse for AdjacencyList {
///     fn converse(&self) -> Self {
///         let order = self.arcs.len();
///         let mut arcs = vec![BTreeSet::<usize>::new(); order];
///
///         for (u, set) in self.arcs.iter().enumerate() {
///             for &v in set {
///                 arcs[v].insert(u);
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
/// let converse = digraph.converse();
///
/// assert_eq!(
///     converse.arcs,
///     vec![
///         BTreeSet::from([3]),
///         BTreeSet::from([0]),
///         BTreeSet::from([1]),
///         BTreeSet::from([2]),
///     ],
/// );
/// ```
pub trait Converse {
    /// Generate a digraph's converse.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    ///     Converse,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(4);
    /// let converse = digraph.converse();
    ///
    /// assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
    /// ```
    #[must_use]
    fn converse(&self) -> Self;
}
