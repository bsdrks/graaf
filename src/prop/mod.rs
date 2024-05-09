//! Cross-module properties and strategies.

mod add_edge_has_edge;
mod add_edge_remove_edge;
mod add_weighted_edge_has_edge;
mod add_weighted_edge_remove_edge;

pub mod strategy;

pub use {
    add_edge_has_edge::add_edge_has_edge,
    add_edge_remove_edge::add_edge_remove_edge,
    add_weighted_edge_has_edge::add_weighted_edge_has_edge,
    add_weighted_edge_remove_edge::add_weighted_edge_remove_edge,
};