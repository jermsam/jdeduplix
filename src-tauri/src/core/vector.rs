// Vector indexing components
pub struct VectorIndex;

use blake3;
use unicode_normalization::UnicodeNormalization;
use serde::Serialize;
use strsim::jaro_winkler;
use super::types::{SimilarityMethod, DedupStrategy};
use super::semantic::{SemanticAnalyzer, DocumentVector};

#[derive(Debug, Serialize)]
pub struct TextVector {
    pub text: String,
    pub normalized_text: String,
    #[serde(skip)]
    hash: blake3::Hash,
    pub strategy: DedupStrategy,
    #[serde(skip)]
    doc_vector: Option<DocumentVector>,
}

impl TextVector {
    pub fn new(text: String, strategy: &DedupStrategy) -> Self {
        let normalized_text = Self::normalize_text(strategy, &text);
        let hash = blake3::hash(text.as_bytes());
        
        Self {
            text,
            normalized_text,
            hash,
            strategy: strategy.clone(),
            doc_vector: None,
        }
    }

    pub fn update_strategy(&mut self, strategy: &DedupStrategy) {
        self.strategy = strategy.clone();
        self.normalized_text = Self::normalize_text(strategy, &self.text);
        self.hash = blake3::hash(self.normalized_text.as_bytes());
    }

    fn normalize_text(strategy: &DedupStrategy, text: &str) -> String {
        let mut result = text.to_string();

        if !strategy.case_sensitive {
            result = result.to_lowercase();
        }

        if strategy.normalize_unicode {
            result = result.nfkc().collect();
        }

        if strategy.ignore_whitespace {
            result = result.split_whitespace().collect::<Vec<_>>().join(" ");
        }

        if strategy.ignore_punctuation {
            result = result.chars()
                .filter(|c| !c.is_ascii_punctuation())
                .collect();
        }

        result
    }

    pub fn prepare_semantic(&mut self, analyzer: &SemanticAnalyzer) {
        self.doc_vector = Some(analyzer.create_vector(&self.normalized_text));
    }

    pub fn is_similar(&self, other: &TextVector, threshold: f64) -> bool {
        self.calculate_similarity(other) >= threshold
    }

    pub fn calculate_similarity(&self, other: &TextVector) -> f64 {
        match self.strategy.similarity_method {
            SimilarityMethod::Exact => {
                if self.hash == other.hash {
                    1.0
                } else {
                    0.0
                }
            }
            SimilarityMethod::Fuzzy => {
                // For case-sensitive comparison, first check if the texts match exactly
                if self.strategy.case_sensitive && self.text != other.text {
                    return 0.0;
                }
                
                // For case-insensitive or if case-sensitive and texts match
                jaro_winkler(
                    &self.normalized_text,
                    &other.normalized_text,
                )
            }
            SimilarityMethod::Semantic => {
                match (&self.doc_vector, &other.doc_vector) {
                    (Some(vec1), Some(vec2)) => vec1.cosine_similarity(vec2),
                    _ => 0.0, // If vectors aren't prepared, consider them dissimilar
                }
            }
        }
    }

    pub fn hash(&self) -> blake3::Hash {
        self.hash
    }

    pub fn content(&self) -> &str {
        &self.text
    }

    pub fn normalized_content(&self) -> &str {
        &self.normalized_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let strategy = DedupStrategy {
            case_sensitive: false,
            ignore_whitespace: true,
            ignore_punctuation: true,
            ..Default::default()
        };
        
        let text = "Hello,    World! Test123";
        let vector = TextVector::new(text.to_string(), &strategy);
        let normalized = vector.normalized_text;
        assert_eq!(normalized.trim(), "hello world test123");
    }

    #[test]
    fn test_similarity() {
        let strategy = DedupStrategy::default();
        let vector1 = TextVector::new("hello world".to_string(), &strategy);
        let vector2 = TextVector::new("hello world".to_string(), &strategy);
        let vector3 = TextVector::new("different text".to_string(), &strategy);

        assert!(vector1.is_similar(&vector2, 0.8));
        assert!(!vector1.is_similar(&vector3, 0.8));
    }

    #[test]
    fn test_edge_cases() {
        let strategy = DedupStrategy::default();
        let empty = TextVector::new("".to_string(), &strategy);
        let special = TextVector::new("Hello! @#$%".to_string(), &strategy);
        let numbers = TextVector::new("123 456".to_string(), &strategy);

        assert!(empty.is_similar(&empty, 0.8));
        assert!(special.is_similar(&special, 0.8));
        assert!(numbers.is_similar(&numbers, 0.8));
    }

    #[test]
    fn test_case_sensitivity() {
        let case_sensitive_strategy = DedupStrategy {
            case_sensitive: true,
            ..Default::default()
        };
        let case_insensitive_strategy = DedupStrategy {
            case_sensitive: false,
            ..Default::default()
        };

        // Case-sensitive comparisons
        let hello_upper_sensitive = TextVector::new("Hello".to_string(), &case_sensitive_strategy);
        let hello_lower_sensitive = TextVector::new("hello".to_string(), &case_sensitive_strategy);
        let hello_mixed_sensitive = TextVector::new("HeLLo".to_string(), &case_sensitive_strategy);

        // Case-insensitive comparisons
        let hello_upper_insensitive = TextVector::new("Hello".to_string(), &case_insensitive_strategy);
        let hello_lower_insensitive = TextVector::new("hello".to_string(), &case_insensitive_strategy);
        let hello_mixed_insensitive = TextVector::new("HeLLo".to_string(), &case_insensitive_strategy);

        // Case-sensitive tests
        assert!(!hello_upper_sensitive.is_similar(&hello_lower_sensitive, 0.8));
        assert!(!hello_upper_sensitive.is_similar(&hello_mixed_sensitive, 0.8));

        // Case-insensitive tests
        assert!(hello_upper_insensitive.is_similar(&hello_lower_insensitive, 0.8));
        assert!(hello_upper_insensitive.is_similar(&hello_mixed_insensitive, 0.8));
    }
}
