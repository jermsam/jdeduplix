export enum SplitStrategy {
  Characters = 'Characters',
  Words = 'Words',
  Sentences = 'Sentences',
  Paragraphs = 'Paragraphs',
  WholeText = 'WholeText'
}

export enum ComparisonScope {
  Local = 'Local',
  Global = 'Global'
}

export enum SimilarityMethod {
  Exact = 'Exact',
  Fuzzy = 'Fuzzy',
  Semantic = 'Semantic'
}

export interface DedupStrategy {
  case_sensitive: boolean
  ignore_whitespace: boolean
  ignore_punctuation: boolean
  normalize_unicode: boolean
  split_strategy: SplitStrategy
  comparison_scope: ComparisonScope
  min_length: number
  similarity_threshold: number
  similarity_method: SimilarityMethod
  use_parallel: boolean
}

export interface DuplicateGroup {
  original: string
  duplicates: string[]
  similarity: number
}

export const DEFAULT_STRATEGY: DedupStrategy = {
  case_sensitive: false,
  ignore_whitespace: true,
  ignore_punctuation: false,
  normalize_unicode: false,
  split_strategy: SplitStrategy.WholeText,
  comparison_scope: ComparisonScope.Global,
  min_length: 5,
  similarity_threshold: 0.15,
  similarity_method: SimilarityMethod.Semantic,
  use_parallel: false
}
