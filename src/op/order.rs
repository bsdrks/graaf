//! Return the number of vertices in a digraph.
//!
//! # Examples
//!
//! ```
//! use graaf::{
//!     AdjacencyList,
//!     Empty,
//!     Order,
//! };
//!
//! let digraph = AdjacencyList::empty(4);
//!
//! assert_eq!(digraph.order(), 4);
//! ```

/// Digraph order
pub trait Order {
    /// Count the vertices in the digraph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graaf::{
    ///     AdjacencyList,
    ///     Empty,
    ///     Order,
    /// };
    ///
    /// let digraph = AdjacencyList::empty(4);
    ///
    /// assert_eq!(digraph.order(), 4);
    /// ```
    #[must_use]
    fn order(&self) -> usize;
}

/// `Order` tests
#[macro_export]
macro_rules! test_order {
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
        fn order_bang_jensen_196() {
            assert_eq!(bang_jensen_196().order(), 8);
        }

        #[test]
        fn order_bang_jensen_34() {
            assert_eq!(bang_jensen_34().order(), 6);
        }

        #[test]
        fn order_bang_jensen_94() {
            assert_eq!(bang_jensen_94().order(), 7);
        }

        #[test]
        fn order_kattis_builddeps() {
            assert_eq!(kattis_builddeps().order(), 6);
        }

        #[test]
        fn order_kattis_cantinaofbabel_1() {
            assert_eq!(kattis_cantinaofbabel_1().order(), 12);
        }

        #[test]
        fn order_kattis_cantinaofbabel_2() {
            assert_eq!(kattis_cantinaofbabel_2().order(), 12);
        }

        #[test]
        fn order_kattis_escapewallmaria_1() {
            assert_eq!(kattis_escapewallmaria_1().order(), 16);
        }

        #[test]
        fn order_kattis_escapewallmaria_2() {
            assert_eq!(kattis_escapewallmaria_2().order(), 16);
        }

        #[test]
        fn order_kattis_escapewallmaria_3() {
            assert_eq!(kattis_escapewallmaria_3().order(), 16);
        }
    };
}
