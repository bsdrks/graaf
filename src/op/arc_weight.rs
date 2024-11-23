//! Return an arc's weight.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArcWeighted,
//!     AdjacencyListWeighted,
//!     ArcWeight,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyListWeighted::<usize>::empty(3);
//!
//! digraph.add_arc_weighted(0, 1, 2);
//! digraph.add_arc_weighted(0, 2, 3);
//! digraph.add_arc_weighted(1, 0, 4);
//! digraph.add_arc_weighted(2, 0, 7);
//! digraph.add_arc_weighted(2, 1, 8);
//!
//! assert_eq!(digraph.arc_weight(0, 0), None);
//! assert_eq!(digraph.arc_weight(0, 1), Some(&2));
//! assert_eq!(digraph.arc_weight(0, 2), Some(&3));
//! assert_eq!(digraph.arc_weight(1, 0), Some(&4));
//! assert_eq!(digraph.arc_weight(1, 1), None);
//! assert_eq!(digraph.arc_weight(2, 0), Some(&7));
//! assert_eq!(digraph.arc_weight(2, 1), Some(&8));
//! assert_eq!(digraph.arc_weight(2, 2), None);
//! ```

/// Arc weight
pub trait ArcWeight<Idx> {
    /// The weight of an arc.
    type Weight;

    /// Return the weight of the arc if the arc exists in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArcWeighted,
    ///     AdjacencyListWeighted,
    ///     ArcWeight,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyListWeighted::<usize>::empty(3);
    ///
    /// digraph.add_arc_weighted(0, 1, 2);
    /// digraph.add_arc_weighted(0, 2, 3);
    /// digraph.add_arc_weighted(1, 0, 4);
    /// digraph.add_arc_weighted(2, 0, 7);
    /// digraph.add_arc_weighted(2, 1, 8);
    ///
    /// assert_eq!(digraph.arc_weight(0, 0), None);
    /// assert_eq!(digraph.arc_weight(0, 1), Some(&2));
    /// assert_eq!(digraph.arc_weight(0, 2), Some(&3));
    /// assert_eq!(digraph.arc_weight(1, 0), Some(&4));
    /// assert_eq!(digraph.arc_weight(1, 1), None);
    /// assert_eq!(digraph.arc_weight(2, 0), Some(&7));
    /// assert_eq!(digraph.arc_weight(2, 1), Some(&8));
    /// assert_eq!(digraph.arc_weight(2, 2), None);
    /// ```
    #[must_use]
    fn arc_weight(&self, u: Idx, v: Idx) -> Option<&Self::Weight>;
}
