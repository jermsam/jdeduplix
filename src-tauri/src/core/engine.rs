// Main deduplication engine
use super::classifier::TextClassifier;
use super::vector::DedupStrategy;

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
        self.classifier = TextClassifier::new(strategy);
    }

    pub fn process_text(&mut self, content: String) -> usize {
        println!("Engine: Processing text: {}", content);
        self.classifier.add_text(content)
    }

    pub fn get_duplicates(&self) -> Vec<Vec<&str>> {
        println!("Engine: Getting duplicates");
        let dupes = self.classifier.find_duplicates();
        println!("Engine: Found {} duplicate groups", dupes.len());
        dupes
    }

    pub fn get_strategy(&self) -> DedupStrategy {
        self.classifier.strategy.clone()
    }

    pub fn clear_duplicates(&mut self) {
        self.classifier.clear();
    }
}
