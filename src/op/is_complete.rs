//! Check whether a digraph is complete.
//!
//! A digraph is complete if, for every pair `u`, `v` of distinct vertices,
//! there is an arc from `u` to `v` and an arc from `v` to `u`.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsComplete,
//!     RandomTournament,
//! };
//!
//! assert!(AdjacencyList::complete(3).is_complete());
//! assert!(!AdjacencyList::circuit(3).is_complete());
//! assert!(!AdjacencyList::empty(3).is_complete());
//! assert!(!AdjacencyList::random_tournament(3, 0).is_complete());
//! ```

/// Check whether a digraph is complete.
pub trait IsComplete {
    /// Check whether the digraph is complete.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsComplete,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(AdjacencyList::complete(3).is_complete());
    /// assert!(!AdjacencyList::circuit(3).is_complete());
    /// assert!(!AdjacencyList::empty(3).is_complete());
    /// assert!(!AdjacencyList::random_tournament(3, 0).is_complete());
    /// ```
    #[must_use]
    fn is_complete(&self) -> bool;
}

/// `IsComplete` tests
#[macro_export]
macro_rules! test_is_complete {
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
        fn is_complete_bang_jensen_196() {
            assert!(!bang_jensen_196().is_complete());
        }

        #[test]
        fn is_complete_bang_jensen_34() {
            assert!(!bang_jensen_34().is_complete());
        }

        #[test]
        fn is_complete_bang_jensen_94() {
            assert!(!bang_jensen_94().is_complete());
        }

        #[test]
        fn is_complete_kattis_builddeps() {
            assert!(!kattis_builddeps().is_complete());
        }

        #[test]
        fn is_complete_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_complete());
        }

        #[test]
        fn is_complete_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_complete());
        }

        #[test]
        fn is_complete_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_complete());
        }

        #[test]
        fn is_complete_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_complete());
        }

        #[test]
        fn is_complete_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_complete());
        }
    };
}
