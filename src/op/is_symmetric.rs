//! Check whether a digraph is symmetric.
//!
//! A digraph is symmetric if for every arc `(u, v)` there is an arc
//! `(v, u)`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSymmetric,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::empty(2);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 0);
//!
//! assert!(digraph.is_symmetric());
//!
//! digraph.remove_arc(1, 0);
//!
//! assert!(!digraph.is_symmetric());
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(!digraph.is_symmetric());
//! ```

use crate::{
    Arcs,
    HasArc,
};

/// Check whether a digraph is symmetric.
pub trait IsSymmetric {
    /// Check whether the digraph is symmetric.
    ///
    /// # Examples
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSymmetric,
    ///     RemoveArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(2);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(1, 0);
    ///
    /// assert!(digraph.is_symmetric());
    ///
    /// digraph.remove_arc(1, 0);
    ///
    /// assert!(!digraph.is_symmetric());
    /// ```
    #[must_use]
    fn is_symmetric(&self) -> bool;
}

impl<D> IsSymmetric for D
where
    D: Arcs + HasArc,
{
    fn is_symmetric(&self) -> bool {
        self.arcs().all(|(u, v)| self.has_arc(v, u))
    }
}

/// `IsSymmetric` tests
#[macro_export]
macro_rules! test_is_symmetric {
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
        fn is_symmetric_bang_jensen_196() {
            assert!(!bang_jensen_196().is_symmetric());
        }

        #[test]
        fn is_symmetric_bang_jensen_34() {
            assert!(!bang_jensen_34().is_symmetric());
        }

        #[test]
        fn is_symmetric_bang_jensen_94() {
            assert!(!bang_jensen_94().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_builddeps() {
            assert!(!kattis_builddeps().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_symmetric());
        }

        #[test]
        fn is_symmetric_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().is_symmetric());
        }
    };
}
