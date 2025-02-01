// Smart classifier for content type detection
pub struct SmartClassifier;

use std::collections::HashSet;
use super::types::{DedupStrategy, SimilarityMethod};
use crate::core::semantic::SemanticAnalyzer;

pub struct TextClassifier {
    texts: Vec<String>,
    strategy: DedupStrategy,
    analyzer: SemanticAnalyzer,
}

impl Default for TextClassifier {
    fn default() -> Self {
        Self::new(DedupStrategy::default())
    }
}

impl TextClassifier {
    pub fn new(strategy: DedupStrategy) -> Self {
        Self {
            texts: Vec::new(),
            strategy,
            analyzer: SemanticAnalyzer::new(),
        }
    }

    pub fn add_text(&mut self, text: String) -> usize {
        self.texts.push(text);
        self.texts.len() - 1
    }

    pub fn get_text(&self, idx: usize) -> Option<String> {
        self.texts.get(idx).cloned()
    }

    pub fn get_all_texts(&self) -> Vec<String> {
        self.texts.clone()
    }

    pub fn clear(&mut self) {
        self.texts.clear();
    }

    pub fn calculate_similarity(&mut self, text1: &str, text2: &str) -> f64 {
        match self.strategy.similarity_method {
            SimilarityMethod::Semantic => {
                self.analyzer.calculate_similarity(text1, text2) as f64
            },
            SimilarityMethod::Exact => self.calculate_exact_similarity(text1, text2),
            SimilarityMethod::Fuzzy => self.calculate_fuzzy_similarity(text1, text2),
        }
    }

    fn calculate_exact_similarity(&self, text1: &str, text2: &str) -> f64 {
        if text1 == text2 {
            1.0
        } else {
            0.0
        }
    }

    fn calculate_fuzzy_similarity(&self, text1: &str, text2: &str) -> f64 {
        let len1 = text1.len();
        let len2 = text2.len();
        
        if len1 == 0 || len2 == 0 {
            return 0.0;
        }
        
        let distance = levenshtein(text1, text2);
        let max_len = len1.max(len2);
        
        1.0 - (distance as f64 / max_len as f64)
    }

    pub fn find_duplicates(&mut self) -> Vec<Vec<usize>> {
        let mut groups: Vec<Vec<usize>> = Vec::new();
        let mut processed: HashSet<usize> = HashSet::new();
        let texts = self.texts.clone(); // Clone texts to avoid borrow checker issues

        for i in 0..texts.len() {
            if processed.contains(&i) {
                continue;
            }

            let mut group = Vec::new();
            group.push(i);
            processed.insert(i);

            for j in (i + 1)..texts.len() {
                if processed.contains(&j) {
                    continue;
                }

                let similarity = self.calculate_similarity(&texts[i], &texts[j]);
                println!("Comparing '{}' with '{}': similarity = {}", texts[i], texts[j], similarity);
                if similarity >= self.strategy.similarity_threshold as f64 {
                    group.push(j);
                    processed.insert(j);
                }
            }

            if group.len() > 1 {
                groups.push(group);
            }
        }

        groups
    }

    pub fn update_strategy(&mut self, strategy: DedupStrategy) {
        self.strategy = strategy;
    }

    pub fn get_strategy(&self) -> &DedupStrategy {
        &self.strategy
    }
}

#[cfg(test)]
pub struct TestTextClassifier {
    texts: Vec<String>,
    strategy: DedupStrategy,
    analyzer: crate::core::semantic::MockSemanticAnalyzer,
}

#[cfg(test)]
impl TestTextClassifier {
    pub fn new(strategy: DedupStrategy, analyzer: crate::core::semantic::MockSemanticAnalyzer) -> Self {
        Self {
            texts: Vec::new(),
            strategy,
            analyzer,
        }
    }

    pub fn add_text(&mut self, text: String) -> usize {
        self.texts.push(text);
        self.texts.len() - 1
    }

    pub fn find_duplicates(&mut self) -> Vec<Vec<usize>> {
        let mut groups: Vec<Vec<usize>> = Vec::new();
        let mut processed: HashSet<usize> = HashSet::new();
        let texts = self.texts.clone();

        for i in 0..texts.len() {
            if processed.contains(&i) {
                continue;
            }

            let mut group = Vec::new();
            group.push(i);
            processed.insert(i);

            for j in (i + 1)..texts.len() {
                if processed.contains(&j) {
                    continue;
                }

                let similarity = match self.strategy.similarity_method {
                    SimilarityMethod::Semantic => {
                        self.analyzer.calculate_similarity(&texts[i], &texts[j]) as f64
                    },
                    _ => 0.0,
                };

                if similarity >= self.strategy.similarity_threshold as f64 {
                    group.push(j);
                    processed.insert(j);
                }
            }

            if group.len() > 1 {
                groups.push(group);
            }
        }

        groups
    }
}

fn levenshtein(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }
    
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }
    
    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::{ComparisonScope, SplitStrategy, DedupStrategy, SimilarityMethod};
    use crate::core::semantic::MockSemanticAnalyzer;

    #[test]
    fn test_exact_duplicates() {
        let mut classifier = TextClassifier::new(DedupStrategy {
            similarity_method: SimilarityMethod::Exact,
            similarity_threshold: 1.0,
            case_sensitive: true,
            ignore_whitespace: false,
            ignore_punctuation: false,
            normalize_unicode: true,
            split_strategy: SplitStrategy::WholeText,
            comparison_scope: ComparisonScope::Global,
            min_length: 1,
            use_parallel: false,
        });

        // Add some test texts
        classifier.add_text("Hello".to_string());
        classifier.add_text("Hello".to_string());
        classifier.add_text("World".to_string());

        let duplicates = classifier.find_duplicates();
        assert_eq!(duplicates.len(), 1); // One group for duplicates
        assert_eq!(duplicates[0].len(), 2); // "Hello" appears twice
    }

    #[test]
    fn test_semantic_similarity() {
        let analyzer = MockSemanticAnalyzer::new()
            .with_similarity(
                "The cat quickly jumped over the fence",
                "A feline leaped across the barrier",
                0.9,
            )
            .with_similarity(
                "The cat quickly jumped over the fence",
                "My dog sleeps on the couch",
                0.1,
            )
            .with_similarity(
                "A feline leaped across the barrier",
                "My dog sleeps on the couch",
                0.1,
            );

        let mut classifier = TestTextClassifier::new(
            DedupStrategy {
                case_sensitive: false,
                ignore_whitespace: true,
                ignore_punctuation: true,
                normalize_unicode: true,
                split_strategy: SplitStrategy::WholeText,
                comparison_scope: ComparisonScope::Global,
                min_length: 5,
                similarity_threshold: 0.8,
                similarity_method: SimilarityMethod::Semantic,
                use_parallel: false,
            },
            analyzer,
        );

        // Add test texts
        classifier.add_text("The cat quickly jumped over the fence".to_string());
        classifier.add_text("A feline leaped across the barrier".to_string());
        classifier.add_text("My dog sleeps on the couch".to_string());

        let duplicates = classifier.find_duplicates();
        
        // Should find one group of similar texts
        assert_eq!(duplicates.len(), 1, "Should have one group for similar texts");
        assert_eq!(duplicates[0].len(), 2, "Group should contain both similar sentences");
        
        // Verify the right texts are grouped
        let group = &duplicates[0];
        assert!(group.contains(&0) && group.contains(&1), "Group should contain indices 0 and 1");
    }
}
