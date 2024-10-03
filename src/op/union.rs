//! Return the union of two digraphs.
//!
//! # Example
//!
//! The union of a cycle digraph and a star digraph form a wheel digraph.
//!
//! The cycle digraph:
//!
//! ![A cycle digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_cycle_1-0.88.4.svg?)
//!
//! The star digraph:
//!
//! ![A star digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_star_1-0.88.4.svg?)
//!
//! The union, a wheel digraph:
//!
//! ![The union forms a wheel digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_union_1-0.88.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AdjacencyList,
//!         Arcs,
//!         Cycle,
//!         Star,
//!         Union,
//!         Wheel,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let cycle = AdjacencyList::from(vec![
//!     BTreeSet::new(),
//!     BTreeSet::from([2, 4]),
//!     BTreeSet::from([1, 3]),
//!     BTreeSet::from([2, 4]),
//!     BTreeSet::from([1, 3]),
//! ]);
//!
//! let star = AdjacencyList::from(vec![
//!     BTreeSet::from([1, 2, 3, 4]),
//!     BTreeSet::from([0]),
//!     BTreeSet::from([0]),
//!     BTreeSet::from([0]),
//!     BTreeSet::from([0]),
//! ]);
//!
//! let wheel = AdjacencyList::wheel(5);
//!
//! assert!(cycle.union(&star).arcs().eq(wheel.arcs()));
//! ```

/// Return the union of two digraphs.
///
/// # Implementing [`Union`] for a custom type
///
/// Provide an implementation of [`union`](Union::union) that returns the union
/// of two digraphs.
///
/// ```
/// use {
///     graaf::Union,
///     std::collections::BTreeSet,
/// };
///
/// struct AdjacencyList {
///     vertices: Vec<BTreeSet<usize>>,
/// }
///
/// impl Union for AdjacencyList {
///     fn union(&self, other: &Self) -> Self {
///         let (mut vertices, other) =
///             if self.vertices.len() < other.vertices.len() {
///                 (self.vertices.clone(), other)
///             } else {
///                 (other.vertices.clone(), self)
///             };
///
///         for (u, set) in other.vertices.iter().enumerate() {
///             vertices[u].extend(set);
///         }
///
///         AdjacencyList { vertices }
///     }
/// }
///
/// let cycle = AdjacencyList {
///     vertices: vec![
///         BTreeSet::new(),
///         BTreeSet::from([2, 4]),
///         BTreeSet::from([1, 3]),
///         BTreeSet::from([2, 4]),
///         BTreeSet::from([1, 3]),
///     ],
/// };
///
/// let star = AdjacencyList {
///     vertices: vec![
///         BTreeSet::from([1, 2, 3, 4]),
///         BTreeSet::from([0]),
///         BTreeSet::from([0]),
///         BTreeSet::from([0]),
///         BTreeSet::from([0]),
///     ],
/// };
///
/// let wheel = AdjacencyList {
///     vertices: vec![
///         BTreeSet::from([1, 2, 3, 4]),
///         BTreeSet::from([0, 2, 4]),
///         BTreeSet::from([0, 1, 3]),
///         BTreeSet::from([0, 2, 4]),
///         BTreeSet::from([0, 1, 3]),
///     ],
/// };
///
/// assert!(cycle.union(&star).vertices.iter().eq(wheel.vertices.iter()));
/// ```
pub trait Union {
    /// Return the union of two digraphs.
    ///
    /// # Arguments
    ///
    /// * `other`: The other digraph.
    ///
    /// # Example
    ///
    /// The union of a cycle digraph and a star digraph form a wheel digraph.
    ///
    /// The cycle digraph:
    ///
    /// ![A cycle digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_cycle_1-0.88.4.svg?)
    ///
    /// The star digraph:
    ///
    /// ![A star digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_star_1-0.88.4.svg?)
    ///
    /// The union, a wheel digraph:
    ///
    /// ![The union forms a Wheel digraph](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/union_union_1-0.88.4.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AdjacencyList,
    ///         Arcs,
    ///         Cycle,
    ///         Star,
    ///         Union,
    ///         Wheel,
    ///     },
    ///     std::collections::BTreeSet,
    /// };
    ///
    /// let cycle = AdjacencyList::from(vec![
    ///     BTreeSet::new(),
    ///     BTreeSet::from([2, 4]),
    ///     BTreeSet::from([1, 3]),
    ///     BTreeSet::from([2, 4]),
    ///     BTreeSet::from([1, 3]),
    /// ]);
    ///
    /// let star = AdjacencyList::from(vec![
    ///     BTreeSet::from([1, 2, 3, 4]),
    ///     BTreeSet::from([0]),
    ///     BTreeSet::from([0]),
    ///     BTreeSet::from([0]),
    ///     BTreeSet::from([0]),
    /// ]);
    ///
    /// let wheel = AdjacencyList::wheel(5);
    ///
    /// assert!(cycle.union(&star).arcs().eq(wheel.arcs()));
    /// ```
    #[must_use]
    fn union(&self, other: &Self) -> Self;
}
