use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashSet;
use crate::config::DynamicConfig;
use rayon::prelude::*;
use std::str::FromStr;
use strsim;
use jaro_winkler::jaro_winkler;
use rphonetic::{Encoder, Soundex};
use crate::core::classifier::TextClassifier;

// ---------------------------------------------------------------------
// Core Types
// ---------------------------------------------------------------------

/// Strategy for aggregating similarity values from the tensor
// #[derive(Debug, Clone,Copy, Serialize, Deserialize)]
// pub enum SimilarityAggregation {
//     /// Use only the first similarity value
//     First,
//     /// Take the average of all similarity values
//     Mean,
//     /// Take the maximum similarity value
//     Max,
//     /// Take the minimum similarity value
//     Min,
// }
//
// impl Default for SimilarityAggregation {
//     fn default() -> Self {
//         SimilarityAggregation::First
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimilarityMethod {
    Exact,           // Exact string matching
    Semantic,        // Semantic similarity using embeddings
    Levenshtein,    // Basic edit distance
    Fuzzy(FuzzyAlgorithm),  // Various fuzzy matching algorithms
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FuzzyAlgorithm {
    DamerauLevenshtein,  // Like Levenshtein but includes transpositions
    JaroWinkler,         // Good for names and short strings, prioritizes prefix matches
    Soundex,             // Phonetic matching
    NGram,               // N-gram based similarity
}

impl FromStr for FuzzyAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dameraulevenshtein" | "damerau_levenshtein" => Ok(FuzzyAlgorithm::DamerauLevenshtein),
            "jarowinkler" => Ok(FuzzyAlgorithm::JaroWinkler),
            "soundex" => Ok(FuzzyAlgorithm::Soundex),
            "ngram" => Ok(FuzzyAlgorithm::NGram),
            _ => Err(format!("Unknown fuzzy algorithm: {}", s))
        }
    }
}

impl Default for SimilarityMethod {
    fn default() -> Self {
        SimilarityMethod::Exact
    }
}

impl Default for FuzzyAlgorithm {
    fn default() -> Self {
        FuzzyAlgorithm::DamerauLevenshtein
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SplitStrategy {
    Words,
    Sentences,
    Paragraphs,
    Characters,
    WholeText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonScope {
    Local,
    Global,
}

/// Options for weighting the similarity.
/// Defines different strategies for adjusting similarity scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeightingStrategy {
    Linear,        // No transformation
    Quadratic,     // similarity^2 (punishes small differences)
    Exponential,   // exp(similarity) - 1
    Logarithmic,   // ln(similarity) + 1 (avoids negatives)
    WeightedMean,  // Custom weighted similarity (uses frequency, position, context)
}

// /// Defines different strategies for adjusting similarity scores.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum WeightingStrategy {
//     Linear,        // No transformation
//     Quadratic,     // similarity^2 (punishes small differences)
//     Exponential,   // exp(similarity) - 1
//     Logarithmic,   // ln(similarity) + 1 (avoids negatives)
//     WeightedMean,  // Custom weighted similarity (uses frequency, position, context)
// }

/// Custom similarity weighting based on text features.
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct SimilarityWeighting {
//     pub frequency: f64, // Importance of word frequency in similarity
//     pub position: f64,  // Importance of word position
//     pub context: f64,   // Importance of context-based similarity
//     pub strategy: WeightingStrategy, // The strategy used to scale similarity
// }

// TODO: In future versions, we will implement multiple similarity methods support
// which will include:
// 1. Composite similarity methods (Vec<SimilarityMethod>)
// 2. Weighting strategies for different methods
// 3. Aggregation strategies for combining scores

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStrategySettings {
    pub case_sensitive: Option<bool>,
    pub ignore_whitespace: Option<bool>,
    pub ignore_punctuation: Option<bool>,
    pub normalize_unicode: Option<bool>,
    pub ignore_stopwords: Option<bool>,
    pub stemming: Option<bool>,
    pub language_detection: Option<bool>,
    pub ngram_size: Option<usize>,
    pub min_length: Option<usize>,
    pub threshold: Option<f64>,
    pub split_strategy: SplitStrategy,
    pub comparison_scope: ComparisonScope,
    pub similarity_method: SimilarityMethod,
    pub use_parallel: Option<bool>,
    pub encoding_normalization: Option<bool>,
    pub adaptive_thresholding: Option<bool>,
    pub config: Option<DynamicConfig>,
}

impl Default for DedupStrategySettings {
    fn default() -> Self {
        Self {
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(true),
            language_detection: Some(true),
            ngram_size: None,
            min_length: None,
            threshold: Some(0.5),
            split_strategy: SplitStrategy::Words,
            comparison_scope: ComparisonScope::Global,
            similarity_method: SimilarityMethod::Exact,
            use_parallel: Some(true),
            encoding_normalization: Some(true),
            adaptive_thresholding: Some(false),
            config: Some(DynamicConfig::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStrategyPreset {
    pub name: String,
    pub description: String,
    pub settings: DedupStrategySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub original: String,
    pub duplicates: Vec<String>,
    pub similarity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupResults {
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub stats: DedupStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStats {
    pub total_items: usize,
    pub unique_items: usize,
    pub duplicate_groups: usize,
}

pub struct DedupManager {
    texts: Vec<String>,
    strategy: DedupStrategySettings,
    classifier: TextClassifier,
}

impl DedupManager {
    pub fn new(strategy: DedupStrategySettings, _similarity_method: SimilarityMethod) -> Self {
        Self {
            strategy: strategy.clone(),
            texts: Vec::new(),
            classifier: TextClassifier::new(strategy),
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        self.texts.clear();
        self.classifier.clear();
        Ok(())
    }

    pub fn add_text(&mut self, text: String) -> usize {
        let idx = self.classifier.add_text(text.clone());
        self.texts.push(text);
        idx
    }

    pub fn update_strategy(&mut self, strategy_json: &str) -> Result<()> {
        let strategy: DedupStrategySettings = serde_json::from_str(strategy_json)?;
        self.strategy = strategy.clone();
        self.classifier.update_strategy(strategy);
        Ok(())
    }

    pub fn get_strategy(&self) -> String {
        serde_json::to_string(&self.strategy).unwrap_or_default()
    }

    pub fn get_text(&self, id: usize) -> Option<String> {
        self.classifier.get_text(id)
    }

    pub fn deduplicate_texts(&mut self) -> Result<DedupResults, String> {
        // Early return if no texts
        if self.texts.is_empty() {
            return Ok(DedupResults {
                duplicate_groups: Vec::new(),
                stats: DedupStats {
                    total_items: 0,
                    unique_items: 0,
                    duplicate_groups: 0,
                }
            });
        }

        // Use TextClassifier to find duplicates
        let duplicate_indices = self.classifier.find_duplicates();
        
        // Convert indices to DuplicateGroups
        let duplicate_groups: Vec<DuplicateGroup> = duplicate_indices
            .into_iter()
            .map(|indices| {
                let texts: Vec<String> = indices
                    .iter()
                    .filter_map(|&idx| self.texts.get(idx).cloned())
                    .collect();
                
                DuplicateGroup {
                    original: texts[0].clone(),
                    duplicates: texts[1..].to_vec(),
                    similarity: 0.0, // TODO: Calculate similarity
                }
            })
            .collect();

        // Calculate stats
        let total_items = self.texts.len();
        let duplicate_groups_count = duplicate_groups.len();
        let unique_items = total_items - duplicate_groups
            .iter()
            .map(|group| group.duplicates.len())
            .sum::<usize>();

        Ok(DedupResults {
            duplicate_groups,
            stats: DedupStats {
                total_items,
                unique_items,
                duplicate_groups: duplicate_groups_count,
            }
        })
    }
}
