//! Proptest strategies

#![cfg(test)]

use proptest::{
    self,
    prelude::prop,
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
/// * `max`: The maximum number of vertices in the graph.
pub fn binop_vertices(min: usize, max: usize) -> impl Strategy<Value = (usize, usize, usize)> {
    prop::num::usize::ANY.prop_flat_map(move |v| {
        let v = (min + v % max).max(1);
        let s = prop::num::usize::ANY.prop_map(move |s| s % v);
        let t = prop::num::usize::ANY.prop_map(move |t| t % v);

        (Just(v), s, t)
    })
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        proptest::proptest,
    };

    proptest! {
        #[test]
        fn binop_vertices_bounds((v, s, t) in binop_vertices(1, 100)) {
            assert!(v >= 1);
            assert!(v <= 100);
            assert!(s < v);
            assert!(t < v);
        }
    }
}
