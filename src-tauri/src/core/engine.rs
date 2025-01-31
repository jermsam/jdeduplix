// Main deduplication engine
use super::classifier::TextClassifier;
use crate::core::types::DedupStrategy;

pub struct DeduplicationEngine {
    classifier: TextClassifier,
}

impl DeduplicationEngine {
    pub fn new() -> Self {
        Self {
            classifier: TextClassifier::new(DedupStrategy::default()),
        }
    }

    pub fn update_strategy(&mut self, strategy: DedupStrategy) {
        println!("Engine: Updating strategy to {:?}", strategy);
        // Create new classifier with updated strategy and clear any existing content
        self.classifier = TextClassifier::new(strategy);
        self.clear_duplicates();
    }

    pub fn process_text(&mut self, content: String) -> usize {
        println!("Engine: Processing text: {}", content);
        // Clear existing content before processing new text
        self.classifier.clear();
        // Return the index where the text was added
        self.classifier.add_text(content)
    }

    pub fn get_duplicates(&self) -> Vec<Vec<&str>> {
        let dupes = self.classifier.find_duplicates();
        dupes.iter()
            .map(|group| {
                group.iter()
                    .filter_map(|&idx| self.classifier.get_text(idx))
                    .map(|s| s.as_str())
                    .collect()
            })
            .collect()
    }

    pub fn get_strategy(&self) -> &DedupStrategy {
        self.classifier.get_strategy()
    }

    pub fn clear_duplicates(&mut self) {
        self.classifier.clear();
    }
}
