import { z } from 'zod';
import {ComparisonScope, SplitStrategy, FuzzyAlgorithm, } from '../enums';

// 1) Zod schemas for enums
// export const SimilarityAggregationSchema = z.nativeEnum(SimilarityAggregation);
export const SplitStrategySchema = z.nativeEnum(SplitStrategy);
// export const WeightingStrategySchema = z.nativeEnum(WeightingStrategy);
export const ComparisonScopeSchema = z.nativeEnum(ComparisonScope);
export const FuzzyAlgorithmSchema = z.nativeEnum(FuzzyAlgorithm);


export const SimilarityMethodSchema = z.object({
  type: z.enum(["Exact", "Semantic", "Levenshtein", "Fuzzy"]),
  algorithm: FuzzyAlgorithmSchema.optional()
}).refine((data) => {
  // Ensure algorithm is present when type is Fuzzy
  if (data.type === "Fuzzy") {
    return data.algorithm !== undefined;
  }
  return true;
}, {
  message: "FuzzyAlgorithm must be specified when type is Fuzzy"
});

/**
 * Configuration for similarity weights used in deduplication.
 * The weights determine how different aspects of text comparison contribute to the overall similarity score.
 * All weights must sum to 1.0 to ensure consistent scoring.
 * 
 * @example
 * // Equal importance to all aspects
 * { frequency: 0.33, position: 0.33, context: 0.34 }
 * 
 * // Emphasize semantic meaning with some consideration for word order
 * { frequency: 0.0, position: 0.3, context: 0.7 }
 */
// export const SimilarityWeightsSchema = z.object({
//   /**
//    * Weight for term frequency comparison (0.0 to 1.0)
//    * Higher values emphasize matching based on how often words appear, regardless of their order.
//    * Effective for:
//    * - Finding documents with similar vocabulary
//    * - Detecting keyword stuffing
//    * - Comparing technical documentation
//    */
//   frequency: z.number().min(0).max(1),

//   /**
//    * Weight for positional comparison (0.0 to 1.0)
//    * Higher values emphasize matching based on word order and structure.
//    * Effective for:
//    * - Finding near-exact duplicates
//    * - Comparing structured text (code, logs)
//    * - Detecting copied content with minor changes
//    */
//   position: z.number().min(0).max(1),

//   /**
//    * Weight for semantic context comparison (0.0 to 1.0)
//    * Higher values emphasize matching based on meaning rather than exact wording.
//    * Effective for:
//    * - Finding paraphrased content
//    * - Detecting AI-generated variations
//    * - Comparing content in different writing styles
//    */
//   context: z.number().min(0).max(1),
//   strategy: WeightingStrategySchema
// }).refine(
//   ({ frequency, position, context }) => {
//     const sum = frequency + position + context;
//     return Math.abs(sum - 1.0) < 0.001; // Allow for small floating point errors
//   },
//   {
//     message: "Weights must sum to 1.0"
//   }
// );



//
// 2) Zod schema for DedupStrategy
//
export const DedupStrategySchema = z.object({
  // similarity_aggregation: SimilarityAggregationSchema,
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
  language_detection: z.boolean(),
  max_duplicate_count: z.number().optional(),
  encoding_normalization: z.boolean(),
  // similarity_weighting: SimilarityWeightsSchema,
  adaptive_thresholding: z.boolean()
});

export const DedupPresetsSchema = z.object({
  name: z.string(),
  description: z.string(),
  settings: DedupStrategySchema
})

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