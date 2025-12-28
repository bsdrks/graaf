//! Iterate a vertex's in-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     InNeighbors,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(2, 0);
//! digraph.add_arc(2, 1);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(3, 0);
//!
//! assert!(digraph.in_neighbors(0).eq([1, 2, 3]));
//! assert!(digraph.in_neighbors(1).eq([0, 2]));
//! assert!(digraph.in_neighbors(2).eq([0]));
//! assert!(digraph.in_neighbors(3).eq([2]));
//! ```
#![doc(alias = "iter_in_neighbours")]

/// Iterate a vertex's in-neighbors.
#[doc(alias = "InNeighbours")]
pub trait InNeighbors {
    /// Iterate the vertex's in-neighbors.
    ///
    /// # Arguments
    ///
    /// * `v`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     InNeighbors,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(2, 1);
    /// digraph.add_arc(2, 3);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(digraph.in_neighbors(0).eq([1, 2, 3]));
    /// assert!(digraph.in_neighbors(1).eq([0, 2]));
    /// assert!(digraph.in_neighbors(2).eq([0]));
    /// assert!(digraph.in_neighbors(3).eq([2]));
    /// ```
    #[doc(alias = "in_neighbours")]
    #[must_use]
    fn in_neighbors(&self, v: usize) -> impl Iterator<Item = usize>;
}

/// `InNeighbors` tests
#[macro_export]
macro_rules! test_in_neighbors {
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
        fn in_neighbors_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.in_neighbors(0).eq([1]));
            assert!(digraph.in_neighbors(1).eq([0]));
            assert!(digraph.in_neighbors(2).eq([1, 3, 4]));
            assert!(digraph.in_neighbors(3).eq([2]));
            assert!(digraph.in_neighbors(4).eq([0, 3]));
            assert!(digraph.in_neighbors(5).eq([7]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([0, 1, 6]));
        }

        #[test]
        fn in_neighbors_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.in_neighbors(0).eq([1]));
            assert!(digraph.in_neighbors(1).eq([2]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([2]));
            assert!(digraph.in_neighbors(4).eq([0, 5]));
            assert!(digraph.in_neighbors(5).eq([2]));
        }

        #[test]
        fn in_neighbors_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([0, 2]));
            assert!(digraph.in_neighbors(2).eq([0]));
            assert!(digraph.in_neighbors(3).eq([1, 2]));
            assert!(digraph.in_neighbors(4).eq([2]));
            assert!(digraph.in_neighbors(5).eq([2, 3]));
            assert!(digraph.in_neighbors(6).eq([4]));
        }

        #[test]
        fn in_neighbors_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([3, 4, 5]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([0, 2]));
            assert!(digraph.in_neighbors(4).eq([0, 2]));
            assert!(digraph.in_neighbors(5).eq([2]));
        }

        #[test]
        fn in_neighbors_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.in_neighbors(0).eq([1]));
            assert!(digraph.in_neighbors(1).eq([0, 2]));
            assert!(digraph.in_neighbors(2).eq([1, 3]));
            assert!(digraph.in_neighbors(3).eq([4, 7]));
            assert!(digraph.in_neighbors(4).eq([1, 3]));
            assert!(digraph.in_neighbors(5).eq([3, 6]));
            assert!(digraph.in_neighbors(6).eq([5, 10]));
            assert!(digraph.in_neighbors(7).eq([3, 8, 9]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([11]));
            assert!(digraph.in_neighbors(10).eq([3, 6, 8]));
            assert!(digraph.in_neighbors(11).eq([3, 9]));
        }

        #[test]
        fn in_neighbors_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.in_neighbors(0).eq([1, 2]));
            assert!(digraph.in_neighbors(1).eq([0]));
            assert!(digraph.in_neighbors(2).eq([7]));
            assert!(digraph.in_neighbors(3).eq([4, 5]));
            assert!(digraph.in_neighbors(4).eq([3]));
            assert!(digraph.in_neighbors(5).eq([2, 6]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([1, 2, 8]));
            assert!(digraph.in_neighbors(8).eq([9]));
            assert!(digraph.in_neighbors(9).eq([8, 10]));
            assert!(digraph.in_neighbors(10).eq([11]));
            assert!(digraph.in_neighbors(11).eq([8, 10]));
        }

        #[test]
        fn in_neighbors_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([]));
            assert!(digraph.in_neighbors(4).eq([]));
            assert!(digraph.in_neighbors(5).eq([6, 9]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([5, 13]));
            assert!(digraph.in_neighbors(10).eq([]));
            assert!(digraph.in_neighbors(11).eq([]));
            assert!(digraph.in_neighbors(12).eq([13]));
            assert!(digraph.in_neighbors(13).eq([9]));
            assert!(digraph.in_neighbors(14).eq([]));
            assert!(digraph.in_neighbors(15).eq([]));
        }

        #[test]
        fn in_neighbors_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([]));
            assert!(digraph.in_neighbors(2).eq([]));
            assert!(digraph.in_neighbors(3).eq([]));
            assert!(digraph.in_neighbors(4).eq([]));
            assert!(digraph.in_neighbors(5).eq([6, 9]));
            assert!(digraph.in_neighbors(6).eq([5]));
            assert!(digraph.in_neighbors(7).eq([]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([5, 13]));
            assert!(digraph.in_neighbors(10).eq([]));
            assert!(digraph.in_neighbors(11).eq([]));
            assert!(digraph.in_neighbors(12).eq([13]));
            assert!(digraph.in_neighbors(13).eq([12]));
            assert!(digraph.in_neighbors(14).eq([]));
            assert!(digraph.in_neighbors(15).eq([]));
        }

        #[test]
        fn in_neighbors_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.in_neighbors(0).eq([]));
            assert!(digraph.in_neighbors(1).eq([2, 5]));
            assert!(digraph.in_neighbors(2).eq([1, 6]));
            assert!(digraph.in_neighbors(3).eq([]));
            assert!(digraph.in_neighbors(4).eq([]));
            assert!(digraph.in_neighbors(5).eq([1, 6, 9]));
            assert!(digraph.in_neighbors(6).eq([2, 5]));
            assert!(digraph.in_neighbors(7).eq([]));
            assert!(digraph.in_neighbors(8).eq([]));
            assert!(digraph.in_neighbors(9).eq([5, 13]));
            assert!(digraph.in_neighbors(10).eq([]));
            assert!(digraph.in_neighbors(11).eq([]));
            assert!(digraph.in_neighbors(12).eq([13]));
            assert!(digraph.in_neighbors(13).eq([9, 12]));
            assert!(digraph.in_neighbors(14).eq([]));
            assert!(digraph.in_neighbors(15).eq([]));
        }
    };
}
