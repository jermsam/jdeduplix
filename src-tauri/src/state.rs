use serde::{Deserialize, Serialize};
use crate::core::types::{DedupStrategy, SimilarityMethod};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupStats {
    pub total_items: usize,
    pub unique_items: usize,
    pub duplicate_groups: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupResults {
    pub groups: Vec<Vec<usize>>,
    pub stats: DedupStats,
}

pub struct DedupManager {
    strategy: DedupStrategy,
    texts: Vec<String>,
}

impl DedupManager {
    pub fn new(strategy: DedupStrategy, _similarity_method: SimilarityMethod) -> Self {
        Self {
            strategy,
            texts: Vec::new(),
        }
    }

    pub fn clear(&mut self) -> Result<()> {
        self.texts.clear();
        Ok(())
    }

    pub fn add_text(&mut self, text: String) -> usize {
        let index = self.texts.len();
        self.texts.push(text);
        index
    }

    pub fn update_strategy(&mut self, strategy_json: &str) -> Result<()> {
        self.strategy = serde_json::from_str(strategy_json)?;
        Ok(())
    }

    pub fn get_strategy(&self) -> String {
        serde_json::to_string(&self.strategy).unwrap_or_default()
    }

    pub fn get_text(&self, id: usize) -> Option<String> {
        self.texts.get(id).cloned()
    }

    pub fn deduplicate_texts(&self) -> Result<DedupResults> {
        let mut groups = Vec::new();
        let mut processed = std::collections::HashSet::new();

        for (i, text1) in self.texts.iter().enumerate() {
            if processed.contains(&i) {
                continue;
            }

            let mut group = vec![i];
            for (j, text2) in self.texts.iter().enumerate().skip(i + 1) {
                if processed.contains(&j) {
                    continue;
                }

                let is_duplicate = match self.strategy.similarity_method {
                    SimilarityMethod::Exact => text1 == text2,
                    SimilarityMethod::Levenshtein => {
                        let distance = levenshtein::levenshtein(text1, text2);
                        let max_len = text1.len().max(text2.len());
                        let similarity = 1.0 - (distance as f64 / max_len as f64);
                        similarity >= self.strategy.similarity_threshold
                    }
                    SimilarityMethod::Semantic => {
                        // TODO: Implement semantic similarity
                        false
                    }
                };

                if is_duplicate {
                    group.push(j);
                    processed.insert(j);
                }
            }

            if group.len() > 1 {
                groups.push(group);
            }
            processed.insert(i);
        }

        let stats = DedupStats {
            total_items: self.texts.len(),
            unique_items: self.texts.len() - processed.len() + groups.len(),
            duplicate_groups: groups.len(),
        };

        Ok(DedupResults { groups, stats })
    }
}
