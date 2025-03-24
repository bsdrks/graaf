//! Iterate a digraph's semidegree sequence.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     SemidegreeSequence,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
//! ```

use crate::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Digraph semidegree sequence
pub trait SemidegreeSequence {
    /// Iterate the semidegree sequence of a digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     SemidegreeSequence,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.semidegree_sequence().eq([(1, 2), (1, 1), (2, 1)]));
    /// ```
    #[must_use]
    fn semidegree_sequence(&self) -> impl Iterator<Item = (usize, usize)>;
}

impl<D> SemidegreeSequence for D
where
    D: Indegree + Outdegree + Vertices,
{
    fn semidegree_sequence(&self) -> impl Iterator<Item = (usize, usize)> {
        self.vertices()
            .map(|u| (self.indegree(u), self.outdegree(u)))
    }
}

/// `SemidegreeSequence` tests
#[macro_export]
macro_rules! test_semidegree_sequence {
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
        fn semidegree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196().semidegree_sequence().eq([
                (1, 3),
                (1, 3),
                (3, 1),
                (1, 2),
                (2, 1),
                (1, 1),
                (1, 1),
                (3, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34().semidegree_sequence().eq([
                (1, 1),
                (1, 1),
                (0, 3),
                (1, 0),
                (2, 0),
                (1, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94().semidegree_sequence().eq([
                (0, 2),
                (2, 1),
                (1, 4),
                (2, 1),
                (1, 1),
                (2, 0),
                (1, 0)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps().semidegree_sequence().eq([
                (0, 2),
                (3, 0),
                (0, 3),
                (2, 1),
                (2, 1),
                (1, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().semidegree_sequence().eq([
                (1, 1),
                (2, 3),
                (2, 1),
                (2, 6),
                (2, 1),
                (2, 1),
                (2, 2),
                (3, 1),
                (0, 2),
                (1, 2),
                (3, 1),
                (2, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().semidegree_sequence().eq([
                (2, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (1, 1),
                (2, 2),
                (1, 1),
                (3, 1),
                (1, 3),
                (2, 1),
                (1, 2),
                (2, 1)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1().semidegree_sequence().eq([
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (2, 2),
                (0, 0),
                (0, 0),
                (1, 0),
                (1, 2),
                (0, 0),
                (0, 0)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2().semidegree_sequence().eq([
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (2, 1),
                (0, 0),
                (0, 0),
                (1, 1),
                (1, 2),
                (0, 0),
                (0, 0)
            ]));
        }

        #[test]
        fn semidegree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().semidegree_sequence().eq([
                (0, 0),
                (2, 2),
                (2, 2),
                (0, 0),
                (0, 0),
                (3, 3),
                (2, 2),
                (0, 0),
                (0, 0),
                (2, 2),
                (0, 0),
                (0, 0),
                (1, 1),
                (2, 2),
                (0, 0),
                (0, 0)
            ]));
        }
    };
}
