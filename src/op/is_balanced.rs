//! Check whether a digraph is balanced.
//!
//! A digraph is balanced if the indegree of each vertex equals its
//! outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsBalanced,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_balanced());
//!
//! digraph.add_arc(2, 1);
//!
//! assert!(digraph.is_balanced());
//! ```
#![doc(alias = "isograph")]
#![doc(alias = "pseudosymmetric")]

use crate::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Check whether a digraph is balanced.
#[doc(alias = "Isograph")]
#[doc(alias = "Pseudosymmetric")]
pub trait IsBalanced {
    /// Check whether the digraph is balanced.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsBalanced,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(!digraph.is_balanced());
    ///
    /// digraph.add_arc(2, 1);
    ///
    /// assert!(digraph.is_balanced());
    /// ```
    #[doc(alias = "isograph")]
    #[doc(alias = "pseudosymmetric")]
    #[must_use]
    fn is_balanced(&self) -> bool;
}

impl<D> IsBalanced for D
where
    D: Indegree + Outdegree + Vertices,
{
    fn is_balanced(&self) -> bool {
        self.vertices()
            .all(|u| self.indegree(u) == self.outdegree(u))
    }
}
