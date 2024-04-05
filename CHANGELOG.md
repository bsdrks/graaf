# Changelog

## [x.x.x] - Planned

- Add `bfs::mssp`.
- Add `bfs::sssp`.
- Add `dfs::*`.
- Add `dijkstra::mssp`.
- Add `dijkstra::sssp`.
- Add doctest for `AdjacencyMatrix::new`.
- Add doctest for `AdjacencyMatrix::toggle`.
- Add doctest for `Indegree`.
- Add doctest for `IsEdge`.
- Add doctest for `IterAllEdges`.
- Add doctest for `IterAllWeightedEdges`.
- Add doctest for `IterEdges`.
- Add doctest for `IterWeightedEdges`.
- Add doctest for `Outdegree`.
- Add doctest for `RemoveEdge`.

## [0.5.3] - Unreleased

### Added

- Add doctest for `ops::add_weighted_edge::AddWeightedEdge`.
- Add doctest for `ops::count_all_edges::CountAllEdges`.
- Add doctest for `ops::count_all_vertices::CountAllVertices`.
- Add doctest for `ops::edge_weight::EdgeWeight`.
- Add documentation for `ops::add_weighted_edge`.
- Add documentation for `ops::count_all_edges`.
- Add documentation for `ops::count_all_vertices`.
- Add documentation for `ops::edge_weight`.

## [0.5.2] - 2024-04-04

### Added

- Add doctest for `ops::add_edge::AddEdge`.
- Add documentation for `ops::add_edge`.
- Add module-level doctest for `algo::bfs`.
- Add module-level doctest for `algo::dijkstra`.

## [0.5.1] - 2024-04-04

### Added

- Add `bfs::predecessors_single_source`.
- Add `dijkstra::predecessors_single_source`.

## [0.5.0] - 2024-04-03

### Added

- Implement `Indegree` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsEdge` for `HashSet<(usize, usize)>`.
- Implement `IterVertices` for `HashSet<T>`.
- Implement `IterWeightedEdges` for `Vec<HashMap<usize, W>>`.
- Implement `IterWeightedEdges` for `[HashMap<usize, W>; V]`.
- Implement `Outdegree` for `Vec<HashMap<usize, W>>`.
- Implement `Outdegree` for `[HashMap<usize, W>; V]`.

### Removed

- Remove `VertexWeight` trait.

## [0.4.2] - 2024-04-03

### Changed

- Rename `*::shortest_paths` to `predecessors`.

## [0.4.1] - 2024-04-03

### Added

- Add `algo::bfs::shortest_paths`.

## [0.4.0] - 2024-04-03

### Added

- Add `algo::dijkstra::shortest_paths`.

### Changed

- Remove `algo::dijkstra::unweighted`.
- Move `algo::dijkstra::weighted` to `algo::dijkstra`.

## [0.3.3] - 2024-04-02

### Added

- Add `algo::bfs::min_distances`.
- Add `algo::bfs::min_distances_single_source`.
- Add benchmarks for `algo::bfs::*`

## [0.3.2] - 2024-04-01

### Added

- Add `algo::dijkstra::unweighted::shortest_paths`.
- Test `algo::dijkstra::unweighted::min_distances` with multiple source vertices.
- Test `algo::dijkstra::weighted::min_distances` without sources.

## [0.3.1] - 2024-04-01

### Changed

- Update function names in `README`.

## [0.3.0] - 2024-04-01

### Added

- Add `algo::dijkstra::unweighted::min_distances_single_source`.
- Add doctest example for `algo::dijkstra::weighted::min_distances_single_source`.
- Add doctest example for `algo::dijkstra::unweighted::min_distances_single_source`
- Add doctest example for `algo::dijkstra::unweighted::min_distances`

### Changed

- Move `algo::dijkstra::dijkstra_sssp_weighted` to `algo::dijkstra::weighted::min_distances_single_source`.
- Move `algo::DijkstraWeighted::dijkstra` to `algo::dijkstra::weighted::min_distances`.
- Move `algo::DijkstraUnweighted::dijkstra` to `algo::dijkstra::unweighted::min_distances`.

### Removed

- Remove `new` benchmark.

## [0.2.3] - 2024-03-31

### Added

- Add `dijkstra_sssp_weighted`.
- Add GitHub Action on push to main and PRs on main.

### Changed

- Test `Dijkstra.Unweighted.dijkstra_sssp_unweighted` for every source vertex.
- Make `CHANGELOG.md` adhere to [keep a changelog](https://keepachangelog.com/en/1.0.0/).

## [0.2.2] - 2024-03-31

### Added

- Add doctest example for `Weighted.dijkstra`.

### Fixed

- Fix trait descriptions in `README.md`.

## [0.2.1] - 2024-03-31

### Added

- Add `dijkstra_sssp_unweighted`.
- Add missing documentation for the public API.

### Changed

- Export `algo`, `ops`, and `repr` modules.
- Enable selected lints from `restriction` group.
- Group lints into groups, restrictions, `rustdoc`, and `rustc` lints.
- Use `core` and `alloc` imports over `std` where possible.

### Fixed

- Make descriptions more consistent.

## [0.2.0] - 2024-03-30

### Changed

- Rename `Indegree.in_degree` to `Indegree.indegree`.
- Rename `Outdegree.out_degree` to `Outdegree.outdegree`.

## [0.1.0] - 2024-03-30

- Initial release
