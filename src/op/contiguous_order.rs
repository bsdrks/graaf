//! Return the number of vertices in a contiguous digraph.
//!
//! In addition to returning the digraph's order, the trait guarantees that the
//! vertex indices are contiguous. The vertex set is `0..contiguous_order()`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     ContiguousOrder,
//!     Empty,
//!     Order,
//! };
//!
//! let digraph = AdjacencyList::empty(4);
//!
//! assert_eq!(digraph.order(), digraph.contiguous_order());
//! ```

use crate::op::order::Order;

/// A trait that guarantees that a digraph’s vertices are contiguous.
///
/// Digraphs implementing `ContiguousOrder` must guaranteed that:
///
/// - `contiguous_order()` returns the digraph's order.
/// - The vertex set is exactly `0..contiguous_order()`.
///
/// This invariant allows algorithms to assume that the vertices are numbered
/// consecutively from 0.
pub trait ContiguousOrder: Order {
    /// Returns the order of the contiguous digraph.
    ///
    /// # Invariants
    ///
    /// Digraphs implementing `ContiguousOrder` must guaranteed that:
    ///
    /// - `contiguous_order()` returns the digraph's order.
    /// - The vertex set is exactly `0..contiguous_order()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     ContiguousOrder,
    ///     Empty,
    ///     Order,
    /// };
    ///
    /// let digraph = AdjacencyList::empty(4);
    ///
    /// assert_eq!(digraph.order(), digraph.contiguous_order());
    /// ```
    #[must_use]
    fn contiguous_order(&self) -> usize {
        self.order()
    }
}

/// `ContiguousOrder` proptests
#[macro_export]
macro_rules! proptest_contiguous_order {
    ($type:ty) => {
        use proptest::proptest;

        proptest! {
            #[test]
            fn empty_contagious_order_equals_order(order in 1..5_usize) {
                let digraph = <$type>::empty(order);

                assert_eq!(digraph.order(), digraph.contiguous_order());
            }
        }
    };
}
