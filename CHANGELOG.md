# Changelog

## Provisional roadmap

- Benchmark against popular graph libraries in other languages.
- Benchmark trait implementations.
- Implement `IterOutNeighborsMut` for additional types.

## [0.63.1] - 2024-06-27

Added

- Add and test `DistanceMatrix::is_connected`.

## [0.63.0] - 2024-06-25

Changed

- Breaking: Fixtures are now `cfg(test)`.

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

- Breaking: `DistanceMatrix::new` now panics if `v` is zero.
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

- Breaking: Rename `BfTree` to `BreadthFirstTree`.
- Breaking: Rename `bf_tree` to `breadth_first_tree`.
- Breaking: `FloydWarshall::distances` now returns a `DistanceMatrix`.

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
- Breaking: `BfTree::new` now panics if `v` is zero.
- Breaking: `BfTree::search_by` now immediately returns if `s` is a target.
- Breaking: `BfTree::search` now immediately returns if `s` equals `t`.

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

- Breaking: `bfs::predecessors` now accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `bfs::shortest_path` now accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `bfs::single_source_predecessors` now returns a `BfsTree`.
- Breaking: `dijkstra::predecessors` now accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `dijkstra::shortest_path` now accepts a `BfsTree` instead of a `Vec<Option<usize>>`.
- Breaking: `dijkstra::single_source_predecessors` now returns a `BfsTree`.

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
- Implement `IterInNeighbors` for `IterArcs + ?Sized`.
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

- Breaking: `AdjacencyMatrix::empty` now panics if `V` is zero.

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
- Change  `Converse` to blanket implementation.
- Change  `Degree` to blanket implementation.
- Change  `HasEdge` to blanket implementation.
- Change  `IsBalanced` to blanket implementation.
- Change  `IsIsolated` to blanket implementation.
- Change  `IsOriented` to blanket implementation.
- Change  `IsPendant` to blanket implementation.
- Change  `IsRegular` to blanket implementation.
- Change  `IsSubdigraph` to blanket implementation.
- Change  `IsSuperdigraph` to blanket implementation.
- Change  `IsSymmetric` to blanket implementation.
- Change  `IsWalk` to blanket implementation.
- Change  `ReverseArc` to blanket implementation.
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

- Breaking: `bellman_ford_moore::distances` now returns an `Option` if a negative cycle is detected.

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
- Add unit test `doctest` for `bellman_ford_moore::distances`.

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

- Breaking: `AdjacencyMatrix::<0>::new` now panics.
- Simplify `AdjacencyMatrix<V>` op implementations with `IterVertices`.
- Simplify `AdjacencyMatrix<V>` tests with `Iterator::eq`.
- Simplify `IsSymmetric` implementations with `HasArc`.
- Simplify arguments to `Iterator::eq` in tests.

## [0.41.0] - 2024-05-11

Changed

- Breaking: `Complete::complete` now panics if `v` is zero.
- Breaking: `CompleteConst::complete_const` now panics if `V` is zero.
- Breaking: `Cycle::cycle` now panics if `v` is zero.
- Breaking: `CycleConst::cycle_const` now panics if `V` is zero.
- Breaking: `Empty::empty` now panics if `v` is zero.
- Breaking: `EmptyConst::empty_const` now panics if `V` is zero.

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

- Breaking: `bfs::predecessors_single_source` now only returns the breadth-first tree.
- Breaking: `dijkstra::predecessors_single_source` now only returns the breadth-first tree.

## [0.18.0] - 2024-04-20

Changed

- Breaking: `repr::AdjacenyMatrix` is now a feature.

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

- `RemoveArc::remove_arc` now returns a `bool` indicating whether it removed the arc.

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

- Breaking: `AddWeightedArc` for `HashMap<_>` now panics if `s` is not in the graph.

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

- `IsSimple` now checks for parallel arcs in `HashSet<(usize, usize, W)>`.

## [0.9.0] - 2024-04-12

Fixed

- `IsSimple` now checks for parallel arcs in `HashSet<(usize, usize)>`.
- `IsSimple` now checks for parallel arcs in `[(usize, usize)]`.
- `IsSimple` now checks for parallel arcs in `[(usize, usize, W)]`.

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
