// Smart classifier for content type detection
pub struct SmartClassifier;

use std::collections::HashSet;
use super::vector::TextVector;
use super::types::{DedupStrategy, SimilarityMethod};
use super::semantic::SemanticAnalyzer;

#[derive(Debug)]
pub struct TextClassifier {
    vectors: Vec<TextVector>,
    strategy: DedupStrategy,
    semantic_analyzer: SemanticAnalyzer,
}

impl TextClassifier {
    pub fn new(strategy: DedupStrategy) -> Self {
        Self {
            vectors: Vec::new(),
            strategy,
            semantic_analyzer: SemanticAnalyzer::new(),
        }
    }

    pub fn get_strategy(&self) -> &DedupStrategy {
        &self.strategy
    }

    pub fn update_strategy(&mut self, strategy: DedupStrategy) {
        self.strategy = strategy;
        
        // Update all vectors with new strategy
        for vector in &mut self.vectors {
            vector.update_strategy(&self.strategy);
            
            // If switching to semantic similarity, prepare document vectors
            if matches!(self.strategy.similarity_method, SimilarityMethod::Semantic) {
                vector.prepare_semantic(&self.semantic_analyzer);
            }
        }
    }

    pub fn add_text(&mut self, text: String) -> usize {
        // If using semantic similarity, add to corpus first
        if matches!(self.strategy.similarity_method, SimilarityMethod::Semantic) {
            self.semantic_analyzer.add_document(&text);
        }
        
        // Create and add the vector
        let mut vector = TextVector::new(text.clone(), &self.strategy);
        
        // If using semantic similarity, prepare the document vector
        if matches!(self.strategy.similarity_method, SimilarityMethod::Semantic) {
            vector.prepare_semantic(&self.semantic_analyzer);
        }
        
        let idx = self.vectors.len();
        self.vectors.push(vector);
        idx
    }

    pub fn get_text(&self, idx: usize) -> Option<&str> {
        self.vectors.get(idx).map(|v| v.content())
    }

    pub fn get_all_texts(&self) -> Vec<String> {
        self.vectors.iter().map(|v| v.content().to_string()).collect()
    }

    pub fn clear(&mut self) {
        self.vectors.clear();
        // Reset semantic analyzer if using semantic similarity
        if matches!(self.strategy.similarity_method, SimilarityMethod::Semantic) {
            self.semantic_analyzer = SemanticAnalyzer::new();
        }
    }

    pub fn find_duplicates(&self) -> Vec<Vec<String>> {
        let mut duplicates = Vec::new();
        let mut processed = HashSet::new();

        for i in 0..self.vectors.len() {
            if processed.contains(&i) {
                continue;
            }

            let mut group = Vec::new();
            group.push(self.vectors[i].content().to_string());
            processed.insert(i);

            for j in i + 1..self.vectors.len() {
                if processed.contains(&j) {
                    continue;
                }

                if self.vectors[i].is_similar(&self.vectors[j], self.strategy.similarity_threshold) {
                    group.push(self.vectors[j].content().to_string());
                    processed.insert(j);
                }
            }

            if group.len() > 1 {
                duplicates.push(group);
            }
        }

        duplicates
    }

    pub fn get_duplicates(&self) -> Vec<Vec<String>> {
        self.find_duplicates()
    }

    pub fn verify_duplicates(&self) -> bool {
        for group in &self.get_duplicates() {
            if group.len() < 2 {
                return false;
            }

            let first_text = group[0].clone();
            for i in 1..group.len() {
                if group[i] != first_text {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::core::types::{ComparisonScope, SplitStrategy};

    use super::*;

    #[test]
    fn test_exact_duplicates() {
        let mut classifier = TextClassifier::new(DedupStrategy {
            case_sensitive: true,
            ignore_whitespace: false,
            ignore_punctuation: false,
            normalize_unicode: false,
            split_strategy: SplitStrategy::WholeText,
            comparison_scope: ComparisonScope::Global,
            min_length: 1,
            similarity_threshold: 1.0,
            similarity_method: SimilarityMethod::Exact,
            use_parallel: false,
        });

        classifier.add_text("Hello".to_string());
        classifier.add_text("Hello".to_string());
        classifier.add_text("World".to_string());

        let duplicates = classifier.find_duplicates();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
    }

    #[test]
    fn test_semantic_similarity() {
        let mut classifier = TextClassifier::new(DedupStrategy {
            case_sensitive: false,
            ignore_whitespace: true,
            ignore_punctuation: true,
            normalize_unicode: true,
            split_strategy: SplitStrategy::WholeText,
            comparison_scope: ComparisonScope::Global,
            min_length: 5,
            similarity_threshold: 0.15, // Lower threshold based on semantic test results
            similarity_method: SimilarityMethod::Semantic,
            use_parallel: false,
        });

        // These sentences have different words but similar meanings
        classifier.add_text("The cat quickly jumped over the fence".to_string());
        classifier.add_text("A feline leaped across the barrier".to_string());
        classifier.add_text("My dog sleeps on the couch".to_string()); // Different meaning

        let duplicates = classifier.find_duplicates();
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].len(), 2);
    }
}
