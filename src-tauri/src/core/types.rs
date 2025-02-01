use serde::{Deserialize, Serialize};

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
}

impl Default for DedupStrategy {
    fn default() -> Self {
        Self {
            similarity_method: SimilarityMethod::default(),
            similarity_threshold: 1.0,
        }
    }
}
