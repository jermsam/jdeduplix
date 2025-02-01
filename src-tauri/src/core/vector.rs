// Vector indexing components

use blake3;

use serde::{Serialize, Deserialize};
use strsim::jaro_winkler;
use super::types::{SimilarityMethod, DedupStrategy};
use super::semantic::{SemanticAnalyzer, DocumentVector};
use std::collections::HashMap;


#[derive(Debug, Serialize)]
pub struct TextVector {
    pub text: String,
    pub normalized_text: String,
    #[serde(skip)]
    hash: blake3::Hash,
    pub strategy: DedupStrategy,
    #[serde(skip)]
    doc_vector: Option<DocumentVector>,
}

impl TextVector {
    pub fn new(text: String, strategy: &DedupStrategy) -> Self {
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

    pub fn update_strategy(&mut self, strategy: &DedupStrategy) {
        self.strategy = strategy.clone();
        self.normalized_text = Self::normalize_text(strategy, &self.text);
        self.hash = blake3::hash(self.normalized_text.as_bytes());
    }

    fn normalize_text(strategy: &DedupStrategy, text: &str) -> String {
        let mut result = text.to_string();

        result
    }

    pub fn prepare_semantic(&mut self, analyzer: &mut SemanticAnalyzer) {
        self.doc_vector = Some(analyzer.encode(&self.normalized_text));
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
                    (Some(v1), Some(v2)) => calculate_cosine_similarity(&v1.vector, &v2.vector) as f64,
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
        let doc = analyzer.encode(&self.text);
        self.embedding = Some(doc.vector);
    }

    pub fn similarity(&self, other: &TextDocument, analyzer: &SemanticAnalyzer) -> f64 {
        analyzer.calculate_similarity(&self.text, &other.text) as f64
    }
}

#[derive(Debug)]
pub struct VectorStore {
    analyzer: SemanticAnalyzer,
    strategy: DedupStrategy,
    documents: HashMap<usize, TextDocument>,
}

impl VectorStore {
    pub fn new(strategy: DedupStrategy) -> Self {
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

                let similarity = doc1.similarity(doc2, &self.analyzer);
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
