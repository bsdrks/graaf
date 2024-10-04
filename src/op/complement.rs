//! Return a digraph's complement.
//!
//! A digraph's complement contains all arcs not in the original digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//!     Complement,
//! };
//!
//! let digraph = AdjacencyList::circuit(4);
//! let converse = digraph.complement();
//!
//! assert!(converse.arcs().eq([
//!     (0, 2),
//!     (0, 3),
//!     (1, 0),
//!     (1, 3),
//!     (2, 0),
//!     (2, 1),
//!     (3, 1),
//!     (3, 2)
//! ]));
//! ```

/// Return a digraph's complement.
///
/// # Implementing [`Complement`] for a custom type
///
/// Provide an implementation of [`complement`](Complement::complement) that
/// returns a digraph with all arcs not present in the original digraph.
///
/// ```
/// use {
///     graaf::Complement,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     arcs: Vec<BTreeSet<usize>>,
/// }
///
/// impl Complement for AdjacencyList {
///     fn complement(&self) -> Self {
///         let order = self.arcs.len();
///         let mut arcs = vec![BTreeSet::<usize>::new(); order];
///
///         for u in 0..order {
///             for v in u + 1..order {
///                 if !self.arcs[u].contains(&v) {
///                     arcs[u].insert(v);
///                 }
///
///                 if !self.arcs[v].contains(&u) {
///                     arcs[v].insert(u);
///                 }
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
/// let complement = digraph.complement();
///
/// assert_eq!(
///     complement.arcs,
///     vec![
///         BTreeSet::from([2, 3]),
///         BTreeSet::from([0, 3]),
///         BTreeSet::from([0, 1]),
///         BTreeSet::from([1, 2]),
///     ],
/// );
/// ```
pub trait Complement {
    /// Generate the digraph's complement.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    ///     Complement,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(4);
    /// let converse = digraph.complement();
    ///
    /// assert!(converse.arcs().eq([
    ///     (0, 2),
    ///     (0, 3),
    ///     (1, 0),
    ///     (1, 3),
    ///     (2, 0),
    ///     (2, 1),
    ///     (3, 1),
    ///     (3, 2)
    /// ]));
    /// ```
    #[must_use]
    fn complement(&self) -> Self;
}
