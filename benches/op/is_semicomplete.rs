//! Benchmarks of different implementations of
//! `IsSemicomplete::is_semicomplete`.
use {
    divan::Bencher,
    graaf::{
        AdjacencyList,
        AdjacencyMap,
        AdjacencyMatrix,
        Complete,
        EdgeList,
        ErdosRenyi,
        HasArc,
        IsSemicomplete,
        Order,
        RandomTournament,
        RemoveArc,
        Size,
    },
};

fn main() {
    divan::main();
}

fn is_semicomplete_all_all_has_arc<D>(digraph: &D) -> bool
where
    D: HasArc + Order,
{
    let order = digraph.order();

    (0..order).all(|u| {
        (u + 1..order).all(|v| digraph.has_arc(u, v) || digraph.has_arc(v, u))
    })
}

fn is_semicomplete_size_all_all_has_arc<D>(digraph: &D) -> bool
where
    D: HasArc + Order + Size,
{
    let order = digraph.order();

    digraph.size() >= order * (order - 1) / 2
        && (0..order).all(|u| {
            (u + 1..order)
                .all(|v| digraph.has_arc(u, v) || digraph.has_arc(v, u))
        })
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_almost_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_almost_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_almost_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_almost_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMap::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_almost_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMap::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_almost_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMap::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_almost_random_tournament(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMatrix::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_almost_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMatrix::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_almost_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = AdjacencyMatrix::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_almost_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let mut digraph = EdgeList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_almost_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = EdgeList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_almost_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let mut digraph = EdgeList::random_tournament(order, 0);
    let a = order - 1;
    let b = order - 2;
    let _ = digraph.remove_arc(a, b);
    let _ = digraph.remove_arc(b, a);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_random_tournament_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::random_tournament(order, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_erdos_renyi_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_erdos_renyi_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_erdos_renyi_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_erdos_renyi_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::erdos_renyi(order, 0.5, 0);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_list_complete_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyList::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_map_complete_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMap::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn adjacency_matrix_complete_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = AdjacencyMatrix::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = digraph.is_semicomplete();
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_all_all_has_arc(bencher: Bencher<'_, '_>, order: usize) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_all_all_has_arc(&digraph);
    });
}

#[divan::bench(args = [10, 100, 1000])]
fn edge_list_complete_size_all_all_has_arc(
    bencher: Bencher<'_, '_>,
    order: usize,
) {
    let digraph = EdgeList::complete(order);

    bencher.bench(|| {
        let _ = is_semicomplete_size_all_all_has_arc(&digraph);
    });
}
