//! Check whether an arc exists in a digraph.
//!
//! To check whether an edge exists between `u` to `v`, see [`HasEdge`].
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     HasArc,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert!(digraph.has_arc(0, 1));
//! assert!(digraph.has_arc(0, 2));
//! assert!(digraph.has_arc(1, 0));
//! assert!(!digraph.has_arc(1, 2));
//! assert!(!digraph.has_arc(2, 0));
//! assert!(!digraph.has_arc(2, 1));
//! ```
//!
//! [`HasEdge`]: crate::HasEdge

/// Check whether an arc exists in a digraph.
pub trait HasArc {
    /// Check whether an arc exists in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Panics
    ///
    /// `has_arc` may not panic if `u` and `v` aren't in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     HasArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(digraph.has_arc(0, 1));
    /// assert!(digraph.has_arc(0, 2));
    /// assert!(digraph.has_arc(1, 0));
    /// assert!(!digraph.has_arc(1, 2));
    /// assert!(!digraph.has_arc(2, 0));
    /// assert!(!digraph.has_arc(2, 1));
    /// ```
    #[must_use]
    fn has_arc(&self, u: usize, v: usize) -> bool;
}

/// `HasArc` proptests
#[macro_export]
macro_rules! proptest_has_arc {
    ($type:ty) => {
        use {
            proptest::proptest,
            $crate::Empty,
        };

        proptest! {
            #[test]
            fn has_arc_out_of_bounds(order in 1..25_usize) {
                let digraph = <$type>::empty(order);

                for u in 0..order {
                    assert!(!digraph.has_arc(u, order));
                    assert!(!digraph.has_arc(order, u));
                }
            }
        }
    };
}
