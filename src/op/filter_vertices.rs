//! Return the subgraph with vertices that satisfy the predicate.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyMap,
//!     Arcs,
//!     FilterVertices,
//!     Vertices,
//!     Wheel,
//! };
//!
//! let mut digraph = AdjacencyMap::wheel(9);
//! let subgraph = digraph.filter_vertices(|u| u % 2 == 0 && u < 6);
//!
//! assert!(subgraph.arcs().eq([(0, 2), (0, 4), (2, 0), (4, 0)]));
//! assert!(subgraph.vertices().eq([0, 2, 4]));
//! ```

/// Return the subgraph with vertices that satisfy the predicate.
///
/// # Implementing [`FilterVertices`] for a custom type
///
/// Provide an implementation of
/// [`filter_vertices`](FilterVertices::filter_vertices) that returns the
/// subgraph with vertices that satisfy the predicate.
///
/// ```
/// use {
///     graaf::FilterVertices,
///     std::collections::HashSet,
/// };
///
/// struct AdjacencyMap {
///     arcs: Vec<HashSet<usize>>,
/// }
///
/// impl FilterVertices for AdjacencyMap {
///     fn filter_vertices<P>(&self, predicate: P) -> Self
///     where
///         P: Fn(usize) -> bool,
///     {
///         let mut order = 0;
///         let mut arcs_raw = HashSet::new();
///
///         for (u, set) in self.arcs.iter().enumerate() {
///             for &v in set {
///                 if predicate(u) && predicate(v) {
///                     order = order.max(u.max(v));
///
///                     let _ = arcs_raw.insert((u, v));
///                 }
///             }
///         }
///
///         let mut arcs = vec![HashSet::new(); order + 1];
///
///         for (u, v) in arcs_raw {
///             let _ = arcs[u].insert(v);
///         }
///
///         Self { arcs }
///     }
/// }
///
/// let digraph = AdjacencyMap {
///     arcs: vec![
///         HashSet::from([1, 2, 3, 4, 5, 6, 7, 8]),
///         HashSet::from([0, 2, 8]),
///         HashSet::from([0, 1, 3]),
///         HashSet::from([0, 2, 4]),
///         HashSet::from([0, 3, 5]),
///         HashSet::from([0, 4, 6]),
///         HashSet::from([0, 5, 7]),
///         HashSet::from([0, 6, 8]),
///         HashSet::from([0, 1, 7]),
///     ],
/// };
///
/// let subgraph = digraph.filter_vertices(|u| u % 2 == 0 && u < 6);
///
/// assert!(subgraph.arcs.iter().eq(&[
///     HashSet::from([2, 4]),
///     HashSet::new(),
///     HashSet::from([0]),
///     HashSet::new(),
///     HashSet::from([0])
/// ]));
///
/// assert_eq!(subgraph.arcs.len(), 5);
/// ```
pub trait FilterVertices {
    /// Return the subgraph with vertices that satisfy the predicate.
    ///
    /// # Panics
    ///
    /// Panics if the subgraph has zero vertices.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyMap,
    ///     Arcs,
    ///     FilterVertices,
    ///     Vertices,
    ///     Wheel,
    /// };
    ///
    /// let mut digraph = AdjacencyMap::wheel(9);
    /// let subgraph = digraph.filter_vertices(|u| u % 2 == 0 && u < 6);
    ///
    /// assert!(subgraph.arcs().eq([(0, 2), (0, 4), (2, 0), (4, 0)]));
    /// assert!(subgraph.vertices().eq([0, 2, 4]));
    /// ```
    #[must_use]
    fn filter_vertices<P>(&self, predicate: P) -> Self
    where
        P: Fn(usize) -> bool;
}
