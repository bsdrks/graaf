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
pub fn v_s_t(max: usize) -> impl Strategy<Value = (usize, usize, usize)> {
    prop::num::usize::ANY
        .prop_map(move |v| v % max)
        .prop_flat_map(|v| (Just(v), 0..v, 0..v))
}
