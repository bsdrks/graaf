//! Tarjan's algorithm.
//!
//! Tarjan's algorithm finds a digraph's strongly connected components.
//!
//! # Examples
//!
//! There are three strongly connected components in this digraph:
//!
//! ![A digraph and its strongly connected components](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/tarjan_1-0.87.4.svg?)
//!
//! ```
//! use {
//!     graaf::{
//!         AddArc,
//!         AdjacencyList,
//!         Empty,
//!         Tarjan,
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
//! assert!(Tarjan::new(&digraph).components().iter().eq(&[
//!     BTreeSet::from([5, 6]),
//!     BTreeSet::from([7, 3, 2]),
//!     BTreeSet::from([4, 1, 0])
//! ]));
//! ```

use {
    crate::{
        OutNeighbors,
        Vertices,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
    },
};

/// Tarjan's algorithm.
///
/// Tarjan's algorithm finds a digraph's strongly connected components.
///
/// # Examples
///
/// There are three strongly connected components in this digraph:
///
/// ![A digraph and its strongly connected components](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/tarjan_1-0.87.4.svg?)
///
/// ```
/// use {
///     graaf::{
///         AddArc,
///         AdjacencyList,
///         Empty,
///         Tarjan,
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
/// assert!(Tarjan::new(&digraph).components().iter().eq(&[
///     BTreeSet::from([5, 6]),
///     BTreeSet::from([7, 3, 2]),
///     BTreeSet::from([4, 1, 0]),
/// ]));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tarjan<'a, D> {
    digraph: &'a D,
    i: usize,
    stack: Vec<usize>,
    on_stack: BTreeSet<usize>,
    index: BTreeMap<usize, usize>,
    low_link: BTreeMap<usize, usize>,
    components: Vec<BTreeSet<usize>>,
}

impl<'a, D> Tarjan<'a, D> {
    /// Construct a new instance of Tarjan's algorithm.
    ///
    /// # Arguments
    ///
    /// * `digraph`: The digraph.
    #[must_use]
    pub const fn new(digraph: &'a D) -> Self
    where
        D: OutNeighbors + Vertices,
    {
        Self {
            digraph,
            i: 0,
            stack: Vec::new(),
            on_stack: BTreeSet::new(),
            index: BTreeMap::new(),
            low_link: BTreeMap::new(),
            components: Vec::new(),
        }
    }

    /// Find a digraph's strongly connected components.
    ///
    /// # Examples
    ///
    /// There are three strongly connected components in this digraph:
    ///
    /// ![A digraph and its strongly connected components](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/tarjan_1-0.87.4.svg?)
    ///
    /// ```
    /// use {
    ///     graaf::{
    ///         AddArc,
    ///         AdjacencyList,
    ///         Empty,
    ///         Tarjan,
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
    /// assert!(Tarjan::new(&digraph).components().iter().eq(&[
    ///     BTreeSet::from([5, 6]),
    ///     BTreeSet::from([7, 3, 2]),
    ///     BTreeSet::from([4, 1, 0])
    /// ]));
    /// ```
    #[must_use]
    pub fn components(&mut self) -> &Vec<BTreeSet<usize>>
    where
        D: OutNeighbors + Vertices,
    {
        for u in self.digraph.vertices() {
            if !self.index.contains_key(&u) {
                self.connect(u);
            }
        }

        &self.components
    }

    fn connect(&mut self, u: usize)
    where
        D: OutNeighbors,
    {
        let _ = self.index.insert(u, self.i);
        let _ = self.low_link.insert(u, self.i);
        let _ = self.on_stack.insert(u);

        self.stack.push(u);

        self.i += 1;

        for v in self.digraph.out_neighbors(u) {
            if let Some(&w) = self.index.get(&v) {
                if self.on_stack.contains(&v) {
                    let _ = self.low_link.insert(u, self.low_link[&u].min(w));
                }
            } else {
                self.connect(v);

                let _ = self
                    .low_link
                    .insert(u, self.low_link[&u].min(self.low_link[&v]));
            }
        }

        if self.index.get(&u) == self.low_link.get(&u) {
            let mut component = BTreeSet::new();

            while let Some(v) = self.stack.pop() {
                let _ = self.on_stack.remove(&v);
                let _ = component.insert(v);

                if u == v {
                    break;
                }
            }

            self.components.push(component);
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            repr::adjacency_list::fixture::{
                bang_jensen_196,
                bang_jensen_34,
                bang_jensen_94,
                kattis_builddeps,
                kattis_cantinaofbabel_1,
                kattis_cantinaofbabel_2,
                kattis_escapewallmaria_1,
                kattis_escapewallmaria_2,
                kattis_escapewallmaria_3,
            },
            AdjacencyList,
            Empty,
        },
    };

    #[test]
    fn components_bang_jensen_196() {
        assert!(Tarjan::new(&bang_jensen_196()).components().iter().eq(&[
            BTreeSet::from([2, 3, 4]),
            BTreeSet::from([5, 6, 7]),
            BTreeSet::from([0, 1]),
        ]));
    }

    #[test]
    fn components_bang_jensen_34() {
        assert!(Tarjan::new(&bang_jensen_34()).components().iter().eq(&[
            BTreeSet::from([4]),
            BTreeSet::from([0]),
            BTreeSet::from([1]),
            BTreeSet::from([3]),
            BTreeSet::from([5]),
            BTreeSet::from([2]),
        ]));
    }

    #[test]
    fn components_bang_jensen_94() {
        assert!(Tarjan::new(&bang_jensen_94()).components().iter().eq(&[
            BTreeSet::from([5]),
            BTreeSet::from([3]),
            BTreeSet::from([1]),
            BTreeSet::from([6]),
            BTreeSet::from([4]),
            BTreeSet::from([2]),
            BTreeSet::from([0]),
        ]));
    }

    #[test]
    fn components_kattis_builddeps() {
        assert!(Tarjan::new(&kattis_builddeps()).components().iter().eq(&[
            BTreeSet::from([1]),
            BTreeSet::from([3]),
            BTreeSet::from([4]),
            BTreeSet::from([0]),
            BTreeSet::from([5]),
            BTreeSet::from([2]),
        ]));
    }

    #[test]
    fn components_kattis_cantinaofbabel_1() {
        assert!(Tarjan::new(&kattis_cantinaofbabel_1())
            .components()
            .iter()
            .eq(&[
                BTreeSet::from([5, 6, 10]),
                BTreeSet::from([0, 1, 2, 3, 4, 7, 9, 11]),
                BTreeSet::from([8]),
            ]));
    }

    #[test]
    fn components_kattis_cantinaofbabel_2() {
        assert!(Tarjan::new(&kattis_cantinaofbabel_2())
            .components()
            .iter()
            .eq(&[
                BTreeSet::from([3, 4]),
                BTreeSet::from([5, 6]),
                BTreeSet::from([0, 1, 2, 7]),
                BTreeSet::from([8, 9, 10, 11]),
            ]));
    }

    #[test]
    fn components_kattis_escapewallmaria_1() {
        assert!(Tarjan::new(&kattis_escapewallmaria_1())
            .components()
            .iter()
            .eq(&[
                BTreeSet::from([0]),
                BTreeSet::from([1]),
                BTreeSet::from([2]),
                BTreeSet::from([3]),
                BTreeSet::from([4]),
                BTreeSet::from([12]),
                BTreeSet::from([5, 6, 9, 13]),
                BTreeSet::from([7]),
                BTreeSet::from([8]),
                BTreeSet::from([10]),
                BTreeSet::from([11]),
                BTreeSet::from([14]),
                BTreeSet::from([15]),
            ]));
    }

    #[test]
    fn components_kattis_escapewallmaria_2() {
        assert!(Tarjan::new(&kattis_escapewallmaria_2())
            .components()
            .iter()
            .eq(&[
                BTreeSet::from([0]),
                BTreeSet::from([1]),
                BTreeSet::from([2]),
                BTreeSet::from([3]),
                BTreeSet::from([4]),
                BTreeSet::from([5, 6, 9]),
                BTreeSet::from([7]),
                BTreeSet::from([8]),
                BTreeSet::from([10]),
                BTreeSet::from([11]),
                BTreeSet::from([12, 13]),
                BTreeSet::from([14]),
                BTreeSet::from([15]),
            ]));
    }

    #[test]
    fn components_kattis_escapewallmaria_3() {
        assert!(Tarjan::new(&kattis_escapewallmaria_3())
            .components()
            .iter()
            .eq(&[
                BTreeSet::from([0]),
                BTreeSet::from([1, 2, 5, 6, 9, 12, 13]),
                BTreeSet::from([3]),
                BTreeSet::from([4]),
                BTreeSet::from([7]),
                BTreeSet::from([8]),
                BTreeSet::from([10]),
                BTreeSet::from([11]),
                BTreeSet::from([14]),
                BTreeSet::from([15]),
            ]));
    }

    #[test]
    fn components_trivial() {
        assert!(Tarjan::new(&AdjacencyList::trivial())
            .components()
            .iter()
            .eq(&[BTreeSet::from([0])]));
    }
}
