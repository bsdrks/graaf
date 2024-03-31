/// Dijkstra's algorithm with binary-heap for unweighted graphs
pub mod dijkstra_unweighted;

/// Dijkstra's algorithm with binary-heap for weighted graphs
pub mod dijkstra_weighted;

pub use {
    dijkstra_unweighted::{
        dijkstra_sssp_unweighted,
        DijkstraUnweighted,
    },
    dijkstra_weighted::DijkstraWeighted,
};
