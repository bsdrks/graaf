//! Check whether a digraph is a tournament.
//!
//! A tournament is a digraph in which there is one arc between every unordered
//! pair of distinct vertices.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Circuit,
//!     Complete,
//!     Empty,
//!     IsTournament,
//!     RandomTournament,
//! };
//!
//! assert!(!AdjacencyList::empty(3).is_tournament());
//! assert!(!AdjacencyList::complete(3).is_tournament());
//! assert!(AdjacencyList::circuit(3).is_tournament());
//! assert!(AdjacencyList::random_tournament(3, 0).is_tournament());
//! ```

/// Check whether a digraph is a tournament.
pub trait IsTournament {
    /// Check whether the digraph is a tournament.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Complete,
    ///     Empty,
    ///     IsTournament,
    ///     RandomTournament,
    /// };
    ///
    /// assert!(!AdjacencyList::empty(3).is_tournament());
    /// assert!(!AdjacencyList::complete(3).is_tournament());
    /// assert!(AdjacencyList::circuit(3).is_tournament());
    /// assert!(AdjacencyList::random_tournament(3, 0).is_tournament());
    /// ```
    #[must_use]
    fn is_tournament(&self) -> bool;
}

/// `IsTournament` tests
#[macro_export]
macro_rules! test_is_tournament {
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
        fn is_tournament_bang_jensen_196() {
            assert!(!bang_jensen_196().is_tournament());
        }

        #[test]
        fn is_tournament_bang_jensen_34() {
            assert!(!bang_jensen_34().is_tournament());
        }

        #[test]
        fn is_tournament_bang_jensen_94() {
            assert!(!bang_jensen_94().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_builddeps() {
            assert!(!kattis_builddeps().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_cantinaofbabel_1() {
            assert!(!kattis_cantinaofbabel_1().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_cantinaofbabel_2() {
            assert!(!kattis_cantinaofbabel_2().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_escapewallmaria_1() {
            assert!(!kattis_escapewallmaria_1().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_escapewallmaria_2() {
            assert!(!kattis_escapewallmaria_2().is_tournament());
        }

        #[test]
        fn is_tournament_kattis_escapewallmaria_3() {
            assert!(!kattis_escapewallmaria_3().is_tournament());
        }
    };
}
