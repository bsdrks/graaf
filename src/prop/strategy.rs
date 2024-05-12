//! Proptest strategies

#![cfg(test)]

use proptest::{
    self,
    strategy::{
        Just,
        Strategy,
    },
};

/// Generates a triple `(v, s, t)` where `v` is the number of vertices in a
/// graph and `s` and `t` are vertices in the graph.
///
/// # Arguments
///
/// * `min`: The minimum number of vertices in the graph.
/// * `max`: The maximum number of vertices in the graph.
///
/// # Panics
///
/// Panics if `min` is zero.
/// Panics if `min` is greater than `max`.
pub fn binop_vertices(min: usize, max: usize) -> impl Strategy<Value = (usize, usize, usize)> {
    assert!(min > 0, "a graph must have at least one vertex");

    (min..=max)
        .prop_flat_map(move |v| {
            let s = 0..v;
            let t = 0..v;

            (Just(v), s, t)
        })
        .prop_filter("s != t", |(_, s, t)| s != t)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn binop_vertices_bounds((v, s, t) in binop_vertices(10, 100)) {
            assert!(v >= 10);
            assert!(v <= 100);
            assert!(s < v);
            assert!(t < v);
        }

        #[test]
        #[should_panic(expected = "a graph must have at least one vertex")]
        fn binop_vertices_0((_, _, _) in binop_vertices(0, 10)) {}

        #[test]
        fn binop_vertices_s_ne_t((_, s, t) in binop_vertices(1, 100)) {
            assert_ne!(s, t);
        }

        #[test]
        fn binop_vertices_min_eq_max((v, s, t) in binop_vertices(10, 10)) {
            assert_eq!(v, 10);
            assert!(s < v);
            assert!(t < v);
        }
    }
}
