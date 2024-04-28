//! Proptest strategies

#![cfg(test)]

use proptest::{
    prelude::prop,
    strategy::{
        Just,
        Strategy,
    },
};

/// Generate a triple `(v, s, t)` where `v` is the number of vertices in a
/// graph and `s` and `t` are vertices in the graph.
///
/// # Arguments
///
/// * `max`: The maximum number of vertices in the graph.
pub fn binop_vertices(max: usize) -> impl Strategy<Value = (usize, usize, usize)> {
    prop::num::usize::ANY
        .prop_map(move |v| v % max)
        .prop_flat_map(|v| {
            let s = prop::num::usize::ANY.prop_map(move |s| s % v.saturating_sub(1));
            let t = prop::num::usize::ANY.prop_map(move |t| t % v.saturating_sub(1));

            (Just(v), s, t)
        })
}
