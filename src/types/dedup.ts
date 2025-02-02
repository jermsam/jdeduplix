import {z} from 'zod';
import {DedupStrategySchema, DuplicateGroupSchema, DuplicateResultSchema, DuplicateStatsSchema} from './schemas';
import {SplitStrategy, ComparisonScope, SimilarityMethod} from './enums'


export type DedupStrategy = z.infer<typeof DedupStrategySchema>;
export type DuplicateGroup = z.infer<typeof DuplicateGroupSchema>;
export type DuplicateStat = z.infer<typeof DuplicateStatsSchema>;
export type DuplicateResult = z.infer<typeof DuplicateResultSchema>;


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
}

