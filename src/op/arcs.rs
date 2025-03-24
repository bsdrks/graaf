//! Iterate a digraph's arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//! };
//!
//! let digraph = AdjacencyList::circuit(3);
//!
//! assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
//! ```

/// Digraph arcs
pub trait Arcs {
    /// Iterate the digraph's arcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(3);
    ///
    /// assert!(digraph.arcs().eq([(0, 1), (1, 2), (2, 0)]));
    /// ```
    #[must_use]
    fn arcs(&self) -> impl Iterator<Item = (usize, usize)>;
}

/// `Arcs` tests
#[macro_export]
macro_rules! test_arcs {
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
        fn arcs_bang_jensen_196() {
            assert!(bang_jensen_196().arcs().eq(vec![
                (0, 1),
                (0, 4),
                (0, 7),
                (1, 0),
                (1, 2),
                (1, 7),
                (2, 3),
                (3, 2),
                (3, 4),
                (4, 2),
                (5, 6),
                (6, 7),
                (7, 5)
            ]));
        }

        #[test]
        fn arcs_bang_jensen_34() {
            assert!(bang_jensen_34().arcs().eq(vec![
                (0, 4),
                (1, 0),
                (2, 1),
                (2, 3),
                (2, 5),
                (5, 4)
            ]));
        }

        #[test]
        fn arcs_bang_jensen_94() {
            assert!(bang_jensen_94().arcs().eq(vec![
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 1),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 5),
                (4, 6)
            ]));
        }

        #[test]
        fn arcs_kattis_builddeps() {
            assert!(kattis_builddeps().arcs().eq(vec![
                (0, 3),
                (0, 4),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 1),
                (4, 1),
                (5, 1)
            ]));
        }

        #[test]
        fn arcs_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().arcs().eq(vec![
                (0, 1),
                (1, 0),
                (1, 2),
                (1, 4),
                (2, 1),
                (3, 2),
                (3, 4),
                (3, 5),
                (3, 7),
                (3, 10),
                (3, 11),
                (4, 3),
                (5, 6),
                (6, 5),
                (6, 10),
                (7, 3),
                (8, 7),
                (8, 10),
                (9, 7),
                (9, 11),
                (10, 6),
                (11, 9)
            ]));
        }

        #[test]
        fn arcs_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().arcs().eq(vec![
                (0, 1),
                (1, 0),
                (1, 7),
                (2, 0),
                (2, 5),
                (2, 7),
                (3, 4),
                (4, 3),
                (5, 3),
                (5, 6),
                (6, 5),
                (7, 2),
                (8, 7),
                (8, 9),
                (8, 11),
                (9, 8),
                (10, 9),
                (10, 11),
                (11, 10)
            ]));
        }

        #[test]
        fn arcs_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1().arcs().eq(vec![
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (13, 9),
                (13, 12)
            ]));
        }

        #[test]
        fn arcs_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2().arcs().eq(vec![
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));
        }

        #[test]
        fn arcs_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().arcs().eq(vec![
                (1, 2),
                (1, 5),
                (2, 1),
                (2, 6),
                (5, 1),
                (5, 6),
                (5, 9),
                (6, 2),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));
        }
    };
}
