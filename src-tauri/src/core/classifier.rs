// Smart classifier for content type detection
pub struct SmartClassifier;

use std::collections::HashSet;
use crate::state::DedupStrategy;
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

    pub fn calculate_similarity(&self, text1: &str, text2: &str) -> f64 {
    1.00
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

