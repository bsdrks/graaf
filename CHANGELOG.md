# Changelog

## [x.x.x] - Planned

- Add `bfs::mssp`.
- Add `bfs::sssp`.
- Add `dfs::*`.
- Add `dijkstra::mssp`.
- Add `dijkstra::sssp`.

- Implement `Indegree` for slices.
- Implement `IsEdge` for slices.
- Implement `IterEdges` for slices.
- Implement `IterVertices` for slices.
- Implement `IterWeightedEdges` for slices.
- Implement `Outdegree` for slices.
- Implement `RemoveEdge` for slices.
- Test implementations of traits with properties in `op::prop`.
- Return borrowed values in traits.

## [0.6.4] - Unreleased

### Added

- Implement `CountAllEdges` for `[Vec<T>]`.
- Implement `CountAllEdges` for `[HashSet<T>]`.
- Implement `CountAllEdges` for `[HashMap<K, W>]`.
- Implement `CountAllVertices` for `[T]`.
- Implement `EdgeWeight` for `[HashMap<usize, W>]`.

## [0.6.3] - 2024-04-06

### Changed

- Fix `README` formatting.

## [0.6.2] - 2024-04-06

### Added

- Add more tests to `algo::bfs`.
- Add more tests to `algo::dijkstra`.
- Implement `AddEdge` for `[HashSet<usize>]`.
- Implement `AddEdge` for `[Vec<usize>]`.
- Implement `AddWeightedEdge` for `[HashMap<usize, W>]`.
- Implement `AddWeightedEdge` for `[HashSet<(usize, W)>]`.
- Implement `AddWeightedEdge` for `[Vec<(usize, W)>]`.
- Implement `IterAllEdges` for `[(usize, usize)]`.
- Implement `IterAllWeightedEdges` for `[(usize, usize, W)]`.

## [0.6.1] - 2024-04-06

### Added

- Add "algorithms" and "mathematics" to `Cargo.toml` categories.
- Add "bfs" and "dijkstra" to `Cargo.toml` keywords.

### Removed

- Remove redundant `homepage` metadata.

## [0.6.0] - 2024-04-06

### Added

- Add `authors` to `README.md`.
- Add doctest for `AdjacencyMatrix::new`.
- Add doctest for `AdjacencyMatrix::toggle`.
- Add implementation documentation for `AddEdge`.
- Add implementation documentation for `AddWeightedEdge`.
- Add implementation documentation for `CountAllEdges`.
- Add implementation documentation for `CountAllVertices`.
- Add implementation documentation for `EdgeWeight`.
- Add implementation documentation for `Indegree`.
- Add implementation documentation for `IsEdge`.
- Add implementation documentation for `IterAllEdges`.
- Add implementation documentation for `IterAllWeightedEdges`.
- Add implementation documentation for `IterEdges`.
- Add implementation documentation for `IterWeightedEdges`.
- Add implementation documentation for `Outdegree`.
- Add implementation documentation for `RemoveEdge`.

### Changed

- **BREAKING** Move `ops` to `op`.
- Adapt benchmark code to linting rules.
- Move doctest trait properties to `op::prop`.
- Move lints from `lib.rs` to `Cargo.toml`.

## [0.5.3] - 2024-04-05

### Added

- Add doctest for `op::add_weighted_edge::AddWeightedEdge`.
- Add doctest for `op::count_all_edges::CountAllEdges`.
- Add doctest for `op::count_all_vertices::CountAllVertices`.
- Add doctest for `op::edge_weight::EdgeWeight`.
- Add doctest for `op::indegree::Indegree`.
- Add doctest for `op::is_edge::IsEdge`.
- Add doctest for `op::iter_all_edges::IterAllEdges`.
- Add doctest for `op::iter_all_weighted_edges::IterAllWeightedEdges`.
- Add doctest for `op::iter_edges::IterEdges`.
- Add doctest for `op::iter_weighted_edges::IterWeightedEdges`.
- Add doctest for `op::iter_vertices::IterVertices`.
- Add doctest for `op::outdegree::OutDegree`.
- Add doctest for `op::remove_edge::RemoveEdge`.
- Add documentation for `op::add_weighted_edge`.
- Add documentation for `op::count_all_edges`.
- Add documentation for `op::count_all_vertices`.
- Add documentation for `op::edge_weight`.
- Add documentation for `op::indegree`.
- Add documentation for `op::is_edge`.
- Add documentation for `op::iter_all_edges`.
- Add documentation for `op::iter_all_weighted_edges`.
- Add documentation for `op::iter_edges`.
- Add documentation for `op::iter_vertices`.
- Add documentation for `op::iter_weighted_edges`.
- Add documentation for `op::outdegree`.
- Add documentation for `op::remove_edge`.

## [0.5.2] - 2024-04-04

### Added

- Add doctest for `op::add_edge::AddEdge`.
- Add documentation for `op::add_edge`.
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

- Export `algo`, `op`, and `repr` modules.
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
