# Changelog

## Provisional roadmap

- Add a trait for path contraction.
- Add a trait for set contraction.
- Add the `Barabási–Albert` generator.
- Add the `IsIsomorphic` trait.
- Benchmark against popular graph libraries in other languages.
- Benchmark trait implementations.
- Check examples for `op::*`.
- Implement `DfsDist::distances`.
- Improve doctest and add image to `BfsDist::distances`.
- Improve doctest and add image to `DijkstraDist::distances`.
- Improve doctest and add image to `DijkstraPred::predecessors`.
- Improve doctest and add image to `DijkstraPred::shortest_path`.
- Improve doctest and add image to `DistanceMatrix::is_connected`.
- Improve doctest and add image to `DistanceMatrix::periphery`.
- Improve doctest and add image to `PredecessorTree::search_by`.
- Improve doctest and add image to `PredecessorTree::search`.
- Mention the order of traversal in the `Bfs` documentation.
- Mention the order of traversal in the `Dfs` documentation.
- Mention the order of traversal in the `Dijkstra` documentation.

## [0.90.1] - 2024-09-16

Changed

- Improved API documentation's wording.

## [0.90.0] - 2024-09-15

Added

- Add doctests for `Johnson75`.

Changed

- Breaking: `Johnson75::new()::find_circuits()` is replaced by `Johnson75::circuits()`.

## [0.89.3] - 2024-09-15

Added

- Add the `GrowingNetwork` trait.

## [0.89.2] - 2024-09-14

Added

- Add `ErdosRenyi` image to `gen/mod` documentation.
- Add `RandomTournament` image to `gen/mod` documentation.

Changed

- Improve `ErdosRenyi` documentation.
- Improve `RandomTournament` documentation.

## [0.89.1] - 2024-09-13

Changed

- Fix link.

## [0.89.0] - 2024-09-13

Added

- Add the `AdjacencyMap` type.
- Add the `FilterVertices` trait.
- Add the `Johnson75` algorithm.
- Implement From<I: IntoIterator<Item = (usize, usize)>> for EdgeList.
- Implement From<I: IntoIterator<Item = BTreeMap<usize, W>>> for AdjacencyListWeighted\<W\>.
- Implement From<I: IntoIterator<Item = BTreeSet\<usize\>>> for AdjacencyList.
- Implement From<I: IntoIterator<Item = BTreeSet\<usize\>>> for AdjacencyMatrix.

Removed

- Breaking: Remove `From<BTreeSet<(usize, usize)>> for EdgeList`.
- Breaking: Remove `From<Vec<BTreeMap<usize, W>>> for AdjacencyListWeighted<W>`.
- Breaking: Remove `From<Vec<BTreeSet<usize>>> for AdjacencyList`.
- Breaking: Remove `From<Vec<BTreeSet<usize>>> for AdjacencyMatrix`.

## [0.88.8] - 2024-09-10

Changed

- Expand `tarjan` tests.

## [0.88.7] - 2024-09-10

Added

- Add `BfsPred::cycles`.

## [0.88.6] - 2024-09-10

Added

- Add `MaxDegree` tests.
- Test `MaxIndegree`.
- Test `MaxOutdegree`.
- Test `MinIndegree`.
- Test `MinOutdegree`.

## [0.88.5] - 2024-09-09

Added

- Add the `MaxIndegree` trait.
- Add the `MaxOutdegree` trait.
- Add the `MinIndegree` trait.
- Add the `MinOutdegree` trait.

## [0.88.4] - 2024-09-09

Added

- Implement `Degree::max_degree`.
- Implement `Degree::min_degree`.

Changed

- Improve `Union` documentation.

## [0.88.3] - 2024-09-09

Added

- Add the `Union` trait.

## [0.88.2] - 2024-09-08

Changed

- Standardize implementation documentation.

## [0.88.1] - 2024-09-08

Added

- Add `Wheel` to `lib.rs` documentation

## [0.88.0] - 2024-09-08

Summary

- Add the `Wheel` generator.
- Lift the public API to the top level.
- Move common unit tests for unweighted digraphs to the `test_unweighted` module.

Added

- Add `gen::Wheel`.

Changed

- Breaking: move `adjacency_list::digraph::Digraph` to `repr::adjacency_list::AdjacencyList`.
- Breaking: move `adjacency_list_weighted::digraph::Digraph` to `repr::adjacency_list_weighted::AdjacencyListWeighted`.
- Breaking: move `adjacency_list_weighted` to `repr::adjacency_list_weighted`.
- Breaking: move `adjacency_list` to `repr::adjacency_list`.
- Breaking: move `adjacency_matrix::digraph::Digraph` to `repr::adjacency_matrix::AdjacencyMatrix`.
- Breaking: move `adjacency_matrix` to `repr::adjacency_matrix`.
- Breaking: move `edge_list::digraph::Digraph` to `repr::edge_list::EdgeList`.
- Breaking: move `edge_list` to `repr::edge_list`.
- Export `graaf::AddArcWeighted`.
- Export `graaf::AddArc`.
- Export `graaf::AdjacencyListWeighted`.
- Export `graaf::AdjacencyList`.
- Export `graaf::AdjacencyMatrix`.
- Export `graaf::ArcWeight`.
- Export `graaf::ArcsWeighted`.
- Export `graaf::Arcs`.
- Export `graaf::Biclique`.
- Export `graaf::Circuit`.
- Export `graaf::Complement`.
- Export `graaf::Complete`.
- Export `graaf::Converse`.
- Export `graaf::Cycle`.
- Export `graaf::DegreeSequence`.
- Export `graaf::Degree`.
- Export `graaf::DistanceMatrix`.
- Export `graaf::EdgeList`.
- Export `graaf::Empty`.
- Export `graaf::ErdosRenyi`.
- Export `graaf::HasArc`.
- Export `graaf::HasEdge`.
- Export `graaf::HasWalk`.
- Export `graaf::InNeighbors`.
- Export `graaf::IndegreeSequence`.
- Export `graaf::Indegree`.
- Export `graaf::IsBalanced`.
- Export `graaf::IsComplete`.
- Export `graaf::IsIsolated`.
- Export `graaf::IsOriented`.
- Export `graaf::IsPendant`.
- Export `graaf::IsRegular`.
- Export `graaf::IsSemicomplete`.
- Export `graaf::IsSimple`.
- Export `graaf::IsSpanningSubdigraph`.
- Export `graaf::IsSubdigraph`.
- Export `graaf::IsSuperdigraph`.
- Export `graaf::IsSymmetric`.
- Export `graaf::IsTournament`.
- Export `graaf::Order`.
- Export `graaf::OutNeighborsWeighted`.
- Export `graaf::OutNeighbors`.
- Export `graaf::OutdegreeSequence`.
- Export `graaf::Outdegree`.
- Export `graaf::Path`.
- Export `graaf::PredecessorTree`.
- Export `graaf::RandomTournament`.
- Export `graaf::RemoveArc`.
- Export `graaf::SemidegreeSequence`.
- Export `graaf::Sinks`.
- Export `graaf::Size`.
- Export `graaf::Sources`.
- Export `graaf::Star`.
- Export `graaf::Vertices`.
- Export `graaf::Wheel`.
- Export `graaf::bellman_ford_moore`.
- Export `graaf::bfs_dist`.
- Export `graaf::bfs_pred`.
- Export `graaf::bfs`.
- Export `graaf::dfs_dist`.
- Export `graaf::dfs_pred`.
- Export `graaf::dfs`.
- Export `graaf::dijkstra_dist`.
- Export `graaf::dijkstra_pred`.
- Export `graaf::dijkstra`.
- Export `graaf::distance_matrix`.
- Export `graaf::floyd_warshall`.
- Export `graaf::predecessor_tree`.
- Export `graaf::tarjan`.
- Move common unit tests for unweighted digraphs to `test_unweighted`.

## [0.87.4] - 2024-09-07

Documentation improvements.

Changed

- Improve documentation images.
- Improve module descriptions.
- Remove `// 0 -> 1` annotations from doctests.

## [0.87.3] - 2024-09-04

Changed

- Improve documentation.

## [0.87.2] - 2024-09-03

Added

- Add images and new examples to the `DistanceMatrix` documentation.

## [0.87.1] - 2024-09-01

Added

- Add unit test `dfs_pred::iter_bang_jensen_196`.
- Add unit test `dfs_pred::iter_bang_jensen_34`.
- Add unit test `dfs_pred::iter_bang_jensen_94`.
- Add unit test `dfs_pred::iter_kattis_builddeps`.
- Add unit test `dfs_pred::iter_kattis_cantinaofbabel_1`.
- Add unit test `dfs_pred::iter_kattis_cantinaofbabel_2`.
- Add unit test `dfs_pred::iter_kattis_escapewallmaria_1`.
- Add unit test `dfs_pred::iter_kattis_escapewallmaria_2`.
- Add unit test `dfs_pred::iter_kattis_escapewallmaria_3`.
- Implement `DfsPred::predecessors`.

Changed

- Improve documentation consistency.

## [0.87.0] - 2024-09-01

Added

- Add `algo::dfs_pred::DfsPred`.
- Add `algo::dfs_pred::Step`.
- Add unit test `dfs_pred::iter_bang_jensen_196`.
- Add unit test `dfs_pred::iter_bang_jensen_34`.
- Add unit test `dfs_pred::iter_bang_jensen_94`.
- Add unit test `dfs_pred::iter_kattis_builddeps`.
- Add unit test `dfs_pred::iter_kattis_cantinaofbabel_1`.
- Add unit test `dfs_pred::iter_kattis_cantinaofbabel_2`.
- Add unit test `dfs_pred::iter_kattis_escapewallmaria_1`.
- Add unit test `dfs_pred::iter_kattis_escapewallmaria_2`.
- Add unit test `dfs_pred::iter_kattis_escapewallmaria_3`.
- Implement `Iterator for DfsPred`.
- Implement `dfs_pred::DfsPred::new`.

Changed

- Breaking: `dfs::Dfs::new` is no longer `const`.
- Breaking: `dfs::Dfs` no longer implements `Hash`.
- Breaking: `dfs::Dfs` no longer implements `Ord`.
- Breaking: `dfs::Dfs` no longer implements `PartialOrd`.
- Breaking: `dfs_dist::DfsDist` no longer implements `Hash`.
- Breaking: `dfs_dist::DfsDist` no longer implements `Ord`.
- Breaking: `dfs_dist::DfsDist` no longer implements `PartialOrd`.
- Use a `HashMap` to track visited vertices in `dfs::Dfs`.
- Use a `HashMap` to track visited vertices in `dfs_dist::DfsDist`.
- `dfs::Dfs::new<D>` is no longer constrained to `D: Order + Outneighbors`.
- `dfs_dist::DfsDist::new<D>` is no longer constrained to `D: Order + Outneighbors`.

## [0.86.0] - 2024-09-01

Changed

- Breaking: rename `bfs_dist::Bfs` to `bfs_dist::BfsDist`.
- Breaking: rename `bfs_pred::Bfs` to `bfs_pred::BfsPred`.
- Breaking: rename `dfs_dist::Dfs` to `dfs_dist::DfsDist`.
- Breaking: rename `dijkstra_dist::Dijkstra` to `dijkstra_dist::DijkstraDist`.
- Breaking: rename `dijkstra_pred::Dijkstra` to `dijkstra_pred::DijkstraPred`.
- Deny `renamed_function_params` lint.
- Rename argument in `Index for DistanceMatrix` to `index`.
- Rename argument in `Index for PredecessorTree` to `index`.
- Rename argument in `IndexMut for DistanceMatrix` to `index`.
- Rename argument in `IndexMut for PredecessorTree` to `index`.

Fixed

- Fix link in `dfs_dist` documentation.

Removed

- Remove naming conventions from `lib.rs`.

## [0.85.0] - 2024-08-31

Changed

- Breaking: `bfs::Bfs` no longer implements `Hash`.
- Breaking: `bfs::Bfs` no longer implements `Ord`.
- Breaking: `bfs::Bfs` no longer implements `PartialOrd`.
- Breaking: `bfs_dist::Bfs` no longer implements `Hash`.
- Breaking: `bfs_dist::Bfs` no longer implements `Ord`.
- Breaking: `bfs_dist::Bfs` no longer implements `PartialOrd`.
- Breaking: `bfs_pred::Bfs` no longer implements `Hash`.
- Breaking: `bfs_pred::Bfs` no longer implements `Ord`.
- Breaking: `bfs_pred::Bfs` no longer implements `PartialOrd`.
- Breaking: speed up `bfs_dist::Bfs::distances` by returning distances in a `Vec`.
- Speed up `Iterator for bfs::Bfs` by tracking visited vertices in a `HashMap`.
- Speed up `Iterator for bfs_dist::Bfs` by tracking visited vertices in a `HashMap`.

## [0.84.5] - 2024-08-31

Added

- Add benchmark `distances/erdos_renyi`.
- Benchmark `BFS.distances` for `adjacency_matrix` in `distances/bang_jensen_94`.
- Benchmark `BFS.distances` for `adjacency_matrix` in `distances/erdos_renyi`.

Changed

- Reduce benchmark overhead.
- Split `algo/single_source_distances` benchmarks into:
  - `distances/bang_jensen_94`
  - `distances/bang_jensen_96`
  - `distances/bang_jensen_99`
  - `distances/kattis_bryr_1`
  - `distances/kattis_bryr_2`
  - `distances/kattis_bryr_3`
  - `distances/kattis_crosscountry`
  - `distances/kattis_shortestpath1`

## [0.84.4] - 2024-08-31

Added

- Add images to `dijkstra_dist` documentation.
- Add images to `dijkstra_pred` documentation.
- Add images to `floyd_warshall` documentation.

## [0.84.3] - 2024-08-30

Added

- Add images to `dijkstra` documentation.

## [0.84.2] - 2024-08-29

Added

- Add images to `dfs` documentation.

Changed

- Remove link to removed documentation header in `lib.rs`.
- Improve labels in `dfs*` documentation images.

## [0.84.1] - 2024-08-26

Added

- Add images to `dfs_dist` documentation.

Fixed

- Fix test coverage.

## [0.84.0] - 2024-08-25

Added

- Add module `edge_list::digraph`.
- Add module `edge_list::fixture`.
- Add module `edge_list`.
- Add struct `edge_list::Digraph`.
- Add time complexity warnings to `Indegree::indegree` for `adjacency_matrix`.
- Add time complexity warnings to `OutNeighbors::out_neighbors` for `adjacency_matrix`.
- Add time complexity warnings to `OutNeighborsWeighted::out_neighbors_weighted` for `adjacency_matrix`.
- Add time complexity warnings to `Outdegree::outdegree` for `adjacency_matrix`.
- Add unit test `circuit_0` for `adjacency_list`.
- Add unit test `circuit_0` for `adjacency_matrix`.
- Implement `AddArc` for `edge_list::Digraph`.
- Implement `AddWeight<usize>` for `edge_list::Digraph`.
- Implement `ArcsWeighted` for `edge_list::Digraph`.
- Implement `Arcs` for `edge_list::Digraph`.
- Implement `Biclique` for `edge_list::Digraph`.
- Implement `Circuit` for `edge_list::Digraph`.
- Implement `Converse` for `edge_list::Digraph`.
- Implement `Empty` for `edge_list::Digraph`.
- Implement `Empty` for `edge_list::Digraph`.
- Implement `From<BTreeSet<(usize, usize)>>` for `edge_list::Digraph`.
- Implement `From<adjacency_list::Digraph>` for `edge_list::Digraph`.
- Implement `From<adjacency_matrix::Digraph>` for `edge_list::Digraph`.
- Implement `IsSimple` for `edge_list::Digraph`.
- Implement `Order` for `edge_list::Digraph`.
- Implement `OutNeighborsWeighted<usize>` for `edge_list::Digraph`.
- Implement `OutNeighbors` for `edge_list::Digraph`.
- Implement `Outdegree` for `edge_list::Digraph`.
- Implement `RemoveArc` for `edge_list::Digraph`.
- Implement `Size` for `edge_list::Digraph`.
- Implement `Vertices` for `edge_list::Digraph`.

Changed

- Breaking: assert that `order > 0` in `Circuit::circuit` for `adjacency_list`.
- Misc. improvements to `README.md`.
- Misc. improvements to `adjacency_list` documentation.
- Misc. improvements to `adjacency_matrix` documentation.
- Misc. improvements to `lib.rs` documentation.

## [0.83.3] - 2024-08-25

Fixed

- Fix links to `bfs_pred` documentation.

## [0.83.2] - 2024-08-25

Changed

- Add images to `bfs_pred` documentation.
- Improve documentation images.

## [0.83.1] - 2024-08-24

Changed

- Add images to `bfs_dist` documentation.
- Improve documentation image consistency.
- Split `README.md` into `README.md`, `GOALS.md`, and `NAMING.md`.

## [0.83.0] - 2024-08-23

Added

- Add alias `algo::bellman_ford` for `algo::bellman_ford_moore`.
- Add images to the `adjacency_list_weighted/fixture` documentation.
- Add images to the `algo/bellman_ford_moore` documentation.
- Add images to the `algo/bfs` documentation.

Changed

- Breaking: change `source: usize` in `dfs::Dfs::new` to `sources: Vec<usize>`.
- Breaking: change `source: usize` in `dfs_dist::Dfs::new` to `sources: IntoIterator<Item = &'b usize>`.
- Breaking: change `sources` in `bfs::Bfs::new` to `IntoIterator<Item = &'b usize>`.
- Breaking: change `sources` in `bfs_dist::Bfs::new` to `IntoIterator<Item = &'b usize>`.
- Breaking: change `sources` in `bfs_pred::Bfs::new` to `IntoIterator<Item = &'b usize>`.
- Breaking: change `sources` in `dijkstra::Dijkstra::new` to `IntoIterator<Item = &'b usize>`.
- Breaking: change `sources` in `dijkstra_dist::Dijkstra::new` to `IntoIterator<Item = &'b usize>`.
- Breaking: change `sources` in `dijkstra_pred::Dijkstra::new` to `IntoIterator<Item = &'b usize>`.
- Improve documentation examples for `algo::bfs`.

## [0.82.3] - 2024-08-21

Added

- Add images to `bfs` documentation.
- Add images to `adjacency_list/fixture` documentation.
- Add images to `adjacency_matrix/fixture` documentation.

Fixed

- Fix `adjacency_matrix::*::*_bang_jensen_196` unit tests.
- Fix `adjacency_matrix::*::*_kattis_caninaofbabel_1` unit tests.
- Fix `adjacency_matrix::*::*_kattis_caninaofbabel_2` unit tests.

## [0.82.2] - 2024-08-20

Added

- Add images to `gen` documentation.
- Add images to `biclique` documentation.
- Add images to `circuit` documentation.
- Add images to `complete` documentation.
- Add images to `cycle` documentation.
- Add images to `empty` documentation.
- Add images to `path` documentation.
- Add images to `star` documentation.

## [0.82.1] - 2024-08-18

Added

- Add images to the examples in `gen/complete`.

## [0.82.0] - 2024-08-17

Changed

- Breaking: move `algo::types::distance_matrix` to `algo::distance_matrix`.
- Breaking: move `algo::types::predecessor_tree` to `algo::predecessor_tree`.
- Misc. documentation improvements.

## [0.81.0] - 2024-08-17

Added

- Add `bfs_dist::Bfs::distances`.
- Add `bfs_pred::Bfs::predecessors`.
- Add `bfs_pred::Bfs::shortest_path`.
- Add `dijkstra_dist::Dijkstra::distances`.
- Add `dijkstra_pred::Dijkstra::predecessors`.
- Add `dijkstra_pred::Dijkstra::shortest_path`.
- Add the `algo::bfs::Bfs` iterator.
- Add the `algo::bfs_dist::Bfs` iterator.
- Add the `algo::bfs_pred::Bfs` iterator.
- Add the `algo::dfs::Dfs` iterator.
- Add the `algo::dfs_dist::Dfs` iterator.
- Add the `algo::dijkstra::Dijkstra` iterator.
- Add the `algo::dijkstra_dist::Dijkstra` iterator.
- Add the `algo::dijkstra_pred::Dijkstra` iterator.

Removed

- Breaking: Remove `algo::bfs` functions, replaced by `bfs*::Bfs` iterators.
- Breaking: Remove `algo::dfs` functions, replaced by `dfs*::Dfs` iterators.
- Breaking: Remove `algo::dijkstra` functions, replaced by `dijkstra*::Dijkstra` iterators.

## [0.80.1] - 2024-08-11

Changed

- Minor documentation improvements.

## [0.80.0] - 2024-08-11

Changed

- Breaking: rename `op::IsWalk` to `op::HasWalk`.
- Simplify the `lib.rs` operations documentation.
- Simplify the `README` operations section.

## [0.79.0] - 2024-08-11

Changed

- Breaking: `random_tournament` now takes a `seed` parameter.
- Breaking: `erdos_renyi` now takes a `seed` parameter.

## [0.78.2] - 2024-08-11

Fixed

- Fix documentation in `erdos_renyi`.

## [0.78.1] - 2024-08-11

Added

- Add the `ErdosRenyi` generator.
- Add property test `erdos_renyi_degree` for `adjacency_matrix`.
- Add property test `erdos_renyi_degree_sum_equals_2size` for `adjacency_matrix`.
- Add property test `erdos_renyi_even_number_odd_degrees` for `adjacency_matrix`.
- Add property test `erdos_renyi_has_arc` for `adjacency_matrix`.
- Add property test `erdos_renyi_indegree` for `adjacency_matrix`.
- Add property test `erdos_renyi_is_complete` for `adjacency_matrix`.
- Add property test `erdos_renyi_is_simple` for `adjacency_matrix`.
- Add property test `erdos_renyi_is_subdigraph` for `adjacency_matrix`.
- Add property test `erdos_renyi_is_superdigraph` for `adjacency_matrix`.
- Add property test `erdos_renyi_order` for `adjacency_matrix`.
- Add property test `erdos_renyi_outdegree` for `adjacency_matrix`.
- Add property test `erdos_renyi_size_p_0` for `adjacency_matrix`.
- Add property test `erdos_renyi_size_p_1` for `adjacency_matrix`.
- Add property test `erdos_renyi_degree` for `adjacency_list`.
- Add property test `erdos_renyi_degree_sum_equals_2size` for `adjacency_list`.
- Add property test `erdos_renyi_even_number_odd_degrees` for `adjacency_list`.
- Add property test `erdos_renyi_has_arc` for `adjacency_list`.
- Add property test `erdos_renyi_indegree` for `adjacency_list`.
- Add property test `erdos_renyi_is_complete` for `adjacency_list`.
- Add property test `erdos_renyi_is_simple` for `adjacency_list`.
- Add property test `erdos_renyi_is_subdigraph` for `adjacency_list`.
- Add property test `erdos_renyi_is_superdigraph` for `adjacency_list`.
- Add property test `erdos_renyi_order` for `adjacency_list`.
- Add property test `erdos_renyi_outdegree` for `adjacency_list`.
- Add property test `erdos_renyi_size_p_0` for `adjacency_list`.
- Add property test `erdos_renyi_size_p_1` for `adjacency_list`.
- Internal: add `next_f64` for `Xoshiro256StarStar`.
- Internal: add property test `next_f64` for `Xoshiro256StarStar`.

Changed

- Misc. documentation improvements.

## [0.78.0] - 2024-08-08

Iterators now underpin the BFS and DFS implementations.

Added

- Add the `dfs::Dfs` iterator.
- Add the `dfs_depth::Dfs` iterator.
- Add the `bfs::Bfs` iterator.
- Add the `bfs_depth::Bfs` iterator.
- Add the `bfs_pred::Bfs` iterator.

Changed

- Breaking: move `Bfs::distances` to `bfs_depth::Bfs::distances`.
- Breaking: move `Bfs::predecessors` to `bfs_pred::Bfs::predecessors`.
- Breaking: move `Bfs::shortest_path` to `bfs_pred::Bfs::shortest_path`.

Removed

- Breaking: remove `dfs::dfsa`.
- Breaking: remove `dfs::dfsa_predecessors`.
- Breaking: remove `dfs::acyclic_ordering`.

## [0.77.1] - 2024-08-04

Fixed

- Fix `Bfs` links in `README.md`.

## [0.77.0] - 2024-08-04

Added

- Add `Bfs::distances`.
- Add `Bfs::predecessors`.
- Add `Bfs::shortest_path`.
- Add type `algo::Bfs` to represent the BFS algorithm.
- Add type `bfs::Step` to represent a step in the BFS algorithm.
- Implement `Bfs::new`.
- Implement `Circuit` for `adjacency_list::Digraph`.
- Implement `Circuit` for `adjacency_matrix::Digraph`.
- Implement `Iterator for Bfs` to iterate over the vertices in the BFS order.

Changed

- Benchmark the new `Circuit` implementation for `adjacency_list` alongside the previous implementation in `bench::algo::circuit`.
- Expand `adjacency_list::fixture::kattis_escapewallmaria_*` digraphs to be of order 16.
- Expand `adjacency_matrix::fixture::kattis_escapewallmaria_*` digraphs to be of order 16.
- Show an implementation that does not use `AddArc` and `Empty` in the `algo::circuit` documentation.
- Sync `adjacency_list::digraph` unit tests with the new `adjacency_list::fixture::kattis_escapewallmaria_*` fixtures.
- Sync `adjacency_matrix::digraph` unit tests with the new `adjacency_matrix::fixture::kattis_escapewallmaria_*` fixtures.
- Use the new `Bfs` API in `bench::algo::single_source_distances`.
- Use the new `Bfs` API in `bfs` documentation test examples.
- Use the new `Bfs` API in `bfs` unit tests.
- Use the new `Bfs` API in `predecessor_tree` documentation test examples.

Removed

- Breaking: remove `Circuit for D: AddArc + Empty` in favor of `Circuit for adjacency_list::Digraph` and `Circuit for adjacency_matrix::Digraph`.
- Breaking: remove `bfs::distances` in favor of `bfs::Bfs::distances`.
- Breaking: remove `bfs::predecessors` in favor of `bfs::Bfs::predecessors`.
- Breaking: remove `bfs::shortest_path` in favor of `bfs::Bfs::shortest_path`.
- Breaking: remove `bfs::single_pair_shortest_path`.
- Breaking: remove `bfs::single_source_distances`.
- Breaking: remove `bfs::single_source_predecessors`.
- Remove references to `algo::bfs` and `algo::floyd_warshall` from the `algo::dijkstra` documentation.
- Remove references to `algo::dijkstra` from the `algo::bfs` documentation.

## [0.76.0] - 2024-08-03

Added

- Add benchmark `biclique`.
- Add benchmark `circuit`.
- Add benchmark `complete`.
- Add benchmark `cycle`.
- Add benchmark `empty`.
- Add benchmark `path`.
- Add benchmark `random_tournament`.
- Add benchmark `star`.
- Add property test `random_tournament_has_arc` for `adjacency_list`.
- Add property test `random_tournament_has_arc` for `adjacency_matrix`.

Changed

- Breaking: remove `Biclique` blanket implementation for `D: AddArc + Empty`.
- Improve performance of `Biclique::biclique` for `adjacency_list`.
- Remove `sample_size` attribute from `single_source_distances` benchmarks.

## [0.75.4] - 2024-07-30

Added

- Add property test `biclique_degree_sequence` for `adjacency_list`.
- Add property test `biclique_degree_sequence` for `adjacency_matrix`.
- Add property test `circuit_degree_sequence` for `adjacency_list`.
- Add property test `circuit_degree_sequence` for `adjacency_matrix`.
- Add property test `complete_degree_sequence` for `adjacency_list`.
- Add property test `complete_degree_sequence` for `adjacency_matrix`.
- Add property test `cycle_degree_sequence` for `adjacency_list`.
- Add property test `cycle_degree_sequence` for `adjacency_matrix`.
- Add property test `empty_degree_sequence` for `adjacency_list`.
- Add property test `empty_degree_sequence` for `adjacency_matrix`.
- Add property test `path_degree_sequence` for `adjacency_list`.
- Add property test `path_degree_sequence` for `adjacency_matrix`.
- Add property test `random_tournament_degree_sequence` for `adjacency_list`.
- Add property test `random_tournament_degree_sequence` for `adjacency_matrix`.
- Add property test `star_degree_sequence` for `adjacency_list`.
- Add property test `star_degree_sequence` for `adjacency_matrix`.
- Add the `DegreeSequence` trait.
- Add unit test `degree_sequence_bang_jensen_196` for `adjacency_list`.
- Add unit test `degree_sequence_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `degree_sequence_bang_jensen_34` for `adjacency_list`.
- Add unit test `degree_sequence_bang_jensen_34` for `adjacency_matrix`.
- Add unit test `degree_sequence_bang_jensen_94` for `adjacency_list`.
- Add unit test `degree_sequence_bang_jensen_94` for `adjacency_matrix`.
- Add unit test `degree_sequence_kattis_builddeps` for `adjacency_list`.
- Add unit test `degree_sequence_kattis_builddeps` for `adjacency_matrix`.
- Add unit test `degree_sequence_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `degree_sequence_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `degree_sequence_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `degree_sequence_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `degree_sequence_kattis_escapewallmaria_1` for `adjacency_list`.
- Add unit test `degree_sequence_kattis_escapewallmaria_1` for `adjacency_matrix`.
- Add unit test `degree_sequence_kattis_escapewallmaria_2` for `adjacency_list`.
- Add unit test `degree_sequence_kattis_escapewallmaria_2` for `adjacency_matrix`.
- Add unit test `degree_sequence_kattis_escapewallmaria_3` for `adjacency_list`.
- Add unit test `degree_sequence_kattis_escapewallmaria_3` for `adjacency_matrix`.

## [0.75.3] - 2024-07-28

Added

- Add the `Sources` trait.
- Add property test `biclique_sources` for `adjacency_list`.
- Add property test `biclique_sources` for `adjacency_matrix`.
- Add property test `circuit_sources` for `adjacency_list`.
- Add property test `circuit_sources` for `adjacency_matrix`.
- Add property test `complete_sources` for `adjacency_list`.
- Add property test `complete_sources` for `adjacency_matrix`.
- Add property test `cycle_sources` for `adjacency_list`.
- Add property test `cycle_sources` for `adjacency_matrix`.
- Add property test `empty_sources` for `adjacency_list`.
- Add property test `empty_sources` for `adjacency_matrix`.
- Add property test `path_sources` for `adjacency_list`.
- Add property test `path_sources` for `adjacency_matrix`.
- Add property test `random_tournament_sources` for `adjacency_list`.
- Add property test `random_tournament_sources` for `adjacency_matrix`.
- Add property test `star_sources` for `adjacency_list`.
- Add property test `star_sources` for `adjacency_matrix`.
- Add unit test `sources_bang_jensen_196` for `adjacency_list`.
- Add unit test `sources_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `sources_bang_jensen_34` for `adjacency_list`.
- Add unit test `sources_bang_jensen_34` for `adjacency_matrix`.
- Add unit test `sources_bang_jensen_94` for `adjacency_list`.
- Add unit test `sources_bang_jensen_94` for `adjacency_matrix`.
- Add unit test `sources_kattis_builddeps` for `adjacency_list`.
- Add unit test `sources_kattis_builddeps` for `adjacency_matrix`.
- Add unit test `sources_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `sources_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `sources_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `sources_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `sources_kattis_escapewallmaria_1` for `adjacency_list`.
- Add unit test `sources_kattis_escapewallmaria_1` for `adjacency_matrix`.
- Add unit test `sources_kattis_escapewallmaria_2` for `adjacency_list`.
- Add unit test `sources_kattis_escapewallmaria_2` for `adjacency_matrix`.
- Add unit test `sources_kattis_escapewallmaria_3` for `adjacency_list`.
- Add unit test `sources_kattis_escapewallmaria_3` for `adjacency_matrix`.

## [0.75.2] - 2024-07-28

Added

- Add unit test `arcs_bang_jensen_196` for `adjacency_list`.
- Add unit test `arcs_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `arcs_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `arcs_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `arcs_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `arcs_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `arcs_weighted_bang_jensen_196` for `adjacency_list`.
- Add unit test `arcs_weighted_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `arcs_weighted_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `arcs_weighted_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `arcs_weighted_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `arcs_weighted_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `converse_bang_jensen_196` for `adjacency_list`.
- Add unit test `converse_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `converse_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `converse_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `converse_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `converse_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `degree_bang_jensen_196` for `adjacency_list`.
- Add unit test `degree_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `degree_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `degree_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `degree_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `degree_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `in_neighbors_bang_jensen_196` for `adjacency_list`.
- Add unit test `in_neighbors_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `in_neighbors_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `in_neighbors_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `in_neighbors_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `in_neighbors_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `indegree_bang_jensen_196` for `adjacency_list`.
- Add unit test `indegree_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `indegree_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `indegree_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `indegree_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `indegree_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `indegree_sequence_bang_jensen_196` for `adjacency_list`.
- Add unit test `indegree_sequence_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `indegree_sequence_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `indegree_sequence_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `indegree_sequence_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `indegree_sequence_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_balanced_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_balanced_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_balanced_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_balanced_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_balanced_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_balanced_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_complete_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_complete_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_complete_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_complete_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_complete_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_complete_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_isolated_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_isolated_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_isolated_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_isolated_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_isolated_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_isolated_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_oriented_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_oriented_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_oriented_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_oriented_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_oriented_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_oriented_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_pendant_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_pendant_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_pendant_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_pendant_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_pendant_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_pendant_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_regular_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_regular_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_regular_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_regular_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_regular_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_regular_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_semicomplete_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_semicomplete_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_semicomplete_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_semicomplete_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_semicomplete_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_semicomplete_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_simple_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_simple_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_simple_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_simple_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_simple_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_simple_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_symmetric_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_symmetric_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_symmetric_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_symmetric_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_symmetric_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_symmetric_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `is_tournament_bang_jensen_196` for `adjacency_list`.
- Add unit test `is_tournament_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `is_tournament_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `is_tournament_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `is_tournament_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `is_tournament_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `order_bang_jensen_196` for `adjacency_list`.
- Add unit test `order_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `order_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `order_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `order_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `order_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `out_neighbors_bang_jensen_196` for `adjacency_list`.
- Add unit test `out_neighbors_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `out_neighbors_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `out_neighbors_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `out_neighbors_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `out_neighbors_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `out_neighbors_weighted_bang_jensen_196` for `adjacency_list`.
- Add unit test `out_neighbors_weighted_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `out_neighbors_weighted_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `out_neighbors_weighted_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `out_neighbors_weighted_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `out_neighbors_weighted_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `outdegree_bang_jensen_196` for `adjacency_list`.
- Add unit test `outdegree_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `outdegree_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `outdegree_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `outdegree_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `outdegree_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_bang_jensen_196` for `adjacency_list`.
- Add unit test `outdegree_sequence_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `outdegree_sequence_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `outdegree_sequence_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `remove_arc_bang_jensen_196` for `adjacency_list`.
- Add unit test `remove_arc_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `remove_arc_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `remove_arc_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `remove_arc_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `remove_arc_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `semidegree_sequence_bang_jensen_196` for `adjacency_list`.
- Add unit test `semidegree_sequence_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `semidegree_sequence_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `semidegree_sequence_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `semidegree_sequence_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `semidegree_sequence_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `sinks_bang_jensen_196` for `adjacency_list`.
- Add unit test `sinks_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `sinks_bang_jensen_34` for `adjacency_list`.
- Add unit test `sinks_bang_jensen_34` for `adjacency_matrix`.
- Add unit test `sinks_bang_jensen_94` for `adjacency_list`.
- Add unit test `sinks_bang_jensen_94` for `adjacency_matrix`.
- Add unit test `sinks_kattis_builddeps` for `adjacency_list`.
- Add unit test `sinks_kattis_builddeps` for `adjacency_matrix`.
- Add unit test `sinks_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `sinks_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `sinks_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `sinks_kattis_cantinaofbabel_2` for `adjacency_matrix`.
- Add unit test `sinks_kattis_escapewallmaria_1` for `adjacency_list`.
- Add unit test `sinks_kattis_escapewallmaria_1` for `adjacency_matrix`.
- Add unit test `sinks_kattis_escapewallmaria_2` for `adjacency_list`.
- Add unit test `sinks_kattis_escapewallmaria_2` for `adjacency_matrix`.
- Add unit test `sinks_kattis_escapewallmaria_3` for `adjacency_list`.
- Add unit test `sinks_kattis_escapewallmaria_3` for `adjacency_matrix`.
- Add unit test `size_bang_jensen_196` for `adjacency_list`.
- Add unit test `size_bang_jensen_196` for `adjacency_matrix`.
- Add unit test `size_kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `size_kattis_cantinaofbabel_1` for `adjacency_matrix`.
- Add unit test `size_kattis_cantinaofbabel_2` for `adjacency_list`.
- Add unit test `size_kattis_cantinaofbabel_2` for `adjacency_matrix`.

## [0.75.1] - 2024-07-28

Added

- Add fixture `kattis_cantinaofbabel_2` for `adjacency_list`.
- Add property test `biclique_sinks` for `adjacency_list`.
- Add property test `biclique_sinks` for `adjacency_matrix`.
- Add property test `circuit_sinks` for `adjacency_list`.
- Add property test `circuit_sinks` for `adjacency_matrix`.
- Add property test `circuit_size` for `adjacency_list`.
- Add property test `circuit_size` for `adjacency_matrix`.
- Add property test `complete_complement_size` for `adjacency_list`.
- Add property test `complete_complement_size` for `adjacency_matrix`.
- Add property test `complete_sinks` for `adjacency_list`.
- Add property test `complete_sinks` for `adjacency_matrix`.
- Add property test `cycle_sinks` for `adjacency_list`.
- Add property test `cycle_sinks` for `adjacency_matrix`.
- Add property test `cycle_size` for `adjacency_list`.
- Add property test `cycle_size` for `adjacency_matrix`.
- Add property test `empty_complement_size` for `adjacency_list`.
- Add property test `empty_complement_size` for `adjacency_matrix`.
- Add property test `empty_even_number_odd_degrees` for `adjacency_list`.
- Add property test `empty_even_number_odd_degrees` for `adjacency_matrix`.
- Add property test `empty_sinks` for `adjacency_list`.
- Add property test `empty_sinks` for `adjacency_matrix`.
- Add property test `empty_size` for `adjacency_list`.
- Add property test `empty_size` for `adjacency_matrix`.
- Add property test `path_sinks` for `adjacency_list`.
- Add property test `path_sinks` for `adjacency_matrix`.
- Add property test `path_size` for `adjacency_list`.
- Add property test `path_size` for `adjacency_matrix`.
- Add property test `random_tournament_complement_size` for `adjacency_list`.
- Add property test `random_tournament_complement_size` for `adjacency_matrix`.
- Add property test `star_sinks` for `adjacency_list`.
- Add property test `star_sinks` for `adjacency_matrix`.
- Add property test `star_size` for `adjacency_list`.
- Add property test `star_size` for `adjacency_matrix`.
- Add trait `op::Sinks`.
- Add unit test `strongly_connected_components_kattis_cantinaofbabel_2` for `algo::tarjan`.

Changed

- Add a human-readable adjacency list to each documentation test digraph.
- Use `Vec<BTreeSet<usize>>` in `op` implementation examples.

## [0.75.0] - 2024-07-27

Changed

- Clean up tests; make all property tests complete.

## [0.74.5] - 2024-07-25

Changed

- Format source code with `max_length = 79`.
- Remove macros from the `dijkstra` tests.

## [0.74.4] - 2024-07-25

Added

- Add a documentation test example for `IndegreeSequence`.
- Add a documentation test example for `OutdegreeSequence`.

## [0.74.3] - 2024-07-24

Fixed

- Fix misc. documentation errors.

## [0.74.2] - 2024-07-24

Added

- Add property test `biclique_outdegree_sequence` for `adjacency_list`.
- Add property test `biclique_outdegree_sequence` for `adjacency_matrix`.
- Add property test `circuit_outdegree_sequence` for `adjacency_list`.
- Add property test `circuit_outdegree_sequence` for `adjacency_matrix`.
- Add property test `circuit_outdegree` for `adjacency_list`.
- Add property test `circuit_outdegree` for `adjacency_matrix`.
- Add property test `complete_outdegree_sequence` for `adjacency_list`.
- Add property test `complete_outdegree_sequence` for `adjacency_matrix`.
- Add property test `cycle_outdegree_sequence` for `adjacency_list`.
- Add property test `cycle_outdegree_sequence` for `adjacency_matrix`.
- Add property test `cycle_outdegree` for `adjacency_list`.
- Add property test `cycle_outdegree` for `adjacency_matrix`.
- Add property test `empty_outdegree_sequence` for `adjacency_list`.
- Add property test `empty_outdegree_sequence` for `adjacency_matrix`.
- Add property test `path_outdegree_sequence` for `adjacency_list`.
- Add property test `path_outdegree_sequence` for `adjacency_matrix`.
- Add property test `path_outdegree` for `adjacency_list`.
- Add property test `path_outdegree` for `adjacency_matrix`.
- Add property test `random_tournament_outdegree_sequence` for `adjacency_list`.
- Add property test `random_tournament_outdegree_sequence` for `adjacency_matrix`.
- Add property test `star_outdegree_sequence` for `adjacency_list`.
- Add property test `star_outdegree_sequence` for `adjacency_matrix`.
- Add property test `star_outdegree` for `adjacency_list`.
- Add property test `star_outdegree` for `adjacency_matrix`.
- Add the `OutdegreeSequence` trait.
- Add unit test `circuit_1_outdegree_sequence` for `adjacency_list`.
- Add unit test `circuit_1_outdegree_sequence` for `adjacency_matrix`.
- Add unit test `circuit_1_outdegree` for `adjacency_list`.
- Add unit test `circuit_1_outdegree` for `adjacency_matrix`.
- Add unit test `cycle_1_outdegree_sequence` for `adjacency_list`.
- Add unit test `cycle_1_outdegree_sequence` for `adjacency_matrix`.
- Add unit test `cycle_1_outdegree` for `adjacency_list`.
- Add unit test `cycle_1_outdegree` for `adjacency_matrix`.
- Add unit test `cycle_2_outdegree` for `adjacency_list`.
- Add unit test `cycle_2_outdegree` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_bang_jensen_34` for `adjacency_list`.
- Add unit test `outdegree_sequence_bang_jensen_34` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_bang_jensen_94` for `adjacency_list`.
- Add unit test `outdegree_sequence_bang_jensen_94` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_kattis_builddeps` for `adjacency_list`.
- Add unit test `outdegree_sequence_kattis_builddeps` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_kattis_escapewallmaria_1` for `adjacency_list`.
- Add unit test `outdegree_sequence_kattis_escapewallmaria_1` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_kattis_escapewallmaria_2` for `adjacency_list`.
- Add unit test `outdegree_sequence_kattis_escapewallmaria_2` for `adjacency_matrix`.
- Add unit test `outdegree_sequence_kattis_escapewallmaria_3` for `adjacency_list`.
- Add unit test `outdegree_sequence_kattis_escapewallmaria_3` for `adjacency_matrix`.

## [0.74.1] - 2024-07-24

Added

- Add `IndegreeSequence` to the `README`.
- Add `IndegreeSequence` to `lib.rs`.

## [0.74.0] - 2024-07-24

Added

- Add property test `biclique_indegree_sequence` for `adjacency_list`.
- Add property test `biclique_indegree_sequence` for `adjacency_matrix`.
- Add property test `circuit_indegree_sequence` for `adjacency_list`.
- Add property test `circuit_indegree_sequence` for `adjacency_matrix`.
- Add property test `complete_indegree_sequence` for `adjacency_list`.
- Add property test `complete_indegree_sequence` for `adjacency_matrix`.
- Add property test `cycle_indegree_sequence` for `adjacency_list`.
- Add property test `cycle_indegree_sequence` for `adjacency_matrix`.
- Add property test `empty_indegree_sequence` for `adjacency_list`.
- Add property test `empty_indegree_sequence` for `adjacency_matrix`.
- Add property test `path_indegree_sequence` for `adjacency_list`.
- Add property test `path_indegree_sequence` for `adjacency_matrix`.
- Add property test `random_tournament_indegree_sequence` for `adjacency_list`.
- Add property test `random_tournament_indegree_sequence` for `adjacency_matrix`.
- Add property test `star_indegree_sequence` for `adjacency_list`.
- Add property test `star_indegree_sequence` for `adjacency_matrix`.
- Add the `IndegreeSequence` trait.
- Add unit test `circuit_1_indegree_sequence` for `adjacency_list`.
- Add unit test `circuit_1_indegree_sequence` for `adjacency_matrix`.
- Add unit test `cycle_1_indegree_sequence` for `adjacency_list`.
- Add unit test `cycle_1_indegree_sequence` for `adjacency_matrix`.
- Add unit test `cycle_2_indegree_sequence` for `adjacency_list`.
- Add unit test `cycle_2_indegree_sequence` for `adjacency_matrix`.
- Add unit test `indegree_sequence_bang_jensen_34` for `adjacency_list`.
- Add unit test `indegree_sequence_bang_jensen_34` for `adjacency_matrix`.
- Add unit test `indegree_sequence_bang_jensen_94` for `adjacency_list`.
- Add unit test `indegree_sequence_bang_jensen_94` for `adjacency_matrix`.
- Add unit test `indegree_sequence_kattis_builddeps` for `adjacency_list`.
- Add unit test `indegree_sequence_kattis_builddeps` for `adjacency_matrix`.
- Add unit test `indegree_sequence_kattis_escapewallmaria_1` for `adjacency_list`.
- Add unit test `indegree_sequence_kattis_escapewallmaria_1` for `adjacency_matrix`.
- Add unit test `indegree_sequence_kattis_escapewallmaria_2` for `adjacency_list`.
- Add unit test `indegree_sequence_kattis_escapewallmaria_2` for `adjacency_matrix`.
- Add unit test `indegree_sequence_kattis_escapewallmaria_3` for `adjacency_list`.
- Add unit test `indegree_sequence_kattis_escapewallmaria_3` for `adjacency_matrix`.

Changed

- Breaking: Rename `DegreeSequence::degree_sequence` to `SemidegreeSequence::semidegree_sequence`.
- Breaking: Rename `DegreeSequence` to `SemidegreeSequence`.
- Breaking: `SemidegreeSequence::semidegree_sequence` now returns an iterator.

## [0.73.1] - 2024-07-22

Changed

- Add `gen::Path` to `README`.
- Add `gen::Path` to `lib.rs`.

## [0.73.0] - 2024-07-22

Added

- Add property test `path_complement_size` for `adjacency_list`.
- Add property test `path_degree_sequence` for `adjacency_list`.
- Add property test `path_degree_sum_equals_2size` for `adjacency_list`.
- Add property test `path_degree` for `adjacency_list`.
- Add property test `path_even_number_odd_degrees` for `adjacency_list`.
- Add property test `path_has_edge` for `adjacency_list`.
- Add property test `path_indegree` for `adjacency_list`.
- Add property test `path_is_balanced` for `adjacency_list`.
- Add property test `path_is_complete` for `adjacency_list`.
- Add property test `path_is_isolated` for `adjacency_list`.
- Add property test `path_is_oriented` for `adjacency_list`.
- Add property test `path_is_pendant` for `adjacency_list`.
- Add property test `path_is_regular` for `adjacency_list`.
- Add property test `path_is_semicomplete` for `adjacency_list`.
- Add property test `path_is_simple` for `adjacency_list`.
- Add property test `path_is_sink` for `adjacency_list`.
- Add property test `path_is_source` for `adjacency_list`.
- Add property test `path_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `path_is_subdigraph` for `adjacency_list`.
- Add property test `path_is_superdigraph` for `adjacency_list`.
- Add property test `path_is_symmetric` for `adjacency_list`.
- Add property test `path_is_tournament` for `adjacency_list`.
- Add property test `star_complement_size` for `adjacency_list`.
- Add unit test `biclique_1_1_complement` for `adjacency_list`.
- Add unit test `biclique_1_2_complement` for `adjacency_list`.
- Add unit test `biclique_2_1_complement` for `adjacency_list`.
- Add unit test `biclique_2_2_complement` for `adjacency_list`.
- Add unit test `circuit_1_complement` for `adjacency_list`.
- Add unit test `circuit_2_complement` for `adjacency_list`.
- Add unit test `circuit_3_complement` for `adjacency_list`.
- Add unit test `complete_1_complement` for `adjacency_list`.
- Add unit test `complete_2_complement` for `adjacency_list`.
- Add unit test `complete_3_complement` for `adjacency_list`.
- Add unit test `cycle_1_complement` for `adjacency_list`.
- Add unit test `cycle_2_complement` for `adjacency_list`.
- Add unit test `cycle_3_complement` for `adjacency_list`.
- Add unit test `empty_1_complement` for `adjacency_list`.
- Add unit test `empty_2_complement` for `adjacency_list`.
- Add unit test `empty_3_complement` for `adjacency_list`.
- Add unit test `path_1_complement` for `adjacency_list`.
- Add unit test `path_1_is_balanced` for `adjacency_list`.
- Add unit test `path_1_is_complete` for `adjacency_list`.
- Add unit test `path_1_is_isolated` for `adjacency_list`.
- Add unit test `path_1_is_pendant` for `adjacency_list`.
- Add unit test `path_1_is_regular` for `adjacency_list`.
- Add unit test `path_1_is_semicomplete` for `adjacency_list`.
- Add unit test `path_1_is_sink` for `adjacency_list`.
- Add unit test `path_1_is_symmetric` for `adjacency_list`.
- Add unit test `path_1_is_tournament` for `adjacency_list`.
- Add unit test `path_1` for `adjacency_list`.
- Add unit test `path_2_complement` for `adjacency_list`.
- Add unit test `path_2_is_balanced` for `adjacency_list`.
- Add unit test `path_2_is_complete` for `adjacency_list`.
- Add unit test `path_2_is_regular` for `adjacency_list`.
- Add unit test `path_2_is_semicomplete` for `adjacency_list`.
- Add unit test `path_2_is_symmetric` for `adjacency_list`.
- Add unit test `path_2_is_tournament` for `adjacency_list`.
- Add unit test `path_2` for `adjacency_list`.
- Add unit test `path_3_complement` for `adjacency_list`.
- Add unit test `path_3` for `adjacency_list`.
- Add unit test `star_1_complement` for `adjacency_list`.
- Add unit test `star_2_complement` for `adjacency_list`.
- Add unit test `star_3_complement` for `adjacency_list`.
- Export `algo::PredecessorTree`.
- Export `algo::DistanceMatrix`.

Changed

- Breaking: Move `algo::predecessor_tree` to `algo::types::predecessor_tree`.
- Breaking: Move `algo::distance_matrix` to `algo::types::distance_matrix`.
- Expand property test `star_has_edge` for `adjacency_list`.
- Expand property test `star_has_edge` for `adjacency_matrix`.
- Improve documentation consistency.

## [0.72.1] - 2024-07-21

Added

- Add fixture `kattis_cantinaofbabel_1` for `adjacency_list`.
- Add unit test `strongly_connected_components_kattis_cantinaofbabel_1` for `algo::tarjan`.

## [0.72.0] - 2024-07-21

Added

- Add proptest strategy `arc`.
- Add unit test `add_arc_out_of_bounds_u` for `adjacency_list`.
- Add unit test `add_arc_out_of_bounds_u` for `adjacency_matrix`.
- Add unit test `add_arc_out_of_bounds_v` for `adjacency_list`.
- Add unit test `add_arc_out_of_bounds_v` for `adjacency_matrix`.
- Add unit test `add_arc_u_equals_v` for `adjacency_list`.
- Add unit test `add_arc_u_equals_v` for `adjacency_matrix`.
- Add unit test `add_arc_weighted_out_of_bounds_u` for `adjacency_list_weighted`.
- Add unit test `add_arc_weighted_out_of_bounds_v` for `adjacency_list_weighted`.
- Add unit test `add_arc_weighted_u_equals_v` for `adjacency_list_weighted`.
- Add unit test `from_vec_out_of_bounds_v` for `adjacency_list`.
- Add unit test `from_vec_out_of_bounds_v` for `adjacency_matrix`.
- Add unit test `from_vec_u_equals_v` for `adjacency_list`.
- Add unit test `from_vec_u_equals_v` for `adjacency_matrix`.
- Add unit test `from_vec` for `adjacency_list`.

Changed

- Breaking: `From<Vec<BTreeMap<usize, W>>>` for `adjacency_list_weighted::Digraph` panics if for any arc `u -> v`, `u` equals `v`.
- Breaking: `From<Vec<BTreeMap<usize, W>>>` for `adjacency_list_weighted::Digraph` panics if for any arc `u -> v`, `v` is not in the digraph.
- Breaking: `From<Vec<BTreeSet<usize>>>` for `adjacency_list::Digraph` panics if for any arc `u -> v`, `u` equals `v`.
- Breaking: `From<Vec<BTreeSet<usize>>>` for `adjacency_list::Digraph` panics if for any arc `u -> v`, `v` is not in the digraph.
- Breaking: `adjacency_list::Digraph::add_arc` panics if `u` equals `v`.
- Breaking: `adjacency_list::Digraph::add_arc` panics if `u` is not in the digraph.
- Breaking: `adjacency_list::Digraph::add_arc` panics if `v` is not in the digraph.
- Breaking: `adjacency_list_weighted::Digraph::add_arc_weighted` panics if `u` equals `v`.
- Breaking: `adjacency_list_weighted::Digraph::add_arc_weighted` panics if `u` is not in the digraph.
- Breaking: `adjacency_list_weighted::Digraph::add_arc_weighted` panics if `v` is not in the digraph.
- Breaking: `adjacency_matrix::Digraph::add_arc` panics if `u` equals `v`.
- Breaking: `adjacency_matrix::Digraph::toggle` panics if `u` equals `v`.
- Change unit test `from_vec` for `adjacency_matrix` to match `adjacency_list`.
- Rename `*_eq_*` unit tests to `*_equals_*`.

Fixed

- Fix documentation tests for `op::is_superdigraph`.
- Fix documentation tests for `op::out_neighbors_weighted`.

## [0.71.7] - 2024-07-20

Added

- Add unit test `acyclic_ordering_trivial` for `dfs`.
- Add unit test `dfsa_trivial` for `dfs`.
- Add unit test `strongly_connected_components_trivial` for `tarjan`.

## [0.71.6] - 2024-07-20

Added

- Add fixture `bang_jensen_196` for `adjacency_list`.
- Add unit test `strongly_connected_components_bang_jensen_196` for `algo::tarjan`.

## [0.71.5] - 2024-07-20

Added

- Add `op::IsSpanningSubdigraph` to the `README`.
- Add `op::IsSpanningSubdigraph` to `lib.rs`.

## [0.71.4] - 2024-07-20

Added

- Add `op::IsSpanningSubdigraph`.
- Add property test `biclique_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `biclique_is_spanning_subdigraph` for `adjacency_matrix`.
- Add property test `circuit_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `circuit_is_spanning_subdigraph` for `adjacency_matrix`.
- Add property test `complete_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `complete_is_spanning_subdigraph` for `adjacency_matrix`.
- Add property test `cycle_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `cycle_is_spanning_subdigraph` for `adjacency_matrix`.
- Add property test `empty_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `empty_is_spanning_subdigraph` for `adjacency_matrix`.
- Add property test `random_tournament_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `random_tournament_is_spanning_subdigraph` for `adjacency_matrix`.
- Add property test `star_is_spanning_subdigraph` for `adjacency_list`.
- Add property test `star_is_spanning_subdigraph` for `adjacency_matrix`.

## [0.71.3] - 2024-07-19

Added

- Add `algo::tarjan::strongly_connected_components`.

## [0.71.2] - 2024-07-18

Added

- Add property test `cycle_complement_size` for `adjacency_list`.
- Add property test `cycle_complement_size` for `adjacency_matrix`.
- Add property test `cycle_degree_sequence` for `adjacency_list`.
- Add property test `cycle_degree_sequence` for `adjacency_matrix`.
- Add property test `cycle_degree_sum_eq_2size` for `adjacency_list`.
- Add property test `cycle_degree_sum_eq_2size` for `adjacency_matrix`.
- Add property test `cycle_degree` for `adjacency_list`.
- Add property test `cycle_degree` for `adjacency_matrix`.
- Add property test `cycle_even_number_odd_degrees` for `adjacency_list`.
- Add property test `cycle_even_number_odd_degrees` for `adjacency_matrix`.
- Add property test `cycle_indegree` for `adjacency_list`.
- Add property test `cycle_indegree` for `adjacency_matrix`.
- Add property test `cycle_is_balanced` for `adjacency_list`.
- Add property test `cycle_is_balanced` for `adjacency_matrix`.
- Add property test `cycle_is_complete` for `adjacency_list`.
- Add property test `cycle_is_complete` for `adjacency_matrix`.
- Add property test `cycle_is_isolated` for `adjacency_list`.
- Add property test `cycle_is_isolated` for `adjacency_matrix`.
- Add property test `cycle_is_oriented` for `adjacency_list`.
- Add property test `cycle_is_oriented` for `adjacency_matrix`.
- Add property test `cycle_is_pendant` for `adjacency_list`.
- Add property test `cycle_is_pendant` for `adjacency_matrix`.
- Add property test `cycle_is_regular` for `adjacency_list`.
- Add property test `cycle_is_regular` for `adjacency_matrix`.
- Add property test `cycle_is_semicomplete` for `adjacency_list`.
- Add property test `cycle_is_semicomplete` for `adjacency_matrix`.
- Add property test `cycle_is_simple` for `adjacency_list`.
- Add property test `cycle_is_simple` for `adjacency_matrix`.
- Add property test `cycle_is_sink` for `adjacency_list`.
- Add property test `cycle_is_sink` for `adjacency_matrix`.
- Add property test `cycle_is_source` for `adjacency_list`.
- Add property test `cycle_is_source` for `adjacency_matrix`.
- Add property test `cycle_is_subdigraph` for `adjacency_list`.
- Add property test `cycle_is_subdigraph` for `adjacency_matrix`.
- Add property test `cycle_is_superdigraph` for `adjacency_list`.
- Add property test `cycle_is_superdigraph` for `adjacency_matrix`.
- Add property test `cycle_is_symmetric` for `adjacency_list`.
- Add property test `cycle_is_symmetric` for `adjacency_matrix`.
- Add property test `cycle_is_tournament` for `adjacency_list`.
- Add property test `cycle_is_tournament` for `adjacency_matrix`.
- Add unit test `cycle_1_degree` for `adjacency_list`.
- Add unit test `cycle_1_degree` for `adjacency_matrix`.
- Add unit test `cycle_1_indegree` for `adjacency_list`.
- Add unit test `cycle_1_indegree` for `adjacency_matrix`.
- Add unit test `cycle_1_is_complete` for `adjacency_list`.
- Add unit test `cycle_1_is_complete` for `adjacency_matrix`.
- Add unit test `cycle_1_is_isolated` for `adjacency_list`.
- Add unit test `cycle_1_is_isolated` for `adjacency_matrix`.
- Add unit test `cycle_1_is_oriented` for `adjacency_list`.
- Add unit test `cycle_1_is_oriented` for `adjacency_matrix`.
- Add unit test `cycle_1_is_semicomplete` for `adjacency_list`.
- Add unit test `cycle_1_is_semicomplete` for `adjacency_matrix`.
- Add unit test `cycle_1_is_sink` for `adjacency_list`.
- Add unit test `cycle_1_is_sink` for `adjacency_matrix`.
- Add unit test `cycle_1_is_source` for `adjacency_list`.
- Add unit test `cycle_1_is_source` for `adjacency_matrix`.
- Add unit test `cycle_1_is_tournament` for `adjacency_list`.
- Add unit test `cycle_1_is_tournament` for `adjacency_matrix`.
- Add unit test `cycle_2_degree` for `adjacency_list`.
- Add unit test `cycle_2_degree` for `adjacency_matrix`.
- Add unit test `cycle_2_indegree` for `adjacency_list`.
- Add unit test `cycle_2_indegree` for `adjacency_matrix`.
- Add unit test `cycle_2_is_complete` for `adjacency_list`.
- Add unit test `cycle_2_is_complete` for `adjacency_matrix`.
- Add unit test `cycle_2_is_semicomplete` for `adjacency_list`.
- Add unit test `cycle_2_is_semicomplete` for `adjacency_matrix`.
- Add unit test `from_adjacency_list_isize` for `adjacency_list_weighted`.
- Add unit test `from_adjacency_list_usize` for `adjacency_list_weighted`.
- Add unit test `from_adjacency_list` for `adjacency_matrix`.
- Add unit test `from_adjacency_matrix_isize` for `adjacency_list_weighted`.
- Add unit test `from_adjacency_matrix_usize` for `adjacency_list_weighted`.
- Add unit test `from_vec` for `adjacency_matrix`.
- Implement `From<adjacency_list::Digraph>` for `adjacency_list_weighted::Digraph<isize>`.
- Implement `From<adjacency_list::Digraph>` for `adjacency_list_weighted::Digraph<usize>`.
- Implement `From<adjacency_list::Digraph>` for `adjacency_matrix::Digraph`.
- Implement `From<adjacency_matrix::Digraph>` for `adjacency_list_weighted::Digraph<isize>`.
- Implement `From<adjacency_matrix::Digraph>` for `adjacency_list_weighted::Digraph<usize>`.

## [0.71.1] - 2024-07-17

Added

- Add `Circuit` to the `README`.
- Add `Circuit` to `lib.rs`.

## [0.71.0] - 2024-07-17

Added

- Add a new `Cycle` trait that generates a bidirectional cycle.
- Add unit test `biclique_1_1` for `adjacency_list`.
- Add unit test `biclique_1_1` for `adjacency_matrix`.
- Add unit test `biclique_1_2` for `adjacency_list`.
- Add unit test `biclique_1_2` for `adjacency_matrix`.
- Add unit test `biclique_2_1` for `adjacency_list`.
- Add unit test `biclique_2_1` for `adjacency_matrix`.
- Add unit test `biclique_2_2` for `adjacency_list`.
- Add unit test `biclique_2_2` for `adjacency_matrix`.
- Add unit test `circuit_1_degree_sequence` for `adjacency_list`.
- Add unit test `circuit_1_degree_sequence` for `adjacency_matrix`.
- Add unit test `circuit_1_degree` for `adjacency_list`.
- Add unit test `circuit_1_degree` for `adjacency_matrix`.
- Add unit test `circuit_1_is_sink` for `adjacency_list`.
- Add unit test `circuit_1_is_sink` for `adjacency_matrix`.
- Add unit test `circuit_1_is_source` for `adjacency_list`.
- Add unit test `circuit_1_is_source` for `adjacency_matrix`.
- Add unit test `circuit_1` for `adjacency_list`.
- Add unit test `circuit_1` for `adjacency_matrix`.
- Add unit test `circuit_2` for `adjacency_list`.
- Add unit test `circuit_2` for `adjacency_matrix`.
- Add unit test `circuit_3` for `adjacency_list`.
- Add unit test `circuit_3` for `adjacency_matrix`.
- Add unit test `complete_1` for `adjacency_list`.
- Add unit test `complete_1` for `adjacency_matrix`.
- Add unit test `complete_2` for `adjacency_list`.
- Add unit test `complete_2` for `adjacency_matrix`.
- Add unit test `complete_3` for `adjacency_list`.
- Add unit test `complete_3` for `adjacency_matrix`.
- Add unit test `cycle_1` for `adjacency_list`.
- Add unit test `cycle_1` for `adjacency_matrix`.
- Add unit test `cycle_2` for `adjacency_list`.
- Add unit test `cycle_2` for `adjacency_matrix`.
- Add unit test `cycle_3` for `adjacency_list`.
- Add unit test `cycle_3` for `adjacency_matrix`.
- Add unit test `cycle_degree_1` for `adjacency_list`.
- Add unit test `cycle_degree_1` for `adjacency_matrix`.
- Add unit test `cycle_degree_2` for `adjacency_list`.
- Add unit test `cycle_degree_2` for `adjacency_matrix`.
- Add unit test `empty_1` for `adjacency_list`.
- Add unit test `empty_1` for `adjacency_matrix`.
- Add unit test `empty_2` for `adjacency_list`.
- Add unit test `empty_2` for `adjacency_matrix`.
- Add unit test `empty_3` for `adjacency_list`.
- Add unit test `empty_3` for `adjacency_matrix`.
- Add unit test `star_1` for `adjacency_list`.
- Add unit test `star_1` for `adjacency_matrix`.
- Add unit test `star_2` for `adjacency_list`.
- Add unit test `star_2` for `adjacency_matrix`.
- Add unit test `star_3` for `adjacency_list`.
- Add unit test `star_3` for `adjacency_matrix`.

Changed

- Breaking: Rename `Cycle` to `Circuit`.
- Breaking: Return a trivial digraph for `Circuit::circuit` for `n = 1`.
- Split `biclique` unit test into `biclique_1`, etc., in `adjacency_list`.
- Split `circuit` unit test into `circuit_1`, etc., in `adjacency_list`.
- Split `complete` unit test into `complete_1`, etc., in `adjacency_list`.
- Split `cycle` unit test into `cycle_1`, etc., in `adjacency_list`.

Removed

- Remove unit test `circuit` for `adjacency_list`.
- Remove unit test `circuit` for `adjacency_matrix`.
- Remove unit test `complete` for `adjacency_list`.
- Remove unit test `complete` for `adjacency_matrix`.
- Remove unit test `cycle` for `adjacency_list`.
- Remove unit test `cycle` for `adjacency_matrix`.
- Remove unit test `empty` for `adjacency_list`.
- Remove unit test `empty` for `adjacency_matrix`.

## [0.70.3] - 2024-07-15

Added

- Add property test `biclique_complement_size` for `adjacency_list`.
- Add property test `biclique_complement_size` for `adjacency_matrix`.
- Add property test `complete_complement_eq_empty` for `adjacency_list`.
- Add property test `complete_complement_eq_empty` for `adjacency_matrix`.
- Add property test `cycle_complement_size` for `adjacency_list`.
- Add property test `cycle_complement_size` for `adjacency_matrix`.
- Add property test `empty_complement_eq_complete` for `adjacency_list`.
- Add property test `empty_complement_eq_complete` for `adjacency_matrix`.
- Add property test `star_complement_size` for `adjacency_list`.
- Add property test `star_complement_size` for `adjacency_matrix`.
- Add the `Complement` trait.

## [0.70.2] - 2024-07-15

Changed

- Simplify documentation.

## [0.70.1] - 2024-07-15

Changed

- Improve documentation consistency.

## [0.70.0] - 2024-07-14

Changed

- Breaking: `is_regular` returns false if the indegree does not equal the outdegree.

## [0.69.3] - 2024-07-14

Changed

- Misc. documentation improvements.

## [0.69.2] - 2024-07-14

Added

- Add `Biclique::utility`.
- Add `DistanceMatrix::periphery`.
- Add unit test `biclique_utility` for `adjacency_list`.
- Add unit test `biclique_utility` for `adjacency_matrix`.
- Add unit test `center_trivial` for `DistanceMatrix`.
- Add unit test `diameter_trivial` for `DistanceMatrix`.
- Add unit test `eccentricities_trivial` for `DistanceMatrix`.
- Add unit test `is_connected_trivial` for `DistanceMatrix`.
- Add unit test `periphery_kattis_bryr_1` for `DistanceMatrix`.
- Add unit test `periphery_kattis_bryr_2` for `DistanceMatrix`.
- Add unit test `periphery_kattis_bryr_3` for `DistanceMatrix`.
- Add unit test `periphery_kattis_crosscountry` for `DistanceMatrix`.
- Add unit test `periphery_trivial` for `DistanceMatrix`.

Changed

- Use `claw` instead of `biclique()` in `biclique_claw` tests.

## [0.69.1] - 2024-07-14

Added

- Add `Biclique::claw`.
- Add unit test `biclique_claw` for `adjacency_list`.
- Add unit test `biclique_claw` for `adjacency_matrix`.
- Add unit test `empty_trivial` for `adjacency_list_weighted`.
- Add unit test `empty_trivial` for `adjacency_list`.
- Add unit test `empty_trivial` for `adjacency_matrix`.
- Add unit test `empty` for `adjacency_list_weighted`.

## [0.69.0] - 2024-07-14

Added

- Add property test `biclique_is_isolated` for `adjacency_list`.
- Add property test `biclique_is_isolated` for `adjacency_matrix`.
- Add property test `biclique_is_order` for `adjacency_list`.
- Add property test `biclique_is_order` for `adjacency_matrix`.
- Add property test `biclique_is_oriented` for `adjacency_list`.
- Add property test `biclique_is_oriented` for `adjacency_matrix`.
- Add property test `biclique_is_out_neighbors_weighted` for `adjacency_list`.
- Add property test `biclique_is_out_neighbors_weighted` for `adjacency_matrix`.
- Add property test `biclique_is_out_neighbors` for `adjacency_list`.
- Add property test `biclique_is_out_neighbors` for `adjacency_matrix`.
- Add property test `biclique_is_outdegree` for `adjacency_list`.
- Add property test `biclique_is_outdegree` for `adjacency_matrix`.
- Add property test `biclique_is_pendant` for `adjacency_list`.
- Add property test `biclique_is_pendant` for `adjacency_matrix`.
- Add property test `biclique_is_regular` for `adjacency_list`.
- Add property test `biclique_is_regular` for `adjacency_matrix`.
- Add property test `biclique_is_semicomplete` for `adjacency_list`.
- Add property test `biclique_is_semicomplete` for `adjacency_matrix`.
- Add property test `biclique_is_simple` for `adjacency_list`.
- Add property test `biclique_is_simple` for `adjacency_matrix`.
- Add property test `biclique_is_size` for `adjacency_list`.
- Add property test `biclique_is_size` for `adjacency_matrix`.
- Add property test `biclique_is_subdigraph` for `adjacency_list`.
- Add property test `biclique_is_subdigraph` for `adjacency_matrix`.
- Add property test `biclique_is_superdigraph` for `adjacency_list`.
- Add property test `biclique_is_superdigraph` for `adjacency_matrix`.
- Add property test `biclique_is_symmetric` for `adjacency_list`.
- Add property test `biclique_is_symmetric` for `adjacency_matrix`.
- Add property test `biclique_is_tournament` for `adjacency_list`.
- Add property test `biclique_is_tournament` for `adjacency_matrix`.
- Add unit test `biclique_0_1` for `adjacency_list`.
- Add unit test `biclique_0_1` for `adjacency_matrix`.
- Add unit test `biclique_1_0` for `adjacency_list`.
- Add unit test `biclique_1_0` for `adjacency_matrix`.
- Add unit test `biclique_1_1_is_complete` for `adjacency_list`.
- Add unit test `biclique_1_1_is_complete` for `adjacency_matrix`.
- Add unit test `biclique_1_1_is_semicomplete` for `adjacency_list`.
- Add unit test `biclique_1_1_is_semicomplete` for `adjacency_matrix`.
- Add unit test `biclique_1_2_is_complete` for `adjacency_list`.
- Add unit test `biclique_1_2_is_complete` for `adjacency_matrix`.
- Add unit test `biclique_1_2_is_semicomplete` for `adjacency_list`.
- Add unit test `biclique_1_2_is_semicomplete` for `adjacency_matrix`.
- Add unit test `biclique_2_1_is_complete` for `adjacency_list`.
- Add unit test `biclique_2_1_is_complete` for `adjacency_matrix`.
- Add unit test `biclique_2_1_is_semicomplete` for `adjacency_list`.
- Add unit test `biclique_2_1_is_semicomplete` for `adjacency_matrix`.
- Add unit test `biclique` for `adjacency_list`.
- Add unit test `biclique` for `adjacency_matrix`.

Changed

- Breaking: `bellman_ford_moore::single_source_distances` is `#[must_use]`.
- Breaking: `bfs::single_source_distances` is `#[must_use]`.
- Breaking: `bfs::single_source_predecessors` is `#[must_use]`.
- Breaking: `bfs::shortest_path` is `#[must_use]`.
- Breaking: `bfs::single_pair_shortest_path` is `#[must_use]`.
- Breaking: `dfs::acyclic_ordering` is `#[must_use]`.
- Breaking: `dijkstra::single_source_distances` is `#[must_use]`.
- Breaking: `dijkstra::single_source_predecessors` is `#[must_use]`.
- Breaking: `dijkstra::shortest_path` is `#[must_use]`.
- Breaking: `dijkstra::single_pair_shortest_path` is `#[must_use]`.
- Breaking: `DistanceMatrix::new` is `#[must_use]`.
- Breaking: `DistanceMatrix::center` is `#[must_use]`.
- Breaking: `DistanceMatrix::diameter` is `#[must_use]`.
- Breaking: `DistanceMatrix::eccentricities` is `#[must_use]`.
- Breaking: `DistanceMatrix::is_connected` is `#[must_use]`.
- Breaking: `floyd_warshall::distances` is `#[must_use]`.
- Rename non `Empty` `*_trivial` tests to `*_1_*`.
- Rename non `Empty` `*_pair` tests to `*_2_*`.
- Rename non `Empty` `*_triplet` tests to `*_3_*`.

## [0.68.0] - 2024-07-13

Added

- Add property test `biclique_has_arc` for `adjacency_list`.
- Add property test `biclique_has_arc` for `adjacency_matrix`.
- Add property test `biclique_has_edge` for `adjacency_list`.
- Add property test `biclique_has_edge` for `adjacency_matrix`.
- Add property test `biclique_in_neighbors` for `adjacency_list`.
- Add property test `biclique_in_neighbors` for `adjacency_matrix`.
- Add property test `biclique_indegree` for `adjacency_list`.
- Add property test `biclique_indegree` for `adjacency_matrix`.
- Add property test `biclique_is_balanced` for `adjacency_list`.
- Add property test `biclique_is_balanced` for `adjacency_matrix`.
- Add property test `biclique_is_complete` for `adjacency_list`.
- Add property test `biclique_is_complete` for `adjacency_matrix`.
- Add property test `biclique_out_neighbors` for `adjacency_list`.
- Add property test `biclique_out_neighbors` for `adjacency_matrix`.
- Add unit test `biclique_is_complete_trivial` for `adjacency_list`.
- Add unit test `biclique_is_complete_trivial` for `adjacency_matrix`.
- Add unit test `indegree_out_of_bounds` for `adjacency_list_weighted`.
- Add unit test `indegree_out_of_bounds` for `adjacency_list`.

Changed

- Breaking: `indegree` panics if `v` is not in the digraph in `adjacency_list_weighted`.
- Breaking: `indegree` panics if `v` is not in the digraph in `adjacency_list`.

## [0.67.2] - 2024-07-13

Added

- Add property test `biclique_degree_sum_eq_2size` for `adjacency_list`.
- Add property test `biclique_degree_sum_eq_2size` for `adjacency_matrix`.
- Add property test `biclique_even_number_odd_degrees` for `adjacency_list`.
- Add property test `biclique_even_number_odd_degrees` for `adjacency_matrix`.
- Add property test `complete_degree_sum_eq_2size` for `adjacency_list`.
- Add property test `complete_degree_sum_eq_2size` for `adjacency_matrix`.
- Add property test `complete_even_number_odd_degrees` for `adjacency_list`.
- Add property test `complete_even_number_odd_degrees` for `adjacency_matrix`.
- Add property test `cycle_degree_sum_eq_2size` for `adjacency_list`.
- Add property test `cycle_degree_sum_eq_2size` for `adjacency_matrix`.
- Add property test `cycle_even_number_odd_degrees` for `adjacency_list`.
- Add property test `cycle_even_number_odd_degrees` for `adjacency_matrix`.
- Add property test `random_tournament_degree_sum_eq_2size` for `adjacency_list`.
- Add property test `random_tournament_degree_sum_eq_2size` for `adjacency_matrix`.
- Add property test `random_tournament_even_number_odd_degrees` for `adjacency_list`.
- Add property test `random_tournament_even_number_odd_degrees` for `adjacency_matrix`.
- Add property test `star_degree_sum_eq_2size` for `adjacency_list`.
- Add property test `star_degree_sum_eq_2size` for `adjacency_matrix`.
- Add property test `star_even_number_odd_degrees` for `adjacency_list`.
- Add property test `star_even_number_odd_degrees` for `adjacency_matrix`.

## [0.67.1] - 2024-07-13

Added

- Add property test `biclique_1_n_eq_star_n_plus_1` for `adjacency_matrix`.
- Add property test `biclique_degree_sequence` for `adjacency_matrix`.
- Add property test `biclique_degree` for `adjacency_matrix`.
- Add property test `biclique_order` for `adjacency_matrix`.
- Add property test `biclique_1_n_eq_star_n_plus_1` for `adjacency_list`.
- Add property test `biclique_degree_sequence` for `adjacency_list`.
- Add property test `biclique_degree` for `adjacency_list`.
- Add property test `biclique_order` for `adjacency_list`.

## [0.67.0] - 2024-07-11

Added

- Add unit test `biclique_m_zero` for `adjacency_list`.
- Add unit test `biclique_m_zero` for `adjacency_matrix`.
- Add unit test `biclique_n_zero` for `adjacency_list`.
- Add unit test `biclique_n_zero` for `adjacency_matrix`.

Changed

- Breaking: `biclique` panics if `a` or `b` is zero in the blanket implementation of `Biclique`.

Fixed

- Fix the description of `biclique` in the module documentation.
- Use `m` and `n` to describe the sizes of the biclique partitions.

## [0.66.0] - 2024-07-10

Added

- Add documentation alias `Circular` for `Cycle`.
- Add documentation alias `Edgeless` for `Empty`.
- Add documentation alias `InDegree` for `Indegree`.
- Add documentation alias `Isograph` for `IsBalanced`.
- Add documentation alias `OutDegree` for `Outdegree`.
- Add documentation alias `PseudoSymmetric` for `IsBalanced`.
- Add documentation alias `Valence` for `Degree`.
- Add documentation alias `Valency` for `Degree`.
- Add documentation alias `circular` for `Cycle::cycle`.
- Add documentation alias `edgeless` for `Empty::empty`.
- Add documentation alias `in_degree` for `Indegree::indegree`.
- Add documentation alias `in_neighbours` for `InNeighbors::in_neighbors`.
- Add documentation alias `isograph` for `IsBalanced::is_balanced`.
- Add documentation alias `out_degree` for `Outdegree::outdegree`.
- Add documentation alias `out_neighbours_weighted` for `OutNeighborsWeighted::out_neighbors_weighted`.
- Add documentation alias `out_neighbours` for `OutNeighbors::out_neighbors`.
- Add documentation alias `pseudo_symmetric` for `IsBalanced::is_balanced`.
- Add documentation alias `valence` for `Degree::degree`.
- Add documentation alias `valence` for `degree`.
- Add documentation alias `valency` for `Degree::degree`.
- Add the `gen::Biclique` trait.
- Add the `gen::Star` trait to the `README`.
- Add the `gen::Star` trait to the `lib.rs`.
- Document arguments for `HasEdge::has_edge`.
- Document arguments for `IsIsolated::is_isolated`.
- Document arguments for `IsPendant::is_pendant`.
- Document arguments for `IsSubdigraph::is_subdigraph`.
- Document arguments for `IsSuperdigraph::is_superdigraph`.
- Document arguments for `IsWalk::is_walk`.

Changed

- Breaking: Make `ArcWeight::arc_weight` `#[must_use]`.
- Breaking: Make `Arcs::arcs` `#[must_use]`.
- Breaking: Make `ArcsWeighted::arcs_weighted` `#[must_use]`.
- Breaking: Make `Degree::degree` `#[must_use]`.
- Breaking: Make `DegreeSequence::degree_sequence` `#[must_use]`.
- Breaking: Make `HasArc::has_arc` `#[must_use]`.
- Breaking: Make `HasEdge::has_edge` `#[must_use]`.
- Breaking: Make `InNeighbors::in_neighbors` `#[must_use]`.
- Breaking: Make `Indegree::indegree` `#[must_use]`.
- Breaking: Make `IsBalanced::is_balanced` `#[must_use]`.
- Breaking: Make `IsComplete::is_complete` `#[must_use]`.
- Breaking: Make `IsIsolated::is_isolated` `#[must_use]`.
- Breaking: Make `IsOriented::is_oriented` `#[must_use]`.
- Breaking: Make `IsPendant::is_pendant` `#[must_use]`.
- Breaking: Make `IsRegular::is_regular` `#[must_use]`.
- Breaking: Make `IsSemicomplete::is_semicomplete` `#[must_use]`.
- Breaking: Make `IsSimple::is_simple` `#[must_use]`.
- Breaking: Make `IsSink::is_sink` `#[must_use]`.
- Breaking: Make `IsSource::is_source` `#[must_use]`.
- Breaking: Make `IsSubdigraph::is_subdigraph` `#[must_use]`.
- Breaking: Make `IsSuperdigraph::is_superdigraph` `#[must_use]`.
- Breaking: Make `IsSymmetric::is_symmetric` `#[must_use]`.
- Breaking: Make `IsTournament::is_tournament` `#[must_use]`.
- Breaking: Make `IsWalk::is_walk` `#[must_use]`.
- Breaking: Make `Order::order` `#[must_use]`.
- Breaking: Make `OutNeighborsWeighted::out_neighbors_weighted` `#[must_use]`.
- Breaking: Make `OutNeighbors::out_neighbors` `#[must_use]`.
- Breaking: Make `Outdegree::outdegree` `#[must_use]`.
- Breaking: Make `RemoveArc::remove_arc` `#[must_use]`.
- Breaking: Make `Size::size` `#[must_use]`.
- Breaking: Make `Star::star` `#[must_use]`.
- Breaking: Make `Vertices::vertices` `#[must_use]`.
- Move module doc aliases under the module doc.

Fixed

- Change documentation alias `weighted_out_neighbors` to `out_neighbors_weighted`.
- Change documentation alias `IterOutNeighbours` to `OutNeighbours`.

## [0.65.1] - 2024-07-10

Added

- Add `star_degree_sequence` unit test for `adjacency_list`.
- Add `star_degree_sequence` unit test for `adjacency_matrix`.
- Add `star_degree` unit test for `adjacency_list`.
- Add `star_degree` unit test for `adjacency_matrix`.
- Add `star_has_edge` unit test for `adjacency_list`.
- Add `star_has_edge` unit test for `adjacency_matrix`.
- Add `star_indegree` unit test for `adjacency_list`.
- Add `star_indegree` unit test for `adjacency_matrix`.
- Add `star_is_balanced_pair` unit test for `adjacency_list`.
- Add `star_is_balanced_pair` unit test for `adjacency_matrix`.
- Add `star_is_balanced_trivial` unit test for `adjacency_list`.
- Add `star_is_balanced_trivial` unit test for `adjacency_matrix`.
- Add `star_is_balanced` prop test for `adjacency_list`.
- Add `star_is_balanced` prop test for `adjacency_matrix`.
- Add `star_is_complete_pair` unit test for `adjacency_list`.
- Add `star_is_complete_pair` unit test for `adjacency_matrix`.
- Add `star_is_complete_trivial` unit test for `adjacency_list`.
- Add `star_is_complete_trivial` unit test for `adjacency_matrix`.
- Add `star_is_complete` prop test for `adjacency_list`.
- Add `star_is_complete` prop test for `adjacency_matrix`.
- Add `star_is_isolated_trivial` unit test for `adjacency_list`.
- Add `star_is_isolated_trivial` unit test for `adjacency_matrix`.
- Add `star_is_isolated` prop test for `adjacency_list`.
- Add `star_is_isolated` prop test for `adjacency_matrix`.
- Add `star_is_oriented_trivial` unit test for `adjacency_list`.
- Add `star_is_oriented_trivial` unit test for `adjacency_matrix`.
- Add `star_is_oriented` prop test for `adjacency_list`.
- Add `star_is_oriented` prop test for `adjacency_matrix`.
- Add `star_is_pendant` prop test for `adjacency_list`.
- Add `star_is_pendant` prop test for `adjacency_matrix`.
- Add `star_is_regular_pair` unit test for `adjacency_list`.
- Add `star_is_regular_pair` unit test for `adjacency_matrix`.
- Add `star_is_regular_trivial` unit test for `adjacency_list`.
- Add `star_is_regular_trivial` unit test for `adjacency_matrix`.
- Add `star_is_regular` prop test for `adjacency_list`.
- Add `star_is_regular` prop test for `adjacency_matrix`.
- Add `star_is_semicomplete_pair` unit test for `adjacency_list`.
- Add `star_is_semicomplete_pair` unit test for `adjacency_matrix`.
- Add `star_is_semicomplete_trivial` unit test for `adjacency_list`.
- Add `star_is_semicomplete_trivial` unit test for `adjacency_matrix`.
- Add `star_is_semicomplete` prop test for `adjacency_list`.
- Add `star_is_semicomplete` prop test for `adjacency_matrix`.
- Add `star_is_simple` prop test for `adjacency_list`.
- Add `star_is_simple` prop test for `adjacency_matrix`.
- Add `star_is_sink_trivial` unit test for `adjacency_list`.
- Add `star_is_sink_trivial` unit test for `adjacency_matrix`.
- Add `star_is_sink` prop test for `adjacency_list`.
- Add `star_is_sink` prop test for `adjacency_matrix`.
- Add `star_is_source_trivial` unit test for `adjacency_list`.
- Add `star_is_source_trivial` unit test for `adjacency_matrix`.
- Add `star_is_source` prop test for `adjacency_list`.
- Add `star_is_source` prop test for `adjacency_matrix`.
- Add `star_is_subdigraph` prop test for `adjacency_list`.
- Add `star_is_subdigraph` prop test for `adjacency_matrix`.
- Add `star_is_superdigraph` prop test for `adjacency_list`.
- Add `star_is_superdigraph` prop test for `adjacency_matrix`.
- Add `star_is_symmetric` prop test for `adjacency_list`.
- Add `star_is_symmetric` prop test for `adjacency_matrix`.
- Add `star_is_tournament_trivial` unit test for `adjacency_list`.
- Add `star_is_tournament_trivial` unit test for `adjacency_matrix`.
- Add `star_is_tournament` prop test for `adjacency_list`.
- Add `star_is_tournament` prop test for `adjacency_matrix`.
- Add naming conventions to `lib.rs`.
- Add project goals to `lib.rs`.
- Add the `gen::Star` trait.

## [0.65.0] - 2024-07-07

Added

- Add unit test `add_arc_out_of_bounds_u` for `adjacency_matrix`.
- Add unit test `add_arc_out_of_bounds_v` for `adjacency_matrix`.
- Add unit test `add_arc_out_of_bounds` for `adjacency_list_weighted`.
- Add unit test `add_arc_out_of_bounds` for `adjacency_list`.
- Add unit test `converse_order_zero` for `adjacency_list_weighted`.
- Add unit test `converse_order_zero` for `adjacency_list`.
- Add unit test `converse_order_zero` for `adjacency_matrix`.
- Add unit test `empty_order_zero` for `adjacency_list_matrix`.
- Add unit test `empty_order_zero` for `adjacency_list_weighted`.
- Add unit test `empty_order_zero` for `adjacency_list`.
- Add unit test `indegree_out_of_bounds` for `adjacency_matrix`.
- Add unit test `out_neighbors_out_of_bounds` for `adjacency_list_weighted`.
- Add unit test `out_neighbors_out_of_bounds` for `adjacency_list`.
- Add unit test `out_neighbors_out_of_bounds` for `adjacency_matrix`.
- Add unit test `out_neighbors_weighted_out_of_bounds` for `adjacency_list_weighted`.
- Add unit test `out_neighbors_weighted_out_of_bounds` for `adjacency_list`.
- Add unit test `out_neighbors_weighted_out_of_bounds` for `adjacency_matrix`.
- Add unit test `outdegree_out_of_bounds` for `adjacency_list_weighted`.
- Add unit test `outdegree_out_of_bounds` for `adjacency_list`.
- Add unit test `outdegree_out_of_bounds` for `adjacency_matrix`.
- Add unit test `remove_arc_out_of_bounds` for `adjacency_list_weighted`.
- Add unit test `remove_arc_out_of_bounds` for `adjacency_list`.
- Add unit test `remove_arc_out_of_bounds` for `adjacency_matrix`.

Changed

- Breaking: `Complete::complete` is `#[must_use]`.
- Breaking: `remove_arc` no longer panics for `adjacency_matrix` if `u` or `v` is not in the digraph.

## [0.64.18] - 2024-07-07

Added

- Add property test `empty_is_tournament` for `adjacency_list_weighted`.
- Add unit test `empty_is_tournament_trivial` for `adjacency_list_weighted`.
- Add unit test `empty_is_tournament_trivial` for `adjacency_matrix_weighted`.
- Add unit test `is_tournament_bang_jensen_34` for `adjacency_list`.
- Add unit test `is_tournament_bang_jensen_34` for `adjacency_matrix`.
- Add unit test `is_tournament_bang_jensen_94_weighted` for `adjacency_list_weighted`.
- Add unit test `is_tournament_bang_jensen_94` for `adjacency_list`.
- Add unit test `is_tournament_bang_jensen_94` for `adjacency_matrix`.
- Add unit test `is_tournament_bang_jensen_96` for `adjacency_list_weighted`.
- Add unit test `is_tournament_bang_jensen_99` for `adjacency_list_weighted`.
- Add unit test `is_tournament_kattis_bryr_1` for `adjacency_list_weighted`.
- Add unit test `is_tournament_kattis_bryr_2` for `adjacency_list_weighted`.
- Add unit test `is_tournament_kattis_bryr_3` for `adjacency_list_weighted`.
- Add unit test `is_tournament_kattis_builddeps` for `adjacency_list`.
- Add unit test `is_tournament_kattis_builddeps` for `adjacency_matrix`.
- Add unit test `is_tournament_kattis_crosscountry` for `adjacency_list_weighted`.
- Add unit test `is_tournament_kattis_escapewallmaria_1` for `adjacency_list`.
- Add unit test `is_tournament_kattis_escapewallmaria_1` for `adjacency_matrix`.
- Add unit test `is_tournament_kattis_escapewallmaria_2` for `adjacency_list`.
- Add unit test `is_tournament_kattis_escapewallmaria_2` for `adjacency_matrix`.
- Add unit test `is_tournament_kattis_escapewallmaria_3` for `adjacency_list`.
- Add unit test `is_tournament_kattis_escapewallmaria_3` for `adjacency_matrix`.
- Add unit test `is_tournament_kattis_shortestpath1` for `adjacency_list_weighted`.

## [0.64.17] - 2024-07-07

Added

- Add property test `complete_is_tournament` for `adjacency_list`.
- Add property test `complete_is_tournament` for `adjacency_matrix`.
- Add property test `cycle_is_tournament` for `adjacency_list`.
- Add property test `cycle_is_tournament` for `adjacency_matrix`.
- Add property test `empty_is_tournament` for `adjacency_list`.
- Add property test `empty_is_tournament` for `adjacency_matrix`.
- Add property test `random_tournament_is_tournament` for `adjacency_list`.
- Add property test `random_tournament_is_tournament` for `adjacency_matrix`.
- Add unit test `complete_is_tournament_trivial` for `adjacency_list`.
- Add unit test `complete_is_tournament_trivial` for `adjacency_matrix`.
- Add unit test `cycle_is_tournament_pair` for `adjacency_list`.
- Add unit test `cycle_is_tournament_pair` for `adjacency_matrix`.
- Add unit test `cycle_is_tournament_triple` for `adjacency_list`.
- Add unit test `cycle_is_tournament_triple` for `adjacency_matrix`.
- Add unit test `cycle_is_tournament_trivial` for `adjacency_list`.
- Add unit test `cycle_is_tournament_trivial` for `adjacency_matrix`.

## [0.64.16] - 2024-07-06

- Add property test `random_tournament_degree_sequence` for `adjacency_list`.
- Add property test `random_tournament_degree_sequence` for `adjacency_matrix`.

## [0.64.15] - 2024-07-06

Added

- Add property test `complete_degree_sequence` for `adjacency_list`.
- Add property test `complete_degree_sequence` for `adjacency_matrix`.
- Add property test `cycle_degree_sequence` for `adjacency_list`.
- Add property test `cycle_degree_sequence` for `adjacency_matrix`.
- Add property test `empty_degree_sequence` for `adjacency_list`.
- Add property test `empty_degree_sequence` for `adjacency_matrix`.

## [0.64.14] - 2024-07-06

Fixed

- Apply internal naming conventions.

## [0.64.13] - 2024-07-05

Added

- Add unit test `new` for `predecessor_tree::PredecessorTree`.
- Add unit test `new_zero` for `predecessor_tree::PredecessorTree`.

Changed

- Apply internal naming conventions in `adjacency_list_weighted`.
- Apply internal naming conventions in `adjacency_matrix`.
- Apply internal naming conventions in `gen`.
- Apply internal naming conventions in `op`.
- Misc. improvements to the API documentation.

## [0.64.12] - 2024-07-04

Fixed

- Fix the `DegreeSequence` link in the `README`.

## [0.64.11] - 2024-07-04

Fixed

- Fix the `DegreeSequence` description in `lib.rs`.

## [0.64.10] - 2024-07-04

Added

- Add `DegreeSequence` to the `README` and the `lib.rs` documentation.

## [0.64.9] - 2024-07-04

Added

- Add documentation alias `centre` for `DistanceMatrix::center`.
- Add documentation alias `jordan_centre` for `DistanceMatrix::center`.
- Add naming conventions to the `README`.
- Add the `DegreeSequence` trait.
- Implement and test `DegreeSequence` for `adjacency_list_weighted`.
- Implement and test `DegreeSequence` for `adjacency_list`.
- Implement and test `DegreeSequence` for `adjacency_matrix`.

Changed

- Apply internal naming conventions in `adjacency_list`.
- Use `super` in `op` imports.

## [0.64.8] - 2024-07-03

Changed

- Improve the length of the project overview descriptions.

## [0.64.7] - 2024-07-03

Changed

- Improve the readability of the project overview.

## [0.64.6] - 2024-07-03

Fixed

- Remove extraneous words from the documentation.

Removed

- Remove implementation tables in `README`.
- Remove implementation tables in `lib.rs`.

## [0.64.5] - 2024-07-03

Changed

- Improve documentation for the `Complete` trait.
- Improve documentation for the `RandomTournament` trait.

## [0.64.4] - 2024-07-02

Added

- Add unit test `bellman_ford_moore::single_source_distances_kattis_shortestpath3`.

Changed

- Misc. improvements to the `README`.

## [0.64.3] - 2024-07-02

Changed

- Improve the `README` project overview.

## [0.64.2] - 2024-07-02

Added

- Add project overview to README.
- Complete `adjacency_matrix` tests.

## [0.64.1] - 2024-07-01

Added

- Add project overview to `lib.rs`.
- Implement `ArcsWeighted` for `adjacency_matrix`.
- Implement `Converse` for `adjacency_matrix`.
- Implement `OutNeighborsWeighted` for `adjacency_list`.
- Implement `OutNeighborsWeighted` for `adjacency_matrix`.

Changed

- Order trait bounds alphabetically.

Removed

- Remove documentation references to deprecated benchmarks.

## [0.64.0] - 2024-06-30

This update is big. Users will find that I changed the entire API except for traits and algorithms. Analyzing the digraph `op` and `gen` implementations for the standard library collections, I settled on two main digraph representations:

- `adjacency_list`
- `adjacency_list_weighted`

These representations offer a good balance between performance and ergonomics. All applicable traits are implemented and tested for these representations. The `adjacency_matrix` representation is still available for dense unweighted digraphs, but blocks are now stored in a `Vec` instead of an array.

The next minor release will add a user guide to the documentation.

## [0.63.1] - 2024-06-27

Added

- Add and test `DistanceMatrix::is_connected`.

## [0.63.0] - 2024-06-25

Changed

- Breaking: Fixtures are `cfg(test)`.

## [0.62.3] - 2024-06-25

Changed

- Test `DistanceMatrix` with existing fixtures.

## [0.62.2] - 2024-06-23

Added

- Add documentation alias `jordan_center` for `DistanceMatrix::center`.
- Add `DistanceMatrix::diameter`.

## [0.62.1] - 2024-06-22

Changed

- Clean up `prop` modules.

## [0.62.0] - 2024-06-20

Added

- Add and test `DistanceMatrix::eccentricities`.
- Add unit test `center` for `DistanceMatrix`.
- Base `DistanceMatrix::center` on `DistanceMatrix::eccentricities`.

Changed

- Breaking: `DistanceMatrix::new` panics if `v` is zero.
- Rename `distance_matrix::tests::test_index_mut` to `distance_matrix::tests::index_mut`.
- Rename `distance_matrix::tests::test_index` to `distance_matrix::tests::index`.
- Rename `distance_matrix::tests::test_new` to `distance_matrix::tests::new`.

## [0.61.2] - 2024-06-20

Fixed

- Fix `distance_matrix::tests::test_new`.
- Fix `distance_matrix::tests::test_index`.

## [0.61.1] - 2024-06-19

Added

- Add `DistanceMatrix::center`.

## [0.61.0] - 2024-06-19

Added

- Add `DistanceMatrix` data structure.
- Implement and test `DistanceMatrix::new`.
- Implement and test `Index<usize>` for `DistanceMatrix`.
- Implement and test `IndexMut<usize>` for `DistanceMatrix`.

Changed

- Breaking: Rename `BfTree` to `PredecessorTree`.
- Breaking: Rename `bf_tree` to `predecessor_tree`.
- Breaking: `FloydWarshall::distances` returns a `DistanceMatrix`.

## [0.60.0] - 2024-06-19

Added

- Add unit test `index_mut` for `BfTree`.
- Add unit test `index` for `BfTree`.
- Add unit test `search_by_cycle` for `BfTree`.
- Add unit test `search_by_no_path` for `BfTree`.
- Add unit test `search_by_path_s_eq_t` for `BfTree`.
- Add unit test `search_by_path_s_ne_t` for `BfTree`.
- Add unit test `search_by_singleton_s_eq_t` for `BfTree`.
- Add unit test `search_by_singleton_s_ne_t` for `BfTree`.
- Add unit test `search_cycle` for `BfTree`.
- Add unit test `search_no_path` for `BfTree`.
- Add unit test `search_path_s_eq_t` for `BfTree`.
- Add unit test `search_path_s_ne_t` for `BfTree`.
- Add unit test `search_singleton_s_eq_t` for `BfTree`.
- Add unit test `search_singleton_s_ne_t` for `BfTree`.

Changed

- Breaking: Rename `BfsTree` to `BfTree`.
- Breaking: `BfTree::new` panics if `v` is zero.
- Breaking: `BfTree::search_by` immediately returns if `s` is a target.
- Breaking: `BfTree::search` immediately returns if `s` equals `t`.

## [0.59.0] - 2024-06-18

Added

- Add `BfsTree` data structure.
- Implement `BfsTree::new`.
- Implement `BfsTree::search_by`.
- Implement `BfsTree::search`.
- Implement `From<Vec<Option<usize>>>` for `BfsTree`.
- Implement `Index<usize>` for `BfsTree`.
- Implement `IndexMut<usize>` for `BfsTree`.
- Implement `IntoIterator` for `BfsTree`.

Changed

- Breaking: `bfs::predecessors` accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `bfs::shortest_path` accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `bfs::single_source_predecessors` returns a `BfsTree`.
- Breaking: `dijkstra::predecessors` accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `dijkstra::shortest_path` accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `dijkstra::single_source_predecessors` returns a `BfsTree`.

Removed

- Breaking: Remove `algo::predecessor`.

## [0.58.0] - 2024-06-16

Added

- Implement `Size` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `Size` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Size` for `BTreeMap<usize, Vec<usize>>`.
- Implement `Size` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `Size` for `HashMap<usize, HashSet<usize>>`.
- Implement `Size` for `HashMap<usize, Vec<usize>>`.
- Implement `Size` for `Vec<BTreeMap<usize, W>>`.
- Implement `Size` for `Vec<BTreeSet<usize>>`.
- Implement `Size` for `Vec<HashMap<usize, W>>`.
- Implement `Size` for `Vec<HashSet<usize>>`.
- Implement `Size` for `Vec<Vec<usize>>`.
- Implement `Size` for `[BTreeMap<usize, W>; V]`.
- Implement `Size` for `[BTreeMap<usize, W>]`.
- Implement `Size` for `[BTreeSet<usize>; V]`.
- Implement `Size` for `[BTreeSet<usize>]`.
- Implement `Size` for `[HashMap<usize, W>; V]`.
- Implement `Size` for `[HashMap<usize, W>]`.
- Implement `Size` for `[HashSet<usize>; V]`.
- Implement `Size` for `[HashSet<usize>]`.
- Implement `Size` for `[Vec<usize>; V]`.
- Implement `Size` for `[Vec<usize>]`.

Removed

- Breaking: Remove the implementation of `Converse` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Breaking: Remove the implementation of `Converse` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Breaking: Remove the implementation of `Converse` for `HashMap<usize, HashSet<(usize, W)>>`.
- Breaking: Remove the implementation of `Converse` for `HashMap<usize, Vec<(usize, W)>>`.
- Breaking: Remove the implementation of `Converse` for `Vec<BTreeSet<(usize, W)>>`.
- Breaking: Remove the implementation of `Converse` for `Vec<HashSet<(usize, W)>>`.
- Breaking: Remove the implementation of `Converse` for `Vec<Vec<(usize, W)>>`.
- Breaking: Remove the implementation of `Size` for `BTreeMap<K, BTreeMap<K, W>>`.
- Breaking: Remove the implementation of `Size` for `BTreeMap<K, BTreeSet<W>>`.
- Breaking: Remove the implementation of `Size` for `BTreeMap<K, Vec<T>>`.
- Breaking: Remove the implementation of `Size` for `HashMap<K, HashMap<K, W>>`.
- Breaking: Remove the implementation of `Size` for `HashMap<K, HashSet<W>>`.
- Breaking: Remove the implementation of `Size` for `HashMap<K, Vec<T>>`.
- Breaking: Remove the implementation of `Size` for `Vec<BTreeMap<K, W>>`.
- Breaking: Remove the implementation of `Size` for `Vec<BTreeSet<T>>`.
- Breaking: Remove the implementation of `Size` for `Vec<HashMap<K, W>>`.
- Breaking: Remove the implementation of `Size` for `Vec<HashSet<T>>`.
- Breaking: Remove the implementation of `Size` for `Vec<Vec<T>>`.
- Breaking: Remove the implementation of `Size` for `[BTreeMap<K, W>; V]`.
- Breaking: Remove the implementation of `Size` for `[BTreeMap<K, W>]`.
- Breaking: Remove the implementation of `Size` for `[BTreeSet<T>; V]`.
- Breaking: Remove the implementation of `Size` for `[BTreeSet<T>]`.
- Breaking: Remove the implementation of `Size` for `[HashMap<K, W>; V]`.
- Breaking: Remove the implementation of `Size` for `[HashMap<K, W>]`.
- Breaking: Remove the implementation of `Size` for `[HashSet<T>; V]`.
- Breaking: Remove the implementation of `Size` for `[HashSet<T>]`.
- Breaking: Remove the implementation of `Size` for `[Vec<T>; V]`.
- Breaking: Remove the implementation of `Size` for `[Vec<T>]`.
- Remove property test `size_btree_map_btree_set_weighted` for `empty`.
- Remove property test `size_btree_map_vec_weighted` for `empty`.
- Remove property test `size_hash_map_hash_set_weighted` for `empty`.
- Remove property test `size_hash_map_vec_weighted` for `empty`.
- Remove property test `size_vec_btree_set_weighted` for `empty`.
- Remove property test `size_vec_hash_set_weighted` for `empty`.
- Remove property test `size_vec_vec_weighted` for `empty`.
- Remove the `crosscountry` benchmark.
- Remove the `shortestpath1` benchmark.
- Remove unit test `size_arr_btree_set_weighted` for `empty_const`.
- Remove unit test `size_arr_hash_set_weighted` for `empty_const`.
- Remove unit test `size_arr_vec_weighted` for `empty_const`.

## [0.57.2] - 2024-06-16

Added

- Add benchmark `add_arc::slice_btree_set`.
- Add benchmark `add_arc::slice_hash_set`.
- Add benchmark `add_arc::slice_vec`.
- Add benchmark `add_arc::vec_tuple`.
- Mention `add_arc` benchmark in `op::add_arc` documentation.

Changed

- Use randomly generated arcs in `add_arc` benchmarks.

## [0.57.1] - 2024-06-16

Added

- Add documentation alias `IterInNeighbors` for `IterInNeighbours`.
- Add documentation alias `IterOutNeighborsMut` for `IterOutNeighboursMut`.
- Add documentation alias `IterOutNeighbors` for `IterOutNeighbours`.
- Add documentation alias `IterWeightedOutNeighbors` for `IterWeightedOutNeighbours`.
- Add documentation alias `iter_in_neighbors` for `iter_in_neighbours`.
- Add documentation alias `iter_out_neighbors_mut` for `iter_out_neighbours_mut`.
- Add documentation alias `iter_out_neighbors` for `iter_out_neighbours`.
- Add documentation alias `iter_weighted_out_neighbors` for `iter_weighted_out_neighbours`.
- Add script `version.sh` to change the version in `Cargo.toml` and `README.md`.

## [0.57.0] - 2024-06-15

Added

- Add example usage in the implementation example for `IterOutNeighbors`.
- Add the `IterInNeighbors` trait.
- Add unit test `arr_btree_set` for `IterInNeighbors`.
- Add unit test `arr_hash_set` for `IterInNeighbors`.
- Add unit test `arr_vec` for `IterInNeighbors`.
- Add unit test `btree_map_btree_set` for `IterInNeighbors`.
- Add unit test `btree_map_vec` for `IterInNeighbors`.
- Add unit test `btree_set_tuple` for `IterInNeighbors`.
- Add unit test `hash_map_hash_set` for `IterInNeighbors`.
- Add unit test `hash_map_vec` for `IterInNeighbors`.
- Add unit test `hash_set_tuple` for `IterInNeighbors`.
- Add unit test `slice_btree_set` for `IterInNeighbors`.
- Add unit test `slice_hash_set` for `IterInNeighbors`.
- Add unit test `slice_tuple` for `IterInNeighbors`.
- Add unit test `slice_vec` for `IterInNeighbors`.
- Add unit test `vec_btree_set` for `IterInNeighbors`.
- Add unit test `vec_hash_set` for `IterInNeighbors`.
- Add unit test `vec_tuple` for `IterInNeighbors`.
- Add unit test `vec_vec` for `IterInNeighbors`.
- Implement `IterInNeighbors` for `IterArcs`.
- Implement and test `EmptyConst` for `BTreeSet<(usize, usize)>`.
- Implement and test `EmptyConst` for `BTreeSet<(usize, usize, W)>`.
- Implement and test `EmptyConst` for `HashSet<(usize, usize)>`.
- Implement and test `EmptyConst` for `HashSet<(usize, usize, W)>`.
- Implement and test `EmptyConst` for `Vec<(usize, usize)>`.
- Implement and test `EmptyConst` for `Vec<(usize, usize, W)>`.
- Implement and test `IterOutNeighbors` for `BTreeSet<(usize, usize)>`.
- Implement and test `IterOutNeighbors` for `BTreeSet<(usize, usize, W)>`.
- Implement and test `IterOutNeighbors` for `HashSet<(usize, usize)>`.
- Implement and test `IterOutNeighbors` for `HashSet<(usize, usize, W)>`.
- Implement and test `IterOutNeighbors` for `Vec<(usize, usize)>`.
- Implement and test `IterOutNeighbors` for `Vec<(usize, usize, W)>`.
- Implement and test `IterOutNeighbors` for `[(usize, usize); V]`.
- Implement and test `IterOutNeighbors` for `[(usize, usize)]`.
- Implement and test `IterOutNeighbors` for `[(usize, usize, W); V]`.
- Implement and test `IterOutNeighbors` for `[(usize, usize, W)]`.

Changed

- Implement `Complete` for `HashSet<(usize, usize)>` with `EmptyConst`.
- Implement `Complete` for `Vec<(usize, usize)>` with `EmptyConst`.
- Implement `Complete` for `Vec<(usize, usize)>` with `EmptyConst`.
- Implement `Cycle` for `BTreeSet<(usize, usize)>` with `EmptyConst`.
- Implement `Cycle` for `BTreeSet<(usize, usize)>` with `EmptyConst`.
- Implement `Cycle` for `HashSet<(usize, usize)>` with `EmptyConst`.
- Implement `RandomTournament` for `BTreeSet<(usize, usize)>` with `EmptyConst`.
- Implement `RandomTournament` for `BTreeSet<(usize, usize)>` with `EmptyConst`.
- Implement `RandomTournament` for `HashSet<(usize, usize)>` with `EmptyConst`.
- Use digraphs in `iter_out_neighbors` tests that better demonstrate the trait.

Removed

- Breaking: Remove `Empty` implementation for `BTreeSet<(usize, usize)>`.
- Breaking: Remove `Empty` implementation for `BTreeSet<(usize, usize, W)>`.
- Breaking: Remove `Empty` implementation for `HashSet<(usize, usize)>`.
- Breaking: Remove `Empty` implementation for `HashSet<(usize, usize, W)>`.
- Breaking: Remove `Empty` implementation for `Vec<(usize, usize)>`.
- Breaking: Remove `Empty` implementation for `Vec<(usize, usize, W)>`.
- Remove `clippy::std_instead_of_core` lint.

## [0.56.0] - 2024-06-13

Added

- Add fixture `kattis_escapewallmaria_1`.
- Add fixture `kattis_escapewallmaria_2`.
- Add fixture `kattis_escapewallmaria_3`.
- Add unit test `distances_kattis_escapewallmaria_1`.
- Add unit test `distances_kattis_escapewallmaria_2`.
- Add unit test `distances_kattis_escapewallmaria_3`.
- Add unit test `predecessors_kattis_escapewallmaria_1`.
- Add unit test `predecessors_kattis_escapewallmaria_2`.
- Add unit test `predecessors_kattis_escapewallmaria_3`.
- Add unit test `shortest_path_kattis_escapewallmaria_1`.
- Add unit test `shortest_path_kattis_escapewallmaria_2`.
- Add unit test `shortest_path_kattis_escapewallmaria_3`.

Fixed

- Breaking: Return early in `bfs::shortest_path` if the source is the target.
- Breaking: Return early in `bfs::single_pair_shortest_path` if the source is the target.

## [0.55.7] - 2024-06-12

Changed

- Unify test setup.

## [0.55.6] - 2024-06-11

Added

- Add benchmark `add_arc::btree_set_tuple`.
- Add benchmark `add_arc::hash_set_tuple`.

## [0.55.5] - 2024-06-10

Changed

- Use ACM citation style for fixture titles.
- Use ACM citation style for citations.

## [0.55.4] - 2024-06-09

Added

- Add `fixture::kattis_builddeps`.
- Add unit test `acyclic_ordering_kattis_builddeps` for `dfs::acyclic_ordering`.
- Add unit test `dfsa_kattis_builddeps` for `dfs::acyclic_ordering`.
- Add unit test `dfsa_predecessors_kattis_builddeps` for `dfs::acyclic_ordering`.

Fixed

- Fix citations.

## [0.55.3] - 2024-06-09

Added

- Add `dfs::acyclic_ordering`.
- Add `dfs::dfsa_predecessors`.
- Add `dfs::dfsa`.

Changed

- Calculate order once in `bfs::single_pair_shortest_path`.
- Calculate order once in `bfs::single_source_predecessors`.
- Clean up and unify the `algo` documentation examples.

## [0.55.2] - 2024-06-08

Changed

- Pass ident to macros in unit tests for `AddArc`.
- Pass ident to macros in unit tests for `AddWeightedArc`.
- Use `std` instead of `alloc`.

## [0.55.1] - 2024-06-08

Added

- Implement and test `ArcWeight` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement and test `ArcWeight` for `BTreeSet<(usize, usize)>`.
- Implement and test `ArcWeight` for `HashMap<usize, HashSet<usize>>`.
- Implement and test `ArcWeight` for `HashSet<(usize, usize)>`.
- Implement and test `ArcWeight` for `Vec<BTreeSet<usize>>`.
- Implement and test `ArcWeight` for `Vec<HashSet<usize>>`.
- Implement and test `ArcWeight` for `[BTreeSet<usize>; V]`.
- Implement and test `ArcWeight` for `[BTreeSet<usize>]`.
- Implement and test `ArcWeight` for `[HashSet<usize>; V]`.
- Implement and test `ArcWeight` for `[HashSet<usize>]`.

## [0.55.0] - 2024-06-08

Changed

- Breaking: `AdjacencyMatrix::empty` panics if `V` is zero.

## [0.54.5] - 2024-06-08

Added

- Implement and test `ArcWeight` for `AdjacencyMatrix`.
- Implement and test `CycleConst` for `AdjacencyMatrix`.
- Implement and test `EmptyConst` for `AdjacencyMatrix`.

## [0.54.4] - 2024-06-06

Added

- Add assertions to the `IsComplete` implementation documentation test.
- Add the `IsSemicomplete` trait.
- Add unit test `arr_btree_set_complete` for `IsSemicomplete`.
- Add unit test `arr_btree_set_cycle` for `IsSemicomplete`.
- Add unit test `arr_btree_set_empty` for `IsSemicomplete`.
- Add unit test `arr_btree_set_random_tournament` for `IsSemicomplete`.
- Add unit test `arr_hash_set_complete` for `IsSemicomplete`.
- Add unit test `arr_hash_set_cycle` for `IsSemicomplete`.
- Add unit test `arr_hash_set_empty` for `IsSemicomplete`.
- Add unit test `arr_hash_set_random_tournament` for `IsSemicomplete`.
- Add unit test `vec_btree_set_complete` for `IsSemicomplete`.
- Add unit test `vec_btree_set_cycle` for `IsSemicomplete`.
- Add unit test `vec_btree_set_empty` for `IsSemicomplete`.
- Add unit test `vec_btree_set_random_tournament` for `IsSemicomplete`.
- Add unit test `vec_hash_set_complete` for `IsSemicomplete`.
- Add unit test `vec_hash_set_cycle` for `IsSemicomplete`.
- Add unit test `vec_hash_set_empty` for `IsSemicomplete`.
- Add unit test `vec_hash_set_random_tournament` for `IsSemicomplete`.
- Assert that `Empty` digraphs are not complete in the `is_complete` documentation test.
- Assert that `RandomTournament` digraphs are not complete in the `is_complete` documentation test.

## [0.54.3] - 2024-06-06

Added

- Add unit test `arr_btree_set_empty` for `IsComplete`.
- Add unit test `arr_btree_set_random_tournament` for `IsComplete`.
- Add unit test `arr_hash_set_empty` for `IsComplete`.
- Add unit test `arr_hash_set_random_tournament` for `IsComplete`.
- Add unit test `vec_btree_set_empty` for `IsComplete`.
- Add unit test `vec_btree_set_random_tournament` for `IsComplete`.
- Add unit test `vec_hash_set_empty` for `IsComplete`.
- Add unit test `vec_hash_set_random_tournament` for `IsComplete`.

## [0.54.2] - 2024-06-06

Added

- Add the `IsComplete` trait.
- Add unit test `arr_btree_set_complete` for `IsComplete`.
- Add unit test `arr_btree_set_cycle` for `IsComplete`.
- Add unit test `arr_hash_set_complete` for `IsComplete`.
- Add unit test `arr_hash_set_cycle` for `IsComplete`.
- Add unit test `vec_btree_set_complete` for `IsComplete`.
- Add unit test `vec_btree_set_cycle` for `IsComplete`.
- Add unit test `vec_hash_set_complete` for `IsComplete`.
- Add unit test `vec_hash_set_cycle` for `IsComplete`.

## [0.54.1] - 2024-06-06

Added

- Add unit test `bang_jensen_94` for `bellman_ford_moore::single_source_distances`.
- Add unit test `bang_jensen_94` for `floyd_warshall::distances`.
- Add unit test `kattis_bryr_1` for `bellman_ford_moore::single_source_distances`.
- Add unit test `kattis_bryr_2` for `bellman_ford_moore::single_source_distances`.
- Add unit test `kattis_bryr_3` for `bellman_ford_moore::single_source_distances`.
- Add unit test `kattis_crosscountry` for `bellman_ford_moore::single_source_distances`.
- Add unit test `kattis_shortestpath1` for `bellman_ford_moore::single_source_distances`.
- Add unit test `predecessors_kattis_crosscountry` for `dijkstra`.
- Add unit test `predecessors_kattis_shortestpath1` for `dijkstra`.
- Add unit test `shortest_path_bang_jensen_94` for `dijkstra`.
- Add unit test `shortest_path_kattis_crosscountry` for `dijkstra`.
- Add unit test `shortest_path_kattis_shortestpath1` for `dijkstra`.
- Add unit test `single_pair_shortest_path_kattis_bang_jensen_94` for `dijkstra`.
- Add unit test `single_pair_shortest_path_kattis_crosscountry` for `dijkstra`.
- Add unit test `single_pair_shortest_path_kattis_shortestpath1` for `dijkstra`.
- Add unit test `single_source_predecessors_kattis_crosscountry` for `dijkstra`.
- Add unit test `single_source_predecessors_kattis_shortestpath1` for `dijkstra`.

## [0.54.0] - 2024-06-05

Added

- Add benchmark `algo::single_source_distances`.

Changed

- Breaking: Rename `bellman_ford_moore::distances` to `bellman_ford_moore::single_source_distances`.

## [0.53.10] - 2024-06-05

Changed

- Change `CompleteConst` to blanket implementation.
- Change `Complete` to blanket implementation.
- Change `CycleConst` to blanket implementation.
- Change `Cycle` to blanket implementation.
- Change `EmptyConst` to blanket implementation.
- Change `RandomTournamentConst` to blanket implementation.
- Change `RandomTournament` to blanket implementation.
- Removed `W: Default` bound on `EmptyConst for [BTreeMap<usize, W>; V]`.
- Removed `W: Default` bound on `EmptyConst for [HashMap<usize, W>; V]`.
- Removed `W: Default` bound on `EmptyConst for [HashSet<(usize, W)>; V]`.

## [0.53.9] - 2024-06-03

Added

- Add `Returns` sections to `dijkstra` documentation.

## [0.53.8] - 2024-06-03

Changed

- Improve documentation.

## [0.53.7] - 2024-06-03

Changed

- Improve documentation.

## [0.53.6] - 2024-06-02

Added

- Add documentation alias `asps` for `flyod_warshall::distances`.

## [0.53.5] - 2024-06-02

Added

- Add unit test `kattis_bryr_1` for `floyd_warshall::distances`.
- Add unit test `kattis_bryr_2` for `floyd_warshall::distances`.
- Add unit test `kattis_bryr_3` for `floyd_warshall::distances`.
- Add unit test `kattis_crosscountry` for `floyd_warshall::distances`.
- Add unit test `kattis_shortestpath1` for `floyd_warshall::distances`.

Changed

- Simplify `floyd_warshall::distances` implementation.

## [0.53.4] - 2024-06-02

Fixed

- Fix the `README` adjacency list in the comments.

## [0.53.3] - 2024-06-02

Added

- Add `floyd_warshall::distances`.
- Add trait documentation example for `CompleteConst`.
- Add trait documentation example for `Complete`.
- Add trait documentation example for `CycleConst`.
- Add trait documentation example for `Cycle`.
- Add trait documentation example for `EmptyConst`.

Removed

- Remove logo.

## [0.53.2] - 2024-06-02

Changed

- Compress `README` example.
- Move project goals up in `README`.

## [0.53.1] - 2024-06-02

Changed

- Abstract the implementations of `AddArc` with macros.
- Abstract the implementations of `AddWeightedArc` with macros.
- Abstract the implementations of `ArcWeight` with macros.

## [0.53.0] - 2024-06-01

Changed

- Abstract the implementations of `HasArc` with macros.
- Abstract the implementations of `Indegree` with macros.
- Abstract the implementations of `IsSimple` with a macro.
- Abstract the implementations of `IterArcs` with macros.
- Abstract the implementations of `IterOutNeighborsMut` with macros.
- Abstract the implementations of `IterOutNeighbors` with macros.
- Abstract the implementations of `IterOutWeightedNeighbors` with macros.
- Abstract the implementations of `IterVertices` with macros.
- Abstract the implementations of `IterWeightedArcs` with macros.
- Abstract the implementations of `Order` with macros.
- Abstract the implementations of `Outdegree` with macros.
- Abstract the implementations of `RemoveArc` with macros.
- Abstract the implementations of `Size` with macros.
- Breaking: `RemoveArc for BTreeMap<usize, BTreeMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for BTreeMap<usize, BTreeSet<usize>>` no longer panics.
- Breaking: `RemoveArc for HashMap<usize, HashMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for HashMap<usize, HashSet<usize>>` no longer panics.
- Breaking: `RemoveArc for Vec<BTreeMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for Vec<BTreeSet<usize>>` no longer panics.
- Breaking: `RemoveArc for Vec<HashMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for Vec<HashSet<usize>>` no longer panics.
- Breaking: `RemoveArc for [BTreeMap<usize, W>; V]` no longer panics.
- Breaking: `RemoveArc for [BTreeMap<usize, W>]` no longer panics.
- Breaking: `RemoveArc for [BTreeSet<usize>; V]` no longer panics.
- Breaking: `RemoveArc for [BTreeSet<usize>]` no longer panics.
- Breaking: `RemoveArc for [HashMap<usize, W>; V]` no longer panics.
- Breaking: `RemoveArc for [HashMap<usize, W>]` no longer panics.
- Breaking: `RemoveArc for [HashSet<usize>; V]` no longer panics.
- Breaking: `RemoveArc for [HashSet<usize>]` no longer panics.
- Change `Converse` to blanket implementation.
- Change `Degree` to blanket implementation.
- Change `HasEdge` to blanket implementation.
- Change `IsBalanced` to blanket implementation.
- Change `IsIsolated` to blanket implementation.
- Change `IsOriented` to blanket implementation.
- Change `IsPendant` to blanket implementation.
- Change `IsRegular` to blanket implementation.
- Change `IsSubdigraph` to blanket implementation.
- Change `IsSuperdigraph` to blanket implementation.
- Change `IsSymmetric` to blanket implementation.
- Change `IsWalk` to blanket implementation.
- Change `ReverseArc` to blanket implementation.
- Use const size in `IterVertices` implementations.

## [0.52.7] - 2024-05-31

Added

- Add trait `IsSuperdigraph`.
- Implement `IsSubdigraph` for `[BTreeMap<usize, W>]`.
- Implement `IsSubdigraph` for `[HashMap<usize, W>]`.
- Implement and test `IsSubdigraph` for `[BTreeSet<usize>]`.
- Implement and test `IsSubdigraph` for `[HashSet<usize>]`.
- Implement and test `IsSuperdigraph` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement and test `IsSuperdigraph` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement and test `IsSuperdigraph` for `HashMap<usize, HashMap<usize, W>>`.
- Implement and test `IsSuperdigraph` for `HashMap<usize, HashSet<usize>>`.
- Implement and test `IsSuperdigraph` for `Vec<BTreeMap<usize, W>>`.
- Implement and test `IsSuperdigraph` for `Vec<BTreeSet<usize>>`.
- Implement and test `IsSuperdigraph` for `Vec<HashMap<usize, W>>`.
- Implement and test `IsSuperdigraph` for `Vec<HashSet<usize>>`.
- Implement and test `IsSuperdigraph` for `[BTreeMap<usize, W>; V]`.
- Implement and test `IsSuperdigraph` for `[BTreeMap<usize, W>]`.
- Implement and test `IsSuperdigraph` for `[BTreeSet<usize>; V]`.
- Implement and test `IsSuperdigraph` for `[BTreeSet<usize>]`.
- Implement and test `IsSuperdigraph` for `[HashMap<usize, W>; V]`.
- Implement and test `IsSuperdigraph` for `[HashMap<usize, W>]`.

## [0.52.6] - 2024-05-30

Added

- Add `ReverseArc` trait.
- Implement and test `ReverseArc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement and test `ReverseArc` for `BTreeSet<(usize, usize)>`.
- Implement and test `ReverseArc` for `HashMap<usize, HashSet<usize>>`.
- Implement and test `ReverseArc` for `HashSet<(usize, usize)>`.
- Implement and test `ReverseArc` for `Vec<BTreeSet<usize>>`.
- Implement and test `ReverseArc` for `Vec<HashSet<usize>>`.
- Implement and test `ReverseArc` for `[BTreeSet<usize>; V]`.
- Implement and test `ReverseArc` for `[BTreeSet<usize>]`.
- Implement and test `ReverseArc` for `[HashSet<usize>; V]`.
- Implement and test `ReverseArc` for `[HashSet<usize>]`.

## [0.52.5] - 2024-05-28

Added

- Add `IsOriented` trait.
- Implement and test `IsOriented` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement and test `IsOriented` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement and test `IsOriented` for `BTreeSet<(usize, usize)>`.
- Implement and test `IsOriented` for `HashMap<usize, HashMap<usize, W>>`.
- Implement and test `IsOriented` for `HashMap<usize, HashSet<usize>>`.
- Implement and test `IsOriented` for `HashSet<(usize, usize)>`.
- Implement and test `IsOriented` for `Vec<BTreeMap<usize, W>>`.
- Implement and test `IsOriented` for `Vec<BTreeSet<usize>>`.
- Implement and test `IsOriented` for `Vec<HashMap<usize, W>>`.
- Implement and test `IsOriented` for `Vec<HashSet<usize>>`.
- Implement and test `IsOriented` for `[BTreeMap<usize, W>; V]`.
- Implement and test `IsOriented` for `[BTreeMap<usize, W>]`.
- Implement and test `IsOriented` for `[BTreeSet<usize>; V]`.
- Implement and test `IsOriented` for `[BTreeSet<usize>]`.
- Implement and test `IsOriented` for `[HashMap<usize, W>; V]`.
- Implement and test `IsOriented` for `[HashMap<usize, W>]`.
- Implement and test `IsOriented` for `[HashSet<usize>; V]`.
- Implement and test `IsOriented` for `[HashSet<usize>]`.
- Implement and test `RandomTournament` for `BTreeSet<(usize, usize)>`.
- Implement and test `RandomTournament` for `HashSet<(usize, usize)>`.
- Implement and test `RandomTournament` for `Vec<(usize, usize)>`.

## [0.52.4] - 2024-05-27

Fixed

- Fix the `IsWalk` trait documentation example.

## [0.52.3] - 2024-05-27

Fixed

- Fix the `IsWalk` module documentation example.

## [0.52.2] - 2024-05-27

Added

- Add the `IsWalk` trait.
- Implement and test `IsWalk` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement and test `IsWalk` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement and test `IsWalk` for `HashMap<usize, HashMap<usize, W>>`.
- Implement and test `IsWalk` for `HashMap<usize, HashSet<usize>>`.
- Implement and test `IsWalk` for `Vec<BTreeMap<usize, W>>`.
- Implement and test `IsWalk` for `Vec<BTreeSet<usize>>`.
- Implement and test `IsWalk` for `Vec<HashMap<usize, W>>`.
- Implement and test `IsWalk` for `Vec<HashSet<usize>>`.
- Implement and test `IsWalk` for `[BTreeMap<usize, W>; V]`.
- Implement and test `IsWalk` for `[BTreeMap<usize, W>]`.
- Implement and test `IsWalk` for `[BTreeSet<usize>; V]`.
- Implement and test `IsWalk` for `[BTreeSet<usize>]`.
- Implement and test `IsWalk` for `[HashMap<usize, W>; V]`.
- Implement and test `IsWalk` for `[HashMap<usize, W>]`.
- Implement and test `IsWalk` for `[HashSet<usize>; V]`.
- Implement and test `IsWalk` for `[HashSet<usize>]`.

## [0.52.1] - 2024-05-26

Changed

- Simplify `op` trait descriptions.

## [0.52.0] - 2024-05-26

Added

- Implement `IterArcs` for `[BTreeMap<usize, W>]`.
- Implement `IterArcs` for `[BTreeSet<(usize, W)>]`.
- Implement `IterArcs` for `[HashMap<usize, W>]`.
- Implement `IterArcs` for `[HashSet<(usize, W)>]`.
- Implement `IterArcs` for `[Vec<usize, W>]`.
- Implement `Size` for `[(usize, usize)]`.
- Implement `Size` for `[(usize, usize, W)]`.

Changed

- Breaking: Rename `iter_all_arcs` to `iter_arcs`.
- Breaking: Rename `iter_all_weighted_arcs` to `iter_weighted_arcs`.
- Breaking: Rename `iter_weighted_arcs` to `iter_out_weighted_neighbors`.

Refactored

- Clean up `arc_weight` tests.
- Clean up `has_arc` tests.
- Clean up `has_edge` tests.
- Clean up `is_simple` tests.
- Clean up `is_symmetric` tests.
- Clean up `iter_weighted_arcs` tests.
- Clean up `iter_out_neighbors_mut` tests.
- Clean up `iter_out_neighbors` tests.
- Clean up `iter_vertices` tests.
- Clean up `iter_weighted_arcs` tests.
- Clean up `order` tests.
- Clean up `remove_arc` tests.
- Clean up `size` tests.

## [0.51.1] - 2024-05-25

Added

- Add trait `IsSubdigraph`.
- Add trait `RandomTournamentConst`.
- Implement and test `IsSubdigraph` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement and test `IsSubdigraph` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement and test `IsSubdigraph` for `HashMap<usize, HashMap<usize, W>>`.
- Implement and test `IsSubdigraph` for `HashMap<usize, HashSet<usize>>`.
- Implement and test `IsSubdigraph` for `Vec<BTreeMap<usize, W>>`.
- Implement and test `IsSubdigraph` for `Vec<BTreeSet<usize>>`.
- Implement and test `IsSubdigraph` for `Vec<HashMap<usize, W>>`.
- Implement and test `IsSubdigraph` for `Vec<HashSet<usize>>`.
- Implement and test `IsSubdigraph` for `[BTreeMap<usize, W>; V]`.
- Implement and test `IsSubdigraph` for `[BTreeSet<usize>; V]`.
- Implement and test `IsSubdigraph` for `[HashMap<usize, W>; V]`.
- Implement and test `IsSubdigraph` for `[HashSet<usize>; V]`.
- Implement and test `IterArcs` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement and test `IterArcs` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement and test `IterArcs` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement and test `IterArcs` for `BTreeSet<(usize, usize, W)>`.
- Implement and test `IterArcs` for `HashMap<usize, HashMap<usize, W>>`.
- Implement and test `IterArcs` for `HashMap<usize, HashSet<(usize, W)>>`.
- Implement and test `IterArcs` for `HashMap<usize, Vec<(usize, W)>>`.
- Implement and test `IterArcs` for `HashSet<(usize, usize, W)>`.
- Implement and test `IterArcs` for `Vec<(usize, usize, W)>`.
- Implement and test `IterArcs` for `Vec<BTreeMap<usize, W>>`.
- Implement and test `IterArcs` for `Vec<BTreeSet<(usize, W)>>`.
- Implement and test `IterArcs` for `Vec<HashMap<usize, W>>`.
- Implement and test `IterArcs` for `Vec<HashSet<(usize, W)>>`.
- Implement and test `IterArcs` for `Vec<Vec<(usize, W)>>`.
- Implement and test `IterArcs` for `[(usize, usize, W); V]`.
- Implement and test `IterArcs` for `[(usize, usize, W)]`.
- Implement and test `IterArcs` for `[BTreeMap<usize, W>; V]`.
- Implement and test `IterArcs` for `[BTreeSet<(usize, W)>; V]`.
- Implement and test `IterArcs` for `[HashMap<usize, W>; V]`.
- Implement and test `IterArcs` for `[HashSet<(usize, W)>; V]`.
- Implement and test `IterArcs` for `[Vec<(usize, W)>; V]`.
- Implement and test `RandomTournamentConst` for `[BTreeSet<usize>; V]`.
- Implement and test `RandomTournamentConst` for `[HashSet<usize>; V]`.
- Implement and test `RandomTournamentConst` for `[Vec<usize>; V]`.

## [0.51.0] - 2024-05-25

Added

- Add `bellman_ford_moore::distances` documentation examples with negative cycles.
- Add `test_negative_cycle` unit test for `bellman_ford_moore::distances`.
- Document the time complexity of `bellman_ford_moore`.
- Document the time complexity of `bfs`.
- Document the time complexity of `dijkstra`.

Changed

- Breaking: `bellman_ford_moore::distances` returns an `Option` if a negative cycle is detected.

## [0.50.2] - 2024-05-24

Added

- Cite source for `fixture::bang_jensen_94`.
- Cite source for `fixture::bang_jensen_96_isize`.
- Cite source for `fixture::bang_jensen_96`.
- Cite source for `fixture::bang_jensen_99`.
- Cite source for `fixture::crosscountry`.
- Cite source for `fixture::kattis_bryr_1`.
- Cite source for `fixture::kattis_bryr_2`.
- Cite source for `fixture::kattis_bryr_3`.
- Cite source for `fixture::shortestpath1`.

Changed

- Move `benches/algo/distances_weighted/cross_country` to `benches/algo/crosscountry`.
- Move `benches/algo/distances_weighted/shortestpath1` to `benches/algo/shortestpath1`.

Removed

- Remove `small_graph_1` benchmark.
- Remove `graph_1` benchmark.

## [0.50.1] - 2024-05-23

Refactored

- Move fixtures to their module.
- Re-use `IsBalanced` implementation with a macro.
- Re-use `IsRegular` implementation with a macro.
- Simplify `bfs` tests.
- Simplify `dijkstra` tests.
- Simplify `is_regular` tests.

Removed

- Remove unit test `slice_btree_map` for `IsRegular`.
- Remove unit test `slice_hash_map` for `IsRegular`.
- Remove unused proptest regressions.

## [0.50.0] - 2024-05-22

Added

- Add `algo::bellman_ford_moore::distances`.
- Add module-level example for `predecessor`.
- Add unit test `bang_jensen_96` for `bellman_ford_moore::distances`.
- Add unit test `bang_jensen_99` for `bellman_ford_moore::distances`.
- Add unit test `documentation test` for `bellman_ford_moore::distances`.

## [0.49.1] - 2024-05-22

Changed

- Use `Self::with_capacity` in `Empty for Vec<(usize, usize)>`.
- Use `Self::with_capacity` in `Empty for Vec<(usize, usize, W)>`.

## [0.49.0] - 2024-05-21

Changed

- Clean up `Empty` bounds.

Added

- Add `Converse` trait.
- Add `arr_btree_map_weighted` unit test for `Converse`.
- Add `arr_btree_set_weighted` unit test for `Converse`.
- Add `arr_btree_set` unit test for `Converse`.
- Add `arr_hash_map_weighted` unit test for `Converse`.
- Add `arr_hash_set_weighted` unit test for `Converse`.
- Add `arr_hash_set` unit test for `Converse`.
- Add `arr_tuple_weighted` unit test for `Converse`.
- Add `arr_tuple` unit test for `Converse`.
- Add `arr_vec_weighted` unit test for `Converse`.
- Add `arr_vec` unit test for `Converse`.
- Add `btree_map_btree_map_weighted` unit test for `Converse`.
- Add `btree_map_btree_set_weighted` unit test for `Converse`.
- Add `btree_map_btree_set` unit test for `Converse`.
- Add `btree_map_vec_weighted` unit test for `Converse`.
- Add `btree_map_vec` unit test for `Converse`.
- Add `btree_set_tuple_weighted` unit test for `Converse`.
- Add `btree_set_tuple` unit test for `Converse`.
- Add `hash_map_hash_map_weighted` unit test for `Converse`.
- Add `hash_map_hash_set_weighted` unit test for `Converse`.
- Add `hash_map_hash_set` unit test for `Converse`.
- Add `hash_map_vec_weighted` unit test for `Converse`.
- Add `hash_map_vec` unit test for `Converse`.
- Add `hash_set_tuple_weighted` unit test for `Converse`.
- Add `hash_set_tuple` unit test for `Converse`.
- Add `prop` module documentation example.
- Add `repr` module documentation example.
- Add `vec_btree_map_weighted` unit test for `Converse`.
- Add `vec_btree_set_weighted` unit test for `Converse`.
- Add `vec_btree_set` unit test for `Converse`.
- Add `vec_hash_map_weighted` unit test for `Converse`.
- Add `vec_hash_set_weighted` unit test for `Converse`.
- Add `vec_hash_set` unit test for `Converse`.
- Add `vec_tuple_weighted` unit test for `Converse`.
- Add `vec_tuple` unit test for `Converse`.
- Add `vec_vec_weighted` unit test for `Converse`.
- Add `vec_vec` unit test for `Converse`.
- Implement `AddArc` for `BTreeSet<(usize, usize)>`.
- Implement `AddArc` for `HashSet<(usize, usize)>`.
- Implement `AddArc` for `Vec<(usize, usize)>`.
- Implement `Converse` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `Converse` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `Converse` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Converse` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `Converse` for `BTreeMap<usize, Vec<usize>>`.
- Implement `Converse` for `BTreeSet<(usize, usize)>`.
- Implement `Converse` for `BTreeSet<(usize, usize, W)>`.
- Implement `Converse` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `Converse` for `HashMap<usize, HashSet<(usize, W)>>`.
- Implement `Converse` for `HashMap<usize, HashSet<usize>>`.
- Implement `Converse` for `HashMap<usize, Vec<(usize, W)>>`.
- Implement `Converse` for `HashMap<usize, Vec<usize>>`.
- Implement `Converse` for `HashSet<(usize, usize)>`.
- Implement `Converse` for `HashSet<(usize, usize, W)>`.
- Implement `Converse` for `Vec<(usize, usize)>`.
- Implement `Converse` for `Vec<(usize, usize, W)>`.
- Implement `Converse` for `Vec<BTreeMap<usize, W>>`.
- Implement `Converse` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `Converse` for `Vec<BTreeSet<usize>>`.
- Implement `Converse` for `Vec<HashMap<usize, W>>`.
- Implement `Converse` for `Vec<HashSet<(usize, W)>>`.
- Implement `Converse` for `Vec<HashSet<usize>>`.
- Implement `Converse` for `Vec<Vec<(usize, W)>>`.
- Implement `Converse` for `Vec<Vec<usize>>`.
- Implement `Converse` for `[(usize, usize); V]`.
- Implement `Converse` for `[(usize, usize, W); V]`.
- Implement `Converse` for `[BTreeMap<usize, W>; V]`.
- Implement `Converse` for `[BTreeSet<(usize, W)>; V]`.
- Implement `Converse` for `[BTreeSet<usize>; V]`.
- Implement `Converse` for `[HashMap<usize, W>; V]`.
- Implement `Converse` for `[HashSet<(usize, W)>; V]`.
- Implement `Converse` for `[HashSet<usize, H>; V]`.
- Implement `Converse` for `[Vec<(usize, W)>; V]`.
- Implement `Converse` for `[Vec<usize>; V]`.

## [0.48.2] - 2024-05-20

Changed

- Shorten the `README` example.

## [0.48.1] - 2024-05-19

Added

- Add `prop::sum_indegrees_eq_sum_outdegrees`.
- Add property test `degree_btree_map_btree_set` for `random_tournament`.
- Add property test `degree_hash_map_hash_set` for `random_tournament`.
- Add property test `degree_vec_btree_set` for `random_tournament`.
- Add property test `degree_vec_hash_set` for `random_tournament`.
- Add property test `indegree_btree_map_btree_set` for `random_tournament`.
- Add property test `indegree_hash_map_hash_set` for `random_tournament`.
- Add property test `indegree_vec_btree_set` for `random_tournament`.
- Add property test `indegree_vec_hash_set` for `random_tournament`.
- Add property test `order_vec_btree_set` for `random_tournament`.
- Add property test `order_vec_hash_set` for `random_tournament`.
- Add property test `order_vec_vec` for `random_tournament`.
- Add property test `outdegree_btree_map_btree_set` for `random_tournament`.
- Add property test `outdegree_btree_map_vec` for `random_tournament`.
- Add property test `outdegree_hash_map_hash_set` for `random_tournament`.
- Add property test `outdegree_hash_map_vec` for `random_tournament`.
- Add property test `outdegree_vec_btree_set` for `random_tournament`.
- Add property test `outdegree_vec_hash_set` for `random_tournament`.
- Add property test `outdegree_vec_vec` for `random_tournament`.
- Add property test `size_btree_map_btree_set` for `random_tournament`.
- Add property test `size_btree_map_vec` for `random_tournament`.
- Add property test `size_hash_map_hash_set` for `random_tournament`.
- Add property test `size_hash_map_vec` for `random_tournament`.
- Add property test `size_vec_btree_set` for `random_tournament`.
- Add property test `size_vec_hash_set` for `random_tournament`.
- Add property test `size_vec_vec` for `random_tournament`.
- Add property test `sum_indegrees_eq_sum_outdegrees_arr_btree_set` for `CompleteConst::complete`.
- Add property test `sum_indegrees_eq_sum_outdegrees_arr_btree_set` for `CycleConst::cycle`.
- Add property test `sum_indegrees_eq_sum_outdegrees_arr_hash_set` for `CompleteConst::complete`.
- Add property test `sum_indegrees_eq_sum_outdegrees_arr_hash_set` for `CycleConst::cycle`.
- Add property test `sum_indegrees_eq_sum_outdegrees_btree_map_btree_set` for `Complete::complete`.
- Add property test `sum_indegrees_eq_sum_outdegrees_btree_map_btree_set` for `Cycle::cycle`.
- Add property test `sum_indegrees_eq_sum_outdegrees_hash_map_hash_set` for `Complete::complete`.
- Add property test `sum_indegrees_eq_sum_outdegrees_hash_map_hash_set` for `Cycle::cycle`.
- Add property test `sum_indegrees_eq_sum_outdegrees_vec_btree_set` for `Complete::complete`.
- Add property test `sum_indegrees_eq_sum_outdegrees_vec_btree_set` for `Cycle::cycle`.
- Add property test `sum_indegrees_eq_sum_outdegrees_vec_hash_set` for `Complete::complete`.
- Add property test `sum_indegrees_eq_sum_outdegrees_vec_hash_set` for `Cycle::cycle`.
- Add unit test `btree_set_tuple` for `iter_weighted_arcs`.

Removed

- Remove unit test `btree_map_btree_set` for `random_tournament`.
- Remove unit test `btree_map_vec` for `random_tournament`.
- Remove unit test `hash_map_hash_set` for `random_tournament`.
- Remove unit test `hash_map_vec` for `random_tournament`.
- Remove unit test `vec_btree_set` for `random_tournament`.
- Remove unit test `vec_hash_set` for `random_tournament`.
- Remove unit test `vec_vec` for `random_tournament`.

## [0.48.0] - 2024-05-19

Added

- Add `is_sink` method to `Outdegree`.
- Add `is_source` method to `Indegree`.
- Add project goals to `README`.
- Add unit test `arr_btree_map_is_sink` for `is_sink`.
- Add unit test `arr_btree_map_is_source` for `is_source`.
- Add unit test `arr_btree_set_is_source` for `is_source`.
- Add unit test `arr_btree_set_unweighted_is_sink` for `is_sink`.
- Add unit test `arr_btree_set_weighted_is_sink` for `is_sink`.
- Add unit test `arr_hash_map_is_sink` for `is_sink`.
- Add unit test `arr_hash_map_is_source` for `is_source`.
- Add unit test `arr_hash_set_is_source` for `is_source`.
- Add unit test `arr_hash_set_unweighted_is_sink` for `is_sink`.
- Add unit test `arr_hash_set_weighted_is_sink` for `is_sink`.
- Add unit test `arr_vec_unweighted_is_sink` for `is_sink`.
- Add unit test `arr_vec_weighted_is_sink` for `is_sink`.
- Add unit test `btree_map_btree_map_is_sink` for `is_sink`.
- Add unit test `btree_map_btree_map_is_source` for `is_source`.
- Add unit test `btree_map_btree_set_is_sink` for `is_sink`.
- Add unit test `btree_map_btree_set_is_source` for `is_source`.
- Add unit test `btree_map_vec_is_sink` for `is_sink`.
- Add unit test `hash_map_hash_map_is_sink` for `is_sink`.
- Add unit test `hash_map_hash_map_is_source` for `is_source`.
- Add unit test `hash_map_hash_set_is_sink` for `is_sink`.
- Add unit test `hash_map_hash_set_is_source` for `is_source`.
- Add unit test `hash_map_vec_is_sink` for `is_sink`.
- Add unit test `slice_btree_map_is_sink` for `is_sink`.
- Add unit test `slice_btree_map_is_source` for `is_source`.
- Add unit test `slice_btree_set_is_source` for `is_source`.
- Add unit test `slice_btree_set_unweighted_is_sink` for `is_sink`.
- Add unit test `slice_btree_set_weighted_is_sink` for `is_sink`.
- Add unit test `slice_hash_map_is_sink` for `is_sink`.
- Add unit test `slice_hash_map_is_source` for `is_source`.
- Add unit test `slice_hash_set_is_source` for `is_source`.
- Add unit test `slice_hash_set_unweighted_is_sink` for `is_sink`.
- Add unit test `slice_hash_set_weighted_is_sink` for `is_sink`.
- Add unit test `slice_vec_unweighted_is_sink` for `is_sink`.
- Add unit test `slice_vec_weighted_is_sink` for `is_sink`.
- Add unit test `vec_btree_map_is_sink` for `is_sink`.
- Add unit test `vec_btree_map_is_source` for `is_source`.
- Add unit test `vec_btree_set_is_source` for `is_source`.
- Add unit test `vec_btree_set_unweighted_is_sink` for `is_sink`.
- Add unit test `vec_btree_set_weighted_is_sink` for `is_sink`.
- Add unit test `vec_hash_map_is_sink` for `is_sink`.
- Add unit test `vec_hash_map_is_source` for `is_source`.
- Add unit test `vec_hash_set_is_source` for `is_source`.
- Add unit test `vec_hash_set_unweighted_is_sink` for `is_sink`.
- Add unit test `vec_hash_set_weighted_is_sink` for `is_sink`.
- Add unit test `vec_vec_unweighted_is_sink` for `is_sink`.
- Add unit test `vec_vec_weighted_is_sink` for `is_sink`.

Changed

- Breaking: Rename `IterArcsMut` to `IterOutNeighborsMut`.
- Breaking: Rename `IterArcs` to `IterOutNeighbors`.

Removed

- Remove documentation alias "in_valence" for `indegree`.
- Remove documentation alias "inward_demidegree" for `indegree`.
- Remove documentation alias "out_valence" for `outdegree`.
- Remove documentation alias "outward_demidegree" for `outdegree`.

## [0.47.3] - 2024-05-19

Changed

- Rename `graph` to `digraph` internally.
- Rename `G` to `D` internally.

## [0.47.2] - 2024-05-19

Added

- Add documentation alias "in_degree" for `op::indegree`.
- Add documentation alias "out_degree" for `op::outdegree`.

Removed

- Remove testing from the `publish` script.

## [0.47.1] - 2024-05-18

Added

- Add alias `singleton` for `trivial`.
- Add method `trivial`.
- Add unit test `btree_map_btree_map_trivial` for `trivial`.
- Add unit test `btree_map_btree_set_unweighted_trivial` for `trivial`.
- Add unit test `btree_map_btree_set_weighted_trivial` for `trivial`.
- Add unit test `btree_map_vec_unweighted_trivial` for `trivial`.
- Add unit test `btree_map_vec_weighted_trivial` for `trivial`.
- Add unit test `btree_set_tuple_unweighted_trivial` for `trivial`.
- Add unit test `btree_set_tuple_weighted_trivial` for `trivial`.
- Add unit test `hash_map_hash_map_trivial` for `trivial`.
- Add unit test `hash_map_hash_set_unweighted_trivial` for `trivial`.
- Add unit test `hash_map_hash_set_weighted_trivial` for `trivial`.
- Add unit test `hash_map_vec_unweighted_trivial` for `trivial`.
- Add unit test `hash_map_vec_weighted_trivial` for `trivial`.
- Add unit test `hash_set_tuple_unweighted_trivial` for `trivial`.
- Add unit test `hash_set_tuple_weighted_trivial` for `trivial`.
- Add unit test `vec_btree_set_unweighted_trivial` for `trivial`.
- Add unit test `vec_btree_set_weighted_trivial` for `trivial`.
- Add unit test `vec_hash_set_unweighted_trivial` for `trivial`.
- Add unit test `vec_hash_set_weighted_trivial` for `trivial`.
- Add unit test `vec_tuple_unweighted_trivial` for `trivial`.
- Add unit test `vec_tuple_weighted_trivial` for `trivial`.
- Add unit test `vec_vec_unweighted_trivial` for `trivial`.
- Add unit test `vec_vec_unweighted_trivial` for `trivial`.
- Add unit test `vec_vec_weighted_trivial` for `trivial`.

Changed

- Mark `Complete::complete` as `#[must_use]`.
- Mark `CompleteConst::complete` as `#[must_use]`.
- Mark `Cycle::cycle` as `#[must_use]`.
- Mark `CycleConst::cycle` as `#[must_use]`.
- Mark `Empty::empty` as `#[must_use]`.
- Mark `EmptyConst::empty` as `#[must_use]`.
- Mark `RandomTournament::random_tournament` as `#[must_use]`.

## [0.47.0] - 2024-05-18

Added

- Add `RandomTournament` example to `README`.
- Add `gen::RandomTournament` generator.
- Add `prng::SplitMix64` PRNG to seed `Xoshiro256StarStar`.
- Add `prng::Xoshiro256StarStar` PRNG.
- Add `prng` module.
- Add unit test `btree_map_btree_set` for `RandomTournament`.
- Add unit test `btree_map_vec` for `RandomTournament`.
- Add unit test `hash_map_hash_set` for `RandomTournament`.
- Add unit test `hash_map_vec` for `RandomTournament`.
- Add unit test `vec_btree_set` for `RandomTournament`.
- Add unit test `vec_hash_set` for `RandomTournament`.
- Add unit test `vec_vec` for `RandomTournament`.
- Implement `RandomTournament` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `RandomTournament` for `BTreeMap<usize, Vec<usize>>`.
- Implement `RandomTournament` for `HashMap<usize, HashSet<usize>>`.
- Implement `RandomTournament` for `HashMap<usize, Vec<usize>>`.
- Implement `RandomTournament` for `Vec<BTreeSet<usize>>`.
- Implement `RandomTournament` for `Vec<HashSet<usize>>`.
- Implement `RandomTournament` for `Vec<Vec<usize>>`.

## [0.46.0] - 2024-05-17

Added

- Add `IterOutNeighborsMut` trait.
- Add `arr_vec` unit test for `IterOutNeighborsMut`.
- Add `btree_map_vec` unit test for `IterOutNeighborsMut`.
- Add `hash_map_vec` unit test for `IterOutNeighborsMut`.
- Add `slice_vec` unit test for `IterOutNeighborsMut`.
- Add `vec_vec` unit test for `IterOutNeighborsMut`.
- Implement `IterOutNeighborsMut` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterOutNeighborsMut` for `HashMap<usize, Vec<usize>>`.
- Implement `IterOutNeighborsMut` for `Vec<Vec<usize>>`.
- Implement `IterOutNeighborsMut` for `[Vec<usize>; V]`.
- Implement `IterOutNeighborsMut` for `[Vec<usize>]`.

## [0.45.1] - 2024-05-17

Fixed

- Fix typo

## [0.45.0] - 2024-05-15

Changed

- Breaking: Rename `*edge*` to `*arc*` in places where it refers to directed edges.

## [0.44.0] - 2024-05-15

Changed

- Breaking: Rename `CountAllVertices` to `Order`.
- Breaking: Rename `CountAllArcs` to `Size`.

## [0.43.0] - 2024-05-13

Added

- Add `IsRegular` trait.
- Add a "How to implement" section to `IsSymmetric` documentation.
- Add prop test `empty_btree_map_btree_map` for `IsRegular`.
- Add prop test `empty_btree_map_btree_set_unweighted` for `IsRegular`.
- Add prop test `empty_hash_map_hash_map` for `IsRegular`.
- Add prop test `empty_hash_map_hash_set_unweighted` for `IsRegular`.
- Add prop test `empty_vec_btree_map` for `IsRegular`.
- Add prop test `empty_vec_btree_set_unweighted` for `IsRegular`.
- Add prop test `empty_vec_hash_map` for `IsRegular`.
- Add prop test `empty_vec_hash_set_unweighted` for `IsRegular`.
- Add unit test `arr_btree_map` for `IsRegular`.
- Add unit test `arr_btree_set_unweighted` for `IsRegular`.
- Add unit test `arr_hash_map` for `IsRegular`.
- Add unit test `arr_hash_set_unweighted` for `IsRegular`.
- Add unit test `btree_map_btree_map()` for `IterVertices`.
- Add unit test `btree_map_btree_map` for `IsRegular`.
- Add unit test `btree_map_btree_set_weighted()` for `IterVertices`.
- Add unit test `btree_map_btree_set` for `IsRegular`.
- Add unit test `btree_map_vec_weighted()` for `IterVertices`.
- Add unit test `hash_map_hash_map()` for `IterVertices`.
- Add unit test `hash_map_hash_map` for `IsRegular`.
- Add unit test `hash_map_hash_set_weighted()` for `IterVertices`.
- Add unit test `hash_map_hash_set` for `IsRegular`.
- Add unit test `hash_map_vec_weighted()` for `IterVertices`.
- Add unit test `slice_btree_map` for `IsRegular`.
- Add unit test `slice_btree_set_unweighted` for `IsRegular`.
- Add unit test `slice_hash_map` for `IsRegular`.
- Add unit test `slice_hash_set_unweighted` for `IsRegular`.
- Add unit test `vec_btree_map` for `IsRegular`.
- Add unit test `vec_btree_set_unweighted` for `IsRegular`.
- Add unit test `vec_hash_map` for `IsRegular`.
- Add unit test `vec_hash_set_unweighted` for `IsRegular`.
- Implement `IsRegular` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsRegular` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsRegular` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsRegular` for `HashMap<usize, HashSet<usize>>`.
- Implement `IsRegular` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsRegular` for `Vec<BTreeSet<usize>>`.
- Implement `IsRegular` for `Vec<HashMap<usize, W>>`.
- Implement `IsRegular` for `Vec<HashSet<usize>>`.
- Implement `IsRegular` for `[BTreeMap<usize, W>; V]`.
- Implement `IsRegular` for `[BTreeMap<usize, W>]`.
- Implement `IsRegular` for `[BTreeSet<usize>; V]`.
- Implement `IsRegular` for `[BTreeSet<usize>]`.
- Implement `IsRegular` for `[HashMap<usize, W>; V]`.
- Implement `IsRegular` for `[HashMap<usize, W>]`.
- Implement `IsRegular` for `[HashSet<usize>; V]`.
- Implement `IsRegular` for `[HashSet<usize>]`.
- Implement `IterVertices` for `BTreeMap<usize, BTreeMap<usize, W>>`
- Implement `IterVertices` for `BTreeMap<usize, BTreeSet<(usize, W)>>`
- Implement `IterVertices` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IterVertices` for `BTreeMap<usize, Vec<(usize, W)>>`
- Implement `IterVertices` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterVertices` for `HashMap<usize, HashMap<usize, W>>`
- Implement `IterVertices` for `HashMap<usize, HashSet<(usize, W)>>`
- Implement `IterVertices` for `HashMap<usize, HashSet<usize>>`.
- Implement `IterVertices` for `HashMap<usize, Vec<(usize, W)>>`
- Implement `IterVertices` for `HashMap<usize, Vec<usize>>`.

## [0.42.4] - 2024-05-12

Added

- Add proptest strategy `simple_v_e`.

Changed

- Simplify `README` examples.

## [0.42.3] - 2024-05-12

Changed

- Improve `binop_vertices` tests.

## [0.42.2] - 2024-05-11

Added

- Add `IsPendant` trait.

## [0.42.1] - 2024-05-11

Added

- Add `Degree` trait.

## [0.42.0] - 2024-05-11

Added

- Add unit test `has_arc` for `AdjacencyMatrix<V>`.
- Implement `HasEdge` for `AdjacencyMatrix<V>`.
- Implement `IsBalanced` for `AdjacencyMatrix<V>`.
- Implement `IsIsolated` for `AdjacencyMatrix<V>`.
- Implement `IsSymmetric` for `AdjacencyMatrix<V>`.
- Implement `IterArcs` for `AdjacencyMatrix<V>`.

Changed

- Breaking: `AdjacencyMatrix::<0>::new` panics.
- Simplify `AdjacencyMatrix<V>` op implementations with `IterVertices`.
- Simplify `AdjacencyMatrix<V>` tests with `Iterator::eq`.
- Simplify `IsSymmetric` implementations with `HasArc`.
- Simplify arguments to `Iterator::eq` in tests.

## [0.41.0] - 2024-05-11

Changed

- Breaking: `Complete::complete` panics if `v` is zero.
- Breaking: `CompleteConst::complete_const` panics if `V` is zero.
- Breaking: `Cycle::cycle` panics if `v` is zero.
- Breaking: `CycleConst::cycle_const` panics if `V` is zero.
- Breaking: `Empty::empty` panics if `v` is zero.
- Breaking: `EmptyConst::empty_const` panics if `V` is zero.

## [0.40.0] - 2024-05-10

Added

- Add documentation alias "in_valence" for `indegree`.
- Add documentation alias "inward_demidegree" for `indegree`.
- Add documentation alias "out_valence" for `outdegree`.
- Add documentation alias "outward_demidegree" for `outdegree`.
- Add unit test `arr_btree_map` for `IsIsolated`.
- Add unit test `arr_btree_set` for `IsIsolated`.
- Add unit test `arr_hash_map` for `IsIsolated`.
- Add unit test `arr_hash_set` for `IsIsolated`.
- Add unit test `btree_map_btree_map` for `IsIsolated`.
- Add unit test `btree_map_btree_set` for `IsIsolated`.
- Add unit test `hash_map_hash_map` for `IsIsolated`.
- Add unit test `hash_map_hash_set` for `IsIsolated`.
- Add unit test `slice_btree_map` for `IsBalanced`.
- Add unit test `slice_btree_map` for `IsIsolated`.
- Add unit test `slice_btree_set` for `IsBalanced`.
- Add unit test `slice_btree_set` for `IsIsolated`.
- Add unit test `slice_hash_map` for `IsBalanced`.
- Add unit test `slice_hash_map` for `IsIsolated`.
- Add unit test `slice_hash_set` for `IsBalanced`.
- Add unit test `slice_hash_set` for `IsIsolated`.
- Add unit test `vec_btree_map` for `IsIsolated`.
- Add unit test `vec_btree_set` for `IsIsolated`.
- Add unit test `vec_hash_map` for `IsIsolated`.
- Add unit test `vec_hash_set` for `IsIsolated`.
- Implement `IsBalanced` for `[BTreeMap<usize, W>]`.
- Implement `IsBalanced` for `[BTreeSet<usize>]`.
- Implement `IsBalanced` for `[HashMap<usize, W>]`.
- Implement `IsBalanced` for `[HashSet<usize>]`.
- Implement `IsIsolated` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsIsolated` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsIsolated` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsIsolated` for `HashMap<usize, HashSet<usize>>`.
- Implement `IsIsolated` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsIsolated` for `Vec<BTreeSet<usize>>`.
- Implement `IsIsolated` for `Vec<HashMap<usize, W>>`.
- Implement `IsIsolated` for `Vec<HashSet<usize>>`.
- Implement `IsIsolated` for `[BTreeMap<usize, W>; V]`.
- Implement `IsIsolated` for `[BTreeMap<usize, W>]`.
- Implement `IsIsolated` for `[BTreeSet<usize>; V]`.
- Implement `IsIsolated` for `[BTreeSet<usize>]`.
- Implement `IsIsolated` for `[HashMap<usize, W>; V]`.
- Implement `IsIsolated` for `[HashMap<usize, W>]`.
- Implement `IsIsolated` for `[HashSet<usize>; V]`.
- Implement `IsIsolated` for `[HashSet<usize>]`.

Changed

- Restrict existing documentation aliases to the module level.
- Simplify `AddArc` "Panics" sections.
- Simplify `Indegree` tests.
- Simplify `IsBalanced` tests.
- Simplify `Outdegree` tests.

Fixed

- Breaking: Fix `IsBalanced`.
- Fix `complete` documentation aliases.
- Fix `empty_const` documentation aliases.

## [0.39.0] - 2024-05-09

Changed

- Breaking: `AddArc for BTreeMap<usize, BTreeSet<usize>>` no longer panics.
- Breaking: `AddArc for BTreeMap<usize, Vec<usize>>` no longer panics.
- Breaking: `AddArc for HashMap<usize, HashSet<usize>>` no longer panics.
- Breaking: `AddArc for HashMap<usize, Vec<usize>>` no longer panics.
- Breaking: `AddWeightedArc for BTreeMap<usize, BTreeMap<usize, W>>` no longer panics.
- Breaking: `AddWeightedArc for BTreeMap<usize, BTreeSet<(usize, W)>>` no longer panics.
- Breaking: `AddWeightedArc for BTreeMap<usize, Vec<(usize, W)>>` no longer panics.
- Breaking: `AddWeightedArc for HashMap<usize, HashMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for BTreeMap<usize, BTreeMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for BTreeMap<usize, BTreeSet<usize>>` no longer panics.
- Breaking: `RemoveArc for HashMap<usize, HashMap<usize, W>>` no longer panics.
- Breaking: `RemoveArc for HashMap<usize, HashSet<usize>>` no longer panics.

Removed

- Remove panic documentation from `Size` implementations.

## [0.38.2] - 2024-05-09

Added

- Add property test `binop_vertices_bounds` for `binop_vertices`.
- Add unit test `arr_vec_parallel_arcs` for `IsSimple`.
- Add unit test `arr_vec_self_loop` for `IsSimple`.
- Add unit test `arr_vec_simple` for `IsSimple`.
- Add unit test `slice_vec_parallel_arcs` for `IsSimple`.
- Add unit test `slice_vec_self_loop` for `IsSimple`.
- Add unit test `slice_vec_simple` for `IsSimple`.
- Add unit test `vec_vec_parallel_arcs` for `IsSimple`.
- Add unit test `vec_vec_self_loop` for `IsSimple`.
- Add unit test `vec_vec_simple` for `IsSimple`.
- Implement `IsSimple` for `Vec<Vec<usize>>`.
- Implement `IsSimple` for `[Vec<usize>; V]`.
- Implement `IsSimple` for `[Vec<usize>]`.

## [0.38.1] - 2024-05-09

Fixed

- Fix formatting

## [0.38.0] - 2024-05-09

Added

- Add documentation alias "isograph" for `IsBalanced`.
- Add documentation alias "pseudosymmetric" for `IsBalanced`.
- Add trait `HasEdge`.
- Add trait `IsSymmetric`.
- Implement `HasEdge` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `HasEdge` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `HasEdge` for `BTreeSet<(usize, usize)>`.
- Implement `HasEdge` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `HasEdge` for `HashMap<usize, HashSet<usize>>`.
- Implement `HasEdge` for `HashSet<(usize, usize)>`.
- Implement `HasEdge` for `Vec<BTreeMap<usize, W>>`.
- Implement `HasEdge` for `Vec<BTreeSet<usize>>`.
- Implement `HasEdge` for `Vec<HashMap<usize, W>>`.
- Implement `HasEdge` for `Vec<HashSet<usize>>`.
- Implement `HasEdge` for `[BTreeMap<usize, W>; V]`.
- Implement `HasEdge` for `[BTreeMap<usize, W>]`.
- Implement `HasEdge` for `[BTreeSet<usize>; V]`.
- Implement `HasEdge` for `[BTreeSet<usize>]`.
- Implement `HasEdge` for `[HashMap<usize, W>; V]`.
- Implement `HasEdge` for `[HashMap<usize, W>]`.
- Implement `HasEdge` for `[HashSet<usize>; V]`.
- Implement `HasEdge` for `[HashSet<usize>]`.
- Implement `IsSymmetric` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsSymmetric` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsSymmetric` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsSymmetric` for `HashMap<usize, HashSet<usize>>`.
- Implement `IsSymmetric` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsSymmetric` for `Vec<BTreeSet<usize>>`.
- Implement `IsSymmetric` for `Vec<HashMap<usize, W>>`.
- Implement `IsSymmetric` for `Vec<HashSet<usize>>`.
- Implement `IsSymmetric` for `[BTreeMap<usize, W>; V]`.
- Implement `IsSymmetric` for `[BTreeMap<usize, W>]`.
- Implement `IsSymmetric` for `[BTreeSet<usize>; V]`.
- Implement `IsSymmetric` for `[BTreeSet<usize>]`.
- Implement `IsSymmetric` for `[HashMap<usize, W>; V]`.
- Implement `IsSymmetric` for `[HashMap<usize, W>]`.
- Implement `IsSymmetric` for `[HashSet<usize>; V]`.
- Implement `IsSymmetric` for `[HashSet<usize>]`.
- Implement `IterWeightedArcs` for `[BTreeMap<usize, W>]`.
- Implement `IterWeightedArcs` for `[BTreeSet<(usize, W)>]`.
- Implement `IterWeightedArcs` for `[HashMap<usize, W>]`.
- Implement `IterWeightedArcs` for `[HashSet<(usize, W)>]`.
- Implement `IterWeightedArcs` for `[Vec<(usize, W)>]`.

Changed

- Breaking: Rename `add_weighted_arc_is_arc` to `add_weighted_arc_has_arc`.
- Breaking: Rename `add_arc_remove_arc` to `add_arc_has_arc`.
- Breaking: Rename `IsArc` to `HasArc`.

Removed

- Remove bound `W: Ord` from `HasArc for [BTreeMap<usize, W>]`.
- Remove bound `W: Ord` from `HasArc for [BTreeMap<usize, W>; V]`.

## [0.37.0] - 2024-05-08

Added

- Implement `IsBalanced` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsBalanced` for `Vec<HashMap<usize, W>>`.
- Implement `IsBalanced` for `[BTreeMap<usize, W>; V]`.
- Implement `IsBalanced` for `[HashMap<usize, W>; V]`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `IterWeightedArcs` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IterWeightedArcs` for `HashMap<usize, HashSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `HashMap<usize, Vec<(usize, W)>>`.
- Implement `IterWeightedArcs` for `Vec<BTreeMap<usize, W>>`.
- Implement `IterWeightedArcs` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `Vec<HashMap<usize, W>>`.
- Implement `IterWeightedArcs` for `Vec<HashSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `Vec<Vec<(usize, W)>>`.
- Implement `IterWeightedArcs` for `[BTreeMap<usize, W>; V]`.
- Implement `IterWeightedArcs` for `[BTreeSet<(usize, W)>; V]`.
- Implement `IterWeightedArcs` for `[HashMap<usize, W>; V]`.
- Implement `IterWeightedArcs` for `[HashSet<(usize, W)>; V]`.
- Implement `IterWeightedArcs` for `[Vec<(usize, W)>; V]`.

## [0.36.0] - 2024-05-08

Added

- Add `IsBalanced` trait.
- Implement `IsBalanced` for `Vec<BTreeSet<usize>>`.
- Implement `IsBalanced` for `Vec<HashSet<usize>>`.
- Implement `IsBalanced` for `[BTreeSet<usize>; V]`.
- Implement `IsBalanced` for `[HashSet<usize>; V]`.
- Implement `IterArcs` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IterArcs` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterArcs` for `HashMap<usize, HashSet<usize>>`.
- Implement `IterArcs` for `HashMap<usize, Vec<usize>>`.
- Implement `IterArcs` for `Vec<BTreeSet<usize>>`.
- Implement `IterArcs` for `Vec<HashSet<usize>>`.
- Implement `IterArcs` for `Vec<Vec<usize>>`.
- Implement `IterArcs` for `[BTreeSet<usize>; V]`.
- Implement `IterArcs` for `[BTreeSet<usize>]`.
- Implement `IterArcs` for `[HashSet<usize>; V]`.
- Implement `IterArcs` for `[HashSet<usize>]`.
- Implement `IterArcs` for `[Vec<usize>; V]`.
- Implement `IterArcs` for `[Vec<usize>]`.

## [0.35.6] - 2024-05-07

Added

- Add back the description of operations in `README`.

## [0.35.5] - 2024-05-07

Changed

- Clean up `Complete` property tests.
- Clean up `Cycle` property tests.
- Clean up `Empty` property tests.
- Simplify `CompleteConst::complete` implementations with `Empty`.
- Simplify `Cycle::cycle` implementations with `Empty`.
- Simplify `CycleConst::cycle` implementations with `Empty`.

Fixed

- Fix `CompleteConst` `outdegree` unit tests.

## [0.35.4] - 2024-05-07

Added

- Add `btree_set` unit test for `iter_weighted_arcs`.

Changed

- Simplify tests with `Iterator::eq`.

## [0.35.3] - 2024-05-06

Changed

- Update `README` examples.

## [0.35.2] - 2024-05-05

Added

- Add `gen::EmptyConst` generator.

## [0.35.1] - 2024-05-05

Added

- Add `gen::Empty` generator.
- Add `publish` script.
- Add documentation alias "circular" for `gen::cyle_const`.
- Add documentation alias "circular" for `gen::cyle`.

Changed

- Simplify implementations of `Complete::complete`.
- Simplify implementations of `CompleteConst::complete_const`.

Removed

- Remove `pre-commit.sh`.
- Remove `setup-pre-commit.sh`.

## [0.35.0] - 2024-05-05

Added

- Add `CompleteConst` unit test `arr_btree_set`.
- Add `CompleteConst` unit test `arr_hash_set`.
- Add `CompleteConst` unit test `arr_vec`.
- Add `CompleteConst` unit test `size_arr_btree_set`.
- Add `CompleteConst` unit test `size_arr_hash_set`.
- Add `CompleteConst` unit test `size_arr_vec`.
- Add `CompleteConst` unit test `order_arr_btree_set`.
- Add `CompleteConst` unit test `order_arr_hash_set`.
- Add `CompleteConst` unit test `order_arr_vec`.
- Add `CompleteConst` unit test `indegree_arr_btree_set`.
- Add `CompleteConst` unit test `indegree_arr_hash_set`.
- Add `CompleteConst` unit test `is_simple_arr_btree_set`.
- Add `CompleteConst` unit test `is_simple_arr_hash_set`.
- Add `CompleteConst` unit test `outdegree_arr_btree_set`.
- Add `CompleteConst` unit test `outdegree_arr_hash_set`.
- Add `CompleteConst` unit test `outdegree_arr_vec`.
- Add `Complete` unit test `size_btree_map_btree_set_0`.
- Add `Complete` unit test `size_btree_map_vec_0`.
- Add `Complete` unit test `size_hash_map_hash_set_0`.
- Add `Complete` unit test `size_hash_map_vec_0`.
- Add `Complete` unit test `size_vec_btree_set_0`.
- Add `Complete` unit test `size_vec_hash_set_0`.
- Add `Complete` unit test `size_vec_vec_0`.
- Add `Complete` unit test `indegree_btree_map_btree_set_0`.
- Add `Complete` unit test `indegree_hash_map_hash_set_0`.
- Add `Complete` unit test `indegree_vec_btree_set_0`.
- Add `Complete` unit test `indegree_vec_hash_set_0`.
- Add `Complete` unit test `outdegree_btree_map_btree_set_0`.
- Add `Complete` unit test `outdegree_btree_map_vec_0`.
- Add `Complete` unit test `outdegree_hash_map_hash_set_0`.
- Add `Complete` unit test `outdegree_hash_map_vec_0`.
- Add `Complete` unit test `outdegree_vec_btree_set_0`.
- Add `Complete` unit test `outdegree_vec_hash_set_0`.
- Add `Complete` unit test `outdegree_vec_vec_0`.

Changed

- Breaking: `Outdegree::outdegree` no longer panics if `s` is not in the graph.

## [0.34.1] - 2024-05-05

Added

- Add the `gen::Complete` generator.
- Add the `gen::CompleteConst` generator.

## [0.34.0] - 2024-05-05

Added

- Add documentation alias "circular" for `gen::Cycle`.
- Add documentation alias "circular" for `gen::CycleConst`.

Changed

- Emphasize the focus on directed graphs in the documentation.

Removed

- Remove `gen::Star`.
- Remove `gen::StarConst`.
- Remove `gen::Linear`
- Remove `gen::LinearConst`.
- Remove `op::Target`.

## [0.33.1] - 2024-05-04

Added

- Add `gen::StarConst` generator.
- Add `gen::StarConst` unit test `arr_btree_set`.
- Add `gen::StarConst` unit test `arr_hash_set`.
- Add `gen::StarConst` unit test `arr_vec`.
- Add `gen::StarConst` unit test `is_simple_arr_btree_set`.
- Add `gen::StarConst` unit test `is_simple_arr_hash_set`.
- Implement `StarConst` for `[BTreeSet<usize>; V]`.
- Implement `StarConst` for `[HashSet<usize>; V]`.
- Implement `StarConst` for `[Vec<usize>; V]`.

## [0.33.0] - 2024-05-04

Added

- Add `gen::Star` property test `indegree_btree_map_btree_set_1`.
- Add `gen::Star` property test `indegree_hash_map_hash_set_1`.
- Add `gen::Star` property test `indegree_vec_btree_set_1`.
- Add `gen::Star` property test `indegree_vec_hash_set_1`.
- Add `gen::Star` property test `outdegree_btree_map_btree_set_1`.
- Add `gen::Star` property test `outdegree_btree_map_vec_1`.
- Add `gen::Star` property test `outdegree_hash_map_hash_set_1`.
- Add `gen::Star` property test `outdegree_hash_map_vec_1`.
- Add `gen::Star` property test `outdegree_vec_btree_set_1`.
- Add `gen::Star` property test `outdegree_vec_hash_set_1`.
- Add `gen::Star` property test `outdegree_vec_vec_1`.
- Link to `CHANGELOG.md` from `README.md`.

Changed

- Breaking: Return an empty arc list for the one vertex in `gen::Star::star(1)`.

## [0.32.3] - 2024-05-04

Added

- Add `gen::Star` property test `size_btree_map_btree_set`.
- Add `gen::Star` property test `size_btree_map_vec`.
- Add `gen::Star` property test `size_hash_map_hash_set`.
- Add `gen::Star` property test `size_hash_map_vec`.
- Add `gen::Star` property test `size_vec_btree_set`.
- Add `gen::Star` property test `size_vec_hash_set`.
- Add `gen::Star` property test `size_vec_vec`.
- Add `gen::Star` property test `order_vec_btree_set`.
- Add `gen::Star` property test `order_vec_hash_set`.
- Add `gen::Star` property test `order_vec_vec`.
- Add `gen::Star` property test `indegree_btree_map_btree_set`.
- Add `gen::Star` property test `indegree_hash_map_hash_set`.
- Add `gen::Star` property test `indegree_vec_btree_set`.
- Add `gen::Star` property test `indegree_vec_hash_set`.
- Add `gen::Star` property test `is_simple_vec_btree_set`.
- Add `gen::Star` property test `is_simple_vec_hash_set`.
- Add `gen::Star` property test `is_simple_vec_vec`.
- Add `gen::Star` property test `outdegree_btree_map_btree_set`.
- Add `gen::Star` property test `outdegree_btree_map_vec`.
- Add `gen::Star` property test `outdegree_hash_map_hash_set`.
- Add `gen::Star` property test `outdegree_hash_map_vec`.
- Add `gen::Star` property test `outdegree_vec_btree_set`.
- Add `gen::Star` property test `outdegree_vec_hash_set`.
- Add `gen::Star` property test `outdegree_vec_vec`.
- Add `gen::Star` unit test `btree_map_btree_set`.
- Add `gen::Star` unit test `btree_map_vec`.
- Add `gen::Star` unit test `hash_map_hash_set`.
- Add `gen::Star` unit test `hash_map_vec`.
- Add `gen::Star` unit test `vec_btree_set`.
- Add `gen::Star` unit test `vec_hash_set`.
- Add `gen::Star` unit test `vec_vec`.
- Add documentation alias for `bfs::single_pair_shortest_path`.
- Add documentation alias for `dijkstra::single_pair_shortest_path`.
- Add generator `gen::Star` for star graphs.
- Implement `gen::Star` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `gen::Star` for `BTreeMap<usize, Vec<usize>>`.
- Implement `gen::Star` for `HashMap<usize, HashSet<usize>>`.
- Implement `gen::Star` for `HashMap<usize, Vec<usize>>`.
- Implement `gen::Star` for `Vec<BTreeSet<usize>>`.
- Implement `gen::Star` for `Vec<HashSet<usize>>`.
- Implement `gen::Star` for `Vec<Vec<usize>>`.

## [0.32.2] - 2024-05-02

Changed

- Add diagram to `README` `bfs::single_pair_shortest_path` example.
- Standardize links in documentation to conform to [RFC-1574](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html#appendix-a-full-conventions-text).

## [0.32.1] - 2024-05-02

Changed

- Standardize documentation summary sentences to conform to [RFC-1574](https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html#appendix-a-full-conventions-text).

## [0.32.0] - 2024-05-02

Changed

- Add MIT license.

## [0.31.0] - 2024-05-01

Added

- Add `dijkstra::single_pair_shortest_path`.

Fixed

- Fix `dijkstra::shortest_path`.

## [0.30.9] - 2024-05-01

Added

- Add `Size` unit test `btree_set_tuple_unweighted`.
- Add `Size` unit test `btree_set_tuple_weighted`.
- Add `Size` unit test `hash_set_tuple_unweighted`.
- Add `Size` unit test `hash_set_tuple_weighted`.
- Add `Size` unit test `vec_tuple_unweighted`.
- Add `Size` unit test `vec_tuple_weighted`.
- Add `CycleConst` unit test `arr_tuple`.
- Add `CycleConst` unit test `is_simple_arr_btree_set`.
- Add `CycleConst` unit test `is_simple_arr_hash_set`.
- Add `CycleConst` unit test `is_simple_arr_tuple`.
- Add `Cycle` property test `size_btree_set_tuple`.
- Add `Cycle` property test `size_hash_set_tuple`.
- Add `Cycle` property test `size_vec_tuple`.
- Add `Cycle` property test `is_simple_btree_set_tuple`.
- Add `Cycle` property test `is_simple_hash_set_tuple`.
- Add `Cycle` property test `is_simple_vec_btree_set`.
- Add `Cycle` property test `is_simple_vec_hash_set`.
- Add `Cycle` property test `is_simple_vec_tuple`.
- Add `Cycle` unit test `btree_set_tuple`.
- Add `Cycle` unit test `hash_set_tuple`.
- Add `Cycle` unit test `is_simple_btree_set_tuple_0`.
- Add `Cycle` unit test `is_simple_btree_set_tuple_1`.
- Add `Cycle` unit test `is_simple_hash_set_tuple_0`.
- Add `Cycle` unit test `is_simple_hash_set_tuple_1`.
- Add `Cycle` unit test `is_simple_vec_btree_set_0`.
- Add `Cycle` unit test `is_simple_vec_btree_set_1`.
- Add `Cycle` unit test `is_simple_vec_hash_set_0`.
- Add `Cycle` unit test `is_simple_vec_hash_set_1`.
- Add `Cycle` unit test `is_simple_vec_tuple_0`.
- Add `Cycle` unit test `is_simple_vec_tuple_1`.
- Add `Cycle` unit test `vec_tuple`.
- Add `LinearConst` unit test `is_simple_arr_btree_set`.
- Add `LinearConst` unit test `is_simple_arr_hash_set`.
- Add `Linear` property test `is_simple_vec_btree_set`.
- Add `Linear` property test `is_simple_vec_hash_set`.
- Implement `Size` for `BTreeSet<(usize, usize)>`.
- Implement `Size` for `BTreeSet<(usize, usize, W)>`.
- Implement `Size` for `HashSet<(usize, usize)>`.
- Implement `Size` for `HashSet<(usize, usize, W)>`.
- Implement `Size` for `Vec<(usize, usize)>`.
- Implement `Size` for `Vec<(usize, usize, W)>`.
- Implement `CycleConst` for `[(usize, usize); V]`.
- Implement `Cycle` for `BTreeSet<(usize, usize)>`.
- Implement `Cycle` for `HashSet<(usize, usize)>`.
- Implement `Cycle` for `Vec<(usize, usize)>`.

## [0.30.8] - 2024-04-30

Changed

- Link to `CycleConst` from the `Cycle` documentation.

## [0.30.7] - 2024-04-30

Added

- Implement `CycleConst` for `[BTreeSet<usize>; V]`.
- Implement `CycleConst` for `[HashSet<usize>; V]`.
- Implement `CycleConst` for `[Vec<usize>; V]`.

## [0.30.6] - 2024-04-30

Changed

- Set `opt-level` to `3` for `proptest`.

## [0.30.5] - 2024-04-29

Added

- Add `gen::Cycle` property test `size_btree_map_btree_set`.
- Add `gen::Cycle` property test `size_btree_map_vec`.
- Add `gen::Cycle` property test `size_hash_map_hash_set`.
- Add `gen::Cycle` property test `size_hash_map_vec`.
- Add `gen::Cycle` property test `size_vec_btree_set`.
- Add `gen::Cycle` property test `size_vec_hash_set`.
- Add `gen::Cycle` property test `size_vec_vec`.
- Add `gen::Cycle` property test `indegree_btree_map_btree_set`.
- Add `gen::Cycle` property test `indegree_hash_map_hash_set`.
- Add `gen::Cycle` property test `outdegree_btree_map_btree_set`.
- Add `gen::Cycle` property test `outdegree_btree_map_vec`.
- Add `gen::Cycle` property test `outdegree_hash_map_hash_set`.
- Add `gen::Cycle` property test `outdegree_hash_map_vec`.
- Add `gen::Cycle` unit test `btree_map_btree_set`.
- Add `gen::Cycle` unit test `btree_map_vec`.
- Add `gen::Cycle` unit test `hash_map_hash_set`.
- Add `gen::Cycle` unit test `hash_map_vec`.
- Add `gen::Linear` property test `size_btree_map_btree_set`.
- Add `gen::Linear` property test `size_btree_map_vec`.
- Add `gen::Linear` property test `size_hash_map_hash_set`.
- Add `gen::Linear` property test `size_hash_map_vec`.
- Add `gen::Linear` property test `size_vec_btree_set`.
- Add `gen::Linear` property test `size_vec_hash_set`.
- Add `gen::Linear` property test `size_vec_vec`.
- Add `gen::Linear` property test `order_vec_btree_set`.
- Add `gen::Linear` property test `order_vec_hash_set`.
- Add `gen::Linear` property test `order_vec_vec`.
- Add `gen::Linear` property test `indegree_btree_map_btree_set`.
- Add `gen::Linear` property test `indegree_hash_map_hash_set`.
- Add `gen::Linear` property test `indegree_vec_btree_set`.
- Add `gen::Linear` property test `indegree_vec_hash_set`.
- Add `gen::Linear` property test `outdegree_btree_map_btree_set`.
- Add `gen::Linear` property test `outdegree_btree_map_vec`.
- Add `gen::Linear` property test `outdegree_hash_map_hash_set`.
- Add `gen::Linear` property test `outdegree_hash_map_vec`.
- Add `gen::Linear` property test `outdegree_vec_btree_set`.
- Add `gen::Linear` property test `outdegree_vec_hash_set`.
- Add `gen::Linear` property test `outdegree_vec_vec`.
- Implement `Cycle` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Cycle` for `BTreeMap<usize, Vec<usize>>`.
- Implement `Cycle` for `HashMap<usize, HashSet<usize>>`.
- Implement `Cycle` for `HashMap<usize, Vec<usize>>`.
- Implement `ArcWeight` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `ArcWeight` for `Vec<BTreeMap<usize, W>>`.
- Implement `ArcWeight` for `[BTreeMap<usize, W>; V]`.
- Implement `ArcWeight` for `[BTreeMap<usize, W>]`.
- Implement `Linear` for `HashMap<usize, HashSet<usize>>`.
- Implement `Linear` for `HashMap<usize, Vec<usize>>`.

## [0.30.4] - 2024-04-29

Added

- Add `gen::Cycle` property test `order_vec_vec`.
- Add `gen::Cycle` property test `order_vec_btree_set`.
- Add `gen::Cycle` property test `order_vec_hash_set`.
- Add `gen::Cycle` property test `indegree_vec_btree_set`.
- Add `gen::Cycle` property test `indegree_vec_hash_set`.
- Add `gen::Cycle` property test `outdegree_vec_vec`.
- Add `gen::Cycle` property test `outdegree_vec_btree_set`.
- Add `gen::Cycle` property test `outdegree_vec_hash_set`.

## [0.30.3] - 2024-04-29

Changed

- Clean up examples in `README`.

## [0.30.2] - 2024-04-29

Changed

- Add more extensive examples to `README`.
- Simplify `strategy::binop_vertices` implementation.

## [0.30.1] - 2024-04-28

Fixed

- Fix `prop::strategy::binop_vertices` when `v` is zero inside `flat_map`.

## [0.30.0] - 2024-04-28

Removed

- Breaking: Remove `prop::remove_arc_is_arc`.

## [0.29.1] - 2024-04-28

Added

- Add `add_weighted_arc_is_arc` property test `btree_map_btree_set`.
- Add `add_weighted_arc_is_arc` property test `hash_map_hash_set`.
- Add `add_weighted_arc_is_arc` property test `slice_btree_set`.
- Add `add_weighted_arc_is_arc` property test `slice_hash_set`.
- Add `add_weighted_arc_is_arc` property test `vec_btree_map`.
- Add `add_weighted_arc_is_arc` property test `vec_hash_map`.
- Add `add_weighted_arc_is_arc` unit test `arr_btree_map`.
- Add `add_weighted_arc_is_arc` unit test `arr_hash_map`.
- Add `add_weighted_arc_remove_arc` property test `btree_map_btree_map`.
- Add `add_weighted_arc_remove_arc` property test `hash_map_hash_map`.
- Add `add_weighted_arc_remove_arc` property test `slice_btree_map`.
- Add `add_weighted_arc_remove_arc` property test `slice_hash_map`.
- Add `add_weighted_arc_remove_arc` property test `vec_btree_map`.
- Add `add_weighted_arc_remove_arc` property test `vec_hash_map`.
- Add `add_weighted_arc_remove_arc` unit test `arr_btree_map`.
- Add `add_weighted_arc_remove_arc` unit test `arr_hash_map`.

Changed

- Loosen bound on `G` in `add_weighted_arc_is_arc` to `?Sized`.

Fixed

- Fix `prop::strategy::binop_vertices` when `v` is zero.

Removed

- Remove `add_weighted_arc_is_arc` unit test `vec_hash_map`.
- Remove `add_weighted_arc_remove_arc` unit test `vec_hash_map`.
- Rename `prop::strategy::v_s_t` to `prop::strategy::binop_vertices`.

## [0.29.0] - 2024-04-28

Added

- Add `add_arc_is_arc` property test `btree_map_btree_set`.
- Add `add_arc_is_arc` property test `hash_map_hash_set`.
- Add `add_arc_is_arc` property test `slice_btree_set`.
- Add `add_arc_is_arc` property test `slice_hash_set`.
- Add `add_arc_is_arc` property test `vec_btree_set`.
- Add `add_arc_is_arc` property test `vec_hash_set`.
- Add `add_arc_is_arc` unit test `arr_btree_set`.
- Add `add_arc_remove_arc` property test `vec_btree_set`.
- Add `add_arc_remove_arc` property test `vec_hash_set`.
- Add `add_arc_remove_arc` unit test `arr_btree_set`.
- Add `prop::strategy::v_s_t` to order, source, and target.
- Add `proptest` as a dev-dependency.

Changed

- Breaking: Move `op::prop` to `prop`.

Removed

- Remove `add_arc_is_arc` unit test `hash_map_hash_set`.
- Remove `add_arc_is_arc` unit test `hash_map_hash_set`.
- Remove `add_arc_is_arc` unit test `vec_hash_set`.
- Remove `add_arc_remove_arc` unit test `vec_hash_set`.

## [0.28.2] - 2024-04-28

Added

- Add `gen::LinearConst` generator.
- Implement `LinearConst` for `[BTreeSet<usize>; V]`.
- Implement `LinearConst` for `[HashSet<usize>; V]`.
- Implement `LinearConst` for `[Vec<usize>; V]`.
- Implement `Linear` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Linear` for `BTreeMap<usize, Vec<usize>>`.

Fixed

- Fix comments in `gen::Linear` test function bodies.

## [0.28.1] - 2024-04-27

Added

- Add `gen::Cycle` generator.

## [0.28.0] - 2024-04-27

Added

- Add `algo::dijkstra::shortest_path`.
- Add `bench` job to GHA workflow.
- Bench `add_arc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Bench `add_arc` for `BTreeMap<usize, Vec<usize>>`.
- Bench `add_arc` for `Vec<BTreeSet<usize>>`.
- Bench `add_arc` for `[BTreeSet<usize>; V]`.
- Bench `size` for `Vec<BTreeMap<usize, usize>>`.

Changed

- Breaking: Return immediately in `bfs::shortest_path` when it finds the target before pushing the target to the queue.
- Compress textual diagrams.
- Cross-link `bfs` and `dijkstra` in module documentation.
- Document panics in `bfs` and `dijkstra`.
- Link to the `op` module inside the documentation of the `repr` module.
- Remove `min_time` from benches.

## [0.27.1] - 2024-04-26

Changed

- Check the top changelog entry version number on pre-commit.

## [0.27.0] - 2024-04-26

Added

- Add `op::Target`, a trait to get the target vertex of an adjacency list arc.

Changed

- Replace `Linear for G: AddArc + Empty` with `Linear for Vec<BTreeSet<usize>>`.
- Replace `Linear for G: AddArc + Empty` with `Linear for Vec<HashSet<usize>>`.
- Replace `Linear for G: AddArc + Empty` with `Linear for Vec<Vec<usize>>`.

Removed

- Breaking: Remove `gen::Empty`.

## [0.26.1] - 2024-04-26

Added

- Add `gen::Empty`, a generator for empty graphs.
- Implement `Linear` for `G: AddArc + Empty`.
- Implement `Empty` for `Vec<T: Clone + Default + IntoIterator<Item = usize>>`.

Changed

- Replace `Linear for Vec<BTreeSet<usize>>` with `Linear for G: AddArc + Empty`.
- Replace `Linear for Vec<HashSet<usize>>` with `Linear for G: AddArc + Empty`.
- Replace `Linear for Vec<Vec<usize>>` with `Linear for G: AddArc + Empty`.

## [0.26.0] - 2024-04-25

Changed

- Breaking: Remove the `nightly` feature, as only `adjacency_matrix` depends on it.
- Simplify `README`.

## [0.25.2] - 2024-04-25

Changed

- Split up GHA workflow into multiple jobs.

## [0.25.1] - 2024-04-25

Changed

- Replace `saturating_sub` with `-` in `gen::linear`.

## [0.25.0] - 2024-04-24

Added

- Implement `Linear` for `BTreeSet<usize>`.
- Implement `Linear` for `HashSet<usize>`.

Fixed

- Fix `Linear::linear`.

## [0.24.1] - 2024-04-24

Added

- Add a documentation test for `gen::Linear`.

## [0.24.0] - 2024-04-23

Added

- Add `gen::Linear`, a generator for linear graphs.

Changed

- `IsArc` for `[BTreeMap<usize, W>]` no longer panics if `s` is not in the graph.
- `IsArc` for `[BTreeSet<usize>]` no longer panics if `s` is not in the graph.
- `IsArc` for `[HashMap<usize, W>]` no longer panics if `s` is not in the graph.
- `IsArc` for `[HashSet<usize>]` no longer panics if `s` is not in the graph.

## [0.23.1] - 2024-04-23

Added

- Add more tests for `bfs::single_pair_shortest_path`.
- Mention `bfs::single_pair_shortest_path` in the module documentation of `bfs`.

## [0.23.0] - 2024-04-22

Added

- Add `bfs::single_pair_shortest_path`.

Changed

- Breaking: Remove `?Sized` bound from `graph` in `bfs::distances`.
- Breaking: Remove `?Sized` bound from `graph` in `bfs::predecessors`.
- Change `is_target` to `impl Fn(W) -> bool` in `bfs::shortest_path`.
- Change `step` type to `impl Fn(W) -> W` in `bfs::distances`.
- Change `step` type to `impl Fn(W) -> W` in `bfs::predecessors`.
- Change `step` type to `impl Fn(W) -> W` in `bfs::shortest_path`.
- Change `step` type to `impl Fn(W) -> W` in `dijkstra::distances`.
- Change `step` type to `impl Fn(W) -> W` in `dijkstra::predecessors`.
- Update `README.md` example to use `bfs::single_pair_shortest_path`.

## [0.22.1] - 2024-04-21

Changed

- Move module documentation into modules.
- Use `HashSet` instead of `AdjacencyMatrix` in the `lib` test.
- Add `Cargo.toml` to `.gitignore`.

## [0.22.0] - 2024-04-21

Changed

- Breaking: Disable `adjacency_matrix` by default to support stable Rust.

## [0.21.0] - 2024-04-21

Added

- Add a `nightly` feature.

Changed

- Breaking: Make the `adjacency_matrix` depend on the `nightly` feature.
- Breaking: Enable `adjacency_matrix` by default.
- Replace `assert_matches!(...)` with `assert!(matches!(...))`.

## [0.20.2] - 2024-04-21

Added

- Add `pre-commit` configuration.

## [0.20.1] - 2024-04-21

Changed

- Derive `Hash` for `AdjacencyMatrix`.
- Derive `Ord` for `AdjacencyMatrix`.

## [0.20.0] - 2024-04-20

Added

- Add `predecessor::search_by`.
- Add `bfs::shortest_path`.

Changed

- Breaking: Rename `bfs::distances_single_source` to `bfs::single_source_distances`.
- Breaking: Rename `bfs::predecessors_single_source` to `bfs::single_source_predecessors`.
- Breaking: Rename `dijkstra::distances_single_source` to `dijkstra::single_source_distances`.
- Breaking: Rename `dijkstra::predecessors_single_source` to `dijkstra::single_source_predecessors`.
- Breaking: `predecessor::search` returns a singleton path if the target is the source.

## [0.19.0] - 2024-04-20

Changed

- Breaking: `bfs::predecessors_single_source` only returns the predecessor tree.
- Breaking: `dijkstra::predecessors_single_source` only returns the predecessor tree.

## [0.18.0] - 2024-04-20

Changed

- Breaking: `repr::AdjacenyMatrix` is a feature.

## [0.17.2] - 2024-04-19

Added

- Add `predecessor::search`.

## [0.17.1] - 2024-04-19

Changed

- Move the list of standard library graph representations from `README` to `op/mod.rs`.

## [0.17.0] - 2024-04-18

Added

- Implement `Indegree` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `Indegree` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Indegree` for `Vec<BTreeMap<usize, W>>`.
- Implement `Indegree` for `Vec<BTreeSet<usize>>`.
- Implement `Indegree` for `[BTreeMap<usize, W>; V]`.
- Implement `Indegree` for `[BTreeMap<usize, W>]`.
- Implement `Indegree` for `[BTreeSet<usize>; V]`.
- Implement `Indegree` for `[BTreeSet<usize>]`.
- Implement `IterVertices` for `Vec<BTreeMap<usize, W>>`.
- Implement `IterVertices` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `IterVertices` for `Vec<BTreeSet<usize>>`.
- Implement `IterVertices` for `Vec<HashMap<usize, W>>`.
- Implement `IterVertices` for `Vec<HashSet<(usize, W)>>`.
- Implement `IterVertices` for `Vec<HashSet<usize>>`.
- Implement `IterVertices` for `Vec<Vec<(usize, W)>>`.
- Implement `IterVertices` for `Vec<Vec<usize>>`.
- Implement `IterVertices` for `[BTreeMap<usize, W>; V]`.
- Implement `IterVertices` for `[BTreeMap<usize, W>]`.
- Implement `IterVertices` for `[BTreeSet<(usize, W); V]`.
- Implement `IterVertices` for `[BTreeSet<(usize, W)]`.
- Implement `IterVertices` for `[BTreeSet<usize>; V]`.
- Implement `IterVertices` for `[BTreeSet<usize>]`.
- Implement `IterVertices` for `[HashMap<usize, W>; V]`.
- Implement `IterVertices` for `[HashMap<usize, W>]`.
- Implement `IterVertices` for `[HashSet<(usize, W); V]`.
- Implement `IterVertices` for `[HashSet<(usize, W)]`.
- Implement `IterVertices` for `[HashSet<usize>; V]`.
- Implement `IterVertices` for `[HashSet<usize>]`.
- Implement `IterVertices` for `[Vec<(usize, W)>; V]`.
- Implement `IterVertices` for `[Vec<(usize, W)>]`.
- Implement `IterVertices` for `[Vec<usize>; V]`.
- Implement `IterVertices` for `[Vec<usize>]`.

Removed

- Breaking: Remove `IterVertices` for `Vec<T>`.
- Breaking: Remove `IterVertices` for `[T]`.
- Breaking: Remove `IterVertices` for `[T; V]`.

Fixed

- Remove unused parameter `W` from `remove_arc_is_arc`.

## [0.16.1] - 2024-04-17

Added

- Implement `RemoveArc` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `RemoveArc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `RemoveArc` for `Vec<BTreeMap<usize, W>>`.
- Implement `RemoveArc` for `Vec<BTreeSet<usize>>`.
- Implement `RemoveArc` for `[BTreeMap<usize, W>; V]`.
- Implement `RemoveArc` for `[BTreeMap<usize, W>]`.
- Implement `RemoveArc` for `[BTreeSet<usize>; V]`.
- Implement `RemoveArc` for `[BTreeSet<usize>]`.

Changed

- `RemoveArc::remove_arc` returns a `bool` indicating whether it removed the arc.

## [0.16.0] - 2024-04-17

Added

- Implement `Outdegree` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `Outdegree` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `Outdegree` for `BTreeMap<usize, Vec<usize>>`.
- Implement `Outdegree` for `Vec<BTreeMap<usize, W>>`.
- Implement `Outdegree` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `Outdegree` for `Vec<BTreeSet<usize>>`.
- Implement `Outdegree` for `Vec<HashMap<usize, W>>`.
- Implement `Outdegree` for `Vec<HashSet<(usize, W)>>`.
- Implement `Outdegree` for `Vec<HashSet<usize>>`.
- Implement `Outdegree` for `Vec<Vec<(usize, W)>>`.
- Implement `Outdegree` for `Vec<Vec<usize>>`.
- Implement `Outdegree` for `[BTreeMap<usize, W>; V]`.
- Implement `Outdegree` for `[BTreeMap<usize, W>]`.
- Implement `Outdegree` for `[BTreeSet<(usize, W)>; V]`.
- Implement `Outdegree` for `[BTreeSet<(usize, W)>]`.
- Implement `Outdegree` for `[BTreeSet<usize>; V]`.
- Implement `Outdegree` for `[BTreeSet<usize>]`.
- Implement `Outdegree` for `[HashMap<usize, W>; V]`.
- Implement `Outdegree` for `[HashMap<usize, W>]`.
- Implement `Outdegree` for `[HashSet<(usize, W)>; V]`.
- Implement `Outdegree` for `[HashSet<(usize, W)>]`.
- Implement `Outdegree` for `[HashSet<usize>]; V`.
- Implement `Outdegree` for `[HashSet<usize>]`.
- Implement `Outdegree` for `[Vec<(usize, W)>; V]`.
- Implement `Outdegree` for `[Vec<(usize, W)>]`.
- Implement `Outdegree` for `[Vec<usize>; V]`.
- Implement `Outdegree` for `[Vec<usize>]`.

Changed

- Breaking: Remove `Outdegree` for `Vec<HashMap<K, W>>`.
- Breaking: Remove `Outdegree` for `Vec<HashSet<T>>`.
- Breaking: Remove `Outdegree` for `Vec<Vec<T>>`.
- Breaking: Remove `Outdegree` for `[HashMap<K, W>; V]`.
- Breaking: Remove `Outdegree` for `[HashMap<K, W>]`.
- Breaking: Remove `Outdegree` for `[HashSet<T>; V]`.
- Breaking: Remove `Outdegree` for `[HashSet<T>]`.
- Breaking: Remove `Outdegree` for `[Vec<T>; V]`.
- Breaking: Remove `Outdegree` for `[Vec<T>]`.

## [0.15.1] - 2024-04-17

Added

- Implement `IterWeightedArcs` for `Vec<BTreeMap<usize, W>>`.
- Implement `IterWeightedArcs` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `[BTreeMap<usize, W>; V]`.
- Implement `IterWeightedArcs` for `[BTreeMap<usize, W>]`.
- Implement `IterWeightedArcs` for `[BTreeSet<(usize, W)>; V]`.
- Implement `IterWeightedArcs` for `[BTreeSet<(usize, W)>]`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `IterWeightedArcs` for `BTreeMap<usize, BTreeMap<usize, W>>`.

## [0.15.0] - 2024-04-16

Removed

- Breaking: Remove `IterVertices` for `HashMap<_>`.

## [0.14.2] - 2024-04-16

Added

- Implement `IterOutNeighbors` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IterOutNeighbors` for `BTreeMap<usize, Vec<usize>>`.
- Implement `IterOutNeighbors` for `Vec<BTreeSet<usize>>`.
- Implement `IterOutNeighbors` for `[BTreeSet<usize>; V]`.
- Implement `IterOutNeighbors` for `[BTreeSet<usize>]`.

Fixes

- Fix benchmark imports.

## [0.14.1] - 2024-04-16

Added

- Implement `IsSimple` for `Vec<BTreeSet<usize>>`.
- Implement `IsSimple` for `[BTreeSet<usize>]`.
- Implement `IsSimple` for `[BTreeSet<usize>; V]`.
- Implement `IsSimple` for `BTreeSet<(usize, usize)>`.
- Implement `IsSimple` for `BTreeSet<(usize, usize, W)>`.
- Implement `IterArcs` for `BTreeSet<(usize, usize)>`.
- Implement `IterWeightedArcs` for `BTreeSet<(usize, usize, W)>`.

## [0.14.0] - 2024-04-15

Changed

- Breaking: Rename `bfs::min_distances` to `bfs::distances`.
- Breaking: Rename `bfs::min_distances_single_source` to `bfs::distances_single_source`.
- Breaking: Rename `dijkstra::min_distances` to `dijkstra::distances`.
- Breaking: Rename `dijkstra::min_distances_single_source` to `dijkstra::distances_single_source`.

## [0.13.3] - 2024-04-15

Added

- Add logo to `README`.

## [0.13.2] - 2024-04-15

Added

- Implement `Order` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `Order` for `Vec<HashSet<(usize, W)>>`.
- Implement `Order` for `[BTreeSet<(usize, W)>; V]`.
- Implement `Order` for `[BTreeSet<(usize, W)>]`.
- Implement `Order` for `[HashSet<(usize, W); V]`.
- Implement `Order` for `[HashSet<(usize, W)]`.

Fixed

- Fix `crosscountry` benchmark.
- Fix `shortestpath1` benchmark.
- Fix `small_graph_1` benchmark.

## [0.13.1] - 2024-04-15

Added

- Implement `IsArc` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `IsArc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `IsArc` for `BTreeSet<(usize, usize)>`.
- Implement `IsArc` for `Vec<BTreeMap<usize, W>>`.
- Implement `IsArc` for `Vec<BTreeSet<usize>>`.
- Implement `IsArc` for `[BTreeMap<usize, W>; V]`.
- Implement `IsArc` for `[BTreeMap<usize, W>]`.
- Implement `IsArc` for `[BTreeSet<usize>; V]`.
- Implement `IsArc` for `[BTreeSet<usize>]`.

## [0.13.0] - 2024-04-14

Added

- Implement `Order` for `Vec<BTreeMap<usize, W>>`.
- Implement `Order` for `Vec<BTreeSet<usize>>`.
- Implement `Order` for `Vec<HashMap<usize, W>>`.
- Implement `Order` for `Vec<HashSet<usize>>`.
- Implement `Order` for `Vec<Vec<(usize, W)>>`.
- Implement `Order` for `Vec<Vec<usize>>`.
- Implement `Order` for `[BTreeMap<usize, W>; V]`.
- Implement `Order` for `[BTreeMap<usize, W>]`.
- Implement `Order` for `[BTreeSet<usize>; V]`.
- Implement `Order` for `[BTreeSet<usize>]`.
- Implement `Order` for `[HashMap<usize, W>; V]`.
- Implement `Order` for `[HashMap<usize, W>]`.
- Implement `Order` for `[HashSet<usize>; V]`.
- Implement `Order` for `[HashSet<usize>]`.
- Implement `Order` for `[Vec<(usize, W)>; V]`.
- Implement `Order` for `[Vec<(usize, W)>]`.
- Implement `Order` for `[Vec<usize>; V]`.
- Implement `Order` for `[Vec<usize>]`.

Removed

- Breaking: Remove `Order` for `HashMap<_>`.
- Breaking: Remove `Order` for `Vec<T>`.
- Breaking: Remove `Order` for `[T; V]`.
- Breaking: Remove `Order` for `[T]`.

## [0.12.1] - 2024-04-14

Added

- Implement `Size` for `BTreeMap<K, BTreeMap<K, W>>`.
- Implement `Size` for `BTreeMap<K, BTreeSet<T>>`.
- Implement `Size` for `BTreeMap<K, Vec<T>>`.
- Implement `Size` for `Vec<BTreeMap<K, W>>`.
- Implement `Size` for `Vec<BTreeSet<T>>`.
- Implement `Size` for `[BTreeMap<K, W>; V]`.
- Implement `Size` for `[BTreeMap<K, W>]`.
- Implement `Size` for `[BTreeSet<T>; V]`.
- Implement `Size` for `[BTreeSet<T>]`.
- Implement `IsArc` for `Vec<HashMap<usize, W>>`.
- Implement `IsArc` for `[HashMap<usize, W>; V]`.

Removed

- Remove `Ord` bound from trait implementations where possible.

## [0.12.0] - 2024-04-14

Added

- Implement `AddWeightedArc` for `BTreeMap<usize, BTreeMap<usize, W>>`.
- Implement `AddWeightedArc` for `BTreeMap<usize, BTreeSet<(usize, W)>>`.
- Implement `AddWeightedArc` for `BTreeMap<usize, Vec<(usize, W)>>`.
- Implement `AddWeightedArc` for `Vec<BTreeMap<usize, W>>`.
- Implement `AddWeightedArc` for `Vec<BTreeSet<(usize, W)>>`.
- Implement `AddWeightedArc` for `[BTreeMap<usize, W>; V]`.
- Implement `AddWeightedArc` for `[BTreeMap<usize, W>]`.
- Implement `AddWeightedArc` for `[BTreeSet<(usize, W)>; V]`.
- Implement `AddWeightedArc` for `[BTreeSet<(usize, W)>]`.

Changed

- Breaking: `AddWeightedArc` for `HashMap<_>` panics if `s` is not in the graph.

## [0.11.1] - 2024-04-14

Added

- Implement `AddArc` for `BTreeMap<usize, BTreeSet<usize>>`.
- Implement `AddArc` for `BTreeMap<usize, Vec<usize>>`.
- Implement `AddArc` for `Vec<BTreeSet<usize>>`.
- Implement `AddArc` for `[BTreeSet<usize>; V]`.
- Implement `AddArc` for `[BTreeSet<usize>]`.

## [0.11.0] - 2024-04-14

Added back the `op` implementations for `Vec` and arrays to simplify use cases.

Added

- Implement `AddArc` for `Vec<HashSet<usize>>`.
- Implement `AddArc` for `Vec<Vec<usizee>>`.
- Implement `AddArc` for `[HashSet<usize>; V]`.
- Implement `AddArc` for `[Vec<usize>; V]`.
- Implement `AddWeightedArc` for `Vec<HashMap<usize, W>>`.
- Implement `AddWeightedArc` for `Vec<HashSet<(usize, W)>>`.
- Implement `AddWeightedArc` for `Vec<Vec<(usize, W)>>`.
- Implement `AddWeightedArc` for `[HashMap<usize, W>; V]`.
- Implement `AddWeightedArc` for `[HashSet<(usize, W)>; V]`.
- Implement `AddWeightedArc` for `[Vec<(usize, W)>; V]`.
- Implement `Size` for `Vec<HashMap<K, W>>`.
- Implement `Size` for `Vec<HashSet<T>>`.
- Implement `Size` for `Vec<Vec<T>>`.
- Implement `Size` for `[HashMap<K, W>; V]`.
- Implement `Size` for `[HashSet<T>; V]`.
- Implement `Size` for `[Vec<T>; V]`.
- Implement `ArcWeight` for `Vec<HashMap<usize, W>>`.
- Implement `ArcWeight` for `[HashMap<usize, W>; V]`.
- Implement `Indegree` for `Vec<HashMap<usize, W>>`.
- Implement `Indegree` for `Vec<HashSet<usize>>`.
- Implement `Indegree` for `[HashMap<usize, W>; V]`.
- Implement `Indegree` for `[HashSet<usize>; V]`.
- Implement `IsArc` for `Vec<HashSet<usize>>`.
- Implement `IsArc` for `[HashMap<usize, W>; V]`.
- Implement `IsSimple` for `Vec<(usize, usize)>`.
- Implement `IsSimple` for `Vec<(usize, usize, W)>`.
- Implement `IsSimple` for `Vec<HashSet<usize>>`.
- Implement `IsSimple` for `[(usize, usize); V]`.
- Implement `IsSimple` for `[(usize, usize, W); V]`.
- Implement `IsSimple` for `[HashSet<usize>; V]`.
- Implement `IterArcs` for `Vec<(usize, usize)>`.
- Implement `IterArcs` for `[(usize, usize); V]`.
- Implement `IterWeightedArcs` for `Vec<(usize, usize, W)>`.
- Implement `IterWeightedArcs` for `[(usize, usize, W); V]`.
- Implement `IterVertices` for `Vec<T>`.
- Implement `IterVertices` for `[T; V]`.
- Implement `Outdegree` for `Vec<HashMap<K, W>>`.
- Implement `Outdegree` for `Vec<HashSet<T>>`.
- Implement `Outdegree` for `Vec<Vec<T>>`.
- Implement `Outdegree` for `[HashMap<K, W>; V]`.
- Implement `Outdegree` for `[HashSet<T>; V]`.
- Implement `Outdegree` for `[Vec<T>; V]`.
- Implement `RemoveArc` for `Vec<HashMap<usize, W>>`.
- Implement `RemoveArc` for `Vec<HashSet<usize>>`.
- Implement `RemoveArc` for `[HashMap<usize, W>; V]`.
- Implement `RemoveArc` for `[HashSet<usize>; V]`.
- Test `add_arc_remove_arc` for `AdjacencyMatrix`.
- Test `add_arc_remove_arc` for `Vec<HashSet<usize>>`.
- Test `add_arc_remove_arc` for `[HashSet<usize>; V]`.

Fixed

- Remove stray `W` type parameter in `add_arc_remove_arc`.

## [0.10.0] - 2024-04-12

Added

- Implement `IsSimple` for `AdjacencyMatrix`.

Fixed

- `IsSimple` checks for parallel arcs in `HashSet<(usize, usize, W)>`.

## [0.9.0] - 2024-04-12

Fixed

- `IsSimple` checks for parallel arcs in `HashSet<(usize, usize)>`.
- `IsSimple` checks for parallel arcs in `[(usize, usize)]`.
- `IsSimple` checks for parallel arcs in `[(usize, usize, W)]`.

## [0.8.4] - 2024-04-11

Fixed

- Add missing property `add_weighted_arc_remove_arc`.

## [0.8.3] - 2024-04-11

Added

- Test `add_arc_is_arc` for implementors of `AddArc` and `IsArc`.

## [0.8.2] - 2024-04-10

Added

- Implement `IsSimple` for `HashSet<(usize, usize)>`.
- Implement `IsSimple` for `HashSet<(usize, usize, W)>`.
- Implement `IsSimple` for `[(usize, usize)]`.
- Implement `IsSimple` for `[(usize, usize, W)]`.
- Implement `IsSimple` for `[HashSet<usize>]`.

## [0.8.1] - 2024-04-10

Added

- Add `IsSimple` trait.
- Implement `IsSimple` for `[HashSet<usize>]`.

Removed

- Remove `ArcWeight` for `Vec<HashMap<usize, W>>`.
- Remove `ArcWeight` for `[HashMap<usize, W>; V]`.
- Remove `IterVertices` for `HashSet<T>`.

## [0.8.0] - 2024-04-09

Added

- Add installation instructions to `README`.
- Add example usage to `README`.

Removed

- Remove `AddArc` for `Vec<HashSet<T>>`.
- Remove `AddArc` for `Vec<Vec<T>>`.
- Remove `AddArc` for `[HashSet<T>; V]`.
- Remove `AddArc` for `[Vec<T>; V]`.
- Remove `AddWeightedArc` for `Vec<HashMap<usize, W>>`.
- Remove `AddWeightedArc` for `Vec<HashSet<(usize, W)>>`.
- Remove `AddWeightedArc` for `Vec<Vec<(usize, W)>>`.
- Remove `AddWeightedArc` for `[HashMap<usize, W>; V]`.
- Remove `AddWeightedArc` for `[HashSet<(usize, W)>; V]`.
- Remove `AddWeightedArc` for `[Vec<(usize, W)>; V]`.

Changed

- Change `iter_arcs` return type to `impl Iterator<Item = (usize, usize)>`.
- Change `iter_weighted_arcs` return type to `impl Iterator<Item = (usize, usize, &W)>`.
- Change `iter_weighted_arcs` return type to `impl Iterator<Item = (usize, &W)>`.

## [0.7.0] - 2024-04-07

Added

- Implement `Size` for `[HashMap<K, W>]`.
- Implement `Size` for `[HashSet<T>]`.
- Implement `Size` for `[Vec<T>]`.
- Implement `Order` for `[T]`.
- Implement `ArcWeight` for `[HashMap<usize, W>]`.
- Implement `Indegree` for `[HashMap<usize, W>]`.
- Implement `Indegree` for `[HashSet<usize>]`.
- Implement `IsArc` for `[HashMap<usize, W>]`.
- Implement `IsArc` for `[HashSet<usize>]`.
- Implement `IterArcs` for `[(usize, usize)]`.
- Implement `IterWeightedArcs` for `[(usize, usize, W)]`.
- Implement `IterOutNeighbors` for `[HashSet<usize>]`.
- Implement `IterOutNeighbors` for `[Vec<usize>]`.
- Implement `IterVertices` for `&[T]`.
- Implement `IterWeightedArcs` for `[HashMap<usize, W>]`.
- Implement `IterWeightedArcs` for `[HashSet<(usize, W)>]`.
- Implement `IterWeightedArcs` for `[Vec<(usize, W)>]`.
- Implement `Outdegree` for `[HashMap<K, W>]`.
- Implement `Outdegree` for `[HashSet<T>]`.
- Implement `Outdegree` for `[Vec<T>]`.
- Implement `RemoveArc` for `[HashMap<usize, W>]`.
- Implement `RemoveArc` for `[HashSet<usize>]`.

Changed

- Return `(&'a usize, &'a W)` from `iter_weighted_arcs`.

Removed

- Remove `Size` for `Vec<HashMap<K, W>>`.
- Remove `Size` for `Vec<HashSet<T>>`.
- Remove `Size` for `Vec<Vec<T>>`.
- Remove `Size` for `[HashMap<K, W>; V]`.
- Remove `Size` for `[HashSet<T>; V]`.
- Remove `Size` for `[Vec<T>; V]`.
- Remove `Indegree` for `Vec<HashMap<usize, W>>`.
- Remove `Indegree` for `Vec<HashSet<usize>>`.
- Remove `Indegree` for `[HashMap<usize, W>; V]`.
- Remove `Indegree` for `[HashSet<usize>; V]`.
- Remove `IsArc` for `Vec<HashMap<usize, W>>`.
- Remove `IsArc` for `Vec<HashSet<usize>>`.
- Remove `IsArc` for `[HashMap<usize, W>; V]`.
- Remove `IsArc` for `[HashSet<usize>; V]`.
- Remove `IterArcs` for `Vec<(usize, usize)>`.
- Remove `IterArcs` for `[(usize, usize); V]`.
- Remove `IterWeightedArcs` for `Vec<(usize, usize, W)>`.
- Remove `IterWeightedArcs` for `[(usize, usize, W); V]`.
- Remove `IterVertices` for `Vec<T>`.
- Remove `IterVertices` for `[T; V]`.
- Remove `Outdegree` for `Vec<HashMap<usize, W>>`.
- Remove `Outdegree` for `Vec<HashSet<usize>>`.
- Remove `Outdegree` for `Vec<Vec<T>>`.
- Remove `Outdegree` for `[HashMap<usize, W>; V]>`.
- Remove `Outdegree` for `[HashSet<usize>; V]>`.
- Remove `Outdegree` for `[Vec<T>; V]`.
- Remove `RemoveArc` for `Vec<HashMap<usize, W>>`.
- Remove `RemoveArc` for `Vec<HashSet<usize>>`.
- Remove `RemoveArc` for `[HashMap<usize, W>; V]`.
- Remove `RemoveArc` for `[HashSet<usize>; V]`.

## [0.6.3] - 2024-04-06

Fix

- Fix `README` formatting.

## [0.6.2] - 2024-04-06

Added

- Add more tests to `algo::bfs`.
- Add more tests to `algo::dijkstra`.
- Implement `AddArc` for `[HashSet<usize>]`.
- Implement `AddArc` for `[Vec<usize>]`.
- Implement `AddWeightedArc` for `[HashMap<usize, W>]`.
- Implement `AddWeightedArc` for `[HashSet<(usize, W)>]`.
- Implement `AddWeightedArc` for `[Vec<(usize, W)>]`.
- Implement `IterArcs` for `[(usize, usize)]`.
- Implement `IterWeightedArcs` for `[(usize, usize, W)]`.

## [0.6.1] - 2024-04-06

Added

- Add "algorithms" and "mathematics" to `Cargo.toml` categories.
- Add "bfs" and "dijkstra" to `Cargo.toml` keywords.

Removed

- Remove redundant `homepage` metadata.

## [0.6.0] - 2024-04-06

Added

- Add `authors` to `README.md`.
- Add a documentation test for `AdjacencyMatrix::new`.
- Add a documentation test for `AdjacencyMatrix::toggle`.
- Add implementation documentation for `AddArc`.
- Add implementation documentation for `AddWeightedArc`.
- Add implementation documentation for `Size`.
- Add implementation documentation for `Order`.
- Add implementation documentation for `ArcWeight`.
- Add implementation documentation for `Indegree`.
- Add implementation documentation for `IsArc`.
- Add implementation documentation for `IterArcs`.
- Add implementation documentation for `IterWeightedArcs`.
- Add implementation documentation for `IterOutNeighbors`.
- Add implementation documentation for `IterWeightedArcs`.
- Add implementation documentation for `Outdegree`.
- Add implementation documentation for `RemoveArc`.

Changed

- Move `ops` to `op`.
- Adapt benchmark code to linting rules.
- Move the documentation test trait properties to `op::prop`.
- Move the lints from `lib.rs` to `Cargo.toml`.

## [0.5.3] - 2024-04-05

Added

- Add a documentation test for `op::add_weighted_arc::AddWeightedArc`.
- Add a documentation test for `op::size::Size`.
- Add a documentation test for `op::order::Order`.
- Add a documentation test for `op::arc_weight::ArcWeight`.
- Add a documentation test for `op::indegree::Indegree`.
- Add a documentation test for `op::is_arc::IsArc`.
- Add a documentation test for `op::iter_arcs::IterArcs`.
- Add a documentation test for `op::iter_weighted_arcs::IterWeightedArcs`.
- Add a documentation test for `op::iter_out_neighbors::IterOutNeighbors`.
- Add a documentation test for `op::iter_vertices::IterVertices`.
- Add a documentation test for `op::iter_weighted_arcs::IterWeightedArcs`.
- Add a documentation test for `op::outdegree::OutDegree`.
- Add a documentation test for `op::remove_arc::RemoveArc`.
- Add documentation for `op::add_weighted_arc`.
- Add documentation for `op::size`.
- Add documentation for `op::order`.
- Add documentation for `op::arc_weight`.
- Add documentation for `op::indegree`.
- Add documentation for `op::is_arc`.
- Add documentation for `op::iter_arcs`.
- Add documentation for `op::iter_weighted_arcs`.
- Add documentation for `op::iter_out_neighbors`.
- Add documentation for `op::iter_vertices`.
- Add documentation for `op::iter_weighted_arcs`.
- Add documentation for `op::outdegree`.
- Add documentation for `op::remove_arc`.

## [0.5.2] - 2024-04-04

Added

- Add a documentation test for `op::add_arc::AddArc`.
- Add documentation for `op::add_arc`.
- Add a module-level documentation test for `algo::bfs`.
- Add a module-level documentation test for `algo::dijkstra`.

## [0.5.1] - 2024-04-04

Added

- Add `bfs::predecessors_single_source`.
- Add `dijkstra::predecessors_single_source`.

## [0.5.0] - 2024-04-03

Added

- Implement `Indegree` for `HashMap<usize, HashMap<usize, W>>`.
- Implement `IsArc` for `HashSet<(usize, usize)>`.
- Implement `IterVertices` for `HashSet<T>`.
- Implement `IterWeightedArcs` for `Vec<HashMap<usize, W>>`.
- Implement `IterWeightedArcs` for `[HashMap<usize, W>; V]`.
- Implement `Outdegree` for `Vec<HashMap<usize, W>>`.
- Implement `Outdegree` for `[HashMap<usize, W>; V]`.

Removed

- Remove the `VertexWeight` trait.

## [0.4.2] - 2024-04-03

Changed

- Rename `*::shortest_paths` to `predecessors`.

## [0.4.1] - 2024-04-03

Added

- Add `algo::bfs::shortest_paths`.

## [0.4.0] - 2024-04-03

Added

- Add `algo::dijkstra::shortest_paths`.

Changed

- Move `algo::dijkstra::weighted` to `algo::dijkstra`.
- Remove `algo::dijkstra::unweighted`.

## [0.3.3] - 2024-04-02

Added

- Add `algo::bfs::min_distances_single_source`.
- Add `algo::bfs::min_distances`.
- Add benchmarks for `algo::bfs::*`

## [0.3.2] - 2024-04-01

Added

- Add `algo::dijkstra::unweighted::shortest_paths`.
- Test `algo::dijkstra::unweighted::min_distances` with multiple source vertices.
- Test `algo::dijkstra::weighted::min_distances` without sources.

## [0.3.1] - 2024-04-01

Changed

- Update function names in `README`.

## [0.3.0] - 2024-04-01

Added

- Add `algo::dijkstra::unweighted::min_distances_single_source`.
- Add a documentation test example for `algo::dijkstra::unweighted::min_distances_single_source`
- Add a documentation test example for `algo::dijkstra::unweighted::min_distances`
- Add a documentation test example for `algo::dijkstra::weighted::min_distances_single_source`.

Changed

- Move `algo::DijkstraUnweighted::dijkstra` to `algo::dijkstra::unweighted::min_distances`.
- Move `algo::DijkstraWeighted::dijkstra` to `algo::dijkstra::weighted::min_distances`.
- Move `algo::dijkstra::dijkstra_sssp_weighted` to `algo::dijkstra::weighted::min_distances_single_source`.

Removed

- Remove the `new` benchmark.

## [0.2.3] - 2024-03-31

Added

- Add GitHub Action on push to main and PRs on main.
- Add `dijkstra_sssp_weighted`.

Changed

- Make `CHANGELOG.md` adhere to [keep a changelog](https://keepachangelog.com/en/1.0.0/).
- Test `Dijkstra.Unweighted.dijkstra_sssp_unweighted` for every source vertex.

## [0.2.2] - 2024-03-31

Added

- Add a documentation test example for `Weighted.dijkstra`.

Fixed

- Fix trait descriptions in `README.md`.

## [0.2.1] - 2024-03-31

Added

- Add `dijkstra_sssp_unweighted`.
- Add missing documentation for the public API.

Changed

- Enable selected lints from the `restriction` group.
- Export `algo`, `op`, and `repr` modules.
- Group lints into groups, restrictions, `rustdoc`, and `rustc` lints.
- Use `core` and `alloc` imports over `std` where possible.

Fixed

- Make descriptions more consistent.

## [0.2.0] - 2024-03-30

Changed

- Rename `Indegree.in_degree` to `Indegree.indegree`.
- Rename `Outdegree.out_degree` to `Outdegree.outdegree`.

## [0.1.0] - 2024-03-30

- Initial release
