//! Benchmarks of different implementations of `IsRegular::is_regular`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList, AdjacencyMap, AdjacencyMatrix, Complete, EdgeList,
        ErdosRenyi, Indegree, IsRegular, Outdegree, SemidegreeSequence,
        Vertices,
    },
};

fn main() {
    divan::main();
}

fn is_regular_all_indegree_eq_outdegree<D>(digraph: &D) -> bool
where
    D: Indegree + Outdegree + Vertices,
{
    let mut vertices = digraph.vertices();
    let u = vertices.next().expect("a digraph has at least one vertex");
    let indegree = digraph.indegree(u);
    let outdegree = digraph.outdegree(u);

    indegree == outdegree
        && vertices.all(|u| {
            digraph.indegree(u) == indegree
                && digraph.outdegree(u) == outdegree
        })
}

fn is_regular_semidegree_sequence<D>(digraph: &D) -> bool
where
    D: SemidegreeSequence,
{
    let mut semidegrees = digraph.semidegree_sequence();

    let (u, v) = semidegrees
        .next()
        .expect("a digraph has at least one vertex");

    u == v && semidegrees.all(|(x, y)| x == u && y == v)
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.99, 0);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_regular();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_all_indegree_eq_outdegree(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_all_indegree_eq_outdegree(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_semidegree_sequence(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_regular_semidegree_sequence(&digraph);
    });
}
