export enum SimilarityAggregation {
  /// Use only the first similarity value
  First = 'First',
  /// Take the average of all similarity values
  Mean = 'Mean',
  /// Take the maximum similarity value
  Max = 'Max',
  /// Take the minimum similarity value
  Min = 'Min',
}

export enum SplitStrategy {
  Characters = 'Characters',
  Words = 'Words',
  Sentences = 'Sentences',
  Paragraphs = 'Paragraphs',
  WholeText = 'WholeText'
}

export enum WeightingStrategy {
  Linear = 'Linear',
  Quadratic = 'Quadratic',
  Exponential = 'Exponential',
  Logarithmic = 'Logarithmic',
  WeightedMean = 'WeightedMean'
}

export enum ComparisonScope {
  Local = 'Local',
  Global = 'Global'
}

export enum FuzzyAlgorithm {
  DamerauLevenshtein = "DamerauLevenshtein",  // Like Levenshtein but includes transpositions
  JaroWinkler = "JaroWinkler",         // Good for names and short strings, prioritizes prefix matches
  Soundex = "Soundex",             // Phonetic matching
  NGram = "NGram",               // N-gram based similarity
}

