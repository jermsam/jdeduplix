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

export enum FuzzyAlgorithm {
  DamerauLevenshtein = "DamerauLevenshtein",
  JaroWinkler = "JaroWinkler",
  Soundex = "Soundex",
  NGram = "NGram",
}

// Because "Exact" is just a string in JSON, you need to account for that:
export type SimilarityMethod =
  | "Exact"
  | "Semantic"
  | "Levenshtein"
  | { Fuzzy: FuzzyAlgorithm };
