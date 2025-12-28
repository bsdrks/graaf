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

/// `IsBalanced` tests
#[macro_export]
macro_rules! test_is_balanced {
    ($fixture:path) => {
        use $fixture::{
            bang_jensen_34,
            bang_jensen_94,
            bang_jensen_196,
            kattis_builddeps,
            kattis_cantinaofbabel_1,
            kattis_cantinaofbabel_2,
            kattis_escapewallmaria_1,
            kattis_escapewallmaria_2,
            kattis_escapewallmaria_3,
        };

        #[test]
        fn is_balanced_bang_jensen_196() {
            assert!(!bang_jensen_196().is_balanced());
        }

        #[test]
        fn is_balanced_bang_jensen_34() {
            assert!(!bang_jensen_34().is_balanced());
        }

        #[test]
        fn is_balanced_bang_jensen_94() {
            assert!(!bang_jensen_94().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_builddeps() {
            assert!(!kattis_builddeps().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_balanced());
        }

        #[test]
        fn is_balanced_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().is_balanced());
        }
    };
}
