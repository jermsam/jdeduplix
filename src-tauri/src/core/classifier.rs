// Smart classifier for content type detection
pub struct SmartClassifier;

use super::vector::{TextVector, DedupStrategy, SplitStrategy, ComparisonScope};
use std::collections::HashMap;
use hex;

#[derive(Default)]
pub struct TextClassifier {
    texts: Vec<TextVector>,
    duplicates: HashMap<String, Vec<usize>>,
    pub strategy: DedupStrategy,
}

impl TextClassifier {
    pub fn new(strategy: DedupStrategy) -> Self {
        Self {
            texts: Vec::new(),
            duplicates: HashMap::new(),
            strategy,
        }
    }

    pub fn add_text(&mut self, content: String) -> usize {
        println!("Adding text: {}", content);
        
        let chunks = match self.strategy.split_strategy {
            SplitStrategy::Characters => content.chars().map(|c| c.to_string()).collect(),
            SplitStrategy::Words => content.split_whitespace().map(|s| s.to_string()).collect(),
            SplitStrategy::Sentences => content
                .split(&['.', '!', '?'][..])
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            SplitStrategy::Paragraphs => content
                .split("\n\n")
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
            SplitStrategy::WholeText => vec![content.clone()],
        };

        let start_index = self.texts.len();
        for chunk in chunks {
            if chunk.len() < self.strategy.min_length {
                continue;
            }

            let vector = TextVector::new(chunk, self.strategy.clone());
            let hash_str = hex::encode(vector.hash().as_bytes());
            let index = self.texts.len();
            
            // Check for similar texts based on strategy
            if self.strategy.similarity_threshold < 1.0 {
                for (i, existing) in self.texts.iter().enumerate() {
                    let similarity = vector.similarity(existing);
                    if similarity >= self.strategy.similarity_threshold {
                        self.duplicates.entry(format!("sim_{}", i)).or_default().push(index);
                        self.duplicates.get_mut(&format!("sim_{}", i)).unwrap().push(i);
                    }
                }
            } else {
                // Exact matches only
                self.duplicates.entry(hash_str).or_default().push(index);
            }
            
            self.texts.push(vector);
        }

        start_index
    }

    pub fn find_duplicates(&self) -> Vec<Vec<&str>> {
        let mut result = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for indices in self.duplicates.values() {
            if indices.len() > 1 {
                // Sort indices to ensure consistent grouping
                let mut sorted_indices: Vec<_> = indices.iter().copied().collect();
                sorted_indices.sort();

                // Use the first index as group identifier
                if seen.insert(sorted_indices[0]) {
                    let group: Vec<&str> = sorted_indices.iter()
                        .map(|&i| self.texts[i].content())
                        .collect();
                    result.push(group);
                }
            }
        }

        result
    }

    pub fn update_strategy(&mut self, strategy: DedupStrategy) {
        self.strategy = strategy;
    }

    pub fn clear(&mut self) {
        println!("Clearing all texts and duplicates");
        self.texts.clear();
        self.duplicates.clear();
    }
}
