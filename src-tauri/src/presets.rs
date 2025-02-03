use crate::state::{DedupStrategyPreset, DedupStrategySettings, SimilarityMethod, SimilarityWeighting, SimilarityAggregation, WeightingStrategy};
use crate::config::DynamicConfig;
use std::collections::HashMap;

// TODO: Should come from Typedb Database or a p2p store
pub const DEDUP_PRESETS: &[DedupStrategyPreset] = &[
    // 🟢 Exact Match (No Aggregation Needed)
    DedupStrategyPreset {
        name: String::from("Exact Match"),
        description: String::from("Find identical text, including spacing and punctuation"),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  
            similarity_method: Some(String::from("Exact")),
            similarity_threshold: Some(0.95),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(false),
            normalize_unicode: Some(false),
            split_strategy: Some(String::from("Words")),
            comparison_scope: Some(String::from("Global")),
            min_length: Some(10),
            use_parallel: Some(true),
            ignore_stopwords: Some(false),
            stemming: Some(false),
            ngram_size: Some(3),
            language_detection: Some(false),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.4,
                position: 0.4,
                context: 0.2,
                strategy: WeightingStrategy::Linear, // No transformation needed
            }),
            adaptive_thresholding: Some(false),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🟢 Near Match (No Aggregation Needed)
    DedupStrategyPreset {
        name: String::from("Near Match"),
        description: String::from("Find text with minor formatting differences"),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  
            similarity_method: Some(String::from("Levenshtein")),
            similarity_threshold: Some(0.8),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            split_strategy: Some(String::from("Words")),
            comparison_scope: Some(String::from("Global")),
            min_length: Some(10),
            use_parallel: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(false),
            ngram_size: Some(3),
            language_detection: Some(false),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.5,
                position: 0.3,
                context: 0.2,
                strategy: WeightingStrategy::Linear, // No transformation needed
            }),
            adaptive_thresholding: Some(true),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🟢 Fuzzy Match (No Aggregation Needed)
    DedupStrategyPreset {
        name: String::from("Fuzzy Match"),
        description: String::from("Find text with typos and small variations"),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  
            similarity_method: Some(String::from("Levenshtein")),
            similarity_threshold: Some(0.7),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            split_strategy: Some(String::from("Sentences")),
            comparison_scope: Some(String::from("Global")),
            min_length: Some(5),
            use_parallel: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(true),
            ngram_size: Some(2),
            language_detection: Some(true),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.6,
                position: 0.2,
                context: 0.2,
                strategy: WeightingStrategy::Quadratic, // Penalizes small differences more
            }),
            adaptive_thresholding: Some(true),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🔵 Similar Ideas (Uses Semantic Similarity, Aggregation Needed)
    DedupStrategyPreset {
        name: String::from("Similar Ideas"),
        description: String::from("Find text expressing similar concepts"),
        settings: DedupStrategySettings {
            similarity_aggregation: Some(SimilarityAggregation::Mean),  
            similarity_method: Some(String::from("Semantic")),
            similarity_threshold: Some(0.6),
            case_sensitive: Some(false),
            ignore_whitespace: Some(true),
            ignore_punctuation: Some(true),
            normalize_unicode: Some(true),
            split_strategy: Some(String::from("Paragraphs")),
            comparison_scope: Some(String::from("Global")),
            min_length: Some(20),
            use_parallel: Some(true),
            ignore_stopwords: Some(true),
            stemming: Some(true),
            ngram_size: Some(3),
            language_detection: Some(true),
            encoding_normalization: Some(true),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.3,
                position: 0.3,
                context: 0.4,
                strategy: WeightingStrategy::WeightedMean, // Average embedding similarity scores
            }),
            adaptive_thresholding: Some(true),
            config: Some(DynamicConfig::default()),
        },
    },

    // 🟢 Strict Large Blocks (No Aggregation Needed)
    DedupStrategyPreset {
        name: String::from("Strict Large Blocks"),
        description: String::from("Looks for large duplicated character sequences (useful for code or logs)"),
        settings: DedupStrategySettings {
            similarity_aggregation: None,  
            similarity_method: Some(String::from("Exact")),
            similarity_threshold: Some(0.9),
            case_sensitive: Some(true),
            ignore_whitespace: Some(false),
            ignore_punctuation: Some(false),
            normalize_unicode: Some(false),
            split_strategy: Some(String::from("Characters")),
            comparison_scope: Some(String::from("Global")),
            min_length: Some(50),
            use_parallel: Some(true),
            ignore_stopwords: Some(false),
            stemming: Some(false),
            ngram_size: Some(5),
            language_detection: Some(false),
            encoding_normalization: Some(false),
            similarity_weighting: Some(SimilarityWeighting {
                frequency: 0.8,
                position: 0.1,
                context: 0.1,
                strategy: WeightingStrategy::Linear, // Direct matching, no weighting needed
            }),
            adaptive_thresholding: Some(false),
            config: Some(DynamicConfig::default()),
        },
    },
];
