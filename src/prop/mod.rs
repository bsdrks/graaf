//! Cross-module properties and strategies.

mod add_arc_has_arc;
mod add_arc_remove_arc;
mod add_weighted_arc_has_arc;
mod add_weighted_arc_remove_arc;
mod sum_indegrees_eq_sum_outdegrees;

pub mod strategy;

pub use {
    add_arc_has_arc::add_arc_has_arc,
    add_arc_remove_arc::add_arc_remove_arc,
    add_weighted_arc_has_arc::add_weighted_arc_has_arc,
    add_weighted_arc_remove_arc::add_weighted_arc_remove_arc,
    sum_indegrees_eq_sum_outdegrees::sum_indegrees_eq_sum_outdegrees,
};
