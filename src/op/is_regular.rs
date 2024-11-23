//! Check whether a digraph is regular.
//!
//! A digraph is regular if all vertices have the same indegree and
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     IsRegular,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::circuit(7);
//!
//! assert!(digraph.is_regular());
//!
//! digraph.remove_arc(6, 0);
//!
//! assert!(!digraph.is_regular());
//! ```

/// Check whether a digraph is regular.
pub trait IsRegular {
    /// Check whether the digraph is regular.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     IsRegular,
    ///     RemoveArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::circuit(7);
    ///
    /// assert!(digraph.is_regular());
    ///
    /// digraph.remove_arc(6, 0);
    ///
    /// assert!(!digraph.is_regular());
    /// ```
    #[must_use]
    fn is_regular(&self) -> bool;
}
