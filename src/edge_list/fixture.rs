//! Unweighted edge list fixtures for testing and benchmarking
//!
//! [`bang_jensen_196`]:
//!
//! ![Bang-Jensen, 196](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_196.svg)
//!
//! [`bang_jensen_34`]:
//!
//! ![Bang-Jensen, 34](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_34.svg)
//!
//! [`bang_jensen_94`]:
//!
//! ![Bang-Jensen, 94](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_94.svg)
//!
//! [`kattis_builddeps`]:
//!
//! ![Kattis, builddeps](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_builddeps.svg)
//!
//! [`kattis_cantinaofbabel_1`]:
//!
//! ![Kattis, cantinaofbabel, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_1.svg)
//!
//! [`kattis_cantinaofbabel_2`]:
//!
//! ![Kattis, cantinaofbabel, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_2.svg)
//!
//! [`kattis_escapewallmaria_1`]:
//!
//! ![Kattis, escapewallmaria, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_1.svg)
//!
//! [`kattis_escapewallmaria_2`]:
//!
//! ![Kattis, escapewallmaria, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_2.svg)
//!
//! [`kattis_escapewallmaria_3`]:
//!
//! ![Kattis, escapewallmaria, sample 3](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_escapewallmaria_3.svg)

use {
    super::Digraph,
    crate::adjacency_list::fixture,
};

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 196.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ![Bang-Jensen, 196](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/bang_jensen_196.svg)
///
/// ```text
/// 0 -> {1, 4, 7}
/// 1 -> {0, 2, 7}
/// 2 -> {3}
/// 3 -> {2, 4}
/// 4 -> {2}
/// 5 -> {6}
/// 6 -> {7}
/// 7 -> {5}
/// ```
#[must_use]
pub fn bang_jensen_196() -> Digraph {
    Digraph::from(fixture::bang_jensen_196())
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 34.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {4}
/// 1 -> {0}
/// 2 -> {1, 3, 5}
/// 3 -> {}
/// 4 -> {}
/// 5 -> {4}
/// ```
#[must_use]
pub fn bang_jensen_34() -> Digraph {
    Digraph::from(fixture::bang_jensen_34())
}

/// Jørgen Bang-Jensen and Gregory Z. Gutin. 2009. Digraphs: Theory,
/// Algorithms and Applications (2nd ed.). Springer, London, 94.
/// <https://doi.org/10.1007/978-1-84800-998-1>
///
/// ```text
/// 0 -> {1, 2}
/// 1 -> {3}
/// 2 -> {1, 3, 4, 5}
/// 3 -> {5}
/// 4 -> {6}
/// 5 -> {}
/// 6 -> {}
/// ```
#[must_use]
pub fn bang_jensen_94() -> Digraph {
    Digraph::from(fixture::bang_jensen_94())
}

/// Jeroen Bransen. 2015. Build Dependencies. Kattis.
/// <https://open.kattis.com/problems/builddeps>
///
/// ```text
/// 0 = gmp
/// 1 = solution
/// 2 = base
/// 3 = set
/// 4 = map
/// 5 = queue
/// ```
///
/// ```text
/// 0 -> {3, 4}
/// 1 -> {}
/// 2 -> {3, 4, 5}
/// 3 -> {1}
/// 4 -> {1}
/// 5 -> {1}
/// ```
#[must_use]
pub fn kattis_builddeps() -> Digraph {
    Digraph::from(fixture::kattis_builddeps())
}

/// David Sturgill. 2015. Cantina of Babel. (Sample Input 1). Kattis.
/// <https://open.kattis.com/problems/cantinaofbabel>
///
/// ![Kattis, cantinaofbabel, sample 1](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_1.svg)
///
/// ```text
/// 0: Jabba-the-Hutt
/// 1: Huttese
/// 2: Bib-Fortuna
/// 3: Basic
/// 4: Boba-Fett
/// 5: Chewbacca
/// 6: Shyriiwook
/// 7: Luke
/// 8: Jawaese
/// 9: Binary
/// 10: Grakchawwaa
/// 11: R2D2
/// ```
///
/// ```text
/// Jabba-the-Hutt Huttese
/// Bib-Fortuna Huttese Basic
/// Boba-Fett Basic Huttese
/// Chewbacca Shyriiwook Basic
/// Luke Basic Jawaese Binary
/// Grakchawwaa Shyriiwook Basic Jawaese
/// R2D2 Binary Basic
/// ```
///
/// ```text
/// 0 -> {1}
/// 1 -> {0, 2, 4}
/// 2 -> {1}
/// 3 -> {2, 4, 5, 7, 10, 11}
/// 4 -> {3}
/// 5 -> {6}
/// 6 -> {5, 10}
/// 7 -> {3}
/// 8 -> {7, 10}
/// 9 -> {7, 11}
/// 10 -> {6}
/// 11 -> {9}
/// ```
#[must_use]
pub fn kattis_cantinaofbabel_1() -> Digraph {
    Digraph::from(fixture::kattis_cantinaofbabel_1())
}

/// David Sturgill. 2015. Cantina of Babel. (Sample Input 2). Kattis.
/// <https://open.kattis.com/problems/cantinaofbabel>
///
/// ![Kattis, cantinaofbabel, sample 2](https://raw.githubusercontent.com/bsdrks/graaf-images/main/out/kattis_cantinaofbabel_2.svg)
///
/// ```text
/// 0: Fran
/// 1: French
/// 2: Italian
/// 3: Enid
/// 4: English
/// 5: German
/// 6: George
/// 7: Ian
/// 8: Spanish
/// 9: Spencer
/// 10: Portugese
/// 11: Polly
/// ```
///
/// ```text
/// Fran French Italian
/// Enid English German
/// George German Italian
/// Ian Italian French Spanish
/// Spencer Spanish Portugese
/// Polly Portugese Spanish
/// ```
///
/// ```text
/// 0 -> {1}
/// 1 -> {0, 7}
/// 2 -> {0, 5, 7}
/// 3 -> {4}
/// 4 -> {3}
/// 5 -> {3, 6}
/// 6 -> {5}
/// 7 -> {2}
/// 8 -> {7, 9, 11}
/// 9 -> {8}
/// 10 -> {9, 11}
/// 11 -> {10}
/// ```
#[must_use]
pub fn kattis_cantinaofbabel_2() -> Digraph {
    Digraph::from(fixture::kattis_cantinaofbabel_2())
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 1)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 0 -> {}
/// 1 -> {}
/// 2 -> {}
/// 3 -> {}
/// 4 -> {}
/// 5 -> {6, 9}
/// 6 -> {5}
/// 7 -> {}
/// 8 -> {}
/// 9 -> {5, 13}
/// 10 -> {}
/// 11 -> {}
/// 12 -> {}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_1() -> Digraph {
    Digraph::from(fixture::kattis_escapewallmaria_1())
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 2)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 0 -> {}
/// 1 -> {}
/// 2 -> {}
/// 3 -> {}
/// 4 -> {}
/// 5 -> {6, 9}
/// 6 -> {5}
/// 7 -> {}
/// 8 -> {}
/// 9 -> {5}
/// 10 -> {}
/// 11 -> {}
/// 12 -> {13}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_2() -> Digraph {
    Digraph::from(fixture::kattis_escapewallmaria_2())
}

/// Arash Behpour. 2019. Escape Wall Maria. Kattis. (Sample Input 3)
/// <https://open.kattis.com/problems/escapewallmaria>
///
/// ```text
/// 0 -> {}
/// 1 -> {2, 5}
/// 2 -> {1, 6}
/// 3 -> {}
/// 4 -> {}
/// 5 -> {1, 6, 9}
/// 6 -> {2, 5}
/// 7 -> {}
/// 8 -> {}
/// 9 -> {5, 13}
/// 10 -> {}
/// 11 -> {}
/// 12 -> {13}
/// 13 -> {9, 12}
/// ```
#[must_use]
pub fn kattis_escapewallmaria_3() -> Digraph {
    Digraph::from(fixture::kattis_escapewallmaria_3())
}
