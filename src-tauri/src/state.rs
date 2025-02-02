use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::config::DynamicConfig;

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
pub struct SimilarityWeighting {
    pub frequency: f64,
    pub position: f64,
    pub context: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStrategySettings {
    pub case_sensitive: Option<bool>,
    pub ignore_whitespace: Option<bool>,
    pub ignore_punctuation: Option<bool>,
    pub normalize_unicode: Option<bool>,
    pub split_strategy: Option<String>,
    pub comparison_scope: Option<String>,
    pub min_length: Option<u32>,
    pub similarity_threshold: Option<f64>,
    pub similarity_method: Option<String>,
    pub use_parallel: Option<bool>,
    pub ignore_stopwords: Option<bool>,
    pub stemming: Option<bool>,
    pub ngram_size: Option<u32>,
    pub language_detection: Option<bool>,
    pub encoding_normalization: Option<bool>,
    pub similarity_weighting: Option<SimilarityWeighting>,
    pub adaptive_thresholding: Option<bool>,
    pub config: Option<DynamicConfig>,
}

impl DedupStrategySettings {
    pub fn get_default_by_preset(name: &str) -> Self {
        use crate::presets::DEDUP_PRESETS;
        DEDUP_PRESETS
            .iter()
            .find(|preset| preset.name == name)
            .map(|preset| preset.settings.clone())
            .unwrap_or_else(|| DEDUP_PRESETS[0].settings.clone())
    }
}

impl Default for DedupStrategySettings {
    fn default() -> Self {
        Self {
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(false),
            normalize_unicode: Some(false),
            split_strategy: Some("Words".to_string()),
            comparison_scope: Some("Global".to_string()),
            min_length: Some(10),
            similarity_threshold: Some(0.8),
            similarity_method: Some("Exact".to_string()),
            use_parallel: Some(true),
            ignore_stopwords: Some(false),
            stemming: Some(false),
            ngram_size: Some(3),
            language_detection: Some(false),
            encoding_normalization: Some(true),
            similarity_weighting: None,
            adaptive_thresholding: Some(false),
            config: Some(DynamicConfig::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStrategyPreset {
    pub name: String,
    pub description: String,
    pub settings: DedupStrategySettings,
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
    strategy: DedupStrategySettings,
    texts: Vec<String>,
}

impl DedupManager {
    pub fn new(strategy: DedupStrategySettings, _similarity_method: SimilarityMethod) -> Self {
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
                    Some("exact") => text1 == text2,
                    Some("levenshtein") => {
                        let distance = levenshtein::levenshtein(text1, text2);
                        let max_len = text1.len().max(text2.len());
                        let similarity = 1.0 - (distance as f64 / max_len as f64);
                        similarity >= self.strategy.similarity_threshold.unwrap_or(0.0)
                    }
                    Some("semantic") => {
                        // TODO: Implement semantic similarity
                        false
                    }
                    _ => false,
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
                    Some("exact") => 1.0,
                    Some("levenshtein") => {
                        let text1 = &self.texts[group[0]];
                        let max_similarity = group[1..].iter().map(|&idx| {
                            let text2 = &self.texts[idx];
                            let distance = levenshtein::levenshtein(text1, text2);
                            let max_len = text1.len().max(text2.len());
                            1.0 - (distance as f64 / max_len as f64)
                        }).fold(0.0, f64::max);
                        max_similarity
                    },
                    Some("semantic") => 1.0, // TODO: Implement semantic similarity
                    _ => 0.0,
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
