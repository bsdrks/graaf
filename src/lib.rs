#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod add_edge;
pub mod add_weighted_edge;
pub mod adjacency_matrix;
pub mod count_all_edges;
pub mod count_all_vertices;
pub mod edge_weight;
pub mod in_degree;
pub mod is_edge;
pub mod iter_all_edges;
pub mod iter_all_weighted_edges;
pub mod iter_edges;
pub mod iter_vertices;
pub mod iter_weighted_edges;
pub mod out_degree;
pub mod remove_edge;
pub mod vertex_weight;
pub mod weight;

pub use {
    add_edge::AddEdge,
    add_weighted_edge::AddWeightedEdge,
    adjacency_matrix::AdjacencyMatrix,
    count_all_edges::CountAllEdges,
    count_all_vertices::CountAllVertices,
    edge_weight::EdgeWeight,
    in_degree::InDegree,
    is_edge::IsEdge,
    iter_all_edges::IterAllEdges,
    iter_all_weighted_edges::IterAllWeightedEdges,
    iter_edges::IterEdges,
    iter_vertices::IterVertices,
    iter_weighted_edges::IterWeightedEdges,
    out_degree::OutDegree,
    remove_edge::RemoveEdge,
    vertex_weight::VertexWeight,
    weight::Weight,
};
