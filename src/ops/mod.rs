/// A trait to add an edge to an unweighted graph
pub mod add_edge;

/// A trait to add an edge to a weighted graph
pub mod add_weighted_edge;

/// A trait to count all edges in a graph
pub mod count_all_edges;

/// A trait to count all vertices in a graph
pub mod count_all_vertices;

/// A trait to get the weight of a given edge
pub mod edge_weight;

/// A trait to get the indegree of a given vertex
pub mod indegree;

/// A trait to check if an edge exists between two vertices
pub mod is_edge;

/// A trait to iterate over all unweighted edges in a graph
pub mod iter_all_edges;

/// A trait to iterate over all weighted edges in a graph
pub mod iter_all_weighted_edges;

/// A trait to iterate over all unweighted edges with a given source vertex
pub mod iter_edges;

/// A trait to iterate over all vertices in a graph
pub mod iter_vertices;

/// A trait to iterate over all weighted edges with a given source vertex
pub mod iter_weighted_edges;

/// A trait to get the outdegree of a given vertex
pub mod outdegree;

/// A trait to remove an edge from a graph
pub mod remove_edge;

pub use {
    add_edge::AddEdge,
    add_weighted_edge::AddWeightedEdge,
    count_all_edges::CountAllEdges,
    count_all_vertices::CountAllVertices,
    edge_weight::EdgeWeight,
    indegree::Indegree,
    is_edge::IsEdge,
    iter_all_edges::IterAllEdges,
    iter_all_weighted_edges::IterAllWeightedEdges,
    iter_edges::IterEdges,
    iter_vertices::IterVertices,
    iter_weighted_edges::IterWeightedEdges,
    outdegree::Outdegree,
    remove_edge::RemoveEdge,
};
