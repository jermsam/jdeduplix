// Vector indexing components
pub struct VectorIndex;

use blake3::Hash;
use unicode_normalization::UnicodeNormalization;
use serde::{Deserialize, Serialize};
use levenshtein;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SplitStrategy {
    Characters,
    Words,
    Sentences,
    Paragraphs,
    WholeText
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonScope {
    WithinUnit,     // Compare within the current unit (e.g., within each paragraph)
    AcrossUnits,    // Compare across all units (e.g., across all paragraphs)
    Both            // Compare both within and across units
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStrategy {
    pub case_sensitive: bool,
    pub ignore_whitespace: bool,
    pub ignore_punctuation: bool,
    pub normalize_unicode: bool,
    pub split_strategy: SplitStrategy,
    pub comparison_scope: ComparisonScope,
    pub min_length: usize,      // Minimum length of unit to consider (e.g., min word length)
    pub similarity_threshold: f64, // Threshold for considering texts as similar (0.0 to 1.0)
}

impl Default for DedupStrategy {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            ignore_whitespace: true,
            ignore_punctuation: false,
            normalize_unicode: false,
            split_strategy: SplitStrategy::Paragraphs,
            comparison_scope: ComparisonScope::Both,
            min_length: 1,
            similarity_threshold: 0.8, // Default similarity threshold of 80%
        }
    }
}

pub struct TextVector {
    content: String,
    strategy: DedupStrategy,
    hash: Hash,
}

impl TextVector {
    pub fn new(content: String, strategy: DedupStrategy) -> Self {
        let normalized = if !strategy.case_sensitive {
            content.to_lowercase()
        } else {
            content.clone()
        };

        let normalized = if strategy.ignore_whitespace {
            normalized.split_whitespace().collect::<Vec<_>>().join(" ")
        } else {
            normalized
        };

        let hash = blake3::hash(normalized.as_bytes());
        Self { content, strategy, hash }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    /// Normalizes text for consistent comparison:
    /// - Converts to lowercase if not case sensitive
    /// - Trims whitespace
    /// - Normalizes internal whitespace
    pub fn normalize_text(text: &str, strategy: &DedupStrategy) -> String {
        let mut result = text.to_string();

        // Case sensitivity
        if !strategy.case_sensitive {
            result = result.to_lowercase();
        }

        // Unicode normalization
        if strategy.normalize_unicode {
            result = result.nfkc().collect();
        }

        // Punctuation handling
        if strategy.ignore_punctuation {
            result = result.chars()
                .filter(|c| !c.is_ascii_punctuation())
                .collect();
        }

        // Whitespace handling
        if strategy.ignore_whitespace {
            // Normalize internal whitespace to single space
            result = result
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
        }

        result.trim().to_string()
    }

    pub fn similarity(&self, other: &TextVector) -> f64 {
        let text1 = Self::normalize_text(&self.content, &self.strategy);
        let text2 = Self::normalize_text(&other.content, &self.strategy);
        
        if text1.is_empty() || text2.is_empty() {
            return 0.0;
        }

        let distance = levenshtein::levenshtein(&text1, &text2) as f64;
        let max_len = text1.len().max(text2.len()) as f64;
        
        if max_len == 0.0 {
            1.0
        } else {
            1.0 - (distance / max_len)
        }
    }
}
