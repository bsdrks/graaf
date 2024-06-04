//! Predecessor trees utilities
//!
//! Many graph algorithms produce predecessor trees, e.g., those defined in
//! [`bfs`] and [`dijkstra`].
//!
//! # Examples
//!
//! ```
//! use graaf::algo::predecessor::search;
//!
//! let pred = [Some(1), Some(2), Some(3), None];
//!
//! assert_eq!(search(&pred, 0, 3), Some(vec![0, 1, 2, 3]));
//! ```
//!
//! [`bfs`]: crate::algo::bfs
//! [`dijkstra`]: crate::algo::dijkstra

use std::collections::HashSet;

/// Searches a predecessor tree for a path from a source vertex to a target
/// vertex.
///
/// # Arguments
///
/// * `pred`: The predecessor tree.
/// * `s`: The source vertex of the path.
/// * `t`: The target vertex of the path.
///
/// # Returns
///
/// If a path from `s` to `t` is found, the function returns it. Otherwise,
/// returns `None`.
///
/// # Examples
///
/// ```
/// use graaf::algo::predecessor::search;
///
/// let pred = [Some(1), Some(2), Some(3), None];
///
/// assert_eq!(search(&pred, 0, 3), Some(vec![0, 1, 2, 3]));
/// ```
#[must_use]
pub fn search(pred: &[Option<usize>], s: usize, t: usize) -> Option<Vec<usize>> {
    search_by(pred, s, |&v, _| v == t)
}

/// Searches a predecessor tree for a path from a source vertex to a target
/// vertex that satisfies a predicate.
///
/// # Arguments
///
/// * `pred`: The predecessor tree.
/// * `s`: The source vertex of the path.
/// * `is_target`: A predicate determining whether a vertex is the target.
///
/// # Returns
///
/// If it finds a target, it returns the path from the source to the target.
/// Otherwise, it returns `None`.
///
/// # Examples
///
/// ```
/// use graaf::algo::predecessor::search_by;
///
/// let pred = [Some(1), Some(2), Some(3), None];
///
/// assert_eq!(search_by(&pred, 0, |&v, _| v > 1), Some(vec![0, 1, 2]));
/// ```
///
/// ```
/// use graaf::algo::predecessor::search_by;
///
/// let pred = [Some(1), Some(2), Some(3), None, Some(0)];
///
/// assert_eq!(
///     search_by(&pred, 0, |_, u| u.is_none()),
///     Some(vec![0, 1, 2, 3])
/// );
/// ```
#[must_use]
pub fn search_by<F>(pred: &[Option<usize>], mut s: usize, is_target: F) -> Option<Vec<usize>>
where
    F: Fn(&usize, &Option<usize>) -> bool,
{
    let mut visited = HashSet::new();
    let mut path = vec![s];

    while let Some(&u) = pred.get(s) {
        if is_target(&s, &u) {
            return Some(path);
        }

        if let Some(v) = u {
            if !visited.insert(v) {
                break;
            }

            if v != s {
                path.push(v);
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
    fn search_empty_s_ne_t() {
        assert_eq!(search(&[], 0, 1), None);
    }

    #[test]
    fn search_empty_s_eq_t() {
        assert_eq!(search(&[], 0, 0), None);
    }

    #[test]
    fn search_singleton_s_ne_t() {
        let pred = [Some(0)];

        assert_eq!(search(&pred, 0, 1), None);
    }

    #[test]
    fn search_singleton_s_eq_t() {
        let pred = [Some(0)];

        assert_eq!(search(&pred, 0, 0), Some(vec![0]));
    }

    #[test]
    fn search_no_path() {
        let pred = [Some(1), Some(2), None, None];

        assert_eq!(search(&pred, 0, 3), None);
    }

    #[test]
    fn search_cycle() {
        let pred = [Some(1), Some(2), Some(0), None];

        assert_eq!(search(&pred, 0, 3), None);
    }

    #[test]
    fn search_path_s_eq_t() {
        let pred = [Some(1), Some(2), Some(0), None];

        assert_eq!(search(&pred, 0, 0), Some(vec![0]));
    }

    #[test]
    fn search_path_s_ne_t() {
        let pred = [Some(1), Some(2), Some(3), None];

        assert_eq!(search(&pred, 0, 3), Some(vec![0, 1, 2, 3]));
    }

    #[test]
    fn search_by_empty_s_ne_t() {
        assert_eq!(search_by(&[], 0, |&t, _| t == 1), None);
    }

    #[test]
    fn search_by_empty_s_eq_t() {
        assert_eq!(search_by(&[], 0, |&t, _| t == 0), None);
    }

    #[test]
    fn search_by_singleton_s_ne_t() {
        let pred = [Some(0)];

        assert_eq!(search_by(&pred, 0, |&t, _| t == 1), None);
    }

    #[test]
    fn search_by_singleton_s_eq_t() {
        let pred = [Some(0)];

        assert_eq!(search_by(&pred, 0, |&t, _| t == 0), Some(vec![0]));
    }

    #[test]
    fn search_by_no_path() {
        let pred = [Some(1), Some(2), None, None];

        assert_eq!(search_by(&pred, 0, |&t, _| t == 3), None);
    }

    #[test]
    fn search_by_cycle() {
        let pred = [Some(1), Some(2), Some(0), None];

        assert_eq!(search_by(&pred, 0, |&t, _| t == 3), None);
    }

    #[test]
    fn search_by_path_s_eq_t() {
        let pred = [Some(1), Some(2), Some(0), None];

        assert_eq!(search_by(&pred, 0, |&t, _| t == 0), Some(vec![0]));
    }

    #[test]
    fn search_by_path_s_ne_t() {
        let pred = [Some(1), Some(2), Some(3), None];

        assert_eq!(search_by(&pred, 0, |&t, _| t == 3), Some(vec![0, 1, 2, 3]));
    }
}
