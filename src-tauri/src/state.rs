use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashSet;
use crate::config::DynamicConfig;
use rayon::prelude::*;

// ---------------------------------------------------------------------
// Core Types
// ---------------------------------------------------------------------

/// Strategy for aggregating similarity values from the tensor
#[derive(Debug, Clone,Copy, Serialize, Deserialize)]
pub enum SimilarityAggregation {
    /// Use only the first similarity value
    First,
    /// Take the average of all similarity values
    Mean,
    /// Take the maximum similarity value
    Max,
    /// Take the minimum similarity value
    Min,
}

impl Default for SimilarityAggregation {
    fn default() -> Self {
        SimilarityAggregation::First
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimilarityMethod {
    Exact,
    Semantic,
    Levenshtein,
}

impl Default for SimilarityMethod {
    fn default() -> Self {
        SimilarityMethod::Exact
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SplitStrategy {
    Words,
    Sentences,
    Paragraphs,
    Characters,
    WholeText,
}

// /// Options for weighting the similarity.
/// Defines different strategies for adjusting similarity scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeightingStrategy {
    Linear,        // No transformation
    Quadratic,     // similarity^2 (punishes small differences)
    Exponential,   // exp(similarity) - 1
    Logarithmic,   // ln(similarity) + 1 (avoids negatives)
    WeightedMean,  // Custom weighted similarity (uses frequency, position, context)
}

/// Custom similarity weighting based on text features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityWeighting {
    pub frequency: f64, // Importance of word frequency in similarity
    pub position: f64,  // Importance of word position
    pub context: f64,   // Importance of context-based similarity
    pub strategy: WeightingStrategy, // The strategy used to scale similarity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStrategySettings {
    pub similarity_aggregation: Option<SimilarityAggregation>,
    pub case_sensitive: Option<bool>,
    pub ignore_whitespace: Option<bool>,
    pub ignore_punctuation: Option<bool>,
    pub normalize_unicode: Option<bool>,
    pub split_strategy: SplitStrategy,
    pub comparison_scope: Option<String>,
    pub min_length: Option<usize>,
    pub similarity_threshold: Option<f64>,
    pub similarity_method: Option<String>,
    pub use_parallel: Option<bool>,
    pub ignore_stopwords: Option<bool>,
    pub stemming: Option<bool>,
    pub ngram_size: Option<usize>,
    pub language_detection: Option<bool>,
    pub encoding_normalization: Option<bool>,
    pub similarity_weighting: Option<SimilarityWeighting>,
    pub adaptive_thresholding: Option<bool>,
    pub config: Option<DynamicConfig>,
}



impl Default for DedupStrategySettings {
    fn default() -> Self {
        Self {
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(false),
            normalize_unicode: Some(false),
            split_strategy: SplitStrategy::Words,
            comparison_scope: Some("Global".to_string()),
            min_length: Some(10),
            similarity_threshold: Some(0.8),
            similarity_method: Some("Exact".to_string()),
            use_parallel: Some(true),
            ignore_stopwords: Some(false),
            stemming: Some(false),
            ngram_size: Some(3),
            language_detection: Some(false),
            encoding_normalization: Some(true),
            similarity_weighting: None,
            adaptive_thresholding: Some(false),
            config: Some(DynamicConfig::default()),
            similarity_aggregation: Some(SimilarityAggregation::First),
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
    strategy: DedupStrategySettings,
    texts: Vec<String>,
}

impl DedupManager {
    pub fn new(strategy: DedupStrategySettings, _similarity_method: SimilarityMethod) -> Self {
        Self {
            strategy,
            texts: Vec::new(),
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        self.texts.clear();
        Ok(())
    }

    pub fn add_text(&mut self, text: String) -> usize {
        let index = self.texts.len();
        self.texts.push(text);
        index
    }

    pub fn update_strategy(&mut self, strategy_json: &str) -> Result<()> {
        self.strategy = serde_json::from_str(strategy_json)?;
        Ok(())
    }

    pub fn get_strategy(&self) -> String {
        serde_json::to_string(&self.strategy).unwrap_or_default()
    }

    pub fn get_text(&self, id: usize) -> Option<String> {
        self.texts.get(id).cloned()
    }

    pub fn deduplicate_texts(&self) -> Result<DedupResults, String> {
        use rayon::prelude::*;
        
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

        // Step 1: Normalize all texts based on settings and detect languages
        let normalized_texts: Vec<(String, String, Option<whatlang::Info>)> = self.texts.par_iter()
            .map(|text| {
                let lang_info = if self.strategy.language_detection.unwrap_or(false) {
                    whatlang::detect(text)
                } else {
                    None
                };
                
                let normalized = if self.strategy.ignore_stopwords.unwrap_or(false) {
                    let stop_words = self.strategy.config.as_ref()
                        .map(|config| config.get_stop_words_for_text(text))
                        .unwrap_or_default();
                    
                    let mut processed = text.to_string();
                    for word in stop_words {
                        processed = processed.replace(&word, " ");
                    }
                    processed
                } else {
                    text.to_string()
                };

                let normalized = if !self.strategy.case_sensitive.unwrap_or(true) {
                    normalized.to_lowercase()
                } else {
                    normalized
                };

                let normalized = if self.strategy.ignore_whitespace.unwrap_or(false) {
                    normalized.split_whitespace().collect::<Vec<_>>().join(" ")
                } else {
                    normalized
                };

                let normalized = if self.strategy.ignore_punctuation.unwrap_or(false) {
                    normalized.chars()
                        .filter(|c| !c.is_ascii_punctuation())
                        .collect::<String>()
                } else {
                    normalized
                };

                (text.to_string(), normalized, lang_info)
            })
            .collect();

        // Step 2: Create similarity matrix using parallel processing
        let n = normalized_texts.len();
        let similarity_matrix: Vec<Vec<f64>> = (0..n)
            .into_par_iter()
            .map(|i| {
                let (_, text1, lang1) = &normalized_texts[i];
                (0..n)
                    .map(|j| {
                        if i == j {
                            return 1.0;
                        }
                        let (_, text2, lang2) = &normalized_texts[j];
                        
                        // Adjust threshold based on text length if adaptive thresholding is enabled
                        let base_threshold = self.strategy.similarity_threshold.unwrap_or(0.8);
                        let threshold = if self.strategy.adaptive_thresholding.unwrap_or(false) {
                            let avg_len = (text1.len() + text2.len()) as f64 / 2.0;
                            // Increase threshold for shorter texts
                            if avg_len < 50.0 {
                                (base_threshold + 0.1).min(1.0)
                            } else if avg_len > 200.0 {
                                (base_threshold - 0.1).max(0.0)
                            } else {
                                base_threshold
                            }
                        } else {
                            base_threshold
                        };

                        // Calculate similarity based on method
                        match self.strategy.similarity_method.as_deref() {
                            Some("exact") => {
                                if text1 == text2 { 1.0 } else { 0.0 }
                            },
                            Some("levenshtein") => {
                                let distance = levenshtein::levenshtein(text1, text2);
                                let max_len = text1.len().max(text2.len());
                                1.0 - (distance as f64 / max_len as f64)
                            },
                            Some("semantic") => {
                                // Check if languages match for semantic comparison
                                if let (Some(l1), Some(l2)) = (lang1, lang2) {
                                    if l1.lang() != l2.lang() {
                                        return 0.0; // Different languages
                                    }
                                }
                                
                                // TODO: Implement proper semantic similarity
                                // For now, use a combination of n-gram and Levenshtein
                                let ngram_size = self.strategy.ngram_size.unwrap_or(3) as usize;
                                let ngrams1 = text1.chars()
                                    .collect::<Vec<_>>()
                                    .windows(ngram_size)
                                    .map(|w| w.iter().collect::<String>())
                                    .collect::<HashSet<_>>();
                                let ngrams2 = text2.chars()
                                    .collect::<Vec<_>>()
                                    .windows(ngram_size)
                                    .map(|w| w.iter().collect::<String>())
                                    .collect::<HashSet<_>>();
                                
                                let intersection = ngrams1.intersection(&ngrams2).count() as f64;
                                let union = ngrams1.union(&ngrams2).count() as f64;
                                
                                if union == 0.0 {
                                    0.0
                                } else {
                                    intersection / union
                                }
                            },
                            _ => 0.0,
                        }
                    })
                    .collect()
            })
            .collect();

        // Step 3: Group similar texts using the similarity matrix
        let mut groups = Vec::new();
        let mut processed = HashSet::new();

        for i in 0..n {
            if processed.contains(&i) {
                continue;
            }

            let mut group = vec![i];
            for j in (i + 1)..n {
                if processed.contains(&j) {
                    continue;
                }

                let similarity = similarity_matrix[i][j];
                let threshold = self.strategy.similarity_threshold.unwrap_or(0.8);
                
                if similarity >= threshold {
                    group.push(j);
                    processed.insert(j);
                }
            }

            if group.len() > 1 {
                let original = normalized_texts[group[0]].0.clone();
                let duplicates = group[1..].iter()
                    .map(|&idx| normalized_texts[idx].0.clone())
                    .collect();
                
                // Use the maximum similarity score for the group
                let similarity = group[1..].iter()
                    .map(|&idx| similarity_matrix[group[0]][idx])
                    .fold(0.0, f64::max);

                groups.push(DuplicateGroup {
                    original,
                    duplicates,
                    similarity,
                });
            }
            processed.insert(i);
        }

        // Step 4: Calculate statistics
        let stats = DedupStats {
            total_items: self.texts.len(),
            unique_items: self.texts.len() - processed.len() + groups.len(),
            duplicate_groups: groups.len(),
        };

        Ok(DedupResults { duplicate_groups: groups, stats })
    }
}
