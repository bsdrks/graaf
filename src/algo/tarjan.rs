//! Tarjan
//!
//! Tarjan's algorithm finds strongly connected components in a digraph.
//!
//! # Examples
//!
//! ```
//! use {
//!     graaf::{
//!         adjacency_list::Digraph,
//!         algo::tarjan::strongly_connected_components,
//!         gen::Empty,
//!         op::AddArc,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! // 0 -> {1}
//! // 1 -> {2, 4, 5}
//! // 2 -> {3, 6}
//! // 3 -> {2, 7}
//! // 4 -> {0, 5}
//! // 5 -> {6}
//! // 6 -> {5}
//! // 7 -> {3, 6}
//!
//! let mut digraph = Digraph::empty(8);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 4);
//! digraph.add_arc(1, 5);
//! digraph.add_arc(2, 3);
//! digraph.add_arc(2, 6);
//! digraph.add_arc(3, 2);
//! digraph.add_arc(3, 7);
//! digraph.add_arc(4, 0);
//! digraph.add_arc(4, 5);
//! digraph.add_arc(5, 6);
//! digraph.add_arc(6, 5);
//! digraph.add_arc(7, 3);
//! digraph.add_arc(7, 6);
//!
//! assert!(strongly_connected_components(&digraph).iter().eq(&[
//!     BTreeSet::from([5, 6]),
//!     BTreeSet::from([7, 3, 2]),
//!     BTreeSet::from([4, 1, 0])
//! ]));
//! ```

use {
    crate::op::{
        Order,
        OutNeighbors,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
    },
};

/// Tarjan's algorithm for finding strongly connected components in a digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
///
/// # Examples
///
/// ```
/// use {
///     graaf::{
///         adjacency_list::Digraph,
///         algo::tarjan::strongly_connected_components,
///         gen::Empty,
///         op::AddArc,
///     },
///     std::collections::BTreeSet,
/// };
///
/// // 0 -> {1}
/// // 1 -> {2}
/// // 2 -> {0}
/// // 3 -> {0}
///
/// let mut digraph = Digraph::empty(4);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(2, 0);
/// digraph.add_arc(3, 0);
///
/// assert!(strongly_connected_components(&digraph)
///     .eq(&[BTreeSet::from([0, 1, 2]), BTreeSet::from([3])]));
/// ```
#[doc(alias = "scc")]
#[must_use]
pub fn strongly_connected_components<D>(digraph: &D) -> Vec<BTreeSet<usize>>
where
    D: Order + OutNeighbors,
{
    let order = digraph.order();
    let mut i = 0;
    let mut stack = Vec::new();
    let mut on_stack = BTreeSet::new();
    let mut index = BTreeMap::new();
    let mut low_link = BTreeMap::new();
    let mut components = Vec::new();

    for u in 0..order {
        if !index.contains_key(&u) {
            connect(
                digraph,
                u,
                &mut i,
                &mut stack,
                &mut on_stack,
                &mut index,
                &mut low_link,
                &mut components,
            );
        }
    }

    components
}

#[allow(clippy::too_many_arguments)]
fn connect<D>(
    digraph: &D,
    u: usize,
    i: &mut usize,
    stack: &mut Vec<usize>,
    on_stack: &mut BTreeSet<usize>,
    index: &mut BTreeMap<usize, usize>,
    low_link: &mut BTreeMap<usize, usize>,
    scc: &mut Vec<BTreeSet<usize>>,
) where
    D: OutNeighbors,
{
    let _ = index.insert(u, *i);
    let _ = low_link.insert(u, *i);
    let _ = on_stack.insert(u);

    stack.push(u);

    *i += 1;

    for v in digraph.out_neighbors(u) {
        if let Some(&w) = index.get(&v) {
            if on_stack.contains(&v) {
                let _ = low_link.insert(u, low_link[&u].min(w));
            }
        } else {
            connect(digraph, v, i, stack, on_stack, index, low_link, scc);

            let _ = low_link.insert(u, low_link[&u].min(low_link[&v]));
        }
    }

    if index.get(&u) == low_link.get(&u) {
        let mut component = BTreeSet::new();

        while let Some(v) = stack.pop() {
            let _ = on_stack.remove(&v);
            let _ = component.insert(v);

            if u == v {
                break;
            }
        }

        scc.push(component);
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            adjacency_list::{
                fixture::bang_jensen_196,
                Digraph,
            },
            gen::Empty,
        },
    };

    #[test]
    fn strongly_connected_components_bang_jensen_196() {
        assert!(strongly_connected_components(&bang_jensen_196())
            .iter()
            .eq(&[
                BTreeSet::from([2, 3, 4]),
                BTreeSet::from([5, 6, 7]),
                BTreeSet::from([0, 1]),
            ]));
    }

    #[test]
    fn strongly_connected_components_trivial() {
        assert!(strongly_connected_components(&Digraph::trivial())
            .iter()
            .eq(&[BTreeSet::from([0])]));
    }
}
