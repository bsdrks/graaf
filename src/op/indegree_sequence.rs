//! Iterate a digraph's indegree sequence.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IndegreeSequence,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.indegree_sequence().eq([1, 1, 2]));
//! ```

/// Digraph indegree sequence
pub trait IndegreeSequence {
    /// Iterate the digraph's indegree sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IndegreeSequence,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    ///
    /// assert!(digraph.indegree_sequence().eq([1, 1, 2]));
    /// ```
    #[must_use]
    fn indegree_sequence(&self) -> impl Iterator<Item = usize>;
}

/// `IndegreeSequence` tests
#[macro_export]
macro_rules! test_indegree_sequence {
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
        fn indegree_sequence_bang_jensen_196() {
            assert!(bang_jensen_196()
                .indegree_sequence()
                .eq([1, 1, 3, 1, 2, 1, 1, 3]));
        }

        #[test]
        fn indegree_sequence_bang_jensen_34() {
            assert!(bang_jensen_34()
                .indegree_sequence()
                .eq([1, 1, 0, 1, 2, 1]));
        }

        #[test]
        fn indegree_sequence_bang_jensen_94() {
            assert!(bang_jensen_94()
                .indegree_sequence()
                .eq([0, 2, 1, 2, 1, 2, 1]));
        }

        #[test]
        fn indegree_sequence_kattis_builddeps() {
            assert!(kattis_builddeps()
                .indegree_sequence()
                .eq([0, 3, 0, 2, 2, 1]));
        }

        #[test]
        fn indegree_sequence_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1()
                .indegree_sequence()
                .eq([1, 2, 2, 2, 2, 2, 2, 3, 0, 1, 3, 2]));
        }

        #[test]
        fn indegree_sequence_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2()
                .indegree_sequence()
                .eq([2, 1, 1, 2, 1, 2, 1, 3, 1, 2, 1, 2]));
        }

        #[test]
        fn indegree_sequence_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1()
                .indegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 1, 1, 0, 0]));
        }

        #[test]
        fn indegree_sequence_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2()
                .indegree_sequence()
                .eq([0, 0, 0, 0, 0, 2, 1, 0, 0, 2, 0, 0, 1, 1, 0, 0]));
        }

        #[test]
        fn indegree_sequence_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3()
                .indegree_sequence()
                .eq([0, 2, 2, 0, 0, 3, 2, 0, 0, 2, 0, 0, 1, 2, 0, 0]));
        }
    };
}
