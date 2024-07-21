//! Proptest strategies

use proptest::strategy::Strategy;

pub fn arc() -> impl Strategy<Value = (usize, usize)> {
    (1..25_usize, 1..25_usize).prop_filter("u != v", |(u, v)| u != v)
}
