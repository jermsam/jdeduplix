import {z} from 'zod';
import {DedupStrategySchema, DuplicateGroupSchema, DuplicateResultSchema, DuplicateStatsSchema, SimilarityWeightsSchema, DedupPresetsSchema} from './schemas';
import {SplitStrategy, ComparisonScope, FuzzyAlgorithm} from './enums'

/**
 * Types for deduplication
 */

export type DedupStrategy = z.infer<typeof DedupStrategySchema>;
export type DuplicateGroup = z.infer<typeof DuplicateGroupSchema>;
export type DuplicateStat = z.infer<typeof DuplicateStatsSchema>;
export type DuplicateResult = z.infer<typeof DuplicateResultSchema>;
export type SimilarityWeights = z.infer<typeof SimilarityWeightsSchema>;
export type DedupPreset = z.infer<typeof DedupPresetsSchema>;



export const DEDUP_PRESETS: DedupPreset[] = [
  {
    name: 'Exact Match',
    description: 'Find identical text, including spacing and punctuation',
    settings: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: false,
      normalize_unicode: false,
      split_strategy: SplitStrategy.Words,
      comparison_scope: ComparisonScope.Global,
      min_length: 10,
      similarity_threshold: 0.95,
      similarity_method: { type: "Exact" },
      use_parallel: true,
      ignore_stopwords: false,
      stemming: false,
      ngram_size: 3,
      language_detection: false,
      encoding_normalization: true,
      similarity_weighting: {
        frequency: 0.4,
        position: 0.4,
        context: 0.2
      },
      adaptive_thresholding: false
    },
  },
  {
    name: 'Near Match',
    description: 'Find text with minor formatting differences',
    settings: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: false,
      normalize_unicode: false,
      split_strategy: SplitStrategy.Words,
      comparison_scope: ComparisonScope.Global,
      min_length: 10,
      similarity_threshold: 0.8,
      similarity_method: { type: "Levenshtein" },
      use_parallel: true,
      ignore_stopwords: false,
      stemming: false,
      ngram_size: 3,
      language_detection: false,
      encoding_normalization: true,
      similarity_weighting: {
        frequency: 0.4,
        position: 0.4,
        context: 0.2
      },
      adaptive_thresholding: false
    },
  },
  {
    name: 'Fuzzy Match',
    description: 'Find text with typos and small variations',
    settings: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: false,
      normalize_unicode: false,
      split_strategy: SplitStrategy.Sentences,
      comparison_scope: ComparisonScope.Global,
      min_length: 5,
      similarity_threshold: 0.7,
      similarity_method: { 
        type: "Fuzzy", 
        algorithm: FuzzyAlgorithm.DamerauLevenshtein 
      },
      use_parallel: true,
      ignore_stopwords: false,
      stemming: false,
      ngram_size: 3,
      language_detection: false,
      encoding_normalization: true,
      similarity_weighting: {
        frequency: 0.4,
        position: 0.4,
        context: 0.2
      },
      adaptive_thresholding: false
    },
  },
  {
    name: 'Similar Ideas',
    description: 'Find text expressing similar concepts',
    settings: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: true,
      split_strategy: SplitStrategy.Paragraphs,
      comparison_scope: ComparisonScope.Global,
      min_length: 10,
      similarity_threshold: 0.8,
      similarity_method: { type: "Semantic" },
      use_parallel: true,
      ignore_stopwords: false,
      stemming: false,
      ngram_size: 3,
      language_detection: false,
      encoding_normalization: true,
      similarity_weighting: {
        frequency: 0.4,
        position: 0.4,
        context: 0.2
      },
      adaptive_thresholding: false
    },
  },
  {
    name: 'Strict Large Blocks',
    description: 'Looks for large duplicated character sequences (useful for code or logs)',
    settings: {
      case_sensitive: false,
      ignore_whitespace: false,
      ignore_punctuation: false,
      normalize_unicode: false,
      split_strategy: SplitStrategy.Characters,
      comparison_scope: ComparisonScope.Global,
      min_length: 50,
      similarity_threshold: 0.9,
      similarity_method: { type: "Exact" },
      use_parallel: true,
      ignore_stopwords: false,
      stemming: false,
      ngram_size: 3,
      language_detection: false,
      encoding_normalization: true,
      similarity_weighting: {
        frequency: 0.4,
        position: 0.4,
        context: 0.2
      },
      adaptive_thresholding: false
    },
  },
  {
    name: 'Loose Paragraph Matching',
    description: 'Groups paragraphs that share a high-level similarity or partial overlap',
    settings: {
      case_sensitive: false,
      ignore_whitespace: true,
      ignore_punctuation: true,
      normalize_unicode: false,
      split_strategy: SplitStrategy.Paragraphs,
      comparison_scope: ComparisonScope.Global,
      min_length: 20,
      similarity_threshold: 0.65,
      similarity_method: { 
        type: "Fuzzy", 
        algorithm: FuzzyAlgorithm.JaroWinkler 
      },
      use_parallel: true,
      ignore_stopwords: false,
      stemming: false,
      ngram_size: 3,
      language_detection: false,
      encoding_normalization: true,
      similarity_weighting: {
        frequency: 0.4,
        position: 0.4,
        context: 0.2
      },
      adaptive_thresholding: false
    },
  },
];

const get_default_strategy_by_preset = (name: string): DedupStrategy => {
  const preset = DEDUP_PRESETS.find((preset) => preset.name === name);
  if (!preset || !preset.settings) {
    throw new Error(`No preset found with name: ${name}`);
  }
  return preset.settings;
}

export const DEFAULT_STRATEGY: DedupStrategy = get_default_strategy_by_preset('Exact Match');
