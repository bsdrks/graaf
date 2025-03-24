//! Return a vertex's degree.
//!
//! A vertex's degree is the sum of its indegree and outdegree.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Degree,
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
//! assert_eq!(digraph.degree(0), 3);
//! assert_eq!(digraph.degree(1), 2);
//! assert_eq!(digraph.degree(2), 3);
//! ```
#![doc(alias = "semidegree")]
#![doc(alias = "valence")]
#![doc(alias = "valency")]

use crate::{
    Indegree,
    Outdegree,
    Vertices,
};

/// Vertex degree
#[doc(alias = "Semidegree")]
#[doc(alias = "Valence")]
#[doc(alias = "Valency")]
pub trait Degree {
    /// Return a vertex's degree.
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
    ///     Degree,
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
    /// assert_eq!(digraph.degree(0), 3);
    /// assert_eq!(digraph.degree(1), 2);
    /// assert_eq!(digraph.degree(2), 3);
    /// ```
    #[doc(alias = "semidegree")]
    #[doc(alias = "valence")]
    #[doc(alias = "valency")]
    #[must_use]
    fn degree(&self, u: usize) -> usize;

    /// Return a digraph's maximum degree.
    ///
    /// # Examples
    ///
    /// The maximum degree of this digraph is `6`. The vertex with the maximum
    /// degree is red.
    ///
    /// ![A digraph and its maximum degree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/max_degree-0.88.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Degree,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(0, 3);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert_eq!(digraph.max_degree(), 6);
    /// ```
    #[must_use]
    fn max_degree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.degree(u)).max().unwrap_or(0)
    }

    /// Return a digraph's minimum degree.
    ///
    /// # Examples
    ///
    /// The minimum degree of this digraph is `2`. The vertex with the minimum
    /// degree is red.
    ///
    /// ![A digraph and its minimum degree](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/min_degree-0.88.4.svg?)
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Degree,
    ///     Empty,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(0, 3);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(1, 2);
    /// digraph.add_arc(2, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert_eq!(digraph.min_degree(), 2);
    /// ```
    #[must_use]
    fn min_degree(&self) -> usize
    where
        Self: Vertices,
    {
        self.vertices().map(|u| self.degree(u)).min().unwrap_or(0)
    }
}

impl<D> Degree for D
where
    D: Indegree + Outdegree,
{
    fn degree(&self, u: usize) -> usize {
        self.indegree(u) + self.outdegree(u)
    }
}

/// `Degree` tests
#[macro_export]
macro_rules! test_degree {
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
        fn degree_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(digraph.degree(0) == 4);
            assert!(digraph.degree(1) == 4);
            assert!(digraph.degree(2) == 4);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 3);
            assert!(digraph.degree(5) == 2);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 4);
        }

        #[test]
        fn degree_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 2);
            assert!(digraph.degree(2) == 3);
            assert!(digraph.degree(3) == 1);
            assert!(digraph.degree(4) == 2);
            assert!(digraph.degree(5) == 2);
        }

        #[test]
        fn degree_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 3);
            assert!(digraph.degree(2) == 5);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 2);
            assert!(digraph.degree(5) == 2);
            assert!(digraph.degree(6) == 1);
        }

        #[test]
        fn degree_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 3);
            assert!(digraph.degree(2) == 3);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 3);
            assert!(digraph.degree(5) == 2);
        }

        #[test]
        fn degree_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(digraph.degree(0) == 2);
            assert!(digraph.degree(1) == 5);
            assert!(digraph.degree(2) == 3);
            assert!(digraph.degree(3) == 8);
            assert!(digraph.degree(4) == 3);
            assert!(digraph.degree(5) == 3);
            assert!(digraph.degree(6) == 4);
            assert!(digraph.degree(7) == 4);
            assert!(digraph.degree(8) == 2);
            assert!(digraph.degree(9) == 3);
            assert!(digraph.degree(10) == 4);
            assert!(digraph.degree(11) == 3);
        }

        #[test]
        fn degree_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(digraph.degree(0) == 3);
            assert!(digraph.degree(1) == 3);
            assert!(digraph.degree(2) == 4);
            assert!(digraph.degree(3) == 3);
            assert!(digraph.degree(4) == 2);
            assert!(digraph.degree(5) == 4);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 4);
            assert!(digraph.degree(8) == 4);
            assert!(digraph.degree(9) == 3);
            assert!(digraph.degree(10) == 3);
            assert!(digraph.degree(11) == 3);
        }

        #[test]
        fn degree_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(digraph.degree(0) == 0);
            assert!(digraph.degree(1) == 0);
            assert!(digraph.degree(2) == 0);
            assert!(digraph.degree(3) == 0);
            assert!(digraph.degree(4) == 0);
            assert!(digraph.degree(5) == 4);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 0);
            assert!(digraph.degree(8) == 0);
            assert!(digraph.degree(9) == 4);
            assert!(digraph.degree(10) == 0);
            assert!(digraph.degree(11) == 0);
            assert!(digraph.degree(12) == 1);
            assert!(digraph.degree(13) == 3);
        }

        #[test]
        fn degree_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(digraph.degree(0) == 0);
            assert!(digraph.degree(1) == 0);
            assert!(digraph.degree(2) == 0);
            assert!(digraph.degree(3) == 0);
            assert!(digraph.degree(4) == 0);
            assert!(digraph.degree(5) == 4);
            assert!(digraph.degree(6) == 2);
            assert!(digraph.degree(7) == 0);
            assert!(digraph.degree(8) == 0);
            assert!(digraph.degree(9) == 3);
            assert!(digraph.degree(10) == 0);
            assert!(digraph.degree(11) == 0);
            assert!(digraph.degree(12) == 2);
            assert!(digraph.degree(13) == 3);
        }

        #[test]
        fn degree_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(digraph.degree(0) == 0);
            assert!(digraph.degree(1) == 4);
            assert!(digraph.degree(2) == 4);
            assert!(digraph.degree(3) == 0);
            assert!(digraph.degree(4) == 0);
            assert!(digraph.degree(5) == 6);
            assert!(digraph.degree(6) == 4);
            assert!(digraph.degree(7) == 0);
            assert!(digraph.degree(8) == 0);
            assert!(digraph.degree(9) == 4);
            assert!(digraph.degree(10) == 0);
            assert!(digraph.degree(11) == 0);
            assert!(digraph.degree(12) == 2);
            assert!(digraph.degree(13) == 4);
        }

        #[test]
        fn max_degree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().max_degree(), 4);
        }

        #[test]
        fn max_degree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().max_degree(), 3);
        }

        #[test]
        fn max_degree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().max_degree(), 5);
        }

        #[test]
        fn max_degree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().max_degree(), 3);
        }

        #[test]
        fn max_degree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().max_degree(), 8);
        }

        #[test]
        fn max_degree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().max_degree(), 4);
        }

        #[test]
        fn max_degree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().max_degree(), 4);
        }

        #[test]
        fn max_degree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().max_degree(), 4);
        }

        #[test]
        fn max_degree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().max_degree(), 6);
        }

        #[test]
        fn min_degree_bang_jensen_196() {
            assert_eq!(bang_jensen_196().min_degree(), 2);
        }

        #[test]
        fn min_degree_bang_jensen_34() {
            assert_eq!(bang_jensen_34().min_degree(), 1);
        }

        #[test]
        fn min_degree_bang_jensen_94() {
            assert_eq!(bang_jensen_94().min_degree(), 1);
        }

        #[test]
        fn min_degree_kattis_builddeps() {
            assert_eq!(kattis_builddeps().min_degree(), 2);
        }

        #[test]
        fn min_degree_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().min_degree(), 2);
        }

        #[test]
        fn min_degree_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().min_degree(), 2);
        }

        #[test]
        fn min_degree_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().min_degree(), 0);
        }

        #[test]
        fn min_degree_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().min_degree(), 0);
        }

        #[test]
        fn min_degree_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().min_degree(), 0);
        }
    };
}
