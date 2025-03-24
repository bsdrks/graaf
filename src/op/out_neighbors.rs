//! Iterate a vertex's out-neighbors.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     OutNeighbors,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 3);
//! digraph.add_arc(2, 0);
//! digraph.add_arc(2, 1);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(3, 1);
//! digraph.add_arc(3, 2);
//!
//! assert!(digraph.out_neighbors(0).eq([1, 2]));
//! assert!(digraph.out_neighbors(1).eq([0, 2, 3]));
//! assert!(digraph.out_neighbors(2).eq([0, 1, 3]));
//! assert!(digraph.out_neighbors(3).eq([1, 2]));
//! ```
#![doc(alias = "out_neighbours")]

/// Vertex out-neighbors
#[doc(alias = "IterOutNeighbours")]
pub trait OutNeighbors {
    /// Iterate the vertex's out-neighbors.
    ///
    /// # Arguments
    ///
    /// * `u`: The vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     OutNeighbors,
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
    /// assert!(digraph.out_neighbors(0).eq([1, 2]));
    /// assert!(digraph.out_neighbors(1).eq([0]));
    /// assert!(digraph.out_neighbors(2).eq([0, 1, 3]));
    /// assert!(digraph.out_neighbors(3).eq([0]));
    /// ```
    #[doc(alias = "out_neighbours")]
    #[must_use]
    fn out_neighbors(&self, u: usize) -> impl Iterator<Item = usize>;
}

/// `OutNeighbors` tests
#[macro_export]
macro_rules! test_out_neighbors {
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
        fn out_neighbors_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.out_neighbors(0).eq([1, 4, 7]));
            assert!(digraph.out_neighbors(1).eq([0, 2, 7]));
            assert!(digraph.out_neighbors(2).eq([3]));
            assert!(digraph.out_neighbors(3).eq([2, 4]));
            assert!(digraph.out_neighbors(4).eq([2]));
            assert!(digraph.out_neighbors(5).eq([6]));
            assert!(digraph.out_neighbors(6).eq([7]));
            assert!(digraph.out_neighbors(7).eq([5]));
        }

        #[test]
        fn out_neighbors_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.out_neighbors(0).eq([4]));
            assert!(digraph.out_neighbors(1).eq([0]));
            assert!(digraph.out_neighbors(2).eq([1, 3, 5]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([4]));
        }

        #[test]
        fn out_neighbors_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.out_neighbors(0).eq([1, 2]));
            assert!(digraph.out_neighbors(1).eq([3]));
            assert!(digraph.out_neighbors(2).eq([1, 3, 4, 5]));
            assert!(digraph.out_neighbors(3).eq([5]));
            assert!(digraph.out_neighbors(4).eq([6]));
            assert!(digraph.out_neighbors(5).eq([]));
            assert!(digraph.out_neighbors(6).eq([]));
        }

        #[test]
        fn out_neighbors_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.out_neighbors(0).eq([3, 4]));
            assert!(digraph.out_neighbors(1).eq([]));
            assert!(digraph.out_neighbors(2).eq([3, 4, 5]));
            assert!(digraph.out_neighbors(3).eq([1]));
            assert!(digraph.out_neighbors(4).eq([1]));
            assert!(digraph.out_neighbors(5).eq([1]));
        }

        #[test]
        fn out_neighbors_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.out_neighbors(0).eq([1]));
            assert!(digraph.out_neighbors(1).eq([0, 2, 4]));
            assert!(digraph.out_neighbors(2).eq([1]));
            assert!(digraph.out_neighbors(3).eq([2, 4, 5, 7, 10, 11]));
            assert!(digraph.out_neighbors(4).eq([3]));
            assert!(digraph.out_neighbors(5).eq([6]));
            assert!(digraph.out_neighbors(6).eq([5, 10]));
            assert!(digraph.out_neighbors(7).eq([3]));
            assert!(digraph.out_neighbors(8).eq([7, 10]));
            assert!(digraph.out_neighbors(9).eq([7, 11]));
            assert!(digraph.out_neighbors(10).eq([6]));
            assert!(digraph.out_neighbors(11).eq([9]));
        }

        #[test]
        fn out_neighbors_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.out_neighbors(0).eq([1]));
            assert!(digraph.out_neighbors(1).eq([0, 7]));
            assert!(digraph.out_neighbors(2).eq([0, 5, 7]));
            assert!(digraph.out_neighbors(3).eq([4]));
            assert!(digraph.out_neighbors(4).eq([3]));
            assert!(digraph.out_neighbors(5).eq([3, 6]));
            assert!(digraph.out_neighbors(6).eq([5]));
            assert!(digraph.out_neighbors(7).eq([2]));
            assert!(digraph.out_neighbors(8).eq([7, 9, 11]));
            assert!(digraph.out_neighbors(9).eq([8]));
            assert!(digraph.out_neighbors(10).eq([9, 11]));
            assert!(digraph.out_neighbors(11).eq([10]));
        }

        #[test]
        fn out_neighbors_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.out_neighbors(0).eq([]));
            assert!(digraph.out_neighbors(1).eq([]));
            assert!(digraph.out_neighbors(2).eq([]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([6, 9]));
            assert!(digraph.out_neighbors(6).eq([5]));
            assert!(digraph.out_neighbors(7).eq([]));
            assert!(digraph.out_neighbors(8).eq([]));
            assert!(digraph.out_neighbors(9).eq([5, 13]));
            assert!(digraph.out_neighbors(10).eq([]));
            assert!(digraph.out_neighbors(11).eq([]));
            assert!(digraph.out_neighbors(12).eq([]));
            assert!(digraph.out_neighbors(13).eq([9, 12]));
        }

        #[test]
        fn out_neighbors_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.out_neighbors(0).eq([]));
            assert!(digraph.out_neighbors(1).eq([]));
            assert!(digraph.out_neighbors(2).eq([]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([6, 9]));
            assert!(digraph.out_neighbors(6).eq([5]));
            assert!(digraph.out_neighbors(7).eq([]));
            assert!(digraph.out_neighbors(8).eq([]));
            assert!(digraph.out_neighbors(9).eq([5]));
            assert!(digraph.out_neighbors(10).eq([]));
            assert!(digraph.out_neighbors(11).eq([]));
            assert!(digraph.out_neighbors(12).eq([13]));
            assert!(digraph.out_neighbors(13).eq([9, 12]));
        }

        #[test]
        fn out_neighbors_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.out_neighbors(0).eq([]));
            assert!(digraph.out_neighbors(1).eq([2, 5]));
            assert!(digraph.out_neighbors(2).eq([1, 6]));
            assert!(digraph.out_neighbors(3).eq([]));
            assert!(digraph.out_neighbors(4).eq([]));
            assert!(digraph.out_neighbors(5).eq([1, 6, 9]));
            assert!(digraph.out_neighbors(6).eq([2, 5]));
            assert!(digraph.out_neighbors(7).eq([]));
            assert!(digraph.out_neighbors(8).eq([]));
            assert!(digraph.out_neighbors(9).eq([5, 13]));
            assert!(digraph.out_neighbors(10).eq([]));
            assert!(digraph.out_neighbors(11).eq([]));
            assert!(digraph.out_neighbors(12).eq([13]));
            assert!(digraph.out_neighbors(13).eq([9, 12]));
        }
    };
}
