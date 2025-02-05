//! Semantic analysis module using Burn, Tokenizers, and NLP utilities.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};
use std::f64::consts::E;

use tokenizers::models::wordpiece::WordPiece;
use tokenizers::normalizers::BertNormalizer;
use tokenizers::pre_tokenizers::bert::BertPreTokenizer;
use tokenizers::Tokenizer;

use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::nn::transformer::{TransformerEncoder, TransformerEncoderConfig};
use burn::tensor::Tensor;

use whatlang::{detect, Lang as Language};
use stop_words::{get, LANGUAGE as StopLanguage};
use rust_stemmers::{Algorithm, Stemmer};
use unicode_normalization::UnicodeNormalization;

use crate::state::{DedupStrategySettings, WeightingStrategy};

// Type aliases for convenience.
type DefaultBackend = NdArray;
type DefaultDevice = NdArrayDevice;

// Model constants.
const EMBEDDING_DIM: usize = 768; // BERT-base dimension
// const MAX_SEQUENCE_LENGTH: usize = 512;
// const VOCAB_SIZE: usize = 30522; // BERT vocabulary size

// ---------------------------------------------------------------------
// Static Resources
// ---------------------------------------------------------------------

/// Lazily initialized tokenizer with WordPiece, Bert normalizer and pre-tokenizer.
static TOKENIZER: OnceLock<Arc<Tokenizer>> = OnceLock::new();

/// Stopwords mapped by language.
static STOPWORDS: OnceLock<HashMap<Language, HashSet<String>>> = OnceLock::new();

/// A text encoder using a transformer model.
#[derive(Debug)]
pub struct TextEncoder {
    encoder: Arc<Mutex<TransformerEncoder<DefaultBackend>>>,
    tokenizer: Arc<Tokenizer>,
    device: DefaultDevice,
}

impl TextEncoder {
    /// Creates a new `TextEncoder` with a default transformer encoder and tokenizer.
    pub fn new() -> Self {
        let device = DefaultDevice::default();

        // Configure and initialize transformer encoder.
        let config = TransformerEncoderConfig::new(
            EMBEDDING_DIM,
            12,              // number of attention heads
            12,              // number of layers
            4 * EMBEDDING_DIM, // feedforward dimension
        );
        let encoder = Arc::new(Mutex::new(config.init(&device)));

        let tokenizer = TOKENIZER.get_or_init(|| {
            let wordpiece = WordPiece::builder()
                .unk_token("[UNK]".into())
                .max_input_chars_per_word(100)
                .continuing_subword_prefix("##".into())
                .build()
                .expect("Failed to build WordPiece model");

            let mut tokenizer = Tokenizer::new(wordpiece);
            // Configure the normalizer and pre-tokenizer for BERT.
            let normalizer = BertNormalizer::new(true, true, Some(true), true);
            tokenizer.with_normalizer(Some(normalizer));
            tokenizer.with_pre_tokenizer(Some(BertPreTokenizer));
            Arc::new(tokenizer)
        });

        Self {
            encoder,
            tokenizer: Arc::clone(tokenizer),
            device,
        }
    }

    /// Encodes text into a tensor.
    pub fn encode_text(&self, text: &str) -> Tensor<DefaultBackend, 2> {
        let encoding = self.tokenizer.encode(text, true)
            .expect("Failed to encode text");
    
        // Collect into a Vec<i64> directly.
        let input_ids: Vec<i64> = encoding.get_ids()
            .iter()
            .map(|&id| id as i64)
            .collect();
    
        // Get a lock on the encoder
        let encoder = self.encoder.lock().expect("Failed to lock encoder");
        
        // Pass the Vec directly.
        Tensor::<DefaultBackend, 2>::from_data(input_ids.as_slice(), &Default::default())
            .reshape([1, input_ids.len()])
    }

    /// Calculates the cosine similarity between two encoded texts.
    pub fn calculate_similarity(
        &self,
        encoding1: &Tensor<DefaultBackend, 2>,
        encoding2: &Tensor<DefaultBackend, 2>,
    ) -> f64 {
        // Get a lock on the encoder
        let _encoder = self.encoder.lock().expect("Failed to lock encoder");
        
        // Clone tensors before operations
        let e1 = encoding1.clone();
        let e2 = encoding2.clone();
        
        // Compute dot product on owned tensors
        let dot_product = e1.matmul(e2).sum();
    
        // Compute norms (L2 norm) on references since we can reuse the originals
        let norm1 = encoding1.clone().powf_scalar(2.0).sum().sqrt();
        let norm2 = encoding2.clone().powf_scalar(2.0).sum().sqrt();
    
        // Compute denominator
        let denominator = norm1 * norm2;
        let denominator_scalar = denominator.into_scalar();
        
        // Early return for zero denominator
        if denominator_scalar == 0.0 {
            return 0.0;
        }
    
        // Compute similarity
        let similarity = dot_product.into_scalar() / denominator_scalar;
        similarity as f64
    }
}

/// Represents the vectorized document along with metadata.
#[derive(Debug, Clone)]
pub struct DocumentVector {
    pub vector: Tensor<DefaultBackend, 2>,
    pub language: Option<Language>,
    pub token_count: usize,
}

/// A semantic analyzer that wraps a text encoder and additional NLP utilities.
#[derive(Debug)]
pub struct SemanticAnalyzer {
    encoder: TextEncoder,
    language_cache: HashMap<String, Language>,
}

impl SemanticAnalyzer {
    /// Creates a new `SemanticAnalyzer`.
    pub fn new() -> Self {
        Self {
            encoder: TextEncoder::new(),
            language_cache: HashMap::new(),
        }
    }

    /// Detects the language of the given text, using cache if available.
    pub fn detect_language(&mut self, text: &str) -> Option<Language> {
        // Check if we have the language in cache
        if let Some(cached_lang) = self.language_cache.get(text) {
            return Some(*cached_lang);
        }

        // If not in cache, detect the language
        let detected_lang = whatlang::detect(text)
            .and_then(|info| Language::try_from(info.lang()).ok());

        // Cache the result if language was detected
        if let Some(lang) = detected_lang {
            self.language_cache.insert(text.to_string(), lang);
        }

        detected_lang
    }

    /// Preprocesses text based on the provided settings.
    pub fn preprocess_text(
        &self,
        text: &str,
        lang: Option<Language>,
        settings: &DedupStrategySettings,
    ) -> String {
        let mut processed = if settings.case_sensitive.unwrap_or(false) {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        // Normalize whitespace if required.
        if !settings.ignore_whitespace.unwrap_or(false) {
            processed = processed.split_whitespace().collect::<Vec<_>>().join(" ");
        }

        // Remove punctuation if enabled.
        if settings.ignore_punctuation.unwrap_or(false) {
            processed = processed.chars().filter(|c| !c.is_ascii_punctuation()).collect();
        }

        // Normalize Unicode if requested.
        if settings.normalize_unicode.unwrap_or(false) {
            processed = processed.nfc().collect();
        }

        // Remove stopwords if enabled.
        if settings.ignore_stopwords.unwrap_or(false) {
            if let Some(lang) = lang {
                if let Some(stopwords) = STOPWORDS.get() {
                    processed = processed
                        .split_whitespace()
                        .filter(|word| stopwords.get(&lang) // Get the HashSet<String> for the given language
                        .map_or(false, |set| set.contains(*word)) )
                        .collect::<Vec<_>>()
                        .join(" ");
                }
            }
        }

        // Apply stemming if enabled.
        if settings.stemming.unwrap_or(false) {
            if let Some(lang) = lang {
                let stemmer = match lang {
                    Language::Eng => Stemmer::create(Algorithm::English),
                    Language::Fra => Stemmer::create(Algorithm::French),
                    Language::Spa => Stemmer::create(Algorithm::Spanish),
                    Language::Por => Stemmer::create(Algorithm::Portuguese),
                    Language::Ita => Stemmer::create(Algorithm::Italian),
                    Language::Deu => Stemmer::create(Algorithm::German),
                    Language::Rus => Stemmer::create(Algorithm::Russian),
                    _ => return processed, // Unsupported language; return unstemmed.
                };

                processed = processed
                    .split_whitespace()
                    .map(|word| stemmer.stem(word).to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
            }
        }

        // Create n-grams if requested.
        if let Some(n) = settings.ngram_size {
            if n > 1 {
                let words: Vec<&str> = processed.split_whitespace().collect();
              // Convert n to usize
                let ngrams: Vec<String> = (0..=words.len().saturating_sub(n))
                    .map(|i| words[i..i + n].join(" "))
                    .collect();
                processed = ngrams.join(" ");
            }
        }

        // Filter out short words if a minimum length is specified.
        if let Some(min_len) = settings.min_length {
            processed = processed
                .split_whitespace()
                .filter(|word| word.len() >= min_len)
                .collect::<Vec<_>>()
                .join(" ");
        }

        processed
    }

    /// Encodes text into a document vector.
    pub fn encode(&mut self, text: &str, settings: &DedupStrategySettings) -> DocumentVector {
        let lang = if settings.language_detection.unwrap_or(false) {
            self.detect_language(text)
        } else {
            None
        };

        let processed_text = self.preprocess_text(text, lang, settings);
        let vector = self.encoder.encode_text(&processed_text);
        let token_count = processed_text.split_whitespace().count();

        DocumentVector {
            vector,
            language: lang,
            token_count,
        }
    }

    /// Calculates the semantic similarity between two texts.
    pub fn calculate_semantic_similarity(
        &mut self,
        text1: &str,
        text2: &str,
        settings: &DedupStrategySettings,
    ) -> f64 {
        let doc1 = self.encode(text1, settings);
        let doc2 = self.encode(text2, settings);

        let language_penalty = if settings.language_detection.unwrap_or(false) {
            match (doc1.language, doc2.language) {
                (Some(l1), Some(l2)) if l1 != l2 => 0.8, // 20% penalty for different languages.
                _ => 1.0,
            }
        } else {
            1.0
        };

        let similarity = self
            .encoder
            .calculate_similarity(&doc1.vector, &doc2.vector);

        // Apply threshold if specified
        if let Some(threshold) = settings.threshold {
            if similarity < threshold {
                return 0.0;
            }
        }

        similarity * language_penalty
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self {
            encoder: TextEncoder::new(),
            language_cache: HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_similarity() {
        let mut analyzer = SemanticAnalyzer::new();

        let text1 = "The quick brown fox jumps over the lazy dog";
        let text2 = "A fast brown fox leaps above a sleepy canine";
        let default_settings = DedupStrategySettings::default();

        let similarity = analyzer.calculate_semantic_similarity(text1, text2, &default_settings);
        assert!(
            similarity > 0.7,
            "Similar sentences should have a high similarity score"
        );

        let text3 = "Completely unrelated text about programming computers";
        let similarity = analyzer.calculate_semantic_similarity(text1, text3, &default_settings);
        assert!(
            similarity < 0.5,
            "Different sentences should have a low similarity score"
        );
    }
}
