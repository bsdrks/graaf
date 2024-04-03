# Graaf &emsp; [![Build status](https://github.com/bsdrks/graaf/actions/workflows/rust.yml/badge.svg)](https://github.com/bsdrks/graaf/actions) [![Crates.io](https://img.shields.io/crates/v/graaf.svg)](https://crates.io/crates/graaf) [![API reference](https://docs.rs/graaf/badge.svg)](https://docs.rs/graaf) [![Coverage Status](https://coveralls.io/repos/github/bsdrks/graaf/badge.svg?branch=main)](https://coveralls.io/github/bsdrks/graaf?branch=main)

Functions and types for working with graphs

**Graaf** is Dutch for

1. graph
2. count
3. dig

**WARNING: this crate is in early alpha. The API is unstable.**

## Algorithms

- `algo::bfs::min_distances_single_source`
- `algo::bfs::min_distances`
- `algo::bfs::predecessors`
- `algo::dijkstra::min_distances_single_source`
- `algo::dijkstra::min_distances`
- `algo::dijkstra::predecessors`

## Graph operation traits

These traits are implemented for various graph representations built from standard library containers.

- `ops::AddEdge` adds an edge to an unweighted graph.
- `ops::AddWeightedEdge` adds an edge to a weighted graph.
- `ops::CountAllEdges` counts all edges in a graph.
- `ops::CountAllVertices` counts all vertices in a graph.
- `ops::EdgeWeight` gets the weight of an edge.
- `ops::Indegree` returns the indegree of a vertex.
- `ops::IsEdge` returns whether an edge exists between two vertices.
- `ops::IterAllEdges` iterates over all unweighted edges in a graph.
- `ops::IterAllWeightedEdges` iterates over all weighted edges in a graph.
- `ops::IterEdges` iterates over all unweighted edges of a source vertex.
- `ops::IterVertices` iterates over all vertices in a graph.
- `ops::IterWeightedEdges` iterates over all weighted edges of a source vertex.
- `ops::Outdegree` returns the outdegree of a vertex.
- `ops::RemoveEdge` removes an edge from a graph.

## Graph representations

### Adjacency list

#### Unweighted

- `Vec<Vec<usize>>`
- `Vec<HashSet<usize>>`
- `[Vec<usize>]`
- `[HashSet<usize>]`
- `HashMap<usize, Vec<usize>>`
- `HashMap<usize, HashSet<usize>>`

#### Weighted

- `Vec<Vec<(usize, W)>>`
- `Vec<HashSet<(usize, W)>>`
- `Vec<HashMap<usize, W>>`
- `[Vec<(usize, W)>]`
- `[HashSet<(usize, W)>]`
- `[HashMap<usize, W>]`
- `HashMap<usize, Vec<(usize, W)>>`
- `HashMap<usize, HashSet<(usize, W)>>`
- `HashMap<usize, HashMap<usize, W>>`

### Adjacency matrix

#### Unweighted

- `AdjacencyMatrix`: an adjacency matrix representation of an unweighted directed graph stored as a bit array.

### Edge list

#### Unweighted

- `Vec<(usize, usize)>`
- `[(usize, usize)]`
- `HashSet<(usize, usize)>`

#### Weighted

- `Vec<(usize, usize, W)>`
- `[(usize, usize, W)]`
- `HashSet<(usize, usize, W)>`
