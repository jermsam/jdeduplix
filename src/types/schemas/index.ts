import { z } from 'zod';
import {ComparisonScope, SimilarityMethod, SplitStrategy} from '../enums';

// 1) Zod schemas for enums
//
export const SplitStrategySchema = z.nativeEnum(SplitStrategy);
export const ComparisonScopeSchema = z.nativeEnum(ComparisonScope);
export const SimilarityMethodSchema = z.nativeEnum(SimilarityMethod);

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