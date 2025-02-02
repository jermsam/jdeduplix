use serde::{Deserialize, Serialize};
use burn::backend::wgpu::WgpuDevice;
use burn::tensor::{backend::Backend, Tensor};
use burn::module::Module;
use burn::nn::{
    transformer::{TransformerEncoder, TransformerEncoderConfig},
    Linear, LayerNorm, Dropout, ReLU,
};
use once_cell::sync::Lazy;
use tokenizers::tokenizer::{Tokenizer, Encoding};
use tokenizers::models::wordpiece::WordPiece;
use tokenizers::normalizers::{Normalizer, BertNormalizer};
use tokenizers::pre_tokenizers::bert::BertPreTokenizer;
use stopwords::{Stopwords, Language};
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashMap;
use whatlang::detect;

type DefaultBackend = WgpuDevice;

const EMBEDDING_DIM: usize = 768; // BERT-base dimension
const MAX_SEQUENCE_LENGTH: usize = 512;
const VOCAB_SIZE: usize = 30522; // BERT vocabulary size

static TOKENIZER: Lazy<Tokenizer> = Lazy::new(|| {
    let mut tokenizer = Tokenizer::new(
        WordPiece::from_file("path/to/vocab.txt")
            .unwrap_or_else(|_| WordPiece::empty())
    );
    
    tokenizer.with_normalizer(Arc::new(BertNormalizer::new(true, true, true, true)));
    tokenizer.with_pre_tokenizer(Arc::new(BertPreTokenizer));
    
    tokenizer
});

static STOPWORDS: Lazy<HashMap<Language, Stopwords>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Language::English, Stopwords::english());
    map.insert(Language::French, Stopwords::french());
    map.insert(Language::Spanish, Stopwords::spanish());
    map.insert(Language::German, Stopwords::german());
    // Add more languages as needed
    map
});

/// Neural network for text encoding
#[derive(Module)]
pub struct TextEncoder {
    embedding: Linear<DefaultBackend>,
    position_embedding: Tensor<DefaultBackend, 2>,
    encoder: TransformerEncoder<DefaultBackend>,
    layer_norm: LayerNorm<DefaultBackend>,
    dropout: Dropout,
    pooler: Linear<DefaultBackend>,
}

impl TextEncoder {
    pub fn new() -> Self {
        let device = DefaultBackend::default();
        
        // Initialize embeddings
        let embedding = Linear::new(VOCAB_SIZE, EMBEDDING_DIM);
        let position_embedding = Tensor::zeros((MAX_SEQUENCE_LENGTH, EMBEDDING_DIM));
        
        // Initialize transformer
        let encoder_config = TransformerEncoderConfig::new(
            EMBEDDING_DIM,
            12, // num_heads (BERT-base)
            12, // num_layers (BERT-base)
            3072, // ff_dim (BERT-base)
        );
        let encoder = TransformerEncoder::new(&encoder_config);
        
        // Output layers
        let layer_norm = LayerNorm::new(EMBEDDING_DIM);
        let dropout = Dropout::new(0.1);
        let pooler = Linear::new(EMBEDDING_DIM, EMBEDDING_DIM);
        
        Self {
            embedding,
            position_embedding,
            encoder,
            layer_norm,
            dropout,
            pooler,
        }
    }
    
    fn forward(&self, input_ids: Tensor<DefaultBackend, 2>, attention_mask: Option<Tensor<DefaultBackend, 2>>) -> Tensor<DefaultBackend, 2> {
        // Embed tokens
        let embedded = self.embedding.forward(input_ids);
        
        // Add position embeddings
        let seq_length = embedded.shape()[0];
        let position_emb = self.position_embedding.slice((0..seq_length, ..));
        let hidden_states = embedded + position_emb;
        
        // Apply transformer layers
        let encoded = self.encoder.forward(hidden_states, attention_mask);
        
        // Pool and normalize
        let pooled = self.pooler.forward(encoded.mean_dim(0, true));
        let normalized = self.layer_norm.forward(pooled);
        
        self.dropout.forward(normalized)
    }
}

/// Semantic analyzer for text similarity using neural networks
pub struct SemanticAnalyzer {
    device: DefaultBackend,
    encoder: TextEncoder,
    stemmers: HashMap<String, Stemmer>,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let device = DefaultBackend::default();
        let encoder = TextEncoder::new();
        
        // Initialize stemmers for different languages
        let mut stemmers = HashMap::new();
        stemmers.insert("en".to_string(), Stemmer::create(Algorithm::English));
        stemmers.insert("fr".to_string(), Stemmer::create(Algorithm::French));
        stemmers.insert("es".to_string(), Stemmer::create(Algorithm::Spanish));
        stemmers.insert("de".to_string(), Stemmer::create(Algorithm::German));
        // Add more languages as needed
        
        Self {
            device,
            encoder,
            stemmers,
        }
    }

    /// Preprocess and tokenize text
    fn tokenize(&self, text: &str, lang: Option<&str>) -> Encoding {
        // Apply stemming if language is specified
        let processed_text = if let Some(lang_code) = lang {
            if let Some(stemmer) = self.stemmers.get(lang_code) {
                text.split_whitespace()
                    .map(|word| stemmer.stem(word).to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        };
        
        TOKENIZER.encode(processed_text, true).unwrap()
    }

    /// Create attention mask for the sequence
    fn create_attention_mask(&self, encoding: &Encoding) -> Tensor<DefaultBackend, 2> {
        let mask: Vec<f32> = encoding.get_ids()
            .iter()
            .map(|&id| if id == 0 { 0.0 } else { 1.0 })
            .collect();
            
        Tensor::from_vec(mask, (mask.len(), 1))
    }

    /// Encode text into a vector representation
    fn encode_text(&self, text: &str, lang: Option<&str>) -> Tensor<DefaultBackend, 2> {
        let encoding = self.tokenize(text, lang);
        let attention_mask = self.create_attention_mask(&encoding);
        
        let input_ids = Tensor::from_vec(
            encoding.get_ids().iter().map(|&id| id as f32).collect(),
            (encoding.get_ids().len(), 1),
        );
        
        self.encoder.forward(input_ids, Some(attention_mask))
    }

    /// Detect the language of the input text
    fn detect_language(&self, text: &str) -> Option<String> {
        detect(text).map(|info| info.lang().code().to_string())
    }

    /// Encode document for vector storage with automatic language detection
    pub fn encode(&self, text: &str, lang: Option<&str>) -> DocumentVector {
        let detected_lang = lang.map(String::from).or_else(|| self.detect_language(text));
        let encoding = self.encode_text(text, detected_lang.as_deref());
        let vector = encoding.to_vec();
        
        DocumentVector {
            text: text.to_string(),
            vector,
        }
    }

    /// Calculate semantic similarity between two texts with advanced features
    pub fn calculate_semantic_similarity(
        &self,
        text1: &str,
        text2: &str,
        lang: Option<&str>,
        use_stopwords: bool,
    ) -> f64 {
        // Remove stopwords if enabled
        let (processed1, processed2) = if use_stopwords {
            let stopwords = lang
                .and_then(|l| match l {
                    "en" => Some(Language::English),
                    "fr" => Some(Language::French),
                    "es" => Some(Language::Spanish),
                    "de" => Some(Language::German),
                    _ => None,
                })
                .and_then(|l| STOPWORDS.get(&l));
                
            let process = |text: &str, sw: Option<&Stopwords>| {
                if let Some(stopwords) = sw {
                    text.split_whitespace()
                        .filter(|word| !stopwords.is_stopword(word))
                        .collect::<Vec<_>>()
                        .join(" ")
                } else {
                    text.to_string()
                }
            };
            
            (process(text1, stopwords), process(text2, stopwords))
        } else {
            (text1.to_string(), text2.to_string())
        };
        
        // Encode both texts
        let encoding1 = self.encode_text(&processed1, lang);
        let encoding2 = self.encode_text(&processed2, lang);
        
        // Calculate cosine similarity with L2 normalization
        let normalized1 = encoding1 / encoding1.square().sum().sqrt();
        let normalized2 = encoding2 / encoding2.square().sum().sqrt();
        
        let similarity = normalized1.matmul(normalized2.transpose()).scalar();
        
        // Convert to f64 and ensure it's between 0 and 1
        (similarity as f64).max(0.0).min(1.0)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentVector {
    pub text: String,
    pub vector: Vec<f32>,
}
