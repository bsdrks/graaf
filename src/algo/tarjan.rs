//! Tarjan's algorithm.
//!
//! Tarjan's algorithm finds strongly connected components in a digraph.
//!
//! # Examples
//!
//! There are three strongly connected components in the following digraph:
//!
//! ![Tarjan](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/tarjan_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         tarjan::strongly_connected_components,
//!         AddArc,
//!         AdjacencyList,
//!         Empty,
//!     },
//!     std::collections::BTreeSet,
//! };
//!
//! let mut digraph = AdjacencyList::empty(8);
//!
//! digraph.add_arc(0, 1);
//! digraph.add_arc(1, 2);
//! digraph.add_arc(1, 4);
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
//! assert_eq!(
//!     strongly_connected_components(&digraph)
//!         .into_iter()
//!         .collect::<BTreeSet<BTreeSet<usize>>>(),
//!     BTreeSet::from([
//!         BTreeSet::from([4, 1, 0]),
//!         BTreeSet::from([5, 6]),
//!         BTreeSet::from([7, 3, 2])
//!     ])
//! );
//! ```

use {
    crate::{
        Order,
        OutNeighbors,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
    },
};

/// Tarjan's algorithm.
///
/// Tarjan's algorithm finds strongly connected components in a digraph.
///
/// # Arguments
///
/// * `digraph`: The digraph.
///
/// # Examples
///
/// There are three strongly connected components in the following digraph:
///
/// ![Tarjan](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/tarjan_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         tarjan::strongly_connected_components,
///         AddArc,
///         AdjacencyList,
///         Empty,
///     },
///     std::collections::BTreeSet,
/// };
///
/// let mut digraph = AdjacencyList::empty(8);
///
/// digraph.add_arc(0, 1);
/// digraph.add_arc(1, 2);
/// digraph.add_arc(1, 4);
/// digraph.add_arc(2, 3);
/// digraph.add_arc(2, 6);
/// digraph.add_arc(3, 2);
/// digraph.add_arc(3, 7);
/// digraph.add_arc(4, 0);
/// digraph.add_arc(4, 5);
/// digraph.add_arc(5, 6);
/// digraph.add_arc(6, 5);
/// digraph.add_arc(7, 3);
/// digraph.add_arc(7, 6);
///
/// assert_eq!(
///     strongly_connected_components(&digraph)
///         .into_iter()
///         .collect::<BTreeSet<BTreeSet<usize>>>(),
///     BTreeSet::from([
///         BTreeSet::from([4, 1, 0]),
///         BTreeSet::from([5, 6]),
///         BTreeSet::from([7, 3, 2])
///     ])
/// );
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
            repr::adjacency_list::fixture::{
                bang_jensen_196,
                kattis_cantinaofbabel_1,
                kattis_cantinaofbabel_2,
            },
            AdjacencyList,
            Empty,
        },
    };

    #[test]
    fn strongly_connected_components_bang_jensen_196() {
        assert_eq!(
            strongly_connected_components(&bang_jensen_196())
                .into_iter()
                .collect::<BTreeSet<BTreeSet<_>>>(),
            BTreeSet::from([
                BTreeSet::from([0, 1]),
                BTreeSet::from([2, 3, 4]),
                BTreeSet::from([5, 6, 7]),
            ])
        );
    }

    #[test]
    fn strongly_connected_components_kattis_cantinaofbabel_1() {
        assert_eq!(
            strongly_connected_components(&kattis_cantinaofbabel_1())
                .into_iter()
                .collect::<BTreeSet<BTreeSet<_>>>(),
            BTreeSet::from([
                BTreeSet::from([0, 1, 2, 3, 4, 7, 9, 11]),
                BTreeSet::from([5, 6, 10]),
                BTreeSet::from([8]),
            ])
        );
    }

    #[test]
    fn strongly_connected_components_kattis_cantinaofbabel_2() {
        assert_eq!(
            strongly_connected_components(&kattis_cantinaofbabel_2())
                .into_iter()
                .collect::<BTreeSet<BTreeSet<_>>>(),
            BTreeSet::from([
                BTreeSet::from([0, 1, 2, 7]),
                BTreeSet::from([3, 4]),
                BTreeSet::from([5, 6]),
                BTreeSet::from([8, 9, 10, 11]),
            ])
        );
    }

    #[test]
    fn strongly_connected_components_trivial() {
        assert_eq!(
            strongly_connected_components(&AdjacencyList::trivial())
                .into_iter()
                .collect::<BTreeSet<BTreeSet<_>>>(),
            BTreeSet::from([BTreeSet::from([0])])
        );
    }
}
