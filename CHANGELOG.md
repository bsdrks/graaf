# Changelog

## [x.x.x] - Planned

- Add Edmonds-Karp algorithm.
- Add Hopcroft-Karp algorithm.
- Add Kruksal's algorithm.
- Add SPFA algorithm.
- Add `IsSimple` trait or function for graphs.
- Add `bfs::mssp`.
- Add `bfs::sssp`.
- Add `dijkstra::mssp`.
- Add `dijkstra::sssp`.
- Add `should_panic` test cases where possible.
- Add biclique generator.
- Add binary tree generator.
- Add bipartite matching algorithm for connected graphs.
- Add bipartite matching algorithm for disconnected graphs.
- Add depth-first search.
- Add linear graph generator.
- Add star graph generator.
- Add topological sorting for DAGs.
- Mention for functions whether they are meant for simple graphs.
- Test implementations of traits with properties in `op::prop`.

## [0.8.0] - Unreleased

### Added

- Add installation instructions to `README`.
- Add example usage to `README`.

### Changed

- Change `iter_all_edges` returns type to `impl Iterator<Item = (usize, usize)>`.
- Change `iter_all_weighted_edges` return type to `impl Iterator<Item = (usize, usize, &'a W)>`.
- Change `iter_weighted_edges` return type to `impl Iterator<Item = (usize, &'a W)>`.

## [0.7.0] - 2024-04-07

### Added

- Implement `CountAllEdges` for `[Vec<T>]`.
- Implement `CountAllEdges` for `[HashSet<T>]`.
- Implement `CountAllEdges` for `[HashMap<K, W>]`.
- Implement `CountAllVertices` for `[T]`.
- Implement `EdgeWeight` for `[HashMap<usize, W>]`.
- Implement `Indegree` for `[HashSet<usize>]`.
- Implement `Indegree` for `[HashMap<usize, W>]`.
- Implement `IsEdge` for `[HashSet<usize>]`.
- Implement `IsEdge` for `[HashMap<usize, W>]`.
- Implement `IterEdges` for `[Vec<usize>]`.
- Implement `IterEdges` for `[HashSet<usize>]`.
- Implement `IterAllEdges` for `[(usize, usize)]`.
- Implement `IterAllWeightedEdges` for `[(usize, usize, W)]`.
- Implement `IterVertices` for `&[T]`.
- Implement `IterWeightedEdges` for `[Vec<(usize, W)>]`.
- Implement `IterWeightedEdges` for `[HashSet<(usize, W)>]`.
- Implement `IterWeightedEdges` for `[HashMap<usize, W>]`.
- Implement `Outdegree` for `[Vec<T>]`.
- Implement `Outdegree` for `[HashSet<T>]`.
- Implement `Outdegree` for `[HashMap<K, W>]`.
- Implement `RemoveEdge` for `[HashSet<usize>]`.
- Implement `RemoveEdge` for `[HashMap<usize, W>]`.

### Changed

- Return `(&'a usize, &'a W)` from `iter_weighted_edges`.

### Removed

- Remove `CountAllEdges` for `Vec<Vec<T>>`. Use `[Vec<T>]`.
- Remove `CountAllEdges` for `Vec<HashSet<T>>`. Use `[HashSet<T>]`.
- Remove `CountAllEdges` for `Vec<HashMap<K, W>>`. Use `[HashMap<K, W>]`.
- Remove `CountAllEdges` for `[Vec<T>; V]`. Use `[Vec<T>]`.
- Remove `CountAllEdges` for `[HashSet<T>; V]`. Use `[HashSet<T>]`.
- Remove `CountAllEdges` for `[HashMap<K, W>; V]`. Use `[HashMap<K, W>]`.
- Remove `Indegree` for `Vec<HashSet<usize>>`. Use `[HashSet<usize>]`.
- Remove `Indegree` for `Vec<HashMap<usize, W>>`. Use `[HashMap<usize, W>]`.
- Remove `Indegree` for `[HashSet<usize>; V]`. Use `[HashSet<usize>]`.
- Remove `Indegree` for `[HashMap<usize, W>; V]`. Use `[HashMap<usize, W>]`.
- Remove `IsEdge` for `Vec<HashSet<usize>>`. Use `[HashSet<usize>]`.
- Remove `IsEdge` for `Vec<HashMap<usize, W>>`. Use `[HashMap<usize, W>]`.
- Remove `IsEdge` for `[HashSet<usize>; V]`. Use `[HashSet<usize>]`.
- Remove `IsEdge` for `[HashMap<usize, W>; V]`. Use `[HashMap<usize, W>]`.
- Remove `IterAllEdges` for `Vec<(usize, usize)>`. Use `[(usize, usize)]`.
- Remove `IterAllEdges` for `[(usize, usize); V]`. Use `[(usize, usize)]`.
- Remove `IterAllWeightedEdges` for `Vec<(usize, usize, W)>`. Use `[(usize, usize, W)]`.
- Remove `IterAllWeightedEdges` for `[(usize, usize, W); V]`. Use `[(usize, usize, W)]`.
- Remove `IterVertices` for `Vec<T>`. Use `[T]`.
- Remove `IterVertices` for `[T; V]`. Use `[T]`.
- Remove `Outdegree` for `Vec<Vec<T>>`. Use `[Vec<T>]`.
- Remove `Outdegree` for `Vec<HashSet<usize>>`. Use `[HashSet<T>]`.
- Remove `Outdegree` for `Vec<HashMap<usize, W>>`. Use `[HashMap<K, W>]`.
- Remove `Outdegree` for `[Vec<T>; V]`. Use `[Vec<T>]`.
- Remove `Outdegree` for `[HashSet<usize>; V]>`. Use `[HashSet<T>]`.
- Remove `Outdegree` for `[HashMap<usize, W>; V]>`. Use `[HashMap<K, W>]`.
- Remove `RemoveEdge` for `Vec<HashSet<usize>>`. Use `[HashSet<usize>]`.
- Remove `RemoveEdge` for `Vec<HashMap<usize, W>>`. Use `[HashMap<usize, W>]`.
- Remove `RemoveEdge` for `[HashSet<usize>; V]`. Use `[HashSet<usize>]`.
- Remove `RemoveEdge` for `[HashMap<usize, W>; V]`. Use `[HashMap<usize, W>]`.

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
