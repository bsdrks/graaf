//! Generate a digraph's converse.
//!
//! A digraph's converse is the digraph with all arcs reversed.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Arcs,
//!     Circuit,
//!     Converse,
//! };
//!
//! let digraph = AdjacencyList::circuit(4);
//! let converse = digraph.converse();
//!
//! assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
//! ```

/// Digraph converse
pub trait Converse {
    /// Generate a digraph's converse.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Arcs,
    ///     Circuit,
    ///     Converse,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(4);
    /// let converse = digraph.converse();
    ///
    /// assert!(converse.arcs().eq([(0, 3), (1, 0), (2, 1), (3, 2)]));
    /// ```
    #[must_use]
    fn converse(&self) -> Self;
}

/// `Converse` tests
#[macro_export]
macro_rules! test_converse {
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
        fn converse_bang_jensen_196() {
            let digraph = bang_jensen_196();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (0, 1),
                (1, 0),
                (2, 1),
                (2, 3),
                (2, 4),
                (3, 2),
                (4, 0),
                (4, 3),
                (5, 7),
                (6, 5),
                (7, 0),
                (7, 1),
                (7, 6),
            ]));
        }

        #[test]
        fn converse_bang_jensen_34() {
            let digraph = bang_jensen_34();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (0, 1),
                (1, 2),
                (3, 2),
                (4, 0),
                (4, 5),
                (5, 2)
            ]));
        }

        #[test]
        fn converse_bang_jensen_94() {
            let digraph = bang_jensen_94();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (1, 0),
                (1, 2),
                (2, 0),
                (3, 1),
                (3, 2),
                (4, 2),
                (5, 2),
                (5, 3),
                (6, 4)
            ]));
        }

        #[test]
        fn converse_kattis_builddeps() {
            let digraph = kattis_builddeps();
            let converse = digraph.converse();

            assert!(converse.arcs().eq([
                (1, 3),
                (1, 4),
                (1, 5),
                (3, 0),
                (3, 2),
                (4, 0),
                (4, 2),
                (5, 2)
            ]));
        }

        #[test]
        fn converse_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();
            let converse = digraph.converse();

            assert!(converse.arcs().eq([
                (0, 1),
                (1, 0),
                (1, 2),
                (2, 1),
                (2, 3),
                (3, 4),
                (3, 7),
                (4, 1),
                (4, 3),
                (5, 3),
                (5, 6),
                (6, 5),
                (6, 10),
                (7, 3),
                (7, 8),
                (7, 9),
                (9, 11),
                (10, 3),
                (10, 6),
                (10, 8),
                (11, 3),
                (11, 9)
            ]));
        }

        #[test]
        fn converse_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (0, 1),
                (0, 2),
                (1, 0),
                (2, 7),
                (3, 4),
                (3, 5),
                (4, 3),
                (5, 2),
                (5, 6),
                (6, 5),
                (7, 1),
                (7, 2),
                (7, 8),
                (8, 9),
                (9, 8),
                (9, 10),
                (10, 11),
                (11, 8),
                (11, 10)
            ]));
        }

        #[test]
        fn converse_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 9)
            ]));
        }

        #[test]
        fn converse_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (12, 13),
                (13, 12)
            ]));
        }

        #[test]
        fn converse_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();
            let converse = digraph.converse();

            assert_eq!(digraph.order(), converse.order());

            assert!(converse.arcs().eq([
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
