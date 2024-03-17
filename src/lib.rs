#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![allow(incomplete_features)]
#![feature(assert_matches)]
#![feature(generic_const_exprs)]

mod algo;

pub use algo::dijkstra;

mod ops;

pub use ops::{
    AddEdge,
    AddWeightedEdge,
    CountAllEdges,
    CountAllVertices,
    EdgeWeight,
    InDegree,
    IsEdge,
    IterAllEdges,
    IterAllWeightedEdges,
    IterEdges,
    IterVertices,
    IterWeightedEdges,
    OutDegree,
    RemoveEdge,
    VertexWeight,
};

mod repr;

pub use repr::AdjacencyMatrix;
