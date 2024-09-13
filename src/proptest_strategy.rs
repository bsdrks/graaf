//! Proptest strategies

use proptest::strategy::Strategy;

/// Generate the head and tail of an arc.
///
/// # Arguments
///
/// * `order`: The order of the digraph.
pub fn arc(order: usize) -> impl Strategy<Value = (usize, usize)> {
    (1..order, 1..order).prop_filter("u != v", |(u, v)| u != v)
}
