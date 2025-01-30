use crate::core::classifier::TextClassifier;
use crate::core::types::{SimilarityMethod, SplitStrategy, ComparisonScope, DedupStrategy};
use tauri::State;
use std::sync::{Arc, Mutex};

type Result<T> = std::result::Result<T, String>;

#[tauri::command]
pub fn find_duplicates(state: State<Arc<Mutex<TextClassifier>>>) -> Result<Vec<Vec<usize>>> {
    let classifier = state.lock().map_err(|e| e.to_string())?;
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
pub fn add_text(text: String, state: State<Arc<Mutex<TextClassifier>>>) -> Result<usize> {
    let mut classifier = state.lock().map_err(|e| e.to_string())?;
    Ok(classifier.add_text(text))
}

#[tauri::command]
pub fn get_text(idx: usize, state: State<Arc<Mutex<TextClassifier>>>) -> Result<Option<String>> {
    let classifier = state.lock().map_err(|e| e.to_string())?;
    Ok(classifier.get_text(idx).map(|s| s.to_string()))
}

#[tauri::command]
pub fn get_all_texts(state: State<Arc<Mutex<TextClassifier>>>) -> Result<Vec<String>> {
    let classifier = state.lock().map_err(|e| e.to_string())?;
    Ok(classifier.get_all_texts())
}

#[tauri::command]
pub fn clear(state: State<Arc<Mutex<TextClassifier>>>) -> Result<()> {
    let mut classifier = state.lock().map_err(|e| e.to_string())?;
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
    state: State<Arc<Mutex<TextClassifier>>>,
) -> Result<()> {
    let split_strategy = match split_strategy.as_str() {
        "Characters" => SplitStrategy::Characters,
        "Words" => SplitStrategy::Words,
        "Sentences" => SplitStrategy::Sentences,
        "Paragraphs" => SplitStrategy::Paragraphs,
        "WholeText" => SplitStrategy::WholeText,
        _ => return Err("Invalid split strategy".into()),
    };

    let comparison_scope = match comparison_scope.as_str() {
        "Local" => ComparisonScope::Local,
        "Global" => ComparisonScope::Global,
        _ => return Err("Invalid comparison scope".into()),
    };

    let similarity_method = match similarity_method.as_str() {
        "Exact" => SimilarityMethod::Exact,
        "Fuzzy" => SimilarityMethod::Fuzzy,
        _ => return Err("Invalid similarity method".into()),
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

    let mut classifier = state.lock().map_err(|e| e.to_string())?;
    classifier.update_strategy(strategy);
    Ok(())
}

#[tauri::command]
pub fn get_strategy(state: State<Arc<Mutex<TextClassifier>>>) -> Result<DedupStrategy> {
    let classifier = state.lock().map_err(|e| e.to_string())?;
    Ok(classifier.get_strategy().clone())
}
