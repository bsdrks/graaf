//! Check whether a digraph is semicomplete.
//!
//! A digraph is semicomplete if there is an arc between every unordered pair
//! of distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsSemicomplete,
//!     RandomTournament,
//! };
//!
//! assert!(!AdjacencyList::empty(3).is_semicomplete());
//! assert!(AdjacencyList::complete(3).is_semicomplete());
//! assert!(AdjacencyList::circuit(3).is_semicomplete());
//! assert!(AdjacencyList::random_tournament(3, 0).is_semicomplete());
//! ```

/// Check whether a digraph is semicomplete.
pub trait IsSemicomplete {
    /// Check whether the digraph is semicomplete.
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsSemicomplete,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(!AdjacencyList::empty(3).is_semicomplete());
    /// assert!(AdjacencyList::complete(3).is_semicomplete());
    /// assert!(AdjacencyList::circuit(3).is_semicomplete());
    /// assert!(AdjacencyList::random_tournament(3, 0).is_semicomplete());
    /// ```
    #[must_use]
    fn is_semicomplete(&self) -> bool;
}

/// `IsSemicomplete` tests
#[macro_export]
macro_rules! test_is_semicomplete {
    ($fixture:path) => {
        use $fixture::{
            bang_jensen_196,
            bang_jensen_34,
            bang_jensen_94,
            kattis_builddeps,
            kattis_cantinaofbabel_1,
            kattis_cantinaofbabel_2,
            kattis_escapewallmaria_1,
            kattis_escapewallmaria_2,
            kattis_escapewallmaria_3,
        };

        #[test]
        fn is_semicomplete_bang_jensen_196() {
            assert!(!bang_jensen_196().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_bang_jensen_34() {
            assert!(!bang_jensen_34().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_bang_jensen_94() {
            assert!(!bang_jensen_94().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_builddeps() {
            assert!(!kattis_builddeps().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_semicomplete());
        }

        #[test]
        fn is_semicomplete_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_semicomplete());
        }
    };
}
