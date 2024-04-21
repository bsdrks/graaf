//! Properties of traits in [`crate::op`].
mod add_edge_is_edge;
mod add_edge_remove_edge;
mod add_weighted_edge_is_edge;
mod add_weighted_edge_remove_edge;
mod remove_edge_is_edge;

pub use {
    add_edge_is_edge::add_edge_is_edge,
    add_edge_remove_edge::add_edge_remove_edge,
    add_weighted_edge_is_edge::add_weighted_edge_is_edge,
    add_weighted_edge_remove_edge::add_weighted_edge_remove_edge,
    remove_edge_is_edge::remove_edge_is_edge,
};
