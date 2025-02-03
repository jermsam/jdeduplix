// Smart classifier for content type detection
pub struct SmartClassifier;
// use std::cell::RefCell;
use std::collections::HashSet;
use rayon::prelude::*;
use crate::core::semantic::SemanticAnalyzer;
use crate::state::{DedupStrategySettings, SplitStrategy};
use crate::config::DynamicConfig;
use rust_stemmers::{Algorithm, Stemmer};
use unicode_normalization::UnicodeNormalization;
use std::sync::RwLock;

/// Text classifier for detecting duplicates
pub struct TextClassifier {
    texts: Vec<String>,
    strategy: DedupStrategySettings,
    semantic_analyzer: RwLock<SemanticAnalyzer>,
    config: DynamicConfig,
}

impl Default for TextClassifier {
    fn default() -> Self {
        Self {
            texts: Vec::new(),
            strategy: DedupStrategySettings::default(),
            semantic_analyzer: RwLock::new(SemanticAnalyzer::default()),
            config: DynamicConfig::default(),
        }
    }
}

impl TextClassifier {
    /// Create a new text classifier with the given strategy
    pub fn new(strategy: DedupStrategySettings) -> Self {
        Self {
            texts: Vec::new(),
            strategy,
            semantic_analyzer: RwLock::new(SemanticAnalyzer::default()),
            config: DynamicConfig::default(),
        }
    }

    /// Add a text to the classifier
    pub fn add_text(&mut self, text: String) -> usize {
        self.texts.push(text);
        self.texts.len() - 1
    }

    /// Get a text by index
    pub fn get_text(&self, idx: usize) -> Option<String> {
        self.texts.get(idx).cloned()
    }

    /// Get all texts
    pub fn get_all_texts(&self) -> Vec<String> {
        self.texts.clone()
    }

    /// Clear all texts
    pub fn clear(&mut self) {
        self.texts.clear();
    }

    fn normalize_text(&self, text: &str) -> String {
        let mut normalized = text.to_string();

        // Case sensitivity
        if !self.strategy.case_sensitive.unwrap_or(true) {
            normalized = normalized.to_lowercase();
        }

        // Whitespace handling
        if self.strategy.ignore_whitespace.unwrap_or(false) {
            normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
        }

        // Punctuation handling
        if self.strategy.ignore_punctuation.unwrap_or(false) {
            normalized = normalized.chars().filter(|c| !c.is_ascii_punctuation()).collect();
        }

        // Unicode normalization
        if self.strategy.normalize_unicode.unwrap_or(false) {
            normalized = normalized.nfd().collect::<String>();
        }

        // Stopwords removal
        if self.strategy.ignore_stopwords.unwrap_or(false) {
            let stop_words = self.config.get_stop_words_for_text(&normalized);
            normalized = normalized
                .split_whitespace()
                .filter(|word| !stop_words.contains(&word.to_lowercase()))
                .collect::<Vec<_>>()
                .join(" ");
        }

        // Stemming
        if self.strategy.stemming.unwrap_or(false) {
            let en_stemmer = Stemmer::create(Algorithm::English);
            normalized = normalized
                .split_whitespace()
                .map(|word| en_stemmer.stem(word).to_string())
                .collect::<Vec<_>>()
                .join(" ");
        }

        // Encoding normalization
        if self.strategy.encoding_normalization.unwrap_or(false) {
            normalized = normalized.chars()
                .filter(|c| c.is_ascii() || c.is_alphanumeric())
                .collect();
        }

        normalized
    }

    fn split_text(&self, text: &str) -> Vec<String> {
        match self.strategy.split_strategy {
            SplitStrategy::Characters => text.chars().map(|c| c.to_string()).collect(),
            SplitStrategy::Words => text.split_whitespace().map(|s| s.to_string()).collect(),
            SplitStrategy::Sentences => {
                let delimiters = self.config.merge_sentence_delimiters();
                text.split(|c| delimiters.contains(&c))
                    .map(|s| s.trim().to_string())
                    .collect()
            }
            SplitStrategy::Paragraphs => {
                let delimiter = self.config.get_paragraph_delimiters();
                text.split(delimiter)
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }
            SplitStrategy::WholeText => vec![text.to_string()],
            _ => vec![text.to_string()],
        }
    }

    /// Calculate the similarity between two texts
    pub fn calculate_similarity(&mut self, text1: &str, text2: &str) -> f64 {
        // Apply minimum length filter
        let min_length: usize = self.strategy.min_length.unwrap_or_else(|| self.config.base.default_min_length  as usize);
        if text1.len() < min_length || text2.len() < min_length {
            return 0.0;
        }

        let normalized1 = self.normalize_text(text1);
        let normalized2 = self.normalize_text(text2);

        match &self.strategy.similarity_method {
            Some(method) => match method.as_str() {
                "exact" => {
                    if normalized1 == normalized2 {
                        1.0
                    } else {
                        0.0
                    }
                }
                "semantic" => {
                    self.semantic_analyzer.write().unwrap().calculate_semantic_similarity(
                        &normalized1,
                        &normalized2,
                        &self.strategy
                    )
                }
                "levenshtein" => {
                    levenshtein::levenshtein(&normalized1, &normalized2) as f64
                }
                _ => 0.0
            }
            None => 0.0
        }
    }

    /// Find duplicate texts using the configured strategy
    pub fn find_duplicates(&mut self) -> Vec<Vec<usize>> {
        if self.texts.is_empty() {
            return vec![];
        }

        let threshold = self.strategy.similarity_threshold.unwrap_or(0.8); // Default to 0.8 if no threshold is specified
        let use_parallel = self.strategy.use_parallel.unwrap_or_default();

        // First, prepare all the texts in a sequential manner
        let prepared_texts: Vec<(usize, String, String)> = self.texts.iter()
            .enumerate()
            .map(|(idx, text)| {
                let normalized = self.normalize_text(text);
                (idx, text.to_string(), normalized)
            })
            .collect();

        // Map phase: Extract features from each text
        let features: Vec<(usize, HashSet<String>)> = if use_parallel {
            prepared_texts.par_iter()
                .map(|(idx, _, normalized)| {
                    let words: HashSet<String> = self.split_text(normalized)
                        .into_iter()
                        .collect();
                    (*idx, words)
                })
                .collect()
        } else {
            prepared_texts.iter()
                .map(|(idx, _, normalized)| {
                    let words: HashSet<String> = self.split_text(normalized)
                        .into_iter()
                        .collect();
                    (*idx, words)
                })
                .collect()
        };

        // Reduce phase: Group similar texts
        let mut groups: Vec<Vec<usize>> = Vec::new();
        let mut processed: HashSet<usize> = HashSet::new();

        for (i, words1) in features.iter() {
            if processed.contains(i) {
                continue;
            }

            let mut group = vec![*i];
            processed.insert(*i);

            // Find similar texts
            for (j, words2) in features.iter().skip(*i + 1) {
                if processed.contains(j) {
                    continue;
                }

                // Calculate Jaccard similarity
                let intersection = words1.intersection(words2).count();
                let union = words1.union(words2).count();
                let similarity = if union > 0 {
                    intersection as f64 / union as f64
                } else {
                    0.0
                };

                if similarity >= threshold {
                    group.push(*j);
                    processed.insert(*j);
                }
            }

            if group.len() > 1 {
                groups.push(group);
            }
        }

        groups
    }

    /// Update the strategy
    pub fn update_strategy(&mut self, strategy: DedupStrategySettings) {
        self.strategy = strategy;
    }

    /// Get the current strategy
    pub fn get_strategy(&self) -> &DedupStrategySettings {
        &self.strategy
    }

    pub fn update_config(&mut self, config: DynamicConfig) {
        self.config = config;
    }
}
