//! Iterate a digraph's degree sequence.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     DegreeSequence,
//!     Empty,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.degree_sequence().eq([3, 2, 3]));
//! ```

/// Digraph degree sequence
pub trait DegreeSequence {
    /// Iterate the digraph's degree sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     DegreeSequence,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.degree_sequence().eq([3, 2, 3]));
    /// ```
    #[must_use]
    fn degree_sequence(&self) -> impl Iterator<Item = usize>;
}

/// `DegreeSequence` tests
#[macro_export]
macro_rules! test_degree_sequence {
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
        fn degree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196()
                .degree_sequence()
                .eq([4, 4, 4, 3, 3, 2, 2, 4]));
        }

        #[test]
        fn degree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34().degree_sequence().eq([2, 2, 3, 1, 2, 2]));
        }

        #[test]
        fn degree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94()
                .degree_sequence()
                .eq([2, 3, 5, 3, 2, 2, 1]));
        }

        #[test]
        fn degree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps()
                .degree_sequence()
                .eq([2, 3, 3, 3, 3, 2]));
        }

        #[test]
        fn degree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1()
                .degree_sequence()
                .eq([2, 5, 3, 8, 3, 3, 4, 4, 2, 3, 4, 3]));
        }

        #[test]
        fn degree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2()
                .degree_sequence()
                .eq([3, 3, 4, 3, 2, 4, 2, 4, 4, 3, 3, 3]));
        }

        #[test]
        fn degree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .degree_sequence()
                .eq([0, 0, 0, 0, 0, 4, 2, 0, 0, 4, 0, 0, 1, 3, 0, 0]));
        }

        #[test]
        fn degree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .degree_sequence()
                .eq([0, 0, 0, 0, 0, 4, 2, 0, 0, 3, 0, 0, 2, 3, 0, 0]));
        }

        #[test]
        fn degree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .degree_sequence()
                .eq([0, 4, 4, 0, 0, 6, 4, 0, 0, 4, 0, 0, 2, 4, 0, 0]));
        }
    };
}
