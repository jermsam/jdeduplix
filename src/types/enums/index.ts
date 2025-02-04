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
  DamerauLevenshtein = "DamerauLevenshtein",  // Like Levenshtein but includes transpositions
  JaroWinkler = "JaroWinkler",         // Good for names and short strings, prioritizes prefix matches
  Soundex = "Soundex",             // Phonetic matching
  NGram = "NGram",               // N-gram based similarity
}

// Because "Exact" is just a string in JSON, you need to account for that:
export type SimilarityMethod = {
  type: "Exact" | "Semantic" | "Levenshtein" | "Fuzzy";
  algorithm?: FuzzyAlgorithm;  // Only required when type is "Fuzzy"
}