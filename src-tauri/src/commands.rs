use crate::core::classifier::TextClassifier;
use crate::core::types::{SimilarityMethod, SplitStrategy, ComparisonScope, DedupStrategy};
use crate::AppState;
use tauri::State;

type Result<T> = std::result::Result<T, String>;

#[tauri::command]
pub fn find_duplicates(state: State<AppState>) -> Result<Vec<Vec<usize>>> {
    let classifier = state.classifier.lock().map_err(|e| e.to_string())?;
    let dupes = classifier.find_duplicates();
    
    // Convert string groups to index groups
    let mut result = Vec::new();
    let all_texts = classifier.get_all_texts();
    
    for group in dupes {
        let mut index_group = Vec::new();
        for text in group {
            if let Some(pos) = all_texts.iter().position(|t| t == &text) {
                index_group.push(pos);
            }
        }
        if !index_group.is_empty() {
            result.push(index_group);
        }
    }
    
    Ok(result)
}

#[tauri::command]
pub fn add_text(text: String, state: State<AppState>) -> Result<usize> {
    let mut classifier = state.classifier.lock().map_err(|e| e.to_string())?;
    Ok(classifier.add_text(text))
}

#[tauri::command]
pub fn get_text(idx: usize, state: State<AppState>) -> Result<Option<String>> {
    let classifier = state.classifier.lock().map_err(|e| e.to_string())?;
    Ok(classifier.get_text(idx).map(|s| s.to_string()))
}

#[tauri::command]
pub fn get_all_texts(state: State<AppState>) -> Result<Vec<String>> {
    let classifier = state.classifier.lock().map_err(|e| e.to_string())?;
    Ok(classifier.get_all_texts())
}

#[tauri::command]
pub fn clear(state: State<AppState>) -> Result<()> {
    let mut classifier = state.classifier.lock().map_err(|e| e.to_string())?;
    classifier.clear();
    Ok(())
}

#[tauri::command]
pub fn update_strategy(
    case_sensitive: bool,
    ignore_whitespace: bool,
    ignore_punctuation: bool,
    normalize_unicode: bool,
    split_strategy: String,
    comparison_scope: String,
    min_length: usize,
    similarity_threshold: f64,
    similarity_method: String,
    use_parallel: bool,
    state: State<AppState>,
) -> Result<()> {
    let split_strategy = match split_strategy.as_str() {
        "Characters" => SplitStrategy::Characters,
        "Words" => SplitStrategy::Words,
        "Sentences" => SplitStrategy::Sentences,
        "Paragraphs" => SplitStrategy::Paragraphs,
        "WholeText" => SplitStrategy::WholeText,
        _ => return Err("Invalid split strategy".to_string()),
    };

    let comparison_scope = match comparison_scope.as_str() {
        "Local" => ComparisonScope::Local,
        "Global" => ComparisonScope::Global,
        _ => return Err("Invalid comparison scope".to_string()),
    };

    let similarity_method = match similarity_method.as_str() {
        "Exact" => SimilarityMethod::Exact,
        "Fuzzy" => SimilarityMethod::Fuzzy,
        "Semantic" => SimilarityMethod::Semantic,
        _ => return Err("Invalid similarity method".to_string()),
    };

    let strategy = DedupStrategy {
        case_sensitive,
        ignore_whitespace,
        ignore_punctuation,
        normalize_unicode,
        split_strategy,
        comparison_scope,
        min_length,
        similarity_threshold,
        similarity_method,
        use_parallel,
    };

    let mut classifier = state.classifier.lock().map_err(|e| e.to_string())?;
    classifier.update_strategy(strategy);
    Ok(())
}

#[tauri::command]
pub fn get_strategy(state: State<AppState>) -> Result<DedupStrategy> {
    let classifier = state.classifier.lock().map_err(|e| e.to_string())?;
    Ok(classifier.get_strategy().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to test the functionality directly without Tauri's State
    fn test_with_classifier<F>(f: F)
    where
        F: FnOnce(&mut TextClassifier),
    {
        let mut classifier = TextClassifier::default();
        f(&mut classifier);
    }

    #[test]
    fn test_add_and_get_text() {
        test_with_classifier(|classifier| {
            let text = "Hello, World!".to_string();
            
            // Test add_text
            let idx = classifier.add_text(text.clone());
            assert_eq!(idx, 0);
            
            // Test get_text
            let retrieved = classifier.get_text(idx);
            assert_eq!(retrieved, Some(text.as_str()));
            
            // Test get_all_texts
            let all_texts = classifier.get_all_texts();
            assert_eq!(all_texts, vec![text]);
        });
    }

    #[test]
    fn test_find_duplicates() {
        test_with_classifier(|classifier| {
            // Add some test texts with more similar wording
            classifier.add_text("The cat quickly jumped over the fence".to_string());
            classifier.add_text("The cat swiftly jumped over the fence".to_string());
            classifier.add_text("My dog sleeps on the couch".to_string());
            
            // Update to semantic strategy
            classifier.update_strategy(DedupStrategy {
                case_sensitive: false,
                ignore_whitespace: true,
                ignore_punctuation: true,
                normalize_unicode: true,
                split_strategy: SplitStrategy::WholeText,
                comparison_scope: ComparisonScope::Global,
                min_length: 5,
                similarity_threshold: 0.15, // Moderate threshold for similar sentences
                similarity_method: SimilarityMethod::Semantic,
                use_parallel: false,
            });
            
            // Find duplicates
            let vectors = classifier.get_vectors();
            println!("Similarity between cat and feline: {}", vectors[0].calculate_similarity(&vectors[1]));
            println!("Similarity between cat and dog: {}", vectors[0].calculate_similarity(&vectors[2]));
            println!("Similarity between feline and dog: {}", vectors[1].calculate_similarity(&vectors[2]));
            
            let duplicates = classifier.find_duplicates();
            assert_eq!(duplicates.len(), 1); // Should find one group of duplicates
            
            // The group should contain exactly the cat/feline sentences
            let group = &duplicates[0];
            assert!(group.contains(&"The cat quickly jumped over the fence".to_string()));
            assert!(group.contains(&"The cat swiftly jumped over the fence".to_string()));
            assert!(!group.contains(&"My dog sleeps on the couch".to_string()));
        });
    }

    #[test]
    fn test_update_and_get_strategy() {
        test_with_classifier(|classifier| {
            let strategy = DedupStrategy {
                case_sensitive: true,
                ignore_whitespace: false,
                ignore_punctuation: false,
                normalize_unicode: true,
                split_strategy: SplitStrategy::Words,
                comparison_scope: ComparisonScope::Local,
                min_length: 3,
                similarity_threshold: 0.8,
                similarity_method: SimilarityMethod::Fuzzy,
                use_parallel: true,
            };
            
            // Update strategy
            classifier.update_strategy(strategy.clone());
            
            // Get and verify strategy
            let retrieved_strategy = classifier.get_strategy();
            assert_eq!(retrieved_strategy.case_sensitive, true);
            assert_eq!(retrieved_strategy.ignore_whitespace, false);
            assert_eq!(retrieved_strategy.ignore_punctuation, false);
            assert_eq!(retrieved_strategy.normalize_unicode, true);
            assert_eq!(retrieved_strategy.split_strategy, SplitStrategy::Words);
            assert_eq!(retrieved_strategy.comparison_scope, ComparisonScope::Local);
            assert_eq!(retrieved_strategy.min_length, 3);
            assert_eq!(retrieved_strategy.similarity_threshold, 0.8);
            assert_eq!(retrieved_strategy.similarity_method, SimilarityMethod::Fuzzy);
            assert_eq!(retrieved_strategy.use_parallel, true);
        });
    }

    #[test]
    fn test_clear() {
        test_with_classifier(|classifier| {
            // Add some texts
            classifier.add_text("Text 1".to_string());
            classifier.add_text("Text 2".to_string());
            
            // Verify texts were added
            assert_eq!(classifier.get_all_texts().len(), 2);
            
            // Clear texts
            classifier.clear();
            
            // Verify texts were cleared
            assert_eq!(classifier.get_all_texts().len(), 0);
        });
    }
}
