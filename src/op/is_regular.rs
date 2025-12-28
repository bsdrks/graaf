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

/// `IsRegular` tests
#[macro_export]
macro_rules! test_is_regular {
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
        fn is_regular_bang_jensen_196() {
            assert!(!bang_jensen_196().is_regular());
        }

        #[test]
        fn is_regular_bang_jensen_34() {
            assert!(!bang_jensen_34().is_regular());
        }

        #[test]
        fn is_regular_bang_jensen_94() {
            assert!(!bang_jensen_94().is_regular());
        }

        #[test]
        fn is_regular_kattis_builddeps() {
            assert!(!kattis_builddeps().is_regular());
        }

        #[test]
        fn is_regular_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_regular());
        }

        #[test]
        fn is_regular_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_regular());
        }

        #[test]
        fn is_regular_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_regular());
        }

        #[test]
        fn is_regular_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_regular());
        }

        #[test]
        fn is_regular_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_regular());
        }
    };
}
