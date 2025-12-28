//! Remove an arc from a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Arcs,
//!     Empty,
//!     RemoveArc,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(2, 1);
//!
//! assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 1)]));
//!
//! assert!(digraph.remove_arc(0, 1));
//! assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
//!
//! assert!(digraph.remove_arc(0, 2));
//! assert!(digraph.arcs().eq([(1, 0), (2, 1)]));
//!
//! assert!(digraph.remove_arc(1, 0));
//! assert!(digraph.arcs().eq([(2, 1)]));
//!
//! assert!(digraph.remove_arc(2, 1));
//! assert!(digraph.arcs().eq([]));
//! ```

/// Remove an arc from a digraph.
pub trait RemoveArc {
    /// Remove the arc from the digraph. Return whether the arc was removed.
    ///
    /// # Arguments
    ///
    /// * `u`: The tail vertex.
    /// * `v`: The head vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Arcs,
    ///     Empty,
    ///     RemoveArc,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(2, 1);
    ///
    /// assert!(digraph.arcs().eq([(0, 1), (0, 2), (1, 0), (2, 1)]));
    ///
    /// assert!(digraph.remove_arc(0, 1));
    /// assert!(digraph.arcs().eq([(0, 2), (1, 0), (2, 1)]));
    ///
    /// assert!(digraph.remove_arc(0, 2));
    /// assert!(digraph.arcs().eq([(1, 0), (2, 1)]));
    ///
    /// assert!(digraph.remove_arc(1, 0));
    /// assert!(digraph.arcs().eq([(2, 1)]));
    ///
    /// assert!(digraph.remove_arc(2, 1));
    /// assert!(digraph.arcs().eq([]));
    /// ```
    #[must_use]
    fn remove_arc(&mut self, u: usize, v: usize) -> bool;
}

/// `RemoveArc` tests
#[macro_export]
macro_rules! test_remove_arc {
    ($type:ty, $fixture:path) => {
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
        fn remove_arc_bang_jensen_196() {
            let mut digraph = bang_jensen_196();

            assert!(digraph.arcs().eq([
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

            assert!(!digraph.remove_arc(8, 0));
            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(0, 4));
            assert!(digraph.remove_arc(0, 7));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(1, 2));
            assert!(digraph.remove_arc(1, 7));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(3, 2));
            assert!(digraph.remove_arc(3, 4));
            assert!(digraph.remove_arc(4, 2));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(6, 7));
            assert!(digraph.remove_arc(7, 5));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(0, 4));
            assert!(!digraph.remove_arc(0, 7));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(1, 2));
            assert!(!digraph.remove_arc(1, 7));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(3, 2));
            assert!(!digraph.remove_arc(3, 4));
            assert!(!digraph.remove_arc(4, 2));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(6, 7));
            assert!(!digraph.remove_arc(7, 5));
        }

        #[test]
        fn remove_arc_bang_jensen_34() {
            let mut digraph = bang_jensen_34();

            assert!(digraph.arcs().eq([
                (0, 4),
                (1, 0),
                (2, 1),
                (2, 3),
                (2, 5),
                (5, 4)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(0, 4));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(5, 4));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 4));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(5, 4));
        }

        #[test]
        fn remove_arc_bang_jensen_94() {
            let mut digraph = bang_jensen_94();

            assert!(digraph.arcs().eq([
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

            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(0, 2));
            assert!(digraph.remove_arc(1, 3));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(2, 4));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(3, 5));
            assert!(digraph.remove_arc(4, 6));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(0, 2));
            assert!(!digraph.remove_arc(1, 3));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(2, 4));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(3, 5));
            assert!(!digraph.remove_arc(4, 6));
        }

        #[test]
        fn remove_arc_kattis_builddeps() {
            let mut digraph = kattis_builddeps();

            assert!(digraph.arcs().eq([
                (0, 3),
                (0, 4),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 1),
                (4, 1),
                (5, 1)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(0, 3));
            assert!(digraph.remove_arc(0, 4));
            assert!(digraph.remove_arc(2, 3));
            assert!(digraph.remove_arc(2, 4));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(3, 1));
            assert!(digraph.remove_arc(4, 1));
            assert!(digraph.remove_arc(5, 1));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 3));
            assert!(!digraph.remove_arc(0, 4));
            assert!(!digraph.remove_arc(2, 3));
            assert!(!digraph.remove_arc(2, 4));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(3, 1));
            assert!(!digraph.remove_arc(4, 1));
            assert!(!digraph.remove_arc(5, 1));
        }

        #[test]
        fn remove_arc_kattis_cantinaofbabel_1() {
            let mut digraph = kattis_cantinaofbabel_1();

            assert!(digraph.arcs().eq([
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

            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(1, 2));
            assert!(digraph.remove_arc(1, 4));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(3, 2));
            assert!(digraph.remove_arc(3, 4));
            assert!(digraph.remove_arc(3, 5));
            assert!(digraph.remove_arc(3, 7));
            assert!(digraph.remove_arc(3, 10));
            assert!(digraph.remove_arc(3, 11));
            assert!(digraph.remove_arc(4, 3));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(6, 10));
            assert!(digraph.remove_arc(7, 3));
            assert!(digraph.remove_arc(8, 7));
            assert!(digraph.remove_arc(8, 10));
            assert!(digraph.remove_arc(9, 7));
            assert!(digraph.remove_arc(9, 11));
            assert!(digraph.remove_arc(10, 6));
            assert!(digraph.remove_arc(11, 9));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(1, 2));
            assert!(!digraph.remove_arc(1, 4));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(3, 2));
            assert!(!digraph.remove_arc(3, 4));
            assert!(!digraph.remove_arc(3, 5));
            assert!(!digraph.remove_arc(3, 7));
            assert!(!digraph.remove_arc(3, 10));
            assert!(!digraph.remove_arc(3, 11));
            assert!(!digraph.remove_arc(4, 3));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(6, 10));
            assert!(!digraph.remove_arc(7, 3));
            assert!(!digraph.remove_arc(8, 7));
            assert!(!digraph.remove_arc(8, 10));
            assert!(!digraph.remove_arc(9, 7));
            assert!(!digraph.remove_arc(9, 11));
            assert!(!digraph.remove_arc(10, 6));
            assert!(!digraph.remove_arc(11, 9));
        }

        #[test]
        fn remove_arc_kattis_cantinaofbabel_2() {
            let mut digraph = kattis_cantinaofbabel_2();

            assert!(digraph.arcs().eq([
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
                (11, 10),
            ]));

            assert!(!digraph.remove_arc(0, 3));

            assert!(digraph.remove_arc(0, 1));
            assert!(digraph.remove_arc(1, 0));
            assert!(digraph.remove_arc(1, 7));
            assert!(digraph.remove_arc(2, 0));
            assert!(digraph.remove_arc(2, 5));
            assert!(digraph.remove_arc(2, 7));
            assert!(digraph.remove_arc(3, 4));
            assert!(digraph.remove_arc(4, 3));
            assert!(digraph.remove_arc(5, 3));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(7, 2));
            assert!(digraph.remove_arc(8, 7));
            assert!(digraph.remove_arc(8, 9));
            assert!(digraph.remove_arc(8, 11));
            assert!(digraph.remove_arc(9, 8));
            assert!(digraph.remove_arc(10, 9));
            assert!(digraph.remove_arc(10, 11));
            assert!(digraph.remove_arc(11, 10));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(0, 1));
            assert!(!digraph.remove_arc(1, 0));
            assert!(!digraph.remove_arc(1, 7));
            assert!(!digraph.remove_arc(2, 0));
            assert!(!digraph.remove_arc(2, 5));
            assert!(!digraph.remove_arc(2, 7));
            assert!(!digraph.remove_arc(3, 4));
            assert!(!digraph.remove_arc(4, 3));
            assert!(!digraph.remove_arc(5, 3));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(7, 2));
            assert!(!digraph.remove_arc(8, 7));
            assert!(!digraph.remove_arc(8, 9));
            assert!(!digraph.remove_arc(8, 11));
            assert!(!digraph.remove_arc(9, 8));
            assert!(!digraph.remove_arc(10, 9));
            assert!(!digraph.remove_arc(10, 11));
            assert!(!digraph.remove_arc(11, 10));
        }

        #[test]
        fn remove_arc_kattis_escapewallmaria_1() {
            let mut digraph = kattis_escapewallmaria_1();

            assert!(digraph.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (9, 13),
                (13, 9),
                (13, 12)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(5, 9));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(9, 5));
            assert!(digraph.remove_arc(9, 13));
            assert!(digraph.remove_arc(13, 9));
            assert!(digraph.remove_arc(13, 12));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(5, 9));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(9, 5));
            assert!(!digraph.remove_arc(9, 13));
            assert!(!digraph.remove_arc(13, 9));
            assert!(!digraph.remove_arc(13, 12));
        }

        #[test]
        fn remove_arc_kattis_escapewallmaria_2() {
            let mut digraph = kattis_escapewallmaria_2();

            assert!(digraph.arcs().eq([
                (5, 6),
                (5, 9),
                (6, 5),
                (9, 5),
                (12, 13),
                (13, 9),
                (13, 12)
            ]));

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(5, 9));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(9, 5));
            assert!(digraph.remove_arc(12, 13));
            assert!(digraph.remove_arc(13, 9));
            assert!(digraph.remove_arc(13, 12));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(5, 9));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(9, 5));
            assert!(!digraph.remove_arc(12, 13));
            assert!(!digraph.remove_arc(13, 9));
            assert!(!digraph.remove_arc(13, 12));
        }

        #[test]
        fn remove_arc_kattis_escapewallmaria_3() {
            let mut digraph = kattis_escapewallmaria_3();

            assert!(digraph.arcs().eq([
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

            assert!(!digraph.remove_arc(0, 1));

            assert!(digraph.remove_arc(1, 2));
            assert!(digraph.remove_arc(1, 5));
            assert!(digraph.remove_arc(2, 1));
            assert!(digraph.remove_arc(2, 6));
            assert!(digraph.remove_arc(5, 1));
            assert!(digraph.remove_arc(5, 6));
            assert!(digraph.remove_arc(5, 9));
            assert!(digraph.remove_arc(6, 2));
            assert!(digraph.remove_arc(6, 5));
            assert!(digraph.remove_arc(9, 5));
            assert!(digraph.remove_arc(9, 13));
            assert!(digraph.remove_arc(12, 13));
            assert!(digraph.remove_arc(13, 9));
            assert!(digraph.remove_arc(13, 12));

            assert!(digraph.arcs().eq([]));

            assert!(!digraph.remove_arc(1, 2));
            assert!(!digraph.remove_arc(1, 5));
            assert!(!digraph.remove_arc(2, 1));
            assert!(!digraph.remove_arc(2, 6));
            assert!(!digraph.remove_arc(5, 1));
            assert!(!digraph.remove_arc(5, 6));
            assert!(!digraph.remove_arc(5, 9));
            assert!(!digraph.remove_arc(6, 2));
            assert!(!digraph.remove_arc(6, 5));
            assert!(!digraph.remove_arc(9, 5));
            assert!(!digraph.remove_arc(9, 13));
            assert!(!digraph.remove_arc(12, 13));
            assert!(!digraph.remove_arc(13, 9));
            assert!(!digraph.remove_arc(13, 12));
        }

        #[test]
        fn remove_arc_out_of_bounds() {
            assert!(!<$type>::trivial().remove_arc(0, 1));
            assert!(!<$type>::trivial().remove_arc(1, 0));
        }
    };
}
