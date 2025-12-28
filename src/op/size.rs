//! Count the arcs in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     Size,
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
//! assert_eq!(digraph.size(), 10);
//! ```

/// Digraph size
pub trait Size {
    /// Count the arcs in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Circuit,
    ///     Size,
    /// };
    ///
    /// let digraph = AdjacencyList::circuit(3);
    ///
    /// assert_eq!(digraph.size(), 3);
    /// ```
    #[must_use]
    fn size(&self) -> usize;
}

/// `Size` tests
#[macro_export]
macro_rules! test_size {
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
        fn size_bang_jensen_196() {
            assert_eq!(bang_jensen_196().size(), 13);
        }

        #[test]
        fn size_bang_jensen_34() {
            assert_eq!(bang_jensen_34().size(), 6);
        }

        #[test]
        fn size_bang_jensen_94() {
            assert_eq!(bang_jensen_94().size(), 9);
        }

        #[test]
        fn size_kattis_builddeps() {
            assert_eq!(kattis_builddeps().size(), 8);
        }

        #[test]
        fn size_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().size(), 22);
        }

        #[test]
        fn size_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().size(), 19);
        }

        #[test]
        fn size_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().size(), 7);
        }

        #[test]
        fn size_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().size(), 7);
        }

        #[test]
        fn size_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().size(), 14);
        }
    };
}
