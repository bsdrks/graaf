//! Predecessor trees
//!
//! Predecessor trees are produced by many graph algorithms, e.g., those defined
//! in [`graaf::algo::bfs`] and [`graaf::algo::dijkstra`].

use std::collections::HashSet;

/// Trace a path in a predecessor tree.
///
/// # Arguments
///
/// * `pred`: The predecessor tree.
/// * `s`: The source vertex of the path.
/// * `t`: The target vertex of the path.
///
/// # Examples
///
/// ```
/// use graaf::algo::predecessor::search;
///
/// let pred = [
///     Some(1), // 0 -> 1
///     Some(2), // 1 -> 2
///     Some(3), // 2 -> 3
///     None,    // 3 -> x
/// ];
///
/// // 0 -> 1 -> 2 -> 3
/// assert_eq!(search(&pred, 0, 3), Some(vec![0, 1, 2, 3]));
/// ```
#[must_use]
pub fn search(pred: &[Option<usize>], mut s: usize, t: usize) -> Option<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut path = vec![s];

    while let Some(&u) = pred.get(s) {
        if let Some(v) = u {
            if !visited.insert(v) {
                break;
            }

            if v != s {
                path.push(v);
            }

            if v == t {
                return Some(path);
            }

            s = v;
        } else {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_empty_s_ne_t() {
        assert_eq!(search(&[], 0, 1), None);
    }

    #[test]
    fn trace_empty_s_eq_t() {
        assert_eq!(search(&[], 0, 0), None);
    }

    #[test]
    fn trace_singleton_s_ne_t() {
        let pred = [Some(0)];

        assert_eq!(search(&pred, 0, 1), None);
    }

    #[test]
    fn trace_singleton_s_eq_t() {
        let pred = [Some(0)];

        assert_eq!(search(&pred, 0, 0), Some(vec![0]));
    }

    #[test]
    fn trace_no_path() {
        let pred = [
            Some(1), // 0 -> 1
            Some(2), // 1 -> 2
            None,    // 2 -> x
            None,    // 3 -> x
        ];

        // 0 -> 1 -> 2 -> x
        assert_eq!(search(&pred, 0, 3), None);
    }

    #[test]
    fn trace_cycle() {
        let pred = [
            Some(1), // 0 -> 1
            Some(2), // 1 -> 2
            Some(0), // 2 -> 1
            None,    // 3 -> x
        ];

        // 0 -> 1 -> 2 -> 1 -> ... -> x
        assert_eq!(search(&pred, 0, 3), None);
    }

    #[test]
    fn trace_path_s_eq_t() {
        let pred = [
            Some(1), // 0 -> 1
            Some(2), // 1 -> 2
            Some(0), // 2 -> 0
            None,    // 3 -> x
        ];

        // 0 -> 1 -> 2 -> 0
        assert_eq!(search(&pred, 0, 0), Some(vec![0, 1, 2, 0]));
    }

    #[test]
    fn trace_path_s_ne_t() {
        let pred = [
            Some(1), // 0 -> 1
            Some(2), // 1 -> 2
            Some(3), // 2 -> 3
            None,    // 3 -> x
        ];

        // 0 -> 1 -> 2 -> 3
        assert_eq!(search(&pred, 0, 3), Some(vec![0, 1, 2, 3]));
    }
}
