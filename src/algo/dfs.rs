//! Depth-first search
//!
//! Depth-first search is a digraph traversal algorithm that explores a digraph
//! by following a path as far as possible before backtracking.
//!
//! The time complexity is *O*(*v* + *a*).

use crate::op::{
    Order,
    OutNeighbors,
};

/// Generates an acyclic ordering of the vertices of an unweighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `ordering`: The ordering.
/// * `t_visit`: The time of the first visit of each vertex.
/// * `t_expl`: The time of the last exploration of each vertex.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::dfs::dfsa,
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {4}
/// // 1 -> {0}
/// // 2 -> {1, 3, 5}
/// // 3 -> {}
/// // 4 -> {}
/// // 5 -> {4}
///
/// let mut digraph = Digraph::empty(6);
///
/// digraph.add_arc(0, 4);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(2, 1);
/// digraph.add_arc(2, 3);
/// digraph.add_arc(2, 5);
/// digraph.add_arc(5, 4);
///
/// let mut ordering = [0; 6];
/// let mut t_visit = [0; 6];
/// let mut t_expl = [0; 6];
///
/// dfsa(&digraph, &mut ordering, &mut t_visit, &mut t_expl);
///
/// assert!(ordering.iter().eq(&[2, 5, 3, 1, 0, 4]));
/// ```
pub fn dfsa<D>(digraph: &D, ordering: &mut [usize], t_visit: &mut [usize], t_expl: &mut [usize])
where
    D: OutNeighbors + Order,
{
    let order = digraph.order();
    let mut t = 0;
    let mut i = order;

    for u in 0..order {
        if t_visit[u] == 0 {
            dfsa_visit(digraph, u, ordering, t_visit, t_expl, &mut i, &mut t);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn dfsa_visit<D>(
    digraph: &D,
    u: usize,
    ordering: &mut [usize],
    t_visit: &mut [usize],
    t_expl: &mut [usize],
    i: &mut usize,
    t: &mut usize,
) where
    D: OutNeighbors,
{
    *t += 1;
    t_visit[u] = *t;

    for v in digraph.out_neighbors(u) {
        if t_visit[v] == 0 {
            dfsa_visit(digraph, v, ordering, t_visit, t_expl, i, t);
        }
    }

    *t += 1;
    *i -= 1;
    t_expl[u] = *t;
    ordering[*i] = u;
}

/// Generates the breadth-first tree and an acyclic ordering of the vertices of
/// an unweighted digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
/// * `ordering`: The ordering.
/// * `pred`: The breadth-first tree.
/// * `t_visit`: The time of the first visit of each vertex.
/// * `t_expl`: The time of the last exploration of each vertex.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::dfs::dfsa_predecessors,
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {4}
/// // 1 -> {0}
/// // 2 -> {1, 3, 5}
/// // 3 -> {}
/// // 4 -> {}
/// // 5 -> {4}
///
/// let mut digraph = Digraph::empty(6);
///
/// digraph.add_arc(0, 4);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(2, 1);
/// digraph.add_arc(2, 3);
/// digraph.add_arc(2, 5);
/// digraph.add_arc(5, 4);
///
/// let mut ordering = [0; 6];
/// let mut pred = [None; 6];
/// let mut t_visit = [0; 6];
/// let mut t_expl = [0; 6];
///
/// dfsa_predecessors(
///     &digraph,
///     &mut ordering,
///     &mut pred,
///     &mut t_visit,
///     &mut t_expl,
/// );
///
/// assert!(ordering.iter().eq(&[2, 5, 3, 1, 0, 4]));
/// ```
pub fn dfsa_predecessors<D>(
    digraph: &D,
    ordering: &mut [usize],
    pred: &mut [Option<usize>],
    t_visit: &mut [usize],
    t_expl: &mut [usize],
) where
    D: Order + OutNeighbors,
{
    let order = digraph.order();
    let mut t = 0;
    let mut i = order;

    for u in 0..order {
        if t_visit[u] == 0 {
            dfsa_predecessors_visit(digraph, u, ordering, pred, t_visit, t_expl, &mut i, &mut t);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn dfsa_predecessors_visit<D>(
    digraph: &D,
    u: usize,
    ordering: &mut [usize],
    pred: &mut [Option<usize>],
    t_visit: &mut [usize],
    t_expl: &mut [usize],
    i: &mut usize,
    t: &mut usize,
) where
    D: OutNeighbors,
{
    *t += 1;
    t_visit[u] = *t;

    for v in digraph.out_neighbors(u) {
        if t_visit[v] == 0 {
            pred[v] = Some(u);

            dfsa_predecessors_visit(digraph, v, ordering, pred, t_visit, t_expl, i, t);
        }
    }

    *t += 1;
    *i -= 1;
    t_expl[u] = *t;
    ordering[*i] = u;
}

/// Generates an acyclic ordering of the vertices of a digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
///
/// # Returns
///
/// An acyclic ordering of the vertices of the digraph.
///
/// # Examples
///
/// ```
/// use graaf::{
///     adjacency_list::Digraph,
///     algo::dfs::acyclic_ordering,
///     gen::Empty,
///     op::AddArc,
/// };
///
/// // 0 -> {4}
/// // 1 -> {0}
/// // 2 -> {1, 3, 5}
/// // 3 -> {}
/// // 4 -> {}
/// // 5 -> {4}
///
/// let mut digraph = Digraph::empty(6);
///
/// digraph.add_arc(0, 4);
/// digraph.add_arc(1, 0);
/// digraph.add_arc(2, 1);
/// digraph.add_arc(2, 3);
/// digraph.add_arc(2, 5);
/// digraph.add_arc(5, 4);
///
/// assert!(acyclic_ordering(&digraph).iter().eq(&[2, 5, 3, 1, 0, 4]));
/// ```
#[must_use]
pub fn acyclic_ordering<D>(digraph: &D) -> Vec<usize>
where
    D: Order + OutNeighbors,
{
    let order = digraph.order();
    let mut ordering = vec![0; order];
    let mut t_visit = vec![0; order];
    let mut t_expl = vec![0; order];

    dfsa(digraph, &mut ordering, &mut t_visit, &mut t_expl);

    ordering
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::adjacency_list::fixture,
    };

    #[test]
    fn dfsa_bang_jensen_34() {
        let digraph = fixture::bang_jensen_34();
        let order = digraph.order();
        let mut ordering = vec![0; order];
        let mut t_visit = vec![0; order];
        let mut t_expl = vec![0; order];

        dfsa(&digraph, &mut ordering, &mut t_visit, &mut t_expl);

        assert!(ordering.iter().eq(&[2, 5, 3, 1, 0, 4]));
    }

    #[test]
    fn dfsa_kattis_builddeps() {
        let digraph = fixture::kattis_builddeps();
        let order = digraph.order();
        let mut ordering = vec![0; order];
        let mut t_visit = vec![0; order];
        let mut t_expl = vec![0; order];

        dfsa(&digraph, &mut ordering, &mut t_visit, &mut t_expl);

        let dependencies = ordering
            .into_iter()
            .skip_while(|&u| u != 0)
            .collect::<Vec<usize>>();

        // 0 = gmp
        // 4 = map
        // 3 = set
        // 1 = solution

        assert!(dependencies.iter().eq(&[0, 4, 3, 1]));
    }

    #[test]
    fn dfsa_predecessors_bang_jensen_34() {
        let digraph = fixture::bang_jensen_34();
        let order = digraph.order();
        let mut ordering = vec![0; order];
        let mut pred = vec![None; order];
        let mut t_visit = vec![0; order];
        let mut t_expl = vec![0; order];

        dfsa_predecessors(
            &digraph,
            &mut ordering,
            &mut pred,
            &mut t_visit,
            &mut t_expl,
        );

        assert!(ordering.iter().eq(&[2, 5, 3, 1, 0, 4]));

        assert!(pred
            .iter()
            .eq(&[None, None, None, Some(2), Some(0), Some(2)]));
    }

    #[test]
    fn dfsa_predecessors_kattis_builddeps() {
        let digraph = fixture::kattis_builddeps();
        let order = digraph.order();
        let mut ordering = vec![0; order];
        let mut pred = vec![None; order];
        let mut t_visit = vec![0; order];
        let mut t_expl = vec![0; order];

        dfsa_predecessors(
            &digraph,
            &mut ordering,
            &mut pred,
            &mut t_visit,
            &mut t_expl,
        );

        let dependencies = ordering
            .into_iter()
            .skip_while(|&u| u != 0)
            .collect::<Vec<usize>>();

        // 0 = gmp
        // 4 = map
        // 3 = set
        // 1 = solution

        assert!(dependencies.iter().eq(&[0, 4, 3, 1]));

        assert!(pred
            .iter()
            .eq(&[None, Some(3), None, Some(0), Some(0), Some(2)]));
    }

    #[test]
    fn acyclic_ordering_bang_jensen_34() {
        assert!(acyclic_ordering(&fixture::bang_jensen_34())
            .iter()
            .eq(&[2, 5, 3, 1, 0, 4]));
    }

    #[test]
    fn acycling_ordering_kattis_builddeps() {
        let ordering = acyclic_ordering(&fixture::kattis_builddeps());

        let dependencies = ordering
            .into_iter()
            .skip_while(|&u| u != 0)
            .collect::<Vec<usize>>();

        // 0 = gmp
        // 4 = map
        // 3 = set
        // 1 = solution

        assert!(dependencies.iter().eq(&[0, 4, 3, 1]));
    }
}
