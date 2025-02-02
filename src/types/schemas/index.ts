import { z } from 'zod';
import {ComparisonScope, SimilarityMethod, SplitStrategy} from '../enums';

// 1) Zod schemas for enums
//
export const SplitStrategySchema = z.nativeEnum(SplitStrategy);
export const ComparisonScopeSchema = z.nativeEnum(ComparisonScope);
export const SimilarityMethodSchema = z.nativeEnum(SimilarityMethod);


/**
 * Weights for different aspects of similarity comparison.
 * All weights must sum to 1.0
 */
export const SimilarityWeightsSchema = z.object({
    /** Weight for term frequency in similarity calculation */
  frequency: z.number().min(0).max(1),
   /** Weight for term position/order in similarity calculation */
  position: z.number().min(0).max(1),
    /** Weight for surrounding context in similarity calculation */
  context: z.number().min(0).max(1)
}).refine((data) => {
  const sum = data.frequency + data.position + data.context;
  return Math.abs(sum - 1.0) < 0.000001; // Allow for floating point imprecision
}, {
  message: "Similarity weights must sum to 1.0"
});

//
// 2) Zod schema for DedupStrategy
//

export const DedupStrategySchema = z.object({
  case_sensitive: z.boolean(),
  ignore_whitespace: z.boolean(),
  ignore_punctuation: z.boolean(),
  normalize_unicode: z.boolean(),
  split_strategy: SplitStrategySchema,
  comparison_scope: ComparisonScopeSchema,
  min_length: z.number(),
  similarity_threshold: z.number(),
  similarity_method: SimilarityMethodSchema,
  use_parallel: z.boolean(),
  ignore_stopwords: z.boolean(),
  stemming: z.boolean(),
  ngram_size: z.number(),
  max_duplicate_count: z.number().optional(),
  language_detection: z.boolean(),
  encoding_normalization: z.boolean(),
  similarity_weighting: SimilarityWeightsSchema,
  adaptive_thresholding: z.boolean()
});

//
// 3) Zod schema for DuplicateGroup
//
export const DuplicateGroupSchema = z.object({
  original: z.string(),
  duplicates: z.array(z.string()),
  similarity: z.number(),
});

export const DuplicateStatsSchema = z.object({
  duplicate_groups: z.number(),
  total_items: z.number(),
  unique_items: z.number(),
});

export const DuplicateResultSchema = z.object({
  duplicate_groups: z.array(DuplicateGroupSchema),
  stats: DuplicateStatsSchema
})