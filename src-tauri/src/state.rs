use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimilarityMethod {
    Exact,
    Semantic,
    Levenshtein,
}

impl Default for SimilarityMethod {
    fn default() -> Self {
        SimilarityMethod::Exact
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStrategy {
    pub similarity_method: SimilarityMethod,
    pub similarity_threshold: f64,
    pub case_sensitive: bool,
    pub ignore_punctuation: bool,
    pub ignore_whitespace: bool,
    pub normalize_unicode: bool,
    pub min_length: Option<u32>,
    pub comparison_scope: Option<String>,
    pub split_strategy: Option<String>,
}

impl Default for DedupStrategy {
    fn default() -> Self {
        Self {
            similarity_method: SimilarityMethod::default(),
            similarity_threshold: 1.0,
            case_sensitive: false,
            ignore_punctuation: true,
            ignore_whitespace: true,
            normalize_unicode: true,
            min_length: None,
            comparison_scope: None,
            split_strategy: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub original: String,
    pub duplicates: Vec<String>,
    pub similarity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupResults {
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub stats: DedupStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStats {
    pub total_items: usize,
    pub unique_items: usize,
    pub duplicate_groups: usize,
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
                let original = self.texts[group[0]].clone();
                let duplicates = group[1..].iter()
                    .map(|&idx| self.texts[idx].clone())
                    .collect();
                let similarity = match self.strategy.similarity_method {
                    SimilarityMethod::Exact => 1.0,
                    SimilarityMethod::Levenshtein => {
                        let text1 = &self.texts[group[0]];
                        let max_similarity = group[1..].iter().map(|&idx| {
                            let text2 = &self.texts[idx];
                            let distance = levenshtein::levenshtein(text1, text2);
                            let max_len = text1.len().max(text2.len());
                            1.0 - (distance as f64 / max_len as f64)
                        }).fold(0.0, f64::max);
                        max_similarity
                    },
                    SimilarityMethod::Semantic => 1.0, // TODO: Implement semantic similarity
                };
                groups.push(DuplicateGroup {
                    original,
                    duplicates,
                    similarity,
                });
            }
            processed.insert(i);
        }

        let stats = DedupStats {
            total_items: self.texts.len(),
            unique_items: self.texts.len() - processed.len() + groups.len(),
            duplicate_groups: groups.len(),
        };

        Ok(DedupResults { duplicate_groups: groups, stats })
    }
}
