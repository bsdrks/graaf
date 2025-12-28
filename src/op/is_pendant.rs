//! Check whether a vertex is pendant.
//!
//! A pendant vertex has a degree of one.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsPendant,
//! };
//!
//! let mut digraph = AdjacencyList::empty(4);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(0, 2);
//! digraph.add_arc(1, 0);
//! digraph.add_arc(3, 0);
//!
//! assert!(!digraph.is_pendant(0));
//! assert!(!digraph.is_pendant(1));
//! assert!(digraph.is_pendant(2));
//! assert!(digraph.is_pendant(3));
//! ```

use crate::Degree;

/// Check whether a vertex is pendant.
pub trait IsPendant {
    /// Check whether the vertex is pendant.
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
    ///     IsPendant,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(4);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 0);
    /// digraph.add_arc(3, 0);
    ///
    /// assert!(!digraph.is_pendant(0));
    /// assert!(!digraph.is_pendant(1));
    /// assert!(digraph.is_pendant(2));
    /// assert!(digraph.is_pendant(3));
    /// ```
    #[must_use]
    fn is_pendant(&self, u: usize) -> bool;
}

impl<D> IsPendant for D
where
    D: Degree,
{
    fn is_pendant(&self, u: usize) -> bool {
        self.degree(u) == 1
    }
}

/// `IsPendant` tests
#[macro_export]
macro_rules! test_is_pendant {
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
        fn is_pendant_bang_jensen_196() {
            let digraph = bang_jensen_196();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
        }

        #[test]
        fn is_pendant_bang_jensen_34() {
            let digraph = bang_jensen_34();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
        }

        #[test]
        fn is_pendant_bang_jensen_94() {
            let digraph = bang_jensen_94();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(digraph.is_pendant(6));
        }

        #[test]
        fn is_pendant_kattis_builddeps() {
            let digraph = kattis_builddeps();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
        }

        #[test]
        fn is_pendant_kattis_cantinaofbabel_1() {
            let digraph = kattis_cantinaofbabel_1();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
        }

        #[test]
        fn is_pendant_kattis_cantinaofbabel_2() {
            let digraph = kattis_cantinaofbabel_2();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
        }

        #[test]
        fn is_pendant_kattis_escapewallmaria_1() {
            let digraph = kattis_escapewallmaria_1();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
            assert!(digraph.is_pendant(12));
            assert!(!digraph.is_pendant(13));
            assert!(!digraph.is_pendant(14));
            assert!(!digraph.is_pendant(15));
        }

        #[test]
        fn is_pendant_kattis_escapewallmaria_2() {
            let digraph = kattis_escapewallmaria_2();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
            assert!(!digraph.is_pendant(12));
            assert!(!digraph.is_pendant(13));
            assert!(!digraph.is_pendant(14));
            assert!(!digraph.is_pendant(15));
        }

        #[test]
        fn is_pendant_kattis_escapewallmaria_3() {
            let digraph = kattis_escapewallmaria_3();

            assert!(!digraph.is_pendant(0));
            assert!(!digraph.is_pendant(1));
            assert!(!digraph.is_pendant(2));
            assert!(!digraph.is_pendant(3));
            assert!(!digraph.is_pendant(4));
            assert!(!digraph.is_pendant(5));
            assert!(!digraph.is_pendant(6));
            assert!(!digraph.is_pendant(7));
            assert!(!digraph.is_pendant(8));
            assert!(!digraph.is_pendant(9));
            assert!(!digraph.is_pendant(10));
            assert!(!digraph.is_pendant(11));
            assert!(!digraph.is_pendant(12));
            assert!(!digraph.is_pendant(13));
            assert!(!digraph.is_pendant(14));
            assert!(!digraph.is_pendant(15));
        }
    };
}
