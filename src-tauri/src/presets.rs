use crate::state::{DedupStrategyPreset, DedupStrategySettings, SimilarityMethod, SimilarityWeighting, SimilarityAggregation};
use crate::config::DynamicConfig;
use std::collections::HashMap;

// TODO: Should come from Typedb Database or a p2p store
/// Predefined presets for common deduplication scenarios
pub const DEDUP_PRESETS: &[DedupStrategyPreset] = &[
    // 🟢 Exact Match (No Aggregation Needed)
    DedupStrategyPreset {
        name: "Exact Match".to_string(),
        description: "Find identical text, including spacing and punctuation".to_string(),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  // Exact match is a single score (no need for aggregation)
            similarity_method: Some("Exact".to_string()),
            similarity_threshold: Some(0.95),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(false),
            normalize_unicode: Some(false),
            split_strategy: Some("Words".to_string()),
            comparison_scope: Some("Global".to_string()),
            min_length: Some(10),
            use_parallel: Some(true),
            ignore_stopwords: Some(false),
            stemming: Some(false),
            ngram_size: Some(3),
            language_detection: Some(false),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.4_f64,
                position: 0.4_f64,
                context: 0.2_f64,
            }),
            adaptive_thresholding: Some(false),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🟢 Near Match (No Aggregation Needed)
    DedupStrategyPreset {
        name: "Near Match".to_string(),
        description: "Find text with minor formatting differences".to_string(),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  // Levenshtein distance provides a single similarity score
            similarity_method: Some("Levenshtein".to_string()),
            similarity_threshold: Some(0.8),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            split_strategy: Some("Words".to_string()),
            comparison_scope: Some("Global".to_string()),
            min_length: Some(10),
            use_parallel: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(false),
            ngram_size: Some(3),
            language_detection: Some(false),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.5_f64,
                position: 0.3_f64,
                context: 0.2_f64,
            }),
            adaptive_thresholding: Some(true),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🟢 Fuzzy Match (No Aggregation Needed)
    DedupStrategyPreset {
        name: "Fuzzy Match".to_string(),
        description: "Find text with typos and small variations".to_string(),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  // Levenshtein distance still applies (single score)
            similarity_method: Some("Levenshtein".to_string()),
            similarity_threshold: Some(0.7),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            split_strategy: Some("Sentences".to_string()),
            comparison_scope: Some("Global".to_string()),
            min_length: Some(5),
            use_parallel: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(true),
            ngram_size: Some(2),
            language_detection: Some(true),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.6_f64,
                position: 0.2_f64,
                context: 0.2_f64,
            }),
            adaptive_thresholding: Some(true),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🔵 Similar Ideas (Uses Semantic Similarity, Aggregation Needed)
    DedupStrategyPreset {
        name: "Similar Ideas".to_string(),
        description: "Find text expressing similar concepts".to_string(),
        settings: DedupStrategySettings {
            similarity_aggregation: Some(SimilarityAggregation::Mean),  // Averaging multiple embeddings' similarity scores
            similarity_method: Some("Semantic".to_string()),
            similarity_threshold: Some(0.6),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            split_strategy: Some("Paragraphs".to_string()),
            comparison_scope: Some("Global".to_string()),
            min_length: Some(20),
            use_parallel: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(true),
            ngram_size: Some(3),
            language_detection: Some(true),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.3_f64,
                position: 0.3_f64,
                context: 0.4_f64,
            }),
            adaptive_thresholding: Some(true),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🟢 Strict Large Blocks (No Aggregation Needed)
    DedupStrategyPreset {
        name: "Strict Large Blocks".to_string(),
        description: "Looks for large duplicated character sequences (useful for code or logs)".to_string(),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  // Exact match means no aggregation needed
            similarity_method: Some("Exact".to_string()),
            similarity_threshold: Some(0.9),
            case_sensitive: Some(true),
            ignore_whitespace: Some(false),
            ignore_punctuation: Some(false),
            normalize_unicode: Some(false),
            split_strategy: Some("Characters".to_string()),
            comparison_scope: Some("Global".to_string()),
            min_length: Some(50),
            use_parallel: Some(true),
            ignore_stopwords: Some(false),
            stemming: Some(false),
            ngram_size: Some(5),
            language_detection: Some(false),
            encoding_normalization: Some(false),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.8_f64,
                position: 0.1_f64,
                context: 0.1_f64,
            }),
            adaptive_thresholding: Some(false),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🔵 Loose Paragraph Matching (Uses Semantic Similarity, Aggregation Needed)
    DedupStrategyPreset {
        name: "Loose Paragraph Matching".to_string(),
        description: "Groups paragraphs that share a high-level similarity or partial overlap".to_string(),
        settings: DedupStrategySettings {
            similarity_aggregation: Some(SimilarityAggregation::Max),  // Uses the most similar part of text
            similarity_method: Some("Semantic".to_string()),
            similarity_threshold: Some(0.65),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            split_strategy: Some("Paragraphs".to_string()),
            comparison_scope: Some("Global".to_string()),
            min_length: Some(20),
            use_parallel: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(true),
            ngram_size: Some(2),
            language_detection: Some(true),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.4_f64,
                position: 0.2_f64,
                context: 0.4_f64,
            }),
            adaptive_thresholding: Some(true),
            config: Some(DynamicConfig::default()),
        },
    },
];
