//! Submodule implementing the report method for the `Corpus` struct, which
//! returns a struct containing several informations regarding the corpus.
//! The `CorpusReport` struct is displayable.

use std::fmt;
use std::fmt::Display;

use crate::prelude::*;

/// A struct containing several informations regarding the corpus.
#[derive(Debug, Clone)]
pub struct CorpusReport {
    /// The number of keys in the corpus.
    pub number_of_keys: usize,
    /// The number of grams in the corpus.
    pub number_of_grams: usize,
    /// The number of edges in the corpus.
    pub number_of_edges: usize,
}

impl Display for CorpusReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We display the report using a markdown-like format.
        writeln!(f, "# Corpus Report")?;

        writeln!(f, "* Number of keys: {}", self.number_of_keys.underscored())?;
        writeln!(
            f,
            "* Number of grams: {}",
            self.number_of_grams.underscored()
        )?;
        writeln!(
            f,
            "* Number of edges: {}",
            self.number_of_edges.underscored()
        )
    }
}

impl<KS, NG, K, G> Corpus<KS, NG, K, G>
where
    NG: Ngram,
    KS: Keys<NG>,
    KS::K: AsRef<K>,
    K: Key<NG, NG::G> + ?Sized,
    G: WeightedBipartiteGraph,
{
    /// Returns a report of the corpus.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ngrammatic::prelude::*;
    ///
    /// let animals: Corpus<_, TriGram<char>> = Corpus::from(ANIMALS);
    /// let report = animals.report();
    ///
    /// assert_eq!(report.number_of_keys, 699);
    /// assert_eq!(report.number_of_grams, 2534);
    /// assert_eq!(report.number_of_edges, 18080);
    /// ```
    pub fn report(&self) -> CorpusReport {
        let number_of_keys = self.keys.len();
        let number_of_grams = self.ngrams.len();
        let number_of_edges = self.graph.number_of_edges() * 2;
        CorpusReport {
            number_of_keys,
            number_of_grams,
            number_of_edges,
        }
    }
}
