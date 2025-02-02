import {z} from 'zod';
import {DedupStrategySchema, DuplicateGroupSchema, DuplicateResultSchema, DuplicateStatsSchema, SimilarityWeightsSchema} from './schemas';
import {SplitStrategy, ComparisonScope, SimilarityMethod} from './enums'




export type DedupStrategy = z.infer<typeof DedupStrategySchema>;
export type DuplicateGroup = z.infer<typeof DuplicateGroupSchema>;
export type DuplicateStat = z.infer<typeof DuplicateStatsSchema>;
export type DuplicateResult = z.infer<typeof DuplicateResultSchema>;
export type SimilarityWeights = z.infer<typeof SimilarityWeightsSchema>;

export {SplitStrategy, ComparisonScope, SimilarityMethod};
export const DEFAULT_STRATEGY: DedupStrategy = {
  case_sensitive: false,
  ignore_whitespace: true,
  ignore_punctuation: false,
  normalize_unicode: false,
  split_strategy: SplitStrategy.Words,
  comparison_scope: ComparisonScope.Global,
  min_length: 10,
  similarity_threshold: 0.95,
  similarity_method: SimilarityMethod.Exact,
  use_parallel: true,
  ignore_stopwords: false,
  stemming: false,
  ngram_size: 3,
  max_duplicate_count: undefined,
  language_detection: false,
  encoding_normalization: true,
  similarity_weighting: {
    frequency: 0.5,  // Most important - how often terms appear
    position: 0.3,   // Second - where terms appear in the text
    context: 0.2     // Third - surrounding context of terms
  },
  adaptive_thresholding: false
}
