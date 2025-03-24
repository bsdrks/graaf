//! Iterate a digraph's outdegree sequence.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     OutdegreeSequence,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
//! ```

use crate::{
    Outdegree,
    Vertices,
};

/// Digraph outdegree sequence
pub trait OutdegreeSequence {
    /// Iterate the digraph's outdegree sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     OutdegreeSequence,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.outdegree_sequence().eq([2, 1, 1]));
    /// ```
    #[must_use]
    fn outdegree_sequence(&self) -> impl Iterator<Item = usize>;
}

impl<D> OutdegreeSequence for D
where
    D: Outdegree + Vertices,
{
    fn outdegree_sequence(&self) -> impl Iterator<Item = usize> {
        self.vertices().map(move |v| self.outdegree(v))
    }
}

/// `OutdegreeSequence` tests
#[macro_export]
macro_rules! test_outdegree_sequence {
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
        fn outdegree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196()
                .outdegree_sequence()
                .eq([3, 3, 1, 2, 1, 1, 1, 1]));
        }

        #[test]
        fn outdegree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34()
                .outdegree_sequence()
                .eq([1, 1, 3, 0, 0, 1]));
        }

        #[test]
        fn outdegree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94()
                .outdegree_sequence()
                .eq([2, 1, 4, 1, 1, 0, 0]));
        }

        #[test]
        fn outdegree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps()
                .outdegree_sequence()
                .eq([2, 0, 3, 1, 1, 1]));
        }

        #[test]
        fn outdegree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1()
                .outdegree_sequence()
                .eq([1, 3, 1, 6, 1, 1, 2, 1, 2, 2, 1, 1]));
        }

        #[test]
        fn outdegree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2()
                .outdegree_sequence()
                .eq([1, 2, 3, 1, 1, 2, 1, 1, 3, 1, 2, 1]));
        }

        #[test]
        fn outdegree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .outdegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 0, 2, 0, 0]));
        }

        #[test]
        fn outdegree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .outdegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 1, 0, 0, 1, 2, 0, 0]));
        }

        #[test]
        fn outdegree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .outdegree_sequence()
                .eq([0, 2, 2, 0, 0, 3, 2, 0, 0, 2, 0, 0, 1, 2, 0, 0]));
        }
    };
}
