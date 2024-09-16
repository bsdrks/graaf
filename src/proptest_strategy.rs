//! Proptest strategies

use proptest::strategy::Strategy;

/// Generate an arc's head and tail.
///
/// # Arguments
///
/// * `order`: The digraph's order.
pub fn arc(order: usize) -> impl Strategy<Value = (usize, usize)> {
    (1..order, 1..order).prop_filter("u != v", |(u, v)| u != v)
}
