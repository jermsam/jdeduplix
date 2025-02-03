// Smart classifier for content type detection
pub struct SmartClassifier;

use std::collections::HashSet;
use rayon::prelude::*;
use crate::core::semantic::{SemanticAnalyzer, DocumentVector};
use crate::core::vector::TextDocument;
use crate::state::DedupStrategySettings;
use crate::config::DynamicConfig;
use burn::module::Module;
use rust_stemmers::{Algorithm, Stemmer};
use unicode_normalization::UnicodeNormalization;

/// Text classifier for detecting duplicates
pub struct TextClassifier {
    texts: Vec<String>,
    strategy: DedupStrategySettings,
    semantic_analyzer: SemanticAnalyzer,
    config: DynamicConfig,
}

impl Default for TextClassifier {
    fn default() -> Self {
        Self {
            texts: Vec::new(),
            strategy: DedupStrategySettings::default(),
            semantic_analyzer: SemanticAnalyzer::default(),
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
            semantic_analyzer: SemanticAnalyzer::default(),
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
            normalized = normalized.nfkd().collect::<String>();
        }

        // Stopwords removal
        if self.strategy.ignore_stopwords.unwrap_or(false) {
            let stop_words = self.config.merge_stop_words();
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
        match self.strategy.split_strategy.as_deref() {
            Some("Characters") => text.chars().map(|c| c.to_string()).collect(),
            Some("Words") => text.split_whitespace().map(|s| s.to_string()).collect(),
            Some("Sentences") => {
                let delimiters = self.config.merge_sentence_delimiters();
                text.split(|c| delimiters.contains(&c))
                    .map(|s| s.trim().to_string())
                    .collect()
            }
            Some("Paragraphs") => {
                let delimiter = self.config.get_paragraph_delimiters();
                text.split(delimiter)
                    .map(|s| s.trim().to_string())
                    .collect()
            }
            _ => vec![text.to_string()],
        }
    }

    /// Calculate the similarity between two texts
    pub fn calculate_similarity(&self, text1: &str, text2: &str) -> f64 {
        // Apply minimum length filter
        let min_length = self.strategy.min_length.unwrap_or_else(|| self.config.base.default_min_length);
        if text1.len() < min_length as usize || text2.len() < min_length as usize {
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
                    self.semantic_analyzer.calculate_semantic_similarity(
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
        let use_parallel = self.strategy.use_parallel.unwrap_or(false);
        let threshold = self.strategy.similarity_threshold.unwrap_or(0.8);

        let mut groups: Vec<Vec<usize>> = Vec::new();
        let mut processed: HashSet<usize> = HashSet::new();

        let text_range: Vec<usize> = (0..self.texts.len()).collect();

        for i in text_range {
            if processed.contains(&i) {
                continue;
            }

            let text1 = &self.texts[i];
            let mut group = vec![i];

            // Use parallel iterator if enabled
            let similar_indices: Vec<usize> = if use_parallel {
                ((i + 1)..self.texts.len())
                    .into_par_iter()
                    .filter(|&j| !processed.contains(&j))
                    .filter(|&j| self.calculate_similarity(text1, &self.texts[j]) >= threshold)
                    .collect()
            } else {
                ((i + 1)..self.texts.len())
                    .filter(|&j| !processed.contains(&j))
                    .filter(|&j| self.calculate_similarity(text1, &self.texts[j]) >= threshold)
                    .collect()
            };

            group.extend(similar_indices);
            
            if group.len() > 1 {
                processed.extend(group.iter());
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
