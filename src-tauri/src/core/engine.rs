// Main deduplication engine
use crate::core::classifier::TextClassifier;
use crate::core::types::DedupStrategy;
use std::collections::HashMap;

pub struct DeduplicationEngine {
    classifier: TextClassifier,
    texts: HashMap<String, String>,
}

impl DeduplicationEngine {
    pub fn new(strategy: DedupStrategy) -> Self {
        Self {
            classifier: TextClassifier::new(strategy),
            texts: HashMap::new(),
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
        let id = self.classifier.add_text(content.clone());
        self.add_text(id.to_string(), content.clone());
        id
    }

    pub fn get_duplicates(&mut self) -> Vec<Vec<String>> {
        let dupes = self.classifier.find_duplicates();
        dupes.iter()
            .map(|group| {
                group.iter()
                    .filter_map(|&idx| self.classifier.get_text(idx))
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

    pub fn add_text(&mut self, id: String, text: String) -> String {
        self.texts.insert(id, text.clone());
        text
    }

    pub fn get_text(&self, id: &str) -> Option<String> {
        self.texts.get(id).cloned()
    }

    pub fn get_texts(&self) -> Vec<String> {
        self.texts.values().cloned().collect()
    }

    pub fn get_text_ids(&self) -> Vec<String> {
        self.texts.keys().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.texts.clear();
    }
}
