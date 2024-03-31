use {
    core::hash::BuildHasher,
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A trait to adding edges to an unweighted graph
pub trait AddEdge {
    /// Add an edge from `s` to `t`.
    ///
    /// # Arguments
    ///
    /// * `s`: The source vertex.
    /// * `t`: The target vertex.
    fn add_edge(&mut self, s: usize, t: usize);
}

// Vec

impl AddEdge for Vec<Vec<usize>> {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl<H> AddEdge for Vec<HashSet<usize, H>>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

// Arr

impl<const V: usize> AddEdge for [Vec<usize>; V] {
    /// # Panics
    ///
    /// Panics if `s` is not in the graph or if the new capacity of the vector
    /// exceeds `isize::MAX`.
    fn add_edge(&mut self, s: usize, t: usize) {
        self[s].push(t);
    }
}

impl<const V: usize, H> AddEdge for [HashSet<usize, H>; V]
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self[s].insert(t);
    }
}

// HashMap

impl<H> AddEdge for HashMap<usize, Vec<usize>, H>
where
    H: BuildHasher,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        self.get_mut(&s).unwrap().push(t);
    }
}

impl<H> AddEdge for HashMap<usize, HashSet<usize, H>, H>
where
    H: BuildHasher,
    HashSet<usize, H>: Default,
{
    /// # Panics
    ///
    /// Panics if `s` is not in the graph.
    fn add_edge(&mut self, s: usize, t: usize) {
        let _ = self.get_mut(&s).unwrap().insert(t);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_vec() {
        let mut graph = vec![Vec::new(); 3];

        graph.add_edge(0, 1);

        assert_eq!(graph, vec![vec![1], Vec::new(), Vec::new()]);

        graph.add_edge(0, 2);

        assert_eq!(graph, vec![vec![1, 2], Vec::new(), Vec::new()]);

        graph.add_edge(1, 2);

        assert_eq!(graph, vec![vec![1, 2], vec![2], Vec::new()]);

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(graph, vec![vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn vec_hash_set() {
        let mut graph = vec![HashSet::new(); 3];

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            vec![HashSet::from([1]), HashSet::new(), HashSet::new()]
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            vec![HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            vec![HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            vec![
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn arr_vec() {
        let mut graph = [Vec::new(), Vec::new(), Vec::new()];

        graph.add_edge(0, 1);

        assert_eq!(graph, [vec![1], Vec::new(), Vec::new()]);

        graph.add_edge(0, 2);

        assert_eq!(graph, [vec![1, 2], Vec::new(), Vec::new()]);

        graph.add_edge(1, 2);

        assert_eq!(graph, [vec![1, 2], vec![2], Vec::new()]);

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(graph, [vec![1, 2], vec![2], vec![0, 1]]);
    }

    #[test]
    fn arr_hash_set() {
        let mut graph = [HashSet::new(), HashSet::new(), HashSet::new()];

        graph.add_edge(0, 1);

        assert_eq!(graph, [HashSet::from([1]), HashSet::new(), HashSet::new()]);

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            [HashSet::from([1, 2]), HashSet::new(), HashSet::new()]
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            [HashSet::from([1, 2]), HashSet::from([2]), HashSet::new()]
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            [
                HashSet::from([1, 2]),
                HashSet::from([2]),
                HashSet::from([0, 1])
            ]
        );
    }

    #[test]
    fn hash_map_vec() {
        let mut graph = HashMap::from([(0, Vec::new()), (1, Vec::new()), (2, Vec::new())]);

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1, 2]), (1, Vec::new()), (2, Vec::new())])
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1, 2]), (1, vec![2]), (2, Vec::new())])
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            HashMap::from([(0, vec![1, 2]), (1, vec![2]), (2, vec![0, 1])])
        );
    }

    #[test]
    fn hash_map_hash_set() {
        let mut graph = HashMap::from([
            (0, HashSet::new()),
            (1, HashSet::new()),
            (2, HashSet::new()),
        ]);

        graph.add_edge(0, 1);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1])),
                (1, HashSet::new()),
                (2, HashSet::new())
            ])
        );

        graph.add_edge(0, 2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::new()),
                (2, HashSet::new())
            ])
        );

        graph.add_edge(1, 2);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([2])),
                (2, HashSet::new())
            ])
        );

        graph.add_edge(2, 0);
        graph.add_edge(2, 1);

        assert_eq!(
            graph,
            HashMap::from([
                (0, HashSet::from([1, 2])),
                (1, HashSet::from([2])),
                (2, HashSet::from([0, 1]))
            ])
        );
    }
}
