//! Check whether a digraph is simple.
//!
//! A simple digraph has no self-loops or parallel arcs.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AddArc,
//!     AdjacencyList,
//!     Empty,
//!     IsSimple,
//! };
//!
//! let mut digraph = AdjacencyList::empty(3);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(2, 0);
//!
//! assert!(digraph.is_simple());
//! ```

/// Check whether a digraph is simple.
pub trait IsSimple {
    /// Check whether the digraph is simple.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AddArc,
    ///     AdjacencyList,
    ///     Empty,
    ///     IsSimple,
    /// };
    ///
    /// let mut digraph = AdjacencyList::empty(3);
    ///
    /// digraph.add_arc(0, 1);
    /// digraph.add_arc(0, 2);
    /// digraph.add_arc(1, 2);
    ///
    /// assert!(digraph.is_simple());
    /// ```
    #[must_use]
    fn is_simple(&self) -> bool;
}

/// `IsSimple` tests
#[macro_export]
macro_rules! test_is_simple {
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
        fn is_simple_bang_jensen_196() {
            assert!(bang_jensen_196().is_simple());
        }

        #[test]
        fn is_simple_bang_jensen_34() {
            assert!(bang_jensen_34().is_simple());
        }

        #[test]
        fn is_simple_bang_jensen_94() {
            assert!(bang_jensen_94().is_simple());
        }

        #[test]
        fn is_simple_kattis_builddeps() {
            assert!(kattis_builddeps().is_simple());
        }

        #[test]
        fn is_simple_kattis_cantinaofbabel_1() {
            assert!(kattis_cantinaofbabel_1().is_simple());
        }

        #[test]
        fn is_simple_kattis_cantinaofbabel_2() {
            assert!(kattis_cantinaofbabel_2().is_simple());
        }

        #[test]
        fn is_simple_kattis_escapewallmaria_1() {
            assert!(kattis_escapewallmaria_1().is_simple());
        }

        #[test]
        fn is_simple_kattis_escapewallmaria_2() {
            assert!(kattis_escapewallmaria_2().is_simple());
        }

        #[test]
        fn is_simple_kattis_escapewallmaria_3() {
            assert!(kattis_escapewallmaria_3().is_simple());
        }
    };
}
