use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVector {
    pub text: String,
    pub vector: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn encode(&self, text: &str) -> DocumentVector {
        // For now, return a simple vector based on character counts
        let mut vector = vec![0.0; 128];
        for c in text.chars() {
            let index = (c as usize) % 128;
            vector[index] += 1.0;
        }
        
        DocumentVector {
            text: text.to_string(),
            vector,
        }
    }

    pub fn calculate_similarity(&self, text1: &str, text2: &str) -> f32 {
        let doc1 = self.encode(text1);
        let doc2 = self.encode(text2);
        
        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;
        
        for (v1, v2) in doc1.vector.iter().zip(doc2.vector.iter()) {
            dot_product += v1 * v2;
            norm1 += v1 * v1;
            norm2 += v2 * v2;
        }
        
        let norm = (norm1.sqrt() * norm2.sqrt()).max(1e-10);
        (dot_product / norm).max(0.0).min(1.0)
    }
}
