//! Check whether a vertex is isolated.
//!
//! A vertex is isolated if it has no incoming or outgoing arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsIsolated,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//!
//! assert!(!digraph.is_isolated(0));
//! assert!(!digraph.is_isolated(1));
//! assert!(!digraph.is_isolated(2));
//! assert!(digraph.is_isolated(3));
//! ```

use crate::{
    Indegree,
    Outdegree,
};

/// Check whether a vertex is isolated.
pub trait IsIsolated {
    /// Check whether the vertex is isolated in the digraph.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsIsolated,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(!digraph.is_isolated(0));
    /// assert!(!digraph.is_isolated(1));
    /// assert!(digraph.is_isolated(2));
    /// ```
    #[must_use]
    fn is_isolated(&self, u: usize) -> bool;
}

impl<D> IsIsolated for D
where
    D: Indegree + Outdegree,
{
    fn is_isolated(&self, u: usize) -> bool {
        self.is_sink(u) && self.is_source(u)
    }
}

/// `IsIsolated` tests
#[macro_export]
macro_rules! test_is_isolated {
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
        fn is_isolated_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(!digraph.is_isolated(7));
        }

        #[test]
        fn is_isolated_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
        }

        #[test]
        fn is_isolated_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
        }

        #[test]
        fn is_isolated_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
        }

        #[test]
        fn is_isolated_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(!digraph.is_isolated(7));
            assert!(!digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(!digraph.is_isolated(10));
            assert!(!digraph.is_isolated(11));
        }

        #[test]
        fn is_isolated_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(!digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(!digraph.is_isolated(3));
            assert!(!digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(!digraph.is_isolated(7));
            assert!(!digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(!digraph.is_isolated(10));
            assert!(!digraph.is_isolated(11));
        }

        #[test]
        fn is_isolated_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.is_isolated(0));
            assert!(digraph.is_isolated(1));
            assert!(digraph.is_isolated(2));
            assert!(digraph.is_isolated(3));
            assert!(digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(digraph.is_isolated(7));
            assert!(digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(digraph.is_isolated(10));
            assert!(digraph.is_isolated(11));
            assert!(!digraph.is_isolated(12));
            assert!(!digraph.is_isolated(13));
            assert!(digraph.is_isolated(14));
            assert!(digraph.is_isolated(15));
        }

        #[test]
        fn is_isolated_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.is_isolated(0));
            assert!(digraph.is_isolated(1));
            assert!(digraph.is_isolated(2));
            assert!(digraph.is_isolated(3));
            assert!(digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(digraph.is_isolated(7));
            assert!(digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(digraph.is_isolated(10));
            assert!(digraph.is_isolated(11));
            assert!(!digraph.is_isolated(12));
            assert!(!digraph.is_isolated(13));
            assert!(digraph.is_isolated(14));
            assert!(digraph.is_isolated(15));
        }

        #[test]
        fn is_isolated_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.is_isolated(0));
            assert!(!digraph.is_isolated(1));
            assert!(!digraph.is_isolated(2));
            assert!(digraph.is_isolated(3));
            assert!(digraph.is_isolated(4));
            assert!(!digraph.is_isolated(5));
            assert!(!digraph.is_isolated(6));
            assert!(digraph.is_isolated(7));
            assert!(digraph.is_isolated(8));
            assert!(!digraph.is_isolated(9));
            assert!(digraph.is_isolated(10));
            assert!(digraph.is_isolated(11));
            assert!(!digraph.is_isolated(12));
            assert!(!digraph.is_isolated(13));
            assert!(digraph.is_isolated(14));
            assert!(digraph.is_isolated(15));
        }
    };
}
