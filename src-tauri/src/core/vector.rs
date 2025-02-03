// Vector indexing components

use blake3;

use serde::{Serialize, Deserialize};
use strsim::jaro_winkler;
use crate::state::{ DedupStrategySettings, SimilarityMethod};
use super::semantic::{SemanticAnalyzer, DocumentVector};
use std::collections::HashMap;
use burn::prelude::Tensor;

/// A vectorized representation of a text document.
#[derive(Debug, Serialize)]
pub struct TextVector {
    pub text: String,
    pub normalized_text: String,
    #[serde(skip)]
    hash: blake3::Hash,
    pub strategy: DedupStrategySettings,
    #[serde(skip)]
    doc_vector: Option<DocumentVector>,
}

impl TextVector {
    pub fn new(text: String, strategy: &DedupStrategySettings) -> Self {
        let normalized_text = Self::normalize_text(strategy, &text);
        let hash = blake3::hash(text.as_bytes());
        
        Self {
            text,
            normalized_text,
            hash,
            strategy: strategy.clone(),
            doc_vector: None,
        }
    }

    pub fn update_strategy(&mut self, strategy: &DedupStrategySettings) {
        self.strategy = strategy.clone();
        self.normalized_text = Self::normalize_text(strategy, &self.text);
        self.hash = blake3::hash(self.normalized_text.as_bytes());
    }

    fn normalize_text(strategy: &DedupStrategySettings, text: &str) -> String {
        let mut result = text.to_string();

        result
    }

    pub fn prepare_semantic(&mut self, analyzer: &mut SemanticAnalyzer) {
        self.doc_vector = Some(analyzer.encode(&self.normalized_text, None));
    }

    pub fn is_similar(&self, other: &TextVector, threshold: f64) -> bool {
        self.calculate_similarity(other) >= threshold
    }

    pub fn calculate_similarity(&self, other: &TextVector) -> f64 {
        match self.strategy.similarity_method {
            SimilarityMethod::Exact => {
                if self.hash == other.hash {
                    1.0
                } else {
                    0.0
                }
            }
            SimilarityMethod::Semantic => {
                match (&self.doc_vector, &other.doc_vector) {
                    (Some(v1), Some(v2)) => {
                        let v1 = v1.vector.to_data();
                        let v2 = v2.vector.to_data();
                        calculate_cosine_similarity(&v1.into_raw().into_iter().collect::<Vec<f32>>(), &v2.into_raw().into_iter().collect::<Vec<f32>>()) as f64
                    }
                    _ => 0.0,
                }
            }
            SimilarityMethod::Levenshtein => {
                let distance = strsim::levenshtein(&self.normalized_text, &other.normalized_text);
                let max_len = self.normalized_text.len().max(other.normalized_text.len());
                if max_len == 0 {
                    1.0
                } else {
                    1.0 - (distance as f64 / max_len as f64)
                }
            }
        }
    }

    pub fn hash(&self) -> blake3::Hash {
        self.hash
    }

    pub fn content(&self) -> &str {
        &self.text
    }

    pub fn normalized_content(&self) -> &str {
        &self.normalized_text
    }
}

// Utility function for cosine similarity calculation
fn calculate_cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.len() != v2.len() || v1.is_empty() {
        return 0.0;
    }

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }

    dot_product / (norm1 * norm2)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocument {
    text: String,
    embedding: Option<Vec<f32>>,
}

impl TextDocument {
    pub fn new(text: String) -> Self {
        Self {
            text,
            embedding: None,
        }
    }

    pub fn update_vector(&mut self, analyzer: &SemanticAnalyzer) {
        let doc = analyzer.encode(&self.text, Some("en"));
        let vec_data = doc.vector.to_data();
        self.embedding = Some(vec_data.value.to_vec());
    }

    pub fn calculate_similarity(&self, other: &TextDocument, analyzer: &SemanticAnalyzer) -> f64 {
        match (&self.embedding, &other.embedding) {
            (Some(v1), Some(v2)) => {
                let v1_tensor = Tensor::from_data(Data::from(v1.as_slice()), &DefaultDevice::default());
                let v2_tensor = Tensor::from_data(Data::from(v2.as_slice()), &DefaultDevice::default());
                let dot_product = v1_tensor.clone().matmul(v2_tensor.clone().transpose());
                let norm1 = v1_tensor.clone().powf(v1_tensor).sum().sqrt();
                let norm2 = v2_tensor.clone().powf(v2_tensor).sum().sqrt();
                (dot_product.into_scalar() / (norm1.into_scalar() * norm2.into_scalar())) as f64
            }
            _ => analyzer.calculate_semantic_similarity(&self.text, &other.text, None, true)
        }
    }

    pub fn similarity(&self, other: &TextDocument, analyzer: &SemanticAnalyzer) -> f64 {
        self.calculate_similarity(other, analyzer)
    }
}

#[derive(Debug)]
pub struct VectorStore {
    analyzer: SemanticAnalyzer,
    strategy: DedupStrategySettings,
    documents: HashMap<usize, TextDocument>,
}

impl VectorStore {
    pub fn new(strategy: DedupStrategySettings) -> Self {
        Self {
            analyzer: SemanticAnalyzer::new(),
            strategy,
            documents: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, id: usize, text: String) {
        let mut doc = TextDocument::new(text);
        doc.update_vector(&self.analyzer);
        self.documents.insert(id, doc);
    }

    pub fn find_duplicates(&self, threshold: f64) -> Vec<Vec<usize>> {
        let mut groups = Vec::new();
        let mut processed = std::collections::HashSet::new();

        for (&id1, doc1) in &self.documents {
            if processed.contains(&id1) {
                continue;
            }

            let mut group = vec![id1];
            for (&id2, doc2) in &self.documents {
                if id1 == id2 || processed.contains(&id2) {
                    continue;
                }

                let similarity = doc1.calculate_similarity(doc2, &self.analyzer);
                if similarity >= threshold {
                    group.push(id2);
                    processed.insert(id2);
                }
            }

            if group.len() > 1 {
                groups.push(group);
            }
            processed.insert(id1);
        }

        groups
    }
}
