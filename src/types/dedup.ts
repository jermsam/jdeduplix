export enum SplitStrategy {
  Characters = 'CHARACTERS',
  Words = 'WORDS',
  Sentences = 'SENTENCES',
  Paragraphs = 'PARAGRAPHS',
  WholeText = 'WHOLETEXT'
}

export enum ComparisonScope {
  WithinUnit = 'WITHINUNIT',
  AcrossUnits = 'ACROSSUNITS',
  Both = 'BOTH'
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
  split_strategy: SplitStrategy.Sentences,
  comparison_scope: ComparisonScope.Both,
  min_length: 0,
  similarity_threshold: 0.8
}
